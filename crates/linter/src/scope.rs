use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::ast::Node;

use crate::context::LintContext;

/// Represents a class-like lexical scope.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ClassLikeScope<'a> {
    /// A `class` scope, containing the class name.
    Class(&'a str),
    /// An `interface` scope, containing the interface name.
    Interface(&'a str),
    /// A `trait` scope, containing the trait name.
    Trait(&'a str),
    /// An `enum` scope, containing the enum name.
    Enum(&'a str),
    /// An anonymous `class` scope, containing the span of the `new class` expression.
    AnonymousClass(Span),
}

/// Represents a function-like lexical scope.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FunctionLikeScope<'a> {
    /// A `function` scope, containing the function name.
    Function(&'a str),
    /// A `method` scope, containing the method name.
    Method(&'a str),
    /// An `fn()` arrow function scope, containing its span.
    ArrowFunction(Span),
    /// A `function()` closure scope, containing its span.
    Closure(Span),
}

/// Represents a single level of lexical scope within the AST.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Scope<'a> {
    /// A `namespace` scope.
    Namespace(&'a str),
    /// Any class-like scope (`class`, `interface`, `trait`, `enum`).
    ClassLike(ClassLikeScope<'a>),
    /// Any function-like scope (`function`, `method`, `closure`).
    FunctionLike(FunctionLikeScope<'a>),
}

/// A stack that tracks the current nesting of lexical scopes during AST traversal.
///
/// As the node walker descends into scope-defining nodes (like classes or functions),
/// it pushes a new `Scope` onto this stack. When it exits that node, it pops the
/// scope off. This allows rules to query the current context at any point.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ScopeStack<'a> {
    stack: Vec<Scope<'a>>,
}

impl<'a> Scope<'a> {
    /// Creates a `Scope` from an AST `Node` if that node defines a new scope.
    ///
    /// Returns `None` if the node does not define a scope.
    pub fn for_node(ctx: &LintContext<'a>, node: Node<'a>) -> Option<Self> {
        Some(match node {
            Node::Namespace(namespace) => {
                let namespace_name = namespace
                    .name
                    .as_ref()
                    .map(|n| ctx.interner.lookup(n.value()))
                    .map(|n| if let Some(n) = n.strip_prefix('\\') { n } else { n })
                    .unwrap_or("");

                Scope::Namespace(namespace_name)
            }
            Node::Class(class) => {
                let class_name = ctx.lookup_name(&class.name);

                Scope::ClassLike(ClassLikeScope::Class(class_name))
            }
            Node::Interface(interface) => {
                let interface_name = ctx.lookup_name(&interface.name);

                Scope::ClassLike(ClassLikeScope::Interface(interface_name))
            }
            Node::Trait(trait_node) => {
                let trait_name = ctx.lookup_name(&trait_node.name);

                Scope::ClassLike(ClassLikeScope::Trait(trait_name))
            }
            Node::Enum(enum_node) => {
                let enum_name = ctx.lookup_name(&enum_node.name);

                Scope::ClassLike(ClassLikeScope::Enum(enum_name))
            }
            Node::AnonymousClass(anonymous_class) => {
                let span = anonymous_class.span();

                Scope::ClassLike(ClassLikeScope::AnonymousClass(span))
            }
            Node::Function(function) => {
                let function_name = ctx.lookup_name(&function.name);

                Scope::FunctionLike(FunctionLikeScope::Function(function_name))
            }
            Node::Method(method) => {
                let method_name = ctx.lookup(&method.name.value);

                Scope::FunctionLike(FunctionLikeScope::Method(method_name))
            }
            Node::Closure(closure) => {
                let span = closure.span();

                Scope::FunctionLike(FunctionLikeScope::Closure(span))
            }
            Node::ArrowFunction(arrow_function) => {
                let span = arrow_function.span();

                Scope::FunctionLike(FunctionLikeScope::ArrowFunction(span))
            }
            _ => {
                return None;
            }
        })
    }
}

impl<'a> ScopeStack<'a> {
    /// Creates a new, empty scope stack.
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    /// Pushes a new scope onto the stack.
    ///
    /// This is called by the walker when it enters a scope-defining node.
    pub fn push(&mut self, scope: Scope<'a>) {
        self.stack.push(scope);
    }

    /// Pops the current scope from the stack.
    ///
    /// This is called by the walker when it exits a scope-defining node.
    pub fn pop(&mut self) -> Option<Scope<'a>> {
        self.stack.pop()
    }

    /// Searches the stack and returns the name of the current namespace.
    ///
    /// Returns an empty string if in the global scope.
    pub fn get_namespace(&self) -> &'a str {
        self.stack
            .iter()
            .rev()
            .find_map(|scope| match scope {
                Scope::Namespace(namespace) => Some(*namespace),
                _ => None,
            })
            .unwrap_or("")
    }

    /// Searches the stack and returns the innermost `ClassLikeScope`.
    pub fn get_class_like_scope(&self) -> Option<ClassLikeScope<'a>> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::ClassLike(class_like) => Some(*class_like),
            _ => None,
        })
    }

    /// Searches the stack and returns the innermost `FunctionLikeScope`.
    pub fn get_function_like_scope(&self) -> Option<FunctionLikeScope<'a>> {
        self.stack.iter().rev().find_map(|scope| match scope {
            Scope::FunctionLike(function_like) => Some(*function_like),
            _ => None,
        })
    }
}

impl Default for ScopeStack<'_> {
    fn default() -> Self {
        Self::new()
    }
}
