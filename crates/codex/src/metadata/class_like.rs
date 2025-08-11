use ahash::HashMap;
use ahash::HashSet;
use ahash::RandomState;
use indexmap::IndexMap;
use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_reporting::Issue;
use mago_span::Span;

use crate::flags::attribute::AttributeFlags;
use crate::metadata::attribute::AttributeMetadata;
use crate::metadata::class_like_constant::ClassLikeConstantMetadata;
use crate::metadata::enum_case::EnumCaseMetadata;
use crate::metadata::flags::MetadataFlags;
use crate::metadata::property::PropertyMetadata;
use crate::misc::GenericParent;
use crate::symbol::SymbolKind;
use crate::ttype::atomic::TAtomic;
use crate::ttype::template::variance::Variance;
use crate::ttype::union::TUnion;
use crate::visibility::Visibility;

type TemplateTuple = (StringIdentifier, Vec<(GenericParent, TUnion)>);

/// Contains comprehensive metadata for a PHP class-like structure (class, interface, trait, enum).
///
/// Aggregates information about inheritance, traits, generics, methods, properties, constants,
/// attributes, docblock tags, analysis flags, and more.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassLikeMetadata {
    pub name: StringIdentifier,
    pub original_name: StringIdentifier,
    pub span: Span,
    pub direct_parent_interfaces: HashSet<StringIdentifier>,
    pub all_parent_interfaces: HashSet<StringIdentifier>,
    pub direct_parent_class: Option<StringIdentifier>,
    pub require_extends: HashSet<StringIdentifier>,
    pub require_implements: HashSet<StringIdentifier>,
    pub all_parent_classes: HashSet<StringIdentifier>,
    pub used_traits: HashSet<StringIdentifier>,
    pub trait_alias_map: HashMap<StringIdentifier, StringIdentifier>,
    pub trait_visibility_map: HashMap<StringIdentifier, Visibility>,
    pub trait_final_map: HashSet<StringIdentifier>,
    pub child_class_likes: Option<HashSet<StringIdentifier>>,
    pub name_span: Option<Span>,
    pub kind: SymbolKind,
    pub template_types: Vec<TemplateTuple>,
    pub template_readonly: HashSet<StringIdentifier>,
    pub template_variance: HashMap<usize, Variance>,
    pub template_extended_offsets: HashMap<StringIdentifier, Vec<TUnion>>,
    pub template_extended_parameters: HashMap<StringIdentifier, IndexMap<StringIdentifier, TUnion, RandomState>>,
    pub template_type_extends_count: HashMap<StringIdentifier, usize>,
    pub template_type_implements_count: HashMap<StringIdentifier, usize>,
    pub template_type_uses_count: HashMap<StringIdentifier, usize>,
    pub methods: HashSet<StringIdentifier>,
    pub pseudo_methods: HashSet<StringIdentifier>,
    pub static_pseudo_methods: HashSet<StringIdentifier>,
    pub declaring_method_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub appearing_method_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub overridden_method_ids: HashMap<StringIdentifier, HashSet<StringIdentifier>>,
    pub inheritable_method_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub potential_declaring_method_ids: HashMap<StringIdentifier, HashSet<StringIdentifier>>,
    pub properties: HashMap<StringIdentifier, PropertyMetadata>,
    pub appearing_property_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub declaring_property_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub inheritable_property_ids: HashMap<StringIdentifier, StringIdentifier>,
    pub overridden_property_ids: HashMap<StringIdentifier, HashSet<StringIdentifier>>,
    pub initialized_properties: HashSet<StringIdentifier>,
    pub constants: IndexMap<StringIdentifier, ClassLikeConstantMetadata, RandomState>,
    pub enum_cases: IndexMap<StringIdentifier, EnumCaseMetadata, RandomState>,
    pub invalid_dependencies: HashSet<StringIdentifier>,
    pub attributes: Vec<AttributeMetadata>,
    pub enum_type: Option<TAtomic>,
    pub has_sealed_methods: Option<bool>,
    pub has_sealed_properties: Option<bool>,
    pub permitted_inheritors: Option<HashSet<StringIdentifier>>,
    pub issues: Vec<Issue>,
    pub attribute_flags: Option<AttributeFlags>,
    pub flags: MetadataFlags,
}

impl ClassLikeMetadata {
    pub fn new(
        name: StringIdentifier,
        original_name: StringIdentifier,
        span: Span,
        name_span: Option<Span>,
        flags: MetadataFlags,
    ) -> ClassLikeMetadata {
        ClassLikeMetadata {
            constants: IndexMap::with_hasher(RandomState::new()),
            enum_cases: IndexMap::with_hasher(RandomState::new()),
            flags,
            kind: SymbolKind::Class,
            direct_parent_interfaces: HashSet::default(),
            all_parent_classes: HashSet::default(),
            appearing_method_ids: HashMap::default(),
            attributes: Vec::new(),
            all_parent_interfaces: HashSet::default(),
            declaring_method_ids: HashMap::default(),
            appearing_property_ids: HashMap::default(),
            declaring_property_ids: HashMap::default(),
            direct_parent_class: None,
            require_extends: HashSet::default(),
            require_implements: HashSet::default(),
            inheritable_method_ids: HashMap::default(),
            enum_type: None,
            inheritable_property_ids: HashMap::default(),
            initialized_properties: HashSet::default(),
            invalid_dependencies: HashSet::default(),
            span,
            name_span,
            methods: HashSet::default(),
            pseudo_methods: HashSet::default(),
            static_pseudo_methods: HashSet::default(),
            overridden_method_ids: HashMap::default(),
            overridden_property_ids: HashMap::default(),
            potential_declaring_method_ids: HashMap::default(),
            properties: HashMap::default(),
            template_variance: HashMap::default(),
            template_type_extends_count: HashMap::default(),
            template_extended_parameters: HashMap::default(),
            template_extended_offsets: HashMap::default(),
            template_type_implements_count: HashMap::default(),
            template_type_uses_count: HashMap::default(),
            template_types: Vec::default(),
            used_traits: HashSet::default(),
            trait_alias_map: HashMap::default(),
            trait_visibility_map: HashMap::default(),
            trait_final_map: HashSet::default(),
            name,
            original_name,
            child_class_likes: None,
            template_readonly: HashSet::default(),
            has_sealed_methods: None,
            has_sealed_properties: None,
            permitted_inheritors: None,
            issues: vec![],
            attribute_flags: None,
        }
    }

    /// Returns a reference to the map of trait method aliases.
    #[inline]
    pub fn get_trait_alias_map(&self) -> &HashMap<StringIdentifier, StringIdentifier> {
        &self.trait_alias_map
    }

    /// Returns a vector of the generic type parameter names.
    #[inline]
    pub fn get_template_type_names(&self) -> Vec<StringIdentifier> {
        self.template_types.iter().map(|(name, _)| *name).collect()
    }

    /// Returns type parameters for a specific generic parameter name.
    #[inline]
    pub fn get_template_type(&self, name: &StringIdentifier) -> Option<&Vec<(GenericParent, TUnion)>> {
        self.template_types.iter().find_map(|(param_name, types)| if param_name == name { Some(types) } else { None })
    }

    /// Returns type parameters for a specific generic parameter name with its index.
    #[inline]
    pub fn get_template_type_with_index(
        &self,
        name: &StringIdentifier,
    ) -> Option<(usize, &Vec<(GenericParent, TUnion)>)> {
        self.template_types
            .iter()
            .enumerate()
            .find_map(|(index, (param_name, types))| if param_name == name { Some((index, types)) } else { None })
    }

    pub fn get_template_for_index(&self, index: usize) -> Option<(StringIdentifier, &Vec<(GenericParent, TUnion)>)> {
        self.template_types.get(index).map(|(name, types)| (*name, types))
    }

    pub fn get_template_name_for_index(&self, index: usize) -> Option<StringIdentifier> {
        self.template_types.get(index).map(|(name, _)| *name)
    }

    pub fn get_template_index_for_name(&self, name: &StringIdentifier) -> Option<usize> {
        self.template_types.iter().position(|(param_name, _)| param_name == name)
    }

    /// Checks if a specific parent is either a parent class or interface.
    #[inline]
    pub fn has_parent(&self, parent: &StringIdentifier) -> bool {
        self.all_parent_classes.contains(parent) || self.all_parent_interfaces.contains(parent)
    }

    /// Checks if a specific parent has template extended parameters.
    #[inline]
    pub fn has_template_extended_parameter(&self, parent: &StringIdentifier) -> bool {
        self.template_extended_parameters.contains_key(parent)
    }

    /// Checks if a specific method appears in this class-like.
    #[inline]
    pub fn has_appearing_method(&self, method: &StringIdentifier) -> bool {
        self.appearing_method_ids.contains_key(method)
    }

    /// Returns a reference to a specific method's potential declaring classes/traits.
    #[inline]
    pub fn get_potential_declaring_method_id(&self, method: &StringIdentifier) -> Option<&HashSet<StringIdentifier>> {
        self.potential_declaring_method_ids.get(method)
    }

    /// Returns a vector of property names.
    #[inline]
    pub fn get_property_names(&self) -> HashSet<StringIdentifier> {
        self.properties.keys().copied().collect()
    }

    /// Checks if a specific property appears in this class-like.
    #[inline]
    pub fn has_appearing_property(&self, name: &StringIdentifier) -> bool {
        self.appearing_property_ids.contains_key(name)
    }

    /// Checks if a specific property is declared in this class-like.
    #[inline]
    pub fn has_declaring_property(&self, name: &StringIdentifier) -> bool {
        self.declaring_property_ids.contains_key(name)
    }

    /// Takes ownership of the issues found for this class-like structure.
    #[inline]
    pub fn take_issues(&mut self) -> Vec<Issue> {
        std::mem::take(&mut self.issues)
    }

    /// Adds a single direct parent interface.
    #[inline]
    pub fn add_direct_parent_interface(&mut self, interface: StringIdentifier) {
        self.direct_parent_interfaces.insert(interface);
        self.all_parent_interfaces.insert(interface);
    }

    /// Adds a single interface to the list of all parent interfaces. Use with caution, normally derived.
    #[inline]
    pub fn add_all_parent_interface(&mut self, interface: StringIdentifier) {
        self.all_parent_interfaces.insert(interface);
    }

    /// Adds multiple interfaces to the list of all parent interfaces. Use with caution.
    #[inline]
    pub fn add_all_parent_interfaces(&mut self, interfaces: impl IntoIterator<Item = StringIdentifier>) {
        self.all_parent_interfaces.extend(interfaces);
    }

    /// Adds multiple ancestor classes. Use with caution.
    #[inline]
    pub fn add_all_parent_classes(&mut self, classes: impl IntoIterator<Item = StringIdentifier>) {
        self.all_parent_classes.extend(classes);
    }

    /// Adds a single used trait. Returns `true` if the trait was not already present.
    #[inline]
    pub fn add_used_trait(&mut self, trait_name: StringIdentifier) -> bool {
        self.used_traits.insert(trait_name)
    }

    /// Adds multiple used traits.
    #[inline]
    pub fn add_used_traits(&mut self, traits: impl IntoIterator<Item = StringIdentifier>) {
        self.used_traits.extend(traits);
    }

    /// Adds or updates a single trait alias. Returns the previous original name if one existed for the alias.
    #[inline]
    pub fn add_trait_alias(&mut self, method: StringIdentifier, alias: StringIdentifier) -> Option<StringIdentifier> {
        self.trait_alias_map.insert(method, alias)
    }

    /// Adds or updates a single trait visibility override. Returns the previous visibility if one existed.
    #[inline]
    pub fn add_trait_visibility(&mut self, method: StringIdentifier, visibility: Visibility) -> Option<Visibility> {
        self.trait_visibility_map.insert(method, visibility)
    }

    /// Adds a single template type definition.
    #[inline]
    pub fn add_template_type(&mut self, template: TemplateTuple) {
        self.template_types.push(template);
    }

    /// Adds or updates the variance for a specific parameter index. Returns the previous variance if one existed.
    #[inline]
    pub fn add_template_variance_parameter(&mut self, index: usize, variance: Variance) -> Option<Variance> {
        self.template_variance.insert(index, variance)
    }

    /// Adds or replaces the offset types for a specific template parameter name.
    #[inline]
    pub fn add_template_extended_offset(&mut self, name: StringIdentifier, types: Vec<TUnion>) -> Option<Vec<TUnion>> {
        self.template_extended_offsets.insert(name, types)
    }

    /// Adds or replaces the resolved parameters for a specific parent FQCN.
    #[inline]
    pub fn extend_template_extended_parameters(
        &mut self,
        template_extended_parameters: HashMap<StringIdentifier, IndexMap<StringIdentifier, TUnion, RandomState>>,
    ) {
        self.template_extended_parameters.extend(template_extended_parameters);
    }

    /// Adds or replaces a single resolved parameter for the parent FQCN.
    #[inline]
    pub fn add_template_extended_parameter(
        &mut self,
        parent_fqcn: StringIdentifier,
        parameter_name: StringIdentifier,
        parameter_type: TUnion,
    ) -> Option<TUnion> {
        self.template_extended_parameters.entry(parent_fqcn).or_default().insert(parameter_name, parameter_type)
    }

    /// Adds or updates the declaring class FQCN for a method name.
    #[inline]
    pub fn add_declaring_method_id(
        &mut self,
        method: StringIdentifier,
        declaring_fqcn: StringIdentifier,
    ) -> Option<StringIdentifier> {
        self.add_appearing_method_id(method, declaring_fqcn);
        self.declaring_method_ids.insert(method, declaring_fqcn)
    }

    /// Adds or updates the appearing class FQCN for a method name.
    #[inline]
    pub fn add_appearing_method_id(
        &mut self,
        method: StringIdentifier,
        appearing_fqcn: StringIdentifier,
    ) -> Option<StringIdentifier> {
        self.appearing_method_ids.insert(method, appearing_fqcn)
    }

    /// Adds a parent FQCN to the set for an overridden method. Initializes set if needed. Returns `true` if added.
    #[inline]
    pub fn add_overridden_method_parent(&mut self, method: StringIdentifier, parent_fqcn: StringIdentifier) -> bool {
        self.overridden_method_ids.entry(method).or_default().insert(parent_fqcn)
    }

    /// Adds a potential declaring FQCN to the set for a method. Initializes set if needed. Returns `true` if added.
    #[inline]
    pub fn add_potential_declaring_method(
        &mut self,
        method: StringIdentifier,
        potential_fqcn: StringIdentifier,
    ) -> bool {
        self.potential_declaring_method_ids.entry(method).or_default().insert(potential_fqcn)
    }

    /// Adds or updates a property's metadata. Returns the previous metadata if the property existed.
    #[inline]
    pub fn add_property(
        &mut self,
        name: StringIdentifier,
        property_metadata: PropertyMetadata,
    ) -> Option<PropertyMetadata> {
        let class_name = self.name;

        self.add_declaring_property_id(name, class_name);
        if property_metadata.flags.has_default() {
            self.initialized_properties.insert(name);
        }

        if !property_metadata.is_final() {
            self.inheritable_property_ids.insert(name, class_name);
        }

        self.properties.insert(name, property_metadata)
    }

    /// Adds or updates a property's metadata using just the property metadata. Returns the previous metadata if the property existed.
    #[inline]
    pub fn add_property_metadata(&mut self, property_metadata: PropertyMetadata) -> Option<PropertyMetadata> {
        let name = property_metadata.get_name().0;

        self.add_property(name, property_metadata)
    }

    /// Adds or updates the declaring class FQCN for a property name.
    #[inline]
    pub fn add_declaring_property_id(
        &mut self,
        prop: StringIdentifier,
        declaring_fqcn: StringIdentifier,
    ) -> Option<StringIdentifier> {
        self.appearing_property_ids.insert(prop, declaring_fqcn);
        self.declaring_property_ids.insert(prop, declaring_fqcn)
    }

    pub fn get_missing_required_interface<'a>(&self, other: &'a ClassLikeMetadata) -> Option<&'a StringIdentifier> {
        for required_interface in &other.require_implements {
            if self.all_parent_interfaces.contains(required_interface) {
                continue;
            }

            if (self.flags.is_abstract() || self.kind.is_trait())
                && self.require_implements.contains(required_interface)
            {
                continue; // Abstract classes and traits can require interfaces they implement
            }

            return Some(required_interface);
        }

        None
    }

    pub fn get_missing_required_extends<'a>(&self, other: &'a ClassLikeMetadata) -> Option<&'a StringIdentifier> {
        for required_extend in &other.require_extends {
            if self.all_parent_classes.contains(required_extend) {
                continue;
            }

            if self.kind.is_interface() && self.all_parent_interfaces.contains(required_extend) {
                continue;
            }

            if (self.flags.is_abstract() || self.kind.is_trait()) && self.require_extends.contains(required_extend) {
                continue; // Abstract classes and traits can require classes they extend
            }

            return Some(required_extend);
        }

        None
    }

    pub fn is_permitted_to_inherit(&self, other: &ClassLikeMetadata) -> bool {
        if self.kind.is_trait() || self.flags.is_abstract() {
            return true; // Traits and abstract classes can always inherit
        }

        let Some(permitted_inheritors) = &other.permitted_inheritors else {
            return true; // No restrictions, inheriting is allowed
        };

        if permitted_inheritors.contains(&self.name) {
            return true; // This class-like is explicitly permitted to inherit
        }

        self.all_parent_interfaces.iter().any(|parent_interface| permitted_inheritors.contains(parent_interface))
            || self.all_parent_classes.iter().any(|parent_class| permitted_inheritors.contains(parent_class))
            || self.used_traits.iter().any(|used_trait| permitted_inheritors.contains(used_trait))
    }

    #[inline]
    pub fn mark_as_populated(&mut self) {
        self.flags |= MetadataFlags::POPULATED;
        self.shrink_to_fit();
    }

    #[inline]
    pub fn shrink_to_fit(&mut self) {
        self.properties.shrink_to_fit();
        self.initialized_properties.shrink_to_fit();
        self.appearing_property_ids.shrink_to_fit();
        self.declaring_property_ids.shrink_to_fit();
        self.inheritable_property_ids.shrink_to_fit();
        self.overridden_property_ids.shrink_to_fit();
        self.appearing_method_ids.shrink_to_fit();
        self.declaring_method_ids.shrink_to_fit();
        self.inheritable_method_ids.shrink_to_fit();
        self.overridden_method_ids.shrink_to_fit();
        self.potential_declaring_method_ids.shrink_to_fit();
        self.attributes.shrink_to_fit();
        self.constants.shrink_to_fit();
        self.enum_cases.shrink_to_fit();
    }
}
