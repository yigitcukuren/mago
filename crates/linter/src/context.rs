use toml::value::Value;

use mago_codex::constant_exists;
use mago_codex::function_exists;
use mago_codex::metadata::CodebaseMetadata;
use mago_database::file::File;
use mago_fixer::FixPlan;
use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;
use mago_php_version::PHPVersion;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_reporting::Level;
use mago_span::HasPosition;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::ast::PreComputedNode;
use crate::directive::LintDirective;
use crate::rule::ConfiguredRule;
use crate::scope::ClassLikeScope;
use crate::scope::FunctionLikeScope;
use crate::scope::Scope;
use crate::scope::ScopeStack;

#[derive(Debug)]
pub struct LintContext<'a> {
    pub php_version: PHPVersion,
    pub rule: &'a ConfiguredRule,
    pub interner: &'a ThreadedInterner,
    pub codebase: &'a CodebaseMetadata,
    pub source_file: &'a File,
    pub resolved_names: &'a ResolvedNames,
    pub scope: ScopeStack,
    pub issues: IssueCollection,
}

impl<'a> LintContext<'a> {
    pub fn new(
        php_version: PHPVersion,
        rule: &'a ConfiguredRule,
        interner: &'a ThreadedInterner,
        codebase: &'a CodebaseMetadata,
        source_file: &'a File,
        resolved_names: &'a ResolvedNames,
    ) -> LintContext<'a> {
        LintContext {
            php_version,
            rule,
            interner,
            codebase,
            source_file,
            resolved_names,
            issues: IssueCollection::new(),
            scope: ScopeStack::new(),
        }
    }

    /// Determines the effective reporting level for a linter rule.
    pub fn level(&self) -> Level {
        self.rule.level
    }

    /// Retrieves the value of a rule-specific option.
    pub fn option(&self, option_name: &'static str) -> Option<&Value> {
        self.rule.settings.get_option(option_name)
    }

    /// Retrieves the string associated with a given identifier.
    ///
    /// # Panics
    ///
    /// Panics if the identifier is not found in the interner.
    pub fn lookup(&self, id: &StringIdentifier) -> &str {
        self.interner.lookup(id)
    }

    /// Checks if a name at a given position is imported.
    pub fn is_name_imported(&self, position: &impl HasPosition) -> bool {
        self.resolved_names.is_imported(&position.position())
    }

    /// Retrieves the name associated with a given position in the code.
    ///
    /// # Panics
    ///
    /// Panics if no name is found at the specified position.
    pub fn lookup_name(&self, position: &impl HasPosition) -> &str {
        let name_id = self.resolved_names.get(&position.position());

        self.lookup(name_id)
    }

    /// Resolves the correct function name based on PHP's dynamic name resolution rules.
    ///
    /// This function determines the fully qualified name (FQN) of a function being called,
    /// accounting for PHP's nuanced resolution rules:
    ///
    /// - If the function is explicitly imported via `use`, it resolves to the imported name.
    /// - If the function name starts with a leading `\`, it is treated as a global function.
    /// - If no `\` is present:
    ///   1. The function name is checked in the current namespace.
    ///   2. If not found, it falls back to the global namespace.
    ///   3. If neither exists, it defaults to the current namespace's FQN.
    ///
    /// # Arguments
    ///
    /// - `identifier`: The identifier representing the function name in the source code.
    ///
    /// # Returns
    ///
    /// - A reference to the resolved function name as a string.
    ///
    /// # Note
    ///
    /// Function names in PHP are case-insensitive; they are stored and looked up in lowercase
    /// within the codebase metadata.
    pub fn resolve_function_name(&self, identifier: &Identifier) -> &str {
        // Check if the function name is explicitly imported via `use` statement.
        // If it is, directly resolve it to the imported name.
        if self.is_name_imported(identifier) {
            return self.lookup_name(identifier);
        }

        // Retrieve the raw name identifier for the function.
        let name_id = identifier.value();
        let name = self.lookup(name_id);

        // Check if the name explicitly refers to the global namespace using a leading `\`.
        if let Some(stripped) = name.strip_prefix('\\') {
            // If yes, return the global function name without the `\`.
            return stripped;
        }

        // If no leading `\`, resolve based on the namespace hierarchy:
        // 1. Check if the fully qualified function name (FQFN) exists in the current context.
        let fqfn_id = self.resolved_names.get(&identifier.position());
        if function_exists(self.codebase, self.interner, fqfn_id) {
            // The FQFN exists, so return it.
            return self.lookup(fqfn_id);
        }

        // If FQFN doesn't exist, check if the global function name exists.
        if !name.contains('\\') && function_exists(self.codebase, self.interner, name_id) {
            // If global function name exists, return it.
            return name;
        }

        // If neither exists, assume the FQFN and return it.
        self.lookup(fqfn_id)
    }

    /// Resolves the correct constant name based on PHP's dynamic name resolution rules.
    ///
    /// This function determines the fully qualified name (FQN) of a constant being referenced,
    /// accounting for PHP's nuanced resolution rules:
    ///
    /// - If the constant is explicitly imported via `use const`, it resolves to the imported name.
    /// - If the constant name starts with a leading `\`, it is treated as a global constant.
    /// - If no `\` is present:
    ///   1. The constant name is checked in the current namespace.
    ///   2. If not found, it falls back to the global namespace.
    ///   3. If neither exists, it defaults to the current namespace's FQN.
    ///
    /// # Arguments
    ///
    /// - `identifier`: The identifier representing the constant name in the source code.
    ///
    /// # Returns
    ///
    /// - A reference to the resolved constant name as a string.
    ///
    /// # Note
    ///
    /// Constant names in PHP are case-sensitive, so exact matches are performed without lowering.
    pub fn resolve_constant_name(&self, identifier: &Identifier) -> &str {
        // Check if the constant name is explicitly imported via `use` statement.
        // If it is, directly resolve it to the imported name.
        if self.is_name_imported(identifier) {
            return self.lookup_name(identifier);
        }

        // Retrieve the raw name identifier for the constant.
        let name_id = identifier.value();
        let name = self.lookup(name_id);

        // Check if the name explicitly refers to the global namespace using a leading `\`.
        if let Some(stripped) = name.strip_prefix('\\') {
            // If yes, return the global constant name without the `\`.
            return stripped;
        }

        // If no leading `\`, resolve based on the namespace hierarchy:
        // 1. Check if the fully qualified constant name (FQCN) exists in the current context.
        let fqcn_id = self.resolved_names.get(&identifier.position());

        if constant_exists(self.codebase, self.interner, fqcn_id) {
            // The FQCN exists, so return it.
            return self.lookup(fqcn_id);
        }

        // If FQCN doesn't exist, check if the global constant name exists.
        if !name.contains('\\') && constant_exists(self.codebase, self.interner, name_id) {
            // If global constant name exists,
            return name;
        }

        // If neither exists, assume the FQCN and return it.
        self.lookup(fqcn_id)
    }

    /// Converts a type hint into a human-readable string representation.
    ///
    /// This function takes a type hint (e.g., an identifier, nullable type, union type)
    /// and resolves it into a string that can be used in error messages or similar contexts.
    /// The return value is not guaranteed to match the exact type representation in the code
    /// or in the internal metadata system—it is specifically formatted for readability.
    ///
    /// # Arguments
    ///
    /// - `hint`: The type hint to resolve. The hint can represent various constructs such as
    ///   identifiers, nullable types, unions, intersections, and keywords like `void`, `self`, etc.
    ///
    /// # Returns
    ///
    /// - A `String` containing a human-readable representation of the type hint.
    ///
    /// # Notes
    ///
    /// - Identifiers are resolved using `lookup_name` for user-defined types or symbols.
    /// - Constructs like nullable types (`?`), unions (`|`), and intersections (`&`) are recursively resolved.
    /// - The output is designed for clarity in messages (e.g., issues) rather than reflecting the exact
    ///   code structure or type definitions.
    ///
    /// # Examples
    ///
    /// - For a nullable type `?Foo`, the function returns `"?Foo"`.
    /// - For a union type `int|string`, the function returns `"int|string"`.
    /// - For a complex type like `(int|float)&string`, the function returns `"(int|float)&string"`.
    pub fn get_readable_hint(&self, hint: &Hint) -> String {
        match hint {
            Hint::Identifier(identifier) => self.lookup_name(identifier).to_string(),
            Hint::Parenthesized(parenthesized_hint) => {
                format!("({})", self.get_readable_hint(&parenthesized_hint.hint))
            }
            Hint::Nullable(nullable_hint) => format!("?{}", self.get_readable_hint(&nullable_hint.hint)),
            Hint::Union(union_hint) => {
                format!("{}|{}", self.get_readable_hint(&union_hint.left), self.get_readable_hint(&union_hint.right))
            }
            Hint::Intersection(intersection_hint) => {
                format!(
                    "{}&{}",
                    self.get_readable_hint(&intersection_hint.left),
                    self.get_readable_hint(&intersection_hint.right)
                )
            }
            Hint::Null(keyword)
            | Hint::True(keyword)
            | Hint::False(keyword)
            | Hint::Array(keyword)
            | Hint::Callable(keyword)
            | Hint::Static(keyword)
            | Hint::Self_(keyword)
            | Hint::Parent(keyword) => self.lookup(&keyword.value).to_string(),
            Hint::Void(identifier)
            | Hint::Never(identifier)
            | Hint::Float(identifier)
            | Hint::Bool(identifier)
            | Hint::Integer(identifier)
            | Hint::String(identifier)
            | Hint::Object(identifier)
            | Hint::Mixed(identifier)
            | Hint::Iterable(identifier) => self.lookup(&identifier.value).to_string(),
        }
    }

    #[inline]
    pub fn report(&mut self, issue: Issue) {
        self.issues.push(issue.with_code(&self.rule.slug))
    }

    #[inline]
    pub fn propose<F>(&mut self, mut issue: Issue, f: F)
    where
        F: FnOnce(&mut FixPlan),
    {
        let mut plan = FixPlan::new();
        f(&mut plan);
        if !plan.is_empty() {
            issue = issue.with_suggestion(self.source_file.id, plan);
        }

        self.report(issue)
    }

    #[inline]
    pub(crate) fn lint(&mut self, ast_node: &PreComputedNode<'_>) -> bool {
        let should_pop_scope = if let Some(scope) = match ast_node.node {
            Node::Class(class) => Some(Scope::ClassLike(ClassLikeScope::Class(*self.resolved_names.get(&class.name)))),
            Node::Interface(interface) => {
                Some(Scope::ClassLike(ClassLikeScope::Interface(*self.resolved_names.get(&interface.name))))
            }
            Node::Trait(r#trait) => {
                Some(Scope::ClassLike(ClassLikeScope::Trait(*self.resolved_names.get(&r#trait.name))))
            }
            Node::Enum(r#enum) => Some(Scope::ClassLike(ClassLikeScope::Enum(*self.resolved_names.get(&r#enum.name)))),
            Node::AnonymousClass(class) => Some(Scope::ClassLike(ClassLikeScope::AnonymousClass(class.span()))),
            Node::Function(func) => {
                Some(Scope::FunctionLike(FunctionLikeScope::Function(*self.resolved_names.get(&func.name))))
            }
            Node::Method(method) => Some(Scope::FunctionLike(FunctionLikeScope::Method(method.name.value))),
            Node::Closure(closure) => {
                let closure_span = closure.span();

                Some(Scope::FunctionLike(FunctionLikeScope::Closure(closure_span.file_id, closure_span.start)))
            }
            Node::ArrowFunction(func) => {
                let arrow_func_span = func.span();

                Some(Scope::FunctionLike(FunctionLikeScope::Closure(arrow_func_span.file_id, arrow_func_span.start)))
            }
            _ => None,
        } {
            self.scope.push(scope);
            true
        } else {
            false
        };

        // Apply the lint rule to the current node.
        let directive = self.rule.rule.lint_node(ast_node.node, self);
        let result = 'lint: {
            match directive {
                LintDirective::Continue => {
                    // Recurse into each child node.
                    for child in &ast_node.children {
                        if self.lint(child) {
                            break 'lint true;
                        }
                    }

                    false
                }
                LintDirective::Prune => false, // Skip children.
                LintDirective::Abort => true,  // Abort the current branch.
            }
        };

        if should_pop_scope {
            self.scope.pop();
        }

        result
    }

    pub fn finish(self) -> IssueCollection {
        self.issues
    }
}
