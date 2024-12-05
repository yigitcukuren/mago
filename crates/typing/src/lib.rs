use fennec_ast::Expression;
use fennec_interner::ThreadedInterner;
use fennec_reflection::r#type::kind::TypeKind;
use fennec_reflection::r#type::TypeReflection;
use fennec_reflection::CodebaseReflection;
use fennec_semantics::Semantics;
use fennec_span::HasSpan;

use crate::resolver::TypeResolver;

mod internal;

pub mod constant;
pub mod resolver;

/// Infers the type of a given expression by initializing a simple type reflection
/// that includes the inferred type kind and its source location.
///
/// - `interner`: Used for managing string interning.
/// - `semantics`: Provides access to the source and semantic information.
/// - `expression`: The expression to analyze.
///
/// Returns a `TypeReflection` with the inferred type kind and span of the expression.
pub fn infere<'ast>(
    interner: &ThreadedInterner,
    semantics: &'ast Semantics,
    expression: &'ast Expression,
) -> TypeReflection {
    let kind = infere_kind(interner, semantics, expression);

    TypeReflection { kind, inferred: true, span: expression.span() }
}

/// Infers the general type kind of an expression without using a codebase for context.
///
/// - `interner`: Manages string interning.
/// - `semantics`: Provides source and semantic data.
/// - `expression`: The expression to infer.
///
/// Returns a `TypeKind` that represents the initial inferred type of the expression.
pub fn infere_kind<'ast>(
    interner: &ThreadedInterner,
    semantics: &'ast Semantics,
    expression: &'ast Expression,
) -> TypeKind {
    let resolver = TypeResolver::new(interner, semantics, None);

    resolver.resolve(expression)
}

/// Resolves the type kind of an expression with additional codebase context for more
/// precise type information.
///
/// - `interner`: Manages string interning.
/// - `semantics`: Provides source and semantic data.
/// - `codebase`: Optional codebase reflection to resolve function/method types.
/// - `expression`: The expression to resolve.
///
/// Returns a `TypeKind` that represents the resolved type of the expression,
/// taking into account any known codebase types.
pub fn resolve_kind<'ast>(
    interner: &ThreadedInterner,
    semantics: &'ast Semantics,
    codebase: &'ast CodebaseReflection,
    expression: &'ast Expression,
) -> TypeKind {
    let resolver = TypeResolver::new(interner, semantics, Some(codebase));

    resolver.resolve(expression)
}
