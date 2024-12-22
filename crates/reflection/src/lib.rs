use std::collections::hash_map::Entry;

use ahash::HashMap;
use ahash::HashSet;
use mago_interner::ThreadedInterner;
use serde::Deserialize;
use serde::Serialize;

use mago_interner::StringIdentifier;
use mago_span::HasPosition;

use crate::class_like::ClassLikeReflection;
use crate::constant::ConstantReflection;
use crate::function_like::FunctionLikeReflection;
use crate::identifier::ClassLikeName;
use crate::identifier::FunctionLikeName;
use crate::identifier::Name;

pub mod assertion;
pub mod attribute;
pub mod class_like;
pub mod constant;
pub mod function_like;
pub mod identifier;
pub mod r#type;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct CodebaseReflection {
    pub constant_reflections: HashMap<Name, ConstantReflection>,
    pub constant_names: HashMap<StringIdentifier, Name>,
    pub constant_names_lowercase: HashMap<StringIdentifier, Name>,

    pub function_like_reflections: HashMap<FunctionLikeName, FunctionLikeReflection>,
    pub function_names: HashMap<StringIdentifier, FunctionLikeName>,
    pub function_names_lowercase: HashMap<StringIdentifier, FunctionLikeName>,

    pub class_like_reflections: HashMap<ClassLikeName, ClassLikeReflection>,
    pub class_like_names: HashMap<StringIdentifier, ClassLikeName>,
    pub class_like_names_lowercase: HashMap<StringIdentifier, ClassLikeName>,

    pub direct_classlike_descendants: HashMap<StringIdentifier, HashSet<StringIdentifier>>,
    pub all_classlike_descendants: HashMap<StringIdentifier, HashSet<StringIdentifier>>,

    pub populated: bool,
}

impl CodebaseReflection {
    /// Creates a new, empty `CodebaseReflection`.
    pub fn new() -> Self {
        Self { populated: false, ..Default::default() }
    }

    /// Registers a new constant in the codebase.
    ///
    /// If the constant already exists, it will not be added again.
    ///
    /// Returns `false` if the constant already exists.
    pub fn register_constant(&mut self, interner: &ThreadedInterner, reflection: ConstantReflection) -> bool {
        let lowercase_name = lower_constant_name(interner, &reflection.name.value);
        if self.constant_names_lowercase.contains_key(&lowercase_name) {
            return false;
        }

        self.constant_names_lowercase.insert(lowercase_name, reflection.name);
        self.constant_names.insert(reflection.name.value, reflection.name);
        self.constant_reflections.insert(reflection.name, reflection);

        true
    }

    /// Registers a new function-like entity in the codebase.
    ///
    /// If the function-like entity already exists, it will not be added again.
    ///
    /// Returns `false` if the function-like entity already exists.
    pub fn register_function_like(&mut self, interner: &ThreadedInterner, reflection: FunctionLikeReflection) -> bool {
        let mut exists = false;

        if let FunctionLikeName::Function(name) = reflection.name {
            let lowercase_name = interner.lowered(&name.value);
            if let Entry::Vacant(e) = self.function_names_lowercase.entry(lowercase_name) {
                self.function_names.insert(name.value, reflection.name);
                e.insert(reflection.name);
            } else {
                exists = true;
            }
        }

        if !exists {
            self.function_like_reflections.insert(reflection.name, reflection);
        }

        exists
    }

    /// Registers a new class-like entity (class, enum, interface, or trait) in the codebase.
    ///
    /// If the class-like entity already exists, it will not be added again.
    ///
    /// Returns `false` if the class-like entity already exists.
    pub fn register_class_like(&mut self, interner: &ThreadedInterner, reflection: ClassLikeReflection) -> bool {
        let mut exists = false;

        match reflection.name {
            ClassLikeName::Class(name) => {
                let lowercase_name = interner.lowered(&name.value);

                if let Entry::Vacant(e) = self.class_like_names_lowercase.entry(lowercase_name) {
                    self.class_like_names.insert(name.value, reflection.name);

                    e.insert(reflection.name);
                } else {
                    exists = true;
                }
            }
            ClassLikeName::Enum(name) => {
                let lowercase_name = interner.lowered(&name.value);

                if let Entry::Vacant(e) = self.class_like_names_lowercase.entry(lowercase_name) {
                    self.class_like_names.insert(name.value, reflection.name);

                    e.insert(reflection.name);
                } else {
                    exists = true;
                }
            }
            ClassLikeName::Interface(name) => {
                let lowercase_name = interner.lowered(&name.value);

                if let Entry::Vacant(e) = self.class_like_names_lowercase.entry(lowercase_name) {
                    self.class_like_names.insert(name.value, reflection.name);

                    e.insert(reflection.name);
                } else {
                    exists = true;
                }
            }
            ClassLikeName::Trait(name) => {
                let lowercase_name = interner.lowered(&name.value);

                if let Entry::Vacant(e) = self.class_like_names_lowercase.entry(lowercase_name) {
                    self.class_like_names.insert(name.value, reflection.name);

                    e.insert(reflection.name);
                } else {
                    exists = true;
                }
            }
            _ => {}
        }

        if !exists {
            self.class_like_reflections.insert(reflection.name, reflection);
        }

        exists
    }

    pub fn constant_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let id = lower_constant_name(interner, id);

        self.constant_names_lowercase.contains_key(&id)
    }

    pub fn function_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let id = interner.lowered(id);

        self.function_names_lowercase.contains_key(&id)
    }

    pub fn class_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let id = interner.lowered(id);

        matches!(self.class_like_names_lowercase.get(&id), Some(ClassLikeName::Class(_)))
    }

    pub fn enum_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let id = interner.lowered(id);

        matches!(self.class_like_names_lowercase.get(&id), Some(ClassLikeName::Enum(_)))
    }

    pub fn interface_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let name = interner.lowered(id);

        matches!(self.class_like_names_lowercase.get(&name), Some(ClassLikeName::Interface(_)))
    }

    pub fn trait_exists(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> bool {
        let id = interner.lowered(id);

        matches!(self.class_like_names_lowercase.get(&id), Some(ClassLikeName::Trait(_)))
    }

    pub fn get_constant(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&ConstantReflection> {
        let id = lower_constant_name(interner, id);

        if let Some(name) = self.constant_names_lowercase.get(&id) {
            self.constant_reflections.get(name)
        } else {
            None
        }
    }

    pub fn get_function_like(&self, name: FunctionLikeName) -> Option<&FunctionLikeReflection> {
        self.function_like_reflections.get(&name)
    }

    /// Retrieves a function by name, if it exists.
    pub fn get_function(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&FunctionLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name) = self.function_names.get(&id) {
            self.function_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Retrieves a closure by its position, if it exists.
    pub fn get_closure(&self, position: &impl HasPosition) -> Option<&FunctionLikeReflection> {
        self.function_like_reflections.iter().find_map(|(identifier, function_like)| match identifier {
            FunctionLikeName::Closure(span) => {
                if span.contains(position) {
                    Some(function_like)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Retrieves an arrow function by its position, if it exists.
    pub fn get_arrow_function(&self, position: &impl HasPosition) -> Option<&FunctionLikeReflection> {
        self.function_like_reflections.iter().find_map(|(identifier, function_like)| match identifier {
            FunctionLikeName::ArrowFunction(span) => {
                if span.contains(position) {
                    Some(function_like)
                } else {
                    None
                }
            }
            _ => None,
        })
    }

    /// Retrieves a class-like entity by its identifier, if it exists.
    pub fn get_class_like(&self, name: ClassLikeName) -> Option<&ClassLikeReflection> {
        self.class_like_reflections.get(&name)
    }

    /// Retrieves a class-like entity by its name, if it exists.
    pub fn get_named_class_like(
        &self,
        interner: &ThreadedInterner,
        id: &StringIdentifier,
    ) -> Option<&ClassLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name) = self.class_like_names_lowercase.get(&id) {
            self.class_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Retrieves a class by name, if it exists.
    pub fn get_class(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&ClassLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name @ ClassLikeName::Class(_)) = self.class_like_names_lowercase.get(&id) {
            self.class_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Retrieves an enum by name, if it exists.
    pub fn get_enum(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&ClassLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name @ ClassLikeName::Enum(_)) = self.class_like_names_lowercase.get(&id) {
            self.class_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Retrieves an interface by name, if it exists.
    pub fn get_interface(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&ClassLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name @ ClassLikeName::Interface(_)) = self.class_like_names.get(&id) {
            self.class_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Retrieves a trait by name, if it exists.
    pub fn get_trait(&self, interner: &ThreadedInterner, id: &StringIdentifier) -> Option<&ClassLikeReflection> {
        let id = interner.lowered(id);

        if let Some(name @ ClassLikeName::Trait(_)) = self.class_like_names.get(&id) {
            self.class_like_reflections.get(name)
        } else {
            None
        }
    }

    /// Returns the function-like reflection (function, closure, etc.) that encloses the given offset.
    ///
    /// This method iterates through the reflections in the codebase, filtering for function-like reflections
    /// that contain the given offset in their definition range. It returns the reflection with the
    /// largest starting offset, effectively finding the innermost function-like reflection containing
    /// the offset.
    ///
    /// # Arguments
    ///
    /// * `has_position` - The position to search for.
    ///
    /// # Returns
    ///
    /// * `Option<&FunctionLikeReflection>` - The enclosing function-like reflection, if found.
    pub fn get_enclosing_function_like(&self, has_position: &impl HasPosition) -> Option<&FunctionLikeReflection> {
        self.function_like_reflections
            .iter()
            .filter(|(_, function_like)| function_like.span.has_offset(has_position.offset()))
            .max_by_key(|(_, function_like)| function_like.span.start.offset)
            .map(|(_, function_like)| function_like)
    }

    /// Returns the class-like reflection (class, trait, etc.) that encloses the given offset.
    ///
    /// This method iterates through the reflections in the codebase, filtering for class-like reflections
    /// that contain the given offset in their definition range. It returns the reflection with the
    /// largest starting offset, effectively finding the innermost class-like reflection containing
    /// the offset.
    ///
    /// # Arguments
    ///
    /// * `has_position` - The position to search for.
    ///
    /// # Returns
    ///
    /// * `Option<&ClassLikeReflection>` - The enclosing class-like reflection, if found.
    pub fn get_enclosing_class_like(&self, has_position: &impl HasPosition) -> Option<&ClassLikeReflection> {
        self.class_like_reflections
            .iter()
            .filter(|(_, class_like)| class_like.span.has_offset(has_position.offset()))
            .max_by_key(|(_, class_like)| class_like.span.start.offset)
            .map(|(_, class_like)| class_like)
    }
}

fn lower_constant_name(interner: &ThreadedInterner, name: &StringIdentifier) -> StringIdentifier {
    let name = interner.lookup(name);

    let mut parts: Vec<_> = name.split('\\').map(str::to_owned).collect();
    let total_parts = parts.len();
    if total_parts > 1 {
        parts = parts
            .into_iter()
            .enumerate()
            .map(|(i, part)| if i < total_parts - 1 { part.to_ascii_lowercase() } else { part })
            .collect::<Vec<_>>();
    }

    interner.intern(parts.join("\\"))
}
