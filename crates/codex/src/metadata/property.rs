use serde::Deserialize;
use serde::Serialize;

use mago_span::Span;

use crate::metadata::ttype::TypeMetadata;
use crate::misc::VariableIdentifier;
use crate::visibility::Visibility;

/// Contains metadata associated with a declared class property in PHP.
///
/// This includes information about its name, location, visibility (potentially asymmetric),
/// type hints, default values, and various modifiers (`static`, `readonly`, `abstract`, etc.).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PropertyMetadata {
    /// The identifier (name) of the property, including the leading '$'.
    pub name: VariableIdentifier,

    /// The specific source code location (span) of the property's name identifier itself.
    /// `None` if the location is unknown or not relevant (e.g., for synthetic properties).
    pub name_span: Option<Span>,

    /// The source code location (span) covering the entire property declaration statement.
    /// `None` if the location is unknown or not relevant.
    pub span: Option<Span>,

    /// The visibility level required for reading the property's value.
    ///
    /// In PHP, this corresponds to the primary visibility keyword specified
    /// (e.g., the `public` in `public private(set) string $prop;`).
    ///
    /// If no asymmetric visibility is specified (e.g., `public string $prop`),
    /// this level applies to both reading and writing. Defaults to `Public`.
    pub read_visibility: Visibility,

    /// The visibility level required for writing/modifying the property's value.
    ///
    /// In PHP, this can differ from `read_visibility` using asymmetric visibility syntax
    /// like `private(set)` (e.g., `public private(set) string $prop;`).
    ///
    /// If asymmetric visibility is not used, this implicitly matches `read_visibility`.
    /// Defaults to `Public`.
    pub write_visibility: Visibility,

    /// The explicit type declaration (type hint) associated with the property, if any.
    ///
    /// e.g., for `public string $name;`, this would contain the metadata for `string`.
    pub type_declaration_metadata: Option<TypeMetadata>,

    /// The type metadata for the property's type, if any.
    ///
    /// This is either the same as `type_declaration_metadata` or the type provided
    /// in a docblock comment (e.g., `@var string`).
    pub type_metadata: Option<TypeMetadata>,

    /// The type inferred from the property's default value, if it has one.
    ///
    /// e.g., for `public $count = 0;`, this would contain the metadata for `int(0)`.
    /// This can be used to compare against `type_signature` for consistency checks.
    pub default_type_metadata: Option<TypeMetadata>,

    /// `true` if the property is declared with the `readonly` modifier.
    pub is_readonly: bool,

    /// `true` if the property is declared with a default value (e.g., `= null`, `= 10`).
    pub has_default: bool,

    /// `true` if this property originates from constructor property promotion.
    pub is_promoted: bool,

    /// `true` if the property is marked as internal, typically via a docblock tag like `@internal`.
    ///
    /// Indicates it's not intended for use outside the defining class or library.
    pub is_internal: bool,

    /// `true` if the property is declared with the `static` modifier.
    pub is_static: bool,

    /// `true` if this property represents an abstract property requirement.
    pub is_abstract: bool,

    /// `true` if the property is marked as deprecated, typically via `@deprecated` docblock tag.
    pub is_deprecated: bool,

    /// `true` if the property uses PHP's Property Hooks feature.
    ///
    /// Such properties have custom `get` and/or `set` logic instead of direct storage,
    /// making them behave like "virtual" properties.
    ///
    /// Note: Properties with hooks cannot have asymmetric visibility (`is_asymmetric` must be `false`).
    pub is_virtual: bool,

    /// `true` if `read_visibility` and `write_visibility` are different,
    ///
    /// indicating that PHP's asymmetric visibility syntax (e.g., `public private(set)`) was used.
    /// Must be `false` if `is_virtual` is `true`.
    pub is_asymmetric: bool,

    /// `true` if the property allows private mutation.
    pub allow_private_mutation: bool,
}

impl PropertyMetadata {
    /// Creates new `PropertyMetadata` with basic defaults (public, non-static, non-readonly, etc.).
    /// Name is mandatory. Spans, types, and flags can be set using modifier methods.
    #[inline]
    pub fn new(name: VariableIdentifier) -> Self {
        Self {
            name,
            name_span: None,
            span: None,
            read_visibility: Visibility::Public,
            write_visibility: Visibility::Public,
            type_declaration_metadata: None,
            type_metadata: None,
            default_type_metadata: None,
            is_readonly: false,
            has_default: false,
            is_promoted: false,
            is_internal: false,
            is_static: false,
            is_abstract: false,
            is_deprecated: false,
            is_virtual: false,
            is_asymmetric: false, // read == write initially
            allow_private_mutation: false,
        }
    }

    #[inline]
    pub fn set_default_type_metadata(&mut self, default_type_metadata: Option<TypeMetadata>) {
        self.default_type_metadata = default_type_metadata;
    }

    #[inline]
    pub fn set_type_declaration_metadata(&mut self, type_declaration_metadata: Option<TypeMetadata>) {
        if self.type_metadata.is_none() {
            self.type_metadata = type_declaration_metadata.clone();
        }

        self.type_declaration_metadata = type_declaration_metadata;
    }

    #[inline]
    pub fn set_type_metadata(&mut self, type_metadata: Option<TypeMetadata>) {
        self.type_metadata = type_metadata;
    }

    /// Returns a reference to the property's name identifier.
    #[inline]
    pub fn get_name(&self) -> &VariableIdentifier {
        &self.name
    }

    /// Checks if the property is declared with a default value.
    #[inline]
    pub fn has_default(&self) -> bool {
        self.has_default
    }

    /// Checks if the property is declared `static`.
    #[inline]
    pub fn is_static(&self) -> bool {
        self.is_static
    }

    /// Checks if the property is effectively final (private read or write access).
    #[inline]
    pub fn is_final(&self) -> bool {
        self.read_visibility.is_private() || self.write_visibility.is_private()
    }

    /// Sets the span for the property name identifier.
    #[inline]
    pub fn set_name_span(&mut self, name_span: Option<Span>) {
        self.name_span = name_span;
    }

    /// Sets the overall span for the property declaration.
    #[inline]
    pub fn set_span(&mut self, span: Option<Span>) {
        self.span = span;
    }

    /// Sets both read and write visibility levels. Updates `is_asymmetric`. Ensures virtual properties remain symmetric.
    #[inline]
    pub fn set_visibility(&mut self, read: Visibility, write: Visibility) {
        self.read_visibility = read;
        self.write_visibility = write;
        self.update_asymmetric();
    }

    /// Sets whether the property is `readonly`.
    #[inline]
    pub fn set_is_readonly(&mut self, is_readonly: bool) {
        self.is_readonly = is_readonly;
    }

    /// Sets whether the property allows private mutation.
    #[inline]
    pub fn set_allow_private_mutation(&mut self, allow_private_mutation: bool) {
        self.allow_private_mutation = allow_private_mutation;
    }

    /// Sets whether the property has a default value.
    #[inline]
    pub fn set_has_default(&mut self, has_default: bool) {
        self.has_default = has_default;
    }

    /// Sets whether the property originates from constructor promotion.
    #[inline]
    pub fn set_is_promoted(&mut self, is_promoted: bool) {
        self.is_promoted = is_promoted;
    }

    /// Sets whether the property is marked `@internal`.
    #[inline]
    pub fn set_is_internal(&mut self, is_internal: bool) {
        self.is_internal = is_internal;
    }

    /// Sets whether the property is `static`.
    #[inline]
    pub fn set_is_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    /// Sets whether the property represents an abstract requirement.
    #[inline]
    pub fn set_is_abstract(&mut self, is_abstract: bool) {
        self.is_abstract = is_abstract;
    }

    /// Sets whether the property is marked `@deprecated`.
    #[inline]
    pub fn set_is_deprecated(&mut self, is_deprecated: bool) {
        self.is_deprecated = is_deprecated;
    }

    /// Sets whether the property uses property hooks. Updates `is_asymmetric`.
    #[inline]
    pub fn set_is_virtual(&mut self, is_virtual: bool) {
        self.is_virtual = is_virtual;
        self.update_asymmetric();
    }

    /// Also ensures virtual properties are not asymmetric.
    #[inline]
    fn update_asymmetric(&mut self) {
        if self.is_virtual {
            if self.read_visibility != self.write_visibility {
                // If virtual and somehow asymmetric, force symmetry (prefer read)
                self.write_visibility = self.read_visibility;
            }

            self.is_asymmetric = false;
        } else {
            self.is_asymmetric = self.read_visibility != self.write_visibility;
        }
    }
}
