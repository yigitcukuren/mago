use std::collections::BTreeMap;
use std::rc::Rc;

use ahash::HashMap;
use ahash::HashSet;
use indexmap::IndexMap;

use mago_algebra::clause::Clause;
use mago_codex::ttype::combine_optional_union_types;
use mago_codex::ttype::combine_union_types;
use mago_codex::ttype::comparator::union_comparator::can_expression_types_be_identical;
use mago_codex::ttype::get_mixed_any;
use mago_codex::ttype::union::TUnion;
use mago_interner::ThreadedInterner;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::ast::Binary;
use mago_syntax::ast::BinaryOperator;
use mago_syntax::ast::DirectVariable;
use mago_syntax::ast::Expression;
use mago_syntax::ast::Statement;
use mago_syntax::ast::Switch;
use mago_syntax::ast::SwitchCase;
use mago_syntax::ast::SwitchExpressionCase;
use mago_syntax::ast::Variable;

use crate::analyzable::Analyzable;
use crate::artifacts::AnalysisArtifacts;
use crate::code::Code;
use crate::context::Context;
use crate::context::block::BlockContext;
use crate::context::block::BreakContext;
use crate::context::scope::case_scope::CaseScope;
use crate::context::scope::control_action::BreakType;
use crate::context::scope::control_action::ControlAction;
use crate::error::AnalysisError;
use crate::formula::get_formula;
use crate::formula::negate_or_synthesize;
use crate::reconciler::ReconcilationContext;
use crate::reconciler::reconcile_keyed_types;
use crate::statement::analyze_statements;
use crate::utils::expression::get_expression_id;
use crate::utils::misc::check_for_paradox;

impl Analyzable for Switch {
    fn analyze<'a>(
        &self,
        context: &mut Context<'a>,
        block_context: &mut BlockContext<'a>,
        artifacts: &mut AnalysisArtifacts,
    ) -> Result<(), AnalysisError> {
        SwitchAnalyzer::new(context, block_context, artifacts).analyze(self)
    }
}

#[derive(Debug)]
struct SwitchAnalyzer<'a, 'b> {
    context: &'b mut Context<'a>,
    block_context: &'b mut BlockContext<'a>,
    artifacts: &'b mut AnalysisArtifacts,
    new_locals: Option<BTreeMap<String, Rc<TUnion>>>,
    redefined_variables: Option<HashMap<String, Rc<TUnion>>>,
    possibly_redefined_variables: Option<BTreeMap<String, TUnion>>,
    leftover_statements: Vec<Statement>,
    leftover_case_equality_expression: Option<Expression>,
    negated_clauses: Vec<Clause>,
    new_assigned_variable_ids: HashMap<String, usize>,
    last_case_exit_type: ControlAction,
    case_exit_types: HashMap<usize, ControlAction>,
    case_actions: HashMap<usize, HashSet<ControlAction>>,
}

impl<'a, 'b> SwitchAnalyzer<'a, 'b> {
    const SYNTHETIC_SWITCH_VAR_PREFIX: &'static str = "$-tmp-switch-";

    pub fn new(
        context: &'b mut Context<'a>,
        block_context: &'b mut BlockContext<'a>,
        artifacts: &'b mut AnalysisArtifacts,
    ) -> Self {
        Self {
            context,
            block_context,
            artifacts,
            new_locals: None,
            redefined_variables: None,
            possibly_redefined_variables: None,
            leftover_statements: vec![],
            leftover_case_equality_expression: None,
            negated_clauses: vec![],
            new_assigned_variable_ids: HashMap::default(),
            last_case_exit_type: ControlAction::Break,
            case_exit_types: HashMap::default(),
            case_actions: HashMap::default(),
        }
    }

    pub fn analyze(mut self, switch: &Switch) -> Result<(), AnalysisError> {
        let was_inside_conditional = self.block_context.inside_conditional;
        self.block_context.inside_conditional = true;
        switch.expression.analyze(self.context, self.block_context, self.artifacts)?;
        self.block_context.inside_conditional = was_inside_conditional;

        let switch_var_id = if let Some(switch_var_id) = get_expression_id(
            &switch.expression,
            self.block_context.scope.get_class_like_name(),
            self.context.resolved_names,
            self.context.interner,
            Some(self.context.codebase),
        ) {
            switch_var_id
        } else {
            let switch_var_id = switch.expression.span();
            let switch_var_id = format!("{}{}", Self::SYNTHETIC_SWITCH_VAR_PREFIX, switch_var_id.start.offset);

            self.block_context.locals.insert(
                switch_var_id.clone(),
                self.artifacts
                    .get_rc_expression_type(&switch.expression)
                    .cloned()
                    .unwrap_or_else(|| Rc::new(get_mixed_any())),
            );

            switch_var_id
        };

        let original_context = self.block_context.clone();

        let has_default = switch.body.has_default_case();
        let cases = switch.body.cases();
        let indexed_cases = cases.iter().enumerate().collect::<IndexMap<_, _>>();

        for (i, case) in indexed_cases.iter().rev() {
            self.update_case_exit_map(case, *i);
        }

        let mut all_options_returned = true;

        let mut condition_is_synthetic = false;

        let synthetic_switch_condition = if switch_var_id.starts_with(Self::SYNTHETIC_SWITCH_VAR_PREFIX) {
            condition_is_synthetic = true;

            Some(new_synthetic_variable(self.context.interner, &switch_var_id))
        } else {
            None
        };

        let mut previous_empty_cases = vec![];

        for (i, case) in indexed_cases {
            let is_last = i == cases.len() - 1;
            let case_exit_type = &self.case_exit_types[&i];
            if case_exit_type != &ControlAction::Return {
                all_options_returned = false;
            }

            if let SwitchCase::Expression(switch_case) = case
                && case.statements().is_empty()
                && !is_last
            {
                previous_empty_cases.push(switch_case);
                continue;
            };

            self.analyze_case(
                synthetic_switch_condition.as_ref().unwrap_or(&switch.expression),
                condition_is_synthetic,
                &switch_var_id,
                case,
                &previous_empty_cases,
                &original_context,
                is_last,
                i,
            )?;

            previous_empty_cases = vec![];
        }

        let mut possibly_redefined_vars = self.possibly_redefined_variables.unwrap_or_default();
        if let Some(new_locals) = self.new_locals {
            possibly_redefined_vars.retain(|k, _| !new_locals.contains_key(k));
            self.block_context.locals.extend(new_locals);
        }

        if let Some(redefined_vars) = self.redefined_variables {
            possibly_redefined_vars.retain(|k, _| !redefined_vars.contains_key(k));
            self.block_context.locals.extend(redefined_vars.clone());
        }

        for (var_id, var_type) in possibly_redefined_vars {
            if let Some(context_type) = self.block_context.locals.get(&var_id).cloned() {
                self.block_context.locals.insert(
                    var_id.clone(),
                    Rc::new(combine_union_types(
                        &var_type,
                        &context_type,
                        self.context.codebase,
                        self.context.interner,
                        false,
                    )),
                );
            }
        }

        self.artifacts.fully_matched_switch_offsets.insert(switch.start_position().offset);
        self.block_context.assigned_variable_ids.extend(self.new_assigned_variable_ids);
        self.block_context.has_returned = all_options_returned && has_default;

        Ok(())
    }

    pub(crate) fn analyze_case(
        &mut self,
        switch_condition: &Expression,
        condition_is_synthetic: bool,
        switch_var_id: &String,
        switch_case: &SwitchCase,
        previous_empty_cases: &Vec<&SwitchExpressionCase>,
        original_block_context: &BlockContext<'a>,
        is_last: bool,
        case_index: usize,
    ) -> Result<(), AnalysisError> {
        let case_actions = &self.case_actions[&case_index];
        let case_exit_type = self.case_exit_types[&case_index];

        let has_ending_statements = case_actions.len() == 1 && case_actions.contains(&ControlAction::End);
        let has_leaving_statements =
            has_ending_statements || (!case_actions.is_empty() && !case_actions.contains(&ControlAction::None));

        let mut case_block_context = original_block_context.clone();

        let mut old_expression_types = self.artifacts.expression_types.clone();
        let mut case_equality_expression = None;

        if let Some(case_condition) = switch_case.expression() {
            case_condition.analyze(self.context, self.block_context, self.artifacts)?;

            if condition_is_synthetic {
                self.artifacts.set_expression_type(
                    switch_condition,
                    if let Some(t) = self.block_context.locals.get(switch_var_id) {
                        (**t).clone()
                    } else {
                        get_mixed_any()
                    },
                );
            }

            let switch_condition_type =
                self.artifacts.get_rc_expression_type(switch_condition).cloned().unwrap_or(Rc::new(get_mixed_any()));

            if let Some(condition_type) = self.artifacts.get_rc_expression_type(&case_condition)
                && !can_expression_types_be_identical(
                    self.context.codebase,
                    self.context.interner,
                    switch_condition_type.as_ref(),
                    condition_type,
                    false,
                    true,
                )
            {
                self.context.collector.report_with_code(
                    Code::PARADOXICAL_CONDITION,
                    Issue::error("Switch case condition is not compatible with switch condition".to_string())
                        .with_annotation(
                            Annotation::primary(case_condition.span())
                                .with_message("this case condition is not compatible with the switch condition"),
                        ),
                );
            }

            case_equality_expression = Some(if !previous_empty_cases.is_empty() {
                for previous_empty_case in previous_empty_cases {
                    previous_empty_case.expression.analyze(self.context, self.block_context, self.artifacts)?;
                }

                new_synthetic_disjunctive_equality(
                    switch_condition,
                    case_condition,
                    previous_empty_cases.clone().into_iter().map(|c| c.expression.as_ref()).collect::<Vec<_>>(),
                )
            } else if switch_condition_type.is_true() {
                case_condition.clone()
            } else {
                new_synthetic_equals(switch_condition, case_condition)
            });
        }

        let mut case_stmts = self.leftover_statements.clone();

        case_stmts.extend(switch_case.statements().iter().cloned());

        if !has_leaving_statements && !is_last {
            // this is safe for non-defaults, and defaults are always last
            let case_equality_expression = case_equality_expression.unwrap();

            self.leftover_case_equality_expression =
                Some(if let Some(leftover_case_equality_expr) = &self.leftover_case_equality_expression {
                    new_synthetic_or(leftover_case_equality_expr, &case_equality_expression)
                } else {
                    case_equality_expression
                });

            self.leftover_statements = case_stmts;

            self.artifacts.expression_types = old_expression_types;

            return Ok(());
        }

        if let Some(leftover_case_equality_expr) = &self.leftover_case_equality_expression {
            case_equality_expression = Some(new_synthetic_or(
                leftover_case_equality_expr,
                &case_equality_expression.unwrap_or_else(|| new_synthetic_equals(switch_condition, switch_condition)),
            ));
        }

        case_block_context.break_types.push(BreakContext::Switch);

        self.leftover_statements = vec![];
        self.leftover_case_equality_expression = None;

        let assertion_context = self.context.get_assertion_context_from_block(self.block_context);

        let case_clauses = if let Some(case_equality_expr) = &case_equality_expression {
            // todo: complexity!!
            get_formula(switch_case.span(), switch_case.span(), case_equality_expr, assertion_context, self.artifacts)
                .unwrap_or_default()
        } else {
            vec![]
        };

        let mut entry_clauses = if !self.negated_clauses.is_empty() && self.negated_clauses.len() < 50 {
            let mut c = original_block_context.clauses.iter().map(|v| &**v).collect::<Vec<_>>();
            c.extend(self.negated_clauses.iter());

            mago_algebra::saturate_clauses(c)
        } else {
            original_block_context.clauses.iter().map(|v| (**v).clone()).collect::<Vec<_>>()
        };

        case_block_context.clauses = if !case_clauses.is_empty() {
            if let Some(case_condition) = switch_case.expression() {
                check_for_paradox(
                    self.context.interner,
                    &mut self.context.collector,
                    &entry_clauses.iter().map(|v| Rc::new(v.clone())).collect::<Vec<_>>(),
                    &case_clauses,
                    case_condition.span(),
                );

                entry_clauses.extend(case_clauses.clone());

                if entry_clauses.len() < 50 {
                    mago_algebra::saturate_clauses(entry_clauses.iter())
                } else {
                    entry_clauses
                }
            } else {
                entry_clauses
            }
        } else {
            entry_clauses
        }
        .into_iter()
        .map(|v| Rc::new(v.clone()))
        .collect();

        let (reconcilable_if_types, _) = mago_algebra::find_satisfying_assignments(
            &case_block_context.clauses.iter().map(|v| v.as_ref().clone()).collect::<Vec<_>>(),
            None,
            &mut HashSet::default(),
        );

        if !reconcilable_if_types.is_empty() {
            let mut changed_var_ids = HashSet::default();

            reconcile_keyed_types(
                &mut ReconcilationContext::new(
                    self.context.interner,
                    self.context.codebase,
                    &mut self.context.collector,
                ),
                &reconcilable_if_types,
                BTreeMap::new(),
                &mut case_block_context,
                &mut changed_var_ids,
                &if !switch_case.is_default() {
                    HashSet::from_iter([switch_var_id.clone()])
                } else {
                    HashSet::default()
                },
                &switch_case.span(),
                true,
                false,
            );

            if !changed_var_ids.is_empty() {
                case_block_context.clauses =
                    BlockContext::remove_reconciled_clause_refs(&case_block_context.clauses, &changed_var_ids).0;
            }
        }

        if !case_clauses.is_empty()
            && let Some(case_equality_expr) = &case_equality_expression
        {
            let assertion_context = self.context.get_assertion_context_from_block(self.block_context);

            self.negated_clauses.extend(negate_or_synthesize(
                case_clauses,
                case_equality_expr,
                assertion_context,
                self.artifacts,
            ));
        }

        self.artifacts.case_scopes.push(CaseScope::new());

        analyze_statements(&case_stmts, self.context, &mut case_block_context, self.artifacts)?;

        let Some(case_scope) = self.artifacts.case_scopes.pop() else {
            return Ok(());
        };

        let new_expression_types = self.artifacts.expression_types.clone();
        old_expression_types.extend(new_expression_types);
        self.artifacts.expression_types = old_expression_types;

        if !matches!(case_exit_type, ControlAction::Return) {
            self.handle_non_returning_case(
                switch_var_id,
                switch_case,
                &case_block_context,
                original_block_context,
                case_exit_type,
            )?;
        }

        if let Some(break_vars) = &case_scope.break_vars {
            if let Some(ref mut possibly_redefined_var_ids) = self.possibly_redefined_variables {
                for (var_id, var_type) in break_vars {
                    possibly_redefined_var_ids.insert(
                        var_id.clone(),
                        combine_optional_union_types(
                            Some(var_type),
                            possibly_redefined_var_ids.get(var_id),
                            self.context.codebase,
                            self.context.interner,
                        ),
                    );
                }
            } else {
                self.possibly_redefined_variables = Some(
                    break_vars
                        .iter()
                        .filter(|(var_id, _)| self.block_context.locals.contains_key(*var_id))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                );
            }

            if let Some(ref mut new_locals) = self.new_locals {
                for (var_id, var_type) in new_locals.clone() {
                    if let Some(break_var_type) = break_vars.get(&var_id) {
                        if case_block_context.locals.contains_key(&var_id) {
                            new_locals.insert(
                                var_id.clone(),
                                Rc::new(combine_union_types(
                                    break_var_type,
                                    &var_type,
                                    self.context.codebase,
                                    self.context.interner,
                                    false,
                                )),
                            );
                        } else {
                            new_locals.remove(&var_id);
                        }
                    } else {
                        new_locals.remove(&var_id);
                    }
                }
            }

            if let Some(ref mut redefined_vars) = self.redefined_variables {
                for (var_id, var_type) in redefined_vars.clone() {
                    if let Some(break_var_type) = break_vars.get(&var_id) {
                        redefined_vars.insert(
                            var_id.clone(),
                            Rc::new(combine_union_types(
                                break_var_type,
                                &var_type,
                                self.context.codebase,
                                self.context.interner,
                                false,
                            )),
                        );
                    } else {
                        redefined_vars.remove(&var_id);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_non_returning_case(
        &mut self,
        switch_var_id: &String,
        switch_case: &SwitchCase,
        case_block_context: &BlockContext<'_>,
        original_block_context: &BlockContext<'_>,
        case_exit_type: ControlAction,
    ) -> Result<(), AnalysisError> {
        if switch_case.is_default()
            && let Some(switch_type) = case_block_context.locals.get(switch_var_id)
            && switch_type.is_never()
        {
            self.context.collector.report_with_code(
                Code::PARADOXICAL_CONDITION,
                Issue::error("All possible case statements have been met, default is impossible here".to_string())
                    .with_annotation(
                        Annotation::primary(switch_case.span()).with_message("this is not going to be reached, ever."),
                    ),
            );

            return Ok(());
        }

        if !matches!(case_exit_type, ControlAction::Continue) {
            let mut removed_var_ids = HashSet::default();
            let case_redefined_vars =
                case_block_context.get_redefined_locals(&original_block_context.locals, false, &mut removed_var_ids);

            if let Some(ref mut possibly_redefined_var_ids) = self.possibly_redefined_variables {
                for (var_id, var_type) in &case_redefined_vars {
                    possibly_redefined_var_ids.insert(
                        var_id.clone(),
                        combine_optional_union_types(
                            Some(var_type),
                            possibly_redefined_var_ids.get(var_id),
                            self.context.codebase,
                            self.context.interner,
                        ),
                    );
                }
            } else {
                self.possibly_redefined_variables = Some(
                    case_redefined_vars
                        .clone()
                        .into_iter()
                        .filter(|(var_id, _)| self.block_context.locals.contains_key(var_id))
                        .collect(),
                );
            }

            if let Some(ref mut redefined_vars) = self.redefined_variables {
                for (var_id, var_type) in redefined_vars.clone() {
                    if let Some(break_var_type) = case_redefined_vars.get(&var_id) {
                        redefined_vars.insert(
                            var_id.clone(),
                            Rc::new(combine_union_types(
                                break_var_type,
                                &var_type,
                                self.context.codebase,
                                self.context.interner,
                                false,
                            )),
                        );
                    } else {
                        redefined_vars.remove(&var_id);
                    }
                }
            } else {
                self.redefined_variables =
                    Some(case_redefined_vars.into_iter().map(|(k, v)| (k, Rc::new(v))).collect());
            }

            if let Some(ref mut new_locals) = self.new_locals {
                for (var_id, var_type) in new_locals.clone() {
                    if case_block_context.locals.contains_key(&var_id) {
                        new_locals.insert(
                            var_id.clone(),
                            Rc::new(combine_union_types(
                                case_block_context.locals.get(&var_id).unwrap(),
                                &var_type,
                                self.context.codebase,
                                self.context.interner,
                                false,
                            )),
                        );
                    } else {
                        new_locals.remove(&var_id);
                    }
                }
            } else {
                self.new_locals = Some(
                    case_block_context
                        .locals
                        .clone()
                        .into_iter()
                        .filter(|(k, _)| !self.block_context.locals.contains_key(k))
                        .collect(),
                );
            }
        }

        Ok(())
    }

    fn update_case_exit_map(&mut self, case: &SwitchCase, case_index: usize) {
        let raw_actions = ControlAction::from_statements(
            case.statements().iter().collect(),
            vec![BreakType::Switch],
            Some(self.artifacts),
            true,
        );

        let actions_set: HashSet<ControlAction> = raw_actions.into_iter().collect();
        let effective_action = Self::get_last_action(&actions_set);

        if let Some(action) = effective_action {
            self.last_case_exit_type = action;
        }

        self.case_exit_types.insert(case_index, self.last_case_exit_type);
        self.case_actions.insert(case_index, actions_set);
    }

    fn get_last_action(case_actions: &HashSet<ControlAction>) -> Option<ControlAction> {
        match (
            case_actions.len(),
            case_actions.contains(&ControlAction::None),
            case_actions.contains(&ControlAction::End),
            case_actions.contains(&ControlAction::Continue),
            case_actions.contains(&ControlAction::LeaveSwitch),
        ) {
            (1, false, true, _, _) => Some(ControlAction::Return),
            (1, false, _, true, _) => Some(ControlAction::Continue),
            (_, false, _, _, true) => Some(ControlAction::Break),
            (len, true, _, _, _) if len > 1 => Some(ControlAction::Break),
            _ => None,
        }
    }
}

fn new_synthetic_disjunctive_equality(subject: &Expression, left: &Expression, right: Vec<&Expression>) -> Expression {
    let mut expr = new_synthetic_equals(subject, left);
    for r in right {
        expr = new_synthetic_or(&expr, &new_synthetic_equals(subject, r));
    }

    expr
}

fn new_synthetic_or(left: &Expression, right: &Expression) -> Expression {
    new_synthetic_binary(left, BinaryOperator::Or(Span::dummy(0, 0)), right)
}

fn new_synthetic_equals(left: &Expression, right: &Expression) -> Expression {
    new_synthetic_binary(left, BinaryOperator::Equal(Span::dummy(0, 0)), right)
}

fn new_synthetic_binary(left: &Expression, operator: BinaryOperator, right: &Expression) -> Expression {
    Expression::Binary(Binary { lhs: Box::new(left.clone()), operator, rhs: Box::new(right.clone()) })
}

fn new_synthetic_variable(interner: &ThreadedInterner, name: &str) -> Expression {
    Expression::Variable(Variable::Direct(DirectVariable { span: Span::dummy(0, 0), name: interner.intern(name) }))
}
