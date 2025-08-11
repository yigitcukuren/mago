use mago_codex::metadata::CodebaseMetadata;
use mago_codex::reference::SymbolReferences;
use mago_database::ReadDatabase;
use mago_interner::ThreadedInterner;
use mago_linter::Linter;
use mago_names::resolver::NameResolver;
use mago_php_version::PHPVersion;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_semantics::SemanticsChecker;
use mago_syntax::parser::parse_file;

use crate::error::Error;
use crate::pipeline::ParallelPipeline;
use crate::pipeline::Reducer;
use crate::pipeline::StatelessParallelPipeline;
use crate::pipeline::StatelessReducer;

const PROGRESS_BAR_THEME: &str = "🧹 Linting";

/// The "reduce" step for the linting pipeline.
///
/// This struct implements both stateful and stateless reduction, aggregating
/// `IssueCollection`s from parallel tasks into a single, final collection.
#[derive(Debug)]
pub struct LintResultReducer;

impl Reducer<IssueCollection, IssueCollection> for LintResultReducer {
    fn reduce(
        &self,
        mut codebase: CodebaseMetadata,
        _symbol_references: SymbolReferences,
        results: Vec<IssueCollection>,
    ) -> Result<IssueCollection, Error> {
        let mut final_issues = codebase.take_issues(true);
        for issues in results {
            final_issues.extend(issues);
        }
        Ok(final_issues)
    }
}

impl StatelessReducer<IssueCollection, IssueCollection> for LintResultReducer {
    fn reduce(&self, results: Vec<IssueCollection>) -> Result<IssueCollection, Error> {
        let mut final_issues = IssueCollection::new();
        for issues in results {
            final_issues.extend(issues);
        }

        Ok(final_issues)
    }
}

/// Shared, read-only context provided to each parallel linting task.
#[derive(Clone)]
pub struct LintContext {
    /// The target PHP version for analysis.
    pub php_version: PHPVersion,
    /// A pre-configured `Linter` instance.
    pub linter: Linter,
    /// The operational mode, determining which checks to run.
    pub mode: LintMode,
}

/// Defines the different operational modes for the linter.
#[derive(Clone, Copy)]
pub enum LintMode {
    /// Runs only parsing and semantic checks. This is the fastest mode.
    SemanticsOnly,
    /// Runs semantic checks and codebase compilation, reporting issues from both.
    Compilation,
    /// Runs all checks: semantics, compilation, and the full linter rule set.
    Full,
}

/// The main entry point for running the linting pipeline.
///
/// This function selects the appropriate parallel pipeline (`Stateful` or `Stateless`)
/// based on the requested [`LintMode`] and executes it.
pub fn run_lint_pipeline(
    interner: &ThreadedInterner,
    database: ReadDatabase,
    context: LintContext,
) -> Result<IssueCollection, Error> {
    match context.mode {
        LintMode::Full => run_full_pipeline(interner, database, context),
        LintMode::Compilation => run_compilation_pipeline(interner, database, context),
        LintMode::SemanticsOnly => run_semantics_pipeline(interner, database, context),
    }
}

/// Executes the full, stateful linting pipeline.
///
/// This pipeline compiles a global `CodebaseMetadata` and provides it to each
/// linting task, enabling rules that require cross-file awareness.
fn run_full_pipeline(
    interner: &ThreadedInterner,
    database: ReadDatabase,
    context: LintContext,
) -> Result<IssueCollection, Error> {
    ParallelPipeline::new(PROGRESS_BAR_THEME, database, interner, context, Box::new(LintResultReducer)).run(
        |context, interner, file, codebase| {
            let (program, parsing_error) = parse_file(&interner, &file);
            let resolved_names = NameResolver::new(&interner).resolve(&program);

            let mut issues = IssueCollection::new();
            if let Some(error) = parsing_error {
                issues.push(Issue::from(&error));
            }

            let semantics_checker = SemanticsChecker::new(&context.php_version, &interner);
            issues.extend(semantics_checker.check(&file, &program, &resolved_names));
            issues.extend(context.linter.lint(&file, &program, &resolved_names, &codebase));

            Ok(issues)
        },
    )
}

/// Executes a stateful pipeline that stops after the compilation phase.
///
/// This mode is used to gather both semantic and compilation-related issues
/// without running the full linter rule set.
fn run_compilation_pipeline(
    interner: &ThreadedInterner,
    database: ReadDatabase,
    context: LintContext,
) -> Result<IssueCollection, Error> {
    ParallelPipeline::new(PROGRESS_BAR_THEME, database, interner, context, Box::new(LintResultReducer)).run(
        |context, interner, file, _| {
            let (program, parsing_error) = parse_file(&interner, &file);
            let resolved_names = NameResolver::new(&interner).resolve(&program);

            let mut issues = IssueCollection::new();
            if let Some(error) = parsing_error {
                issues.push(Issue::from(&error));
            }

            let semantics_checker = SemanticsChecker::new(&context.php_version, &interner);
            issues.extend(semantics_checker.check(&file, &program, &resolved_names));

            Ok(issues)
        },
    )
}

/// Executes a fast, stateless pipeline for semantic checks only.
///
/// This pipeline does not compile a global `CodebaseMetadata`, making it much
/// faster. It is suitable for quick, syntax-aware checks.
fn run_semantics_pipeline(
    interner: &ThreadedInterner,
    database: ReadDatabase,
    context: LintContext,
) -> Result<IssueCollection, Error> {
    StatelessParallelPipeline::new(PROGRESS_BAR_THEME, database, interner, context, Box::new(LintResultReducer)).run(
        |context, interner, file| {
            let (program, parsing_error) = parse_file(&interner, &file);
            let resolved_names = NameResolver::new(&interner).resolve(&program);

            let mut issues = IssueCollection::new();
            if let Some(error) = parsing_error {
                issues.push(Issue::from(&error));
            }

            let semantics_checker = SemanticsChecker::new(&context.php_version, &interner);
            issues.extend(semantics_checker.check(&file, &program, &resolved_names));

            Ok(issues)
        },
    )
}
