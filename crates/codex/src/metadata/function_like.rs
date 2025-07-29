use std::collections::BTreeMap;

use ahash::HashMap;
use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_reporting::Issue;
use mago_span::Span;

use crate::assertion::Assertion;
use crate::metadata::attribute::AttributeMetadata;
use crate::metadata::parameter::FunctionLikeParameterMetadata;
use crate::metadata::ttype::TypeMetadata;
use crate::misc::GenericParent;
use crate::ttype::resolution::TypeResolutionContext;
use crate::ttype::union::TUnion;
use crate::visibility::Visibility;

pub type TemplateTuple = (StringIdentifier, Vec<(GenericParent, TUnion)>);

/// Contains metadata specific to methods defined within classes, interfaces, enums, or traits.
///
/// This complements the more general `FunctionLikeMetadata`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct MethodMetadata {
    /// Marks whether this method is declared as `final`, preventing further overriding.
    pub is_final: bool,

    /// Marks whether this method is declared as `abstract`, requiring implementation in subclasses.
    pub is_abstract: bool,

    /// Marks whether this method is declared as `static`, allowing it to be called without an instance.
    pub is_static: bool,

    /// Marks whether this method is a constructor (`__construct`).
    pub is_constructor: bool,

    /// Marks whether this method is declared as `public`, `protected`, or `private`.
    pub visibility: Visibility,

    /// A map of constraints defined by `@where` docblock tags.
    ///
    /// The key is the name of a class-level template parameter (e.g., `T`), and the value
    /// is the `TUnion` type constraint that `T` must satisfy for this specific method
    /// to be considered callable.
    pub where_constraints: HashMap<StringIdentifier, TypeMetadata>,

    /// The type that the object instance (`$this`) will have *after* this method is called.
    ///
    /// This is populated from the `@this-out` tag (and its aliases: `@self-out`,
    /// `@psalm-this-out`, `@phpstan-this-out`, etc.). It is crucial for typing mutable
    /// objects, builders, and fluent interfaces where a method call changes the
    /// object's state in a way that affects its type, such as specializing a generic
    /// template parameter (e.g., a `Box<T>` becoming a `Box<string>`).
    pub this_out_type: Option<TypeMetadata>,
}

/// Distinguishes between different kinds of callable constructs in PHP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FunctionLikeKind {
    /// Represents a standard function declared in the global scope or a namespace (`function foo() {}`).
    Function,
    /// Represents a method defined within a class, trait, enum, or interface (`class C { function bar() {} }`).
    Method,
    /// Represents an anonymous function created using `function() {}`.
    Closure,
    /// Represents an arrow function (short closure syntax) introduced in PHP 7.4 (`fn() => ...`).
    ArrowFunction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FunctionLikeMetadata {
    /// The kind of function-like structure this metadata represents.
    pub kind: FunctionLikeKind,

    /// The source code location (span) covering the entire function/method/closure definition.
    /// For closures/arrow functions, this covers the `function(...) { ... }` or `fn(...) => ...` part.
    pub span: Span,

    /// The name of the function or method, if applicable.
    /// `None` for closures and arrow functions unless assigned to a variable later.
    /// Example: `processRequest`, `__construct`, `my_global_func`.
    pub name: Option<StringIdentifier>,

    /// The specific source code location (span) of the function or method name identifier.
    /// `None` if the function/method has no name (closures/arrow functions).
    pub name_span: Option<Span>,

    /// Ordered list of metadata for each parameter defined in the signature.
    pub parameters: Vec<FunctionLikeParameterMetadata>,

    /// The explicit return type declaration (type hint).
    ///
    /// Example: For `function getName(): string`, this holds metadata for `string`.
    /// `None` if no return type is specified.
    pub return_type_declaration_metadata: Option<TypeMetadata>,

    /// The explicit return type declaration (type hint) or docblock type (`@return`).
    ///
    /// Example: For `function getName(): string`, this holds metadata for `string`,
    /// or for ` /** @return string */ function getName() { .. }`, this holds metadata for `string`.
    /// `None` if neither is specified.
    pub return_type_metadata: Option<TypeMetadata>,

    /// Generic type parameters (templates) defined for the function/method (e.g., `@template T`).
    /// Stores the template name and its constraints (parent type and bound type).
    /// Example: `[("T", [(GenericParent::Function("funcName"), Arc<TUnion::object()>)])]`
    pub template_types: Vec<TemplateTuple>,

    /// Attributes attached to the function/method/closure declaration (`#[Attribute] function foo() {}`).
    pub attributes: Vec<AttributeMetadata>,

    /// Specific metadata relevant only to methods (visibility, final, static, etc.).
    /// This is `Some` if `kind` is `FunctionLikeKind::Method`, `None` otherwise.
    pub method_metadata: Option<MethodMetadata>,

    /// Contains context information needed for resolving types within this function's scope
    /// (e.g., `use` statements, current namespace, class context). Often populated during analysis.
    pub type_resolution_context: Option<TypeResolutionContext>,

    /// `true` if this function/method is defined in user-controlled code (vs. internal stubs/PHP core).
    /// Often determined from the source file info within the `span`.
    pub(crate) user_defined: bool,

    /// A list of types that this function/method might throw, derived from `@throws` docblock tags
    /// or inferred from `throw` statements within the body.
    pub thrown_types: Vec<TypeMetadata>,

    /// `true` if the function/method body contains a `yield` statement, indicating it's a generator.
    pub has_yield: bool,

    /// `true` if the function/method is marked with `@psalm-must-use`, `@phpstan-must-use`,
    /// or `@must-use`, indicating its return value should not be ignored.
    pub must_use: bool,

    /// `true` if the function/method body contains a `throw` statement.
    pub has_throw: bool,

    pub specialize_call: bool,

    /// Internal flag indicating whether this metadata structure has been fully populated
    /// by all analysis stages. Used to control analysis flow. (User requested minimal documentation).
    pub(crate) is_populated: bool,

    /// `true` if the function/method is marked as deprecated via docblock tags
    /// (`@deprecated`, `@psalm-deprecated`, `@phpstan-deprecated`).
    pub is_deprecated: bool,

    /// `true` if the function/method is marked as internal via docblock tags
    /// (`@internal`, `@psalm-internal`, `@phpstan-internal`), indicating it's not part of the lic API.
    pub is_internal: bool,

    /// `true` if the function/method is marked as pure via docblock tags
    /// (`@pure`, `@psalm-pure`, `@phpstan-pure`), indicating it has no side effects.
    pub is_pure: bool,

    /// `true` if marked with `@psalm-ignore-nullable-return` or equivalent, suppressing
    /// issues related to returning `null` when the signature doesn't explicitly allow it.
    pub ignore_nullable_return: bool,

    /// `true` if marked with `@psalm-ignore-falsable-return` or equivalent, suppressing
    /// issues related to returning `false` when the signature doesn't explicitly allow it.
    pub ignore_falsable_return: bool,

    /// `true` if the function/method's docblock includes `{@inheritdoc}` or implicitly inherits
    /// documentation from a parent method.
    pub inherits_docs: bool,

    /// `true` if marked with `@psalm-mutation-free`, `@phpstan-mutation-free`, indicating the function
    /// does not modify any state (including object properties or global state). Implies `@pure`.
    pub is_mutation_free: bool,

    /// `true` if marked with `@psalm-external-mutation-free`, `@phpstan-external-mutation-free`,
    /// indicating the function does not modify *external* state but may modify its own arguments
    /// or locally created objects.
    pub is_external_mutation_free: bool,

    /// `true` if the function/method accepts named arguments (PHP 8.0+ default).
    /// Can be set to `false` if the `#[NoNamedArguments]` attribute is present (PHP 8.2+).
    pub allows_named_arguments: bool,

    /// List of issues specifically related to parsing or interpreting this function's docblock.
    pub issues: Vec<Issue>,

    /// Assertions about parameter types or variable types that are guaranteed to be true
    /// *after* this function/method returns normally. From `@psalm-assert`, `@phpstan-assert`, etc.
    /// Maps variable/parameter name to a list of type assertions.
    pub assertions: BTreeMap<StringIdentifier, Vec<Assertion>>,

    /// Assertions about parameter/variable types that are guaranteed to be true if this
    /// function/method returns `true`. From `@psalm-assert-if-true`, etc.
    pub if_true_assertions: BTreeMap<StringIdentifier, Vec<Assertion>>,

    /// Assertions about parameter/variable types that are guaranteed to be true if this
    /// function/method returns `false`. From `@psalm-assert-if-false`, etc.
    pub if_false_assertions: BTreeMap<StringIdentifier, Vec<Assertion>>,

    /// A flag indicating if this function-like should be treated as unchecked.
    pub unchecked: bool,
}

impl FunctionLikeKind {
    /// Checks if this kind represents a class/trait/enum/interface method.
    #[inline]
    pub const fn is_method(&self) -> bool {
        matches!(self, Self::Method)
    }

    /// Checks if this kind represents a globally/namespace-scoped function.
    #[inline]
    pub const fn is_function(&self) -> bool {
        matches!(self, Self::Function)
    }

    /// Checks if this kind represents an anonymous function (`function() {}`).
    #[inline]
    pub const fn is_closure(&self) -> bool {
        matches!(self, Self::Closure)
    }

    /// Checks if this kind represents an arrow function (`fn() => ...`).
    #[inline]
    pub const fn is_arrow_function(&self) -> bool {
        matches!(self, Self::ArrowFunction)
    }
}

/// Contains comprehensive metadata for any function-like structure in PHP.
/// Provides a flexible API with setters (`set_*`), consuming builders (`with_*`),
/// adders (`add_*`), consuming adders (`with_added_*`), unsetters (`unset_*`),
/// and consuming unsetters (`without_*`), along with clearly named getters
/// (`get_*`, `is_*`, `has_*`, `allows_*`).
impl FunctionLikeMetadata {
    /// Creates new `FunctionLikeMetadata` with basic information and default flags.
    pub fn new(kind: FunctionLikeKind, span: Span) -> Self {
        let user_defined = span.start.source.1.is_user_defined();
        let method_metadata = if kind.is_method() { Some(MethodMetadata::default()) } else { None };

        Self {
            kind,
            span,
            user_defined,
            name: None,
            name_span: None,
            parameters: vec![],
            return_type_declaration_metadata: None,
            return_type_metadata: None,
            template_types: vec![],
            attributes: vec![],
            method_metadata,
            type_resolution_context: None,
            has_throw: false,
            thrown_types: vec![],
            is_pure: false,
            must_use: false,
            is_deprecated: false,
            specialize_call: false,
            is_populated: false,
            has_yield: false,
            is_internal: false,
            ignore_nullable_return: false,
            ignore_falsable_return: false,
            inherits_docs: false,
            is_mutation_free: false,
            is_external_mutation_free: false,
            allows_named_arguments: true,
            assertions: BTreeMap::new(),
            if_true_assertions: BTreeMap::new(),
            if_false_assertions: BTreeMap::new(),
            unchecked: false,
            issues: vec![],
        }
    }

    /// Returns the kind of function-like (Function, Method, Closure, ArrowFunction).
    #[inline]
    pub fn get_kind(&self) -> FunctionLikeKind {
        self.kind
    }

    /// Returns a mutable slice of the parameter metadata.
    #[inline]
    pub fn get_parameters_mut(&mut self) -> &mut [FunctionLikeParameterMetadata] {
        &mut self.parameters
    }

    /// Returns a reference to specific parameter metadata by name, if it exists.
    #[inline]
    pub fn get_parameter(&self, name: StringIdentifier) -> Option<&FunctionLikeParameterMetadata> {
        self.parameters.iter().find(|parameter| parameter.get_name().0 == name)
    }

    /// Returns a mutable reference to specific parameter metadata by name, if it exists.
    #[inline]
    pub fn get_parameter_mut(&mut self, name: StringIdentifier) -> Option<&mut FunctionLikeParameterMetadata> {
        self.parameters.iter_mut().find(|parameter| parameter.get_name().0 == name)
    }

    /// Returns a mutable slice of the template type parameters.
    #[inline]
    pub fn get_template_types_mut(&mut self) -> &mut [TemplateTuple] {
        &mut self.template_types
    }

    /// Returns a slice of the attributes.
    #[inline]
    pub fn get_attributes(&self) -> &[AttributeMetadata] {
        &self.attributes
    }

    /// Returns a mutable reference to the method-specific info, if this is a method.
    #[inline]
    pub fn get_method_metadata_mut(&mut self) -> Option<&mut MethodMetadata> {
        self.method_metadata.as_mut()
    }

    /// Returns a mutable slice of docblock issues.
    #[inline]
    pub fn take_issues(&mut self) -> Vec<Issue> {
        std::mem::take(&mut self.issues)
    }

    /// Checks if the function contains `yield`.
    #[inline]
    pub const fn has_yield(&self) -> bool {
        self.has_yield
    }

    /// Sets the name and corresponding name span. Clears both if name is `None`. Updates constructor status.
    #[inline]
    pub fn set_name(&mut self, name: Option<StringIdentifier>, name_span: Option<Span>) {
        if name.is_some() {
            self.name = name;
            self.name_span = name_span;
        } else {
            self.unset_name();
        }
    }

    /// Returns a new instance with the name and name span set. Updates constructor status.
    #[inline]
    pub fn with_name(mut self, name: Option<StringIdentifier>, name_span: Option<Span>) -> Self {
        self.set_name(name, name_span);
        self
    }

    /// Sets the name and name span to `None`. Updates constructor status.
    #[inline]
    pub fn unset_name(&mut self) {
        self.name = None;
        self.name_span = None;
        if let Some(ref mut info) = self.method_metadata {
            info.is_constructor = false;
        }
    }

    /// Sets the parameters, replacing existing ones.
    #[inline]
    pub fn set_parameters(&mut self, parameters: impl IntoIterator<Item = FunctionLikeParameterMetadata>) {
        self.parameters = parameters.into_iter().collect();
    }

    /// Returns a new instance with the parameters replaced.
    #[inline]
    pub fn with_parameters(mut self, parameters: impl IntoIterator<Item = FunctionLikeParameterMetadata>) -> Self {
        self.set_parameters(parameters);
        self
    }

    #[inline]
    pub fn set_return_type_metadata(&mut self, return_type: Option<TypeMetadata>) {
        self.return_type_metadata = return_type;
    }

    #[inline]
    pub fn set_return_type_declaration_metadata(&mut self, return_type: Option<TypeMetadata>) {
        if self.return_type_metadata.is_none() {
            self.return_type_metadata = return_type.clone();
        }

        self.return_type_declaration_metadata = return_type;
    }

    /// Adds a single template type definition.
    #[inline]
    pub fn add_template_type(&mut self, template: TemplateTuple) {
        self.template_types.push(template);
    }
}
