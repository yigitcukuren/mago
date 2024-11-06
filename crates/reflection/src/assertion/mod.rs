/// This module is derived from Hakana (https://github.com/slackhq/hakana) and has been modified
/// for the purposes of this project. The original license is included below.
///
/// Author(s):
/// -  Matthew Brown ( https://github.com/muglug )
///
/// License (MIT) Copyright (c) 2022 Slack Technologies, Inc.
/// - https://github.com/slackhq/hakana/blob/cd9b46548e8fa9e540cdd28fc0ec71c21a4837b5/LICENSE
use serde::Deserialize;
use serde::Serialize;

use fennec_interner::ThreadedInterner;

use crate::r#type::kind::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Assertion {
    Any,
    IsType(TypeKind),
    IsNotType(TypeKind),
    Falsy,
    Truthy,
    IsEqual(TypeKind),
    IsNotEqual(TypeKind),
    IsGreaterThan(TypeKind),
    IsGreaterThanOrEqual(TypeKind),
    IsLessThan(TypeKind),
    IsLessThanOrEqual(TypeKind),
    IsEqualIsset,
    IsIsset,
    IsNotIsset,
    HasStringArrayAccess,
    HasIntOrStringArrayAccess,
    ArrayKeyExists,
    ArrayKeyDoesNotExist,
    InArray(TypeKind),
    NotInArray(TypeKind),
    HasArrayKey(ArrayShapePropertyKey),
    DoesNotHaveArrayKey(ArrayShapePropertyKey),
    HasNonnullEntryForKey(ArrayShapePropertyKey),
    DoesNotHaveNonnullEntryForKey(ArrayShapePropertyKey),
    NonEmptyCountable(bool),
    EmptyCountable,
    HasExactCount(usize),
    DoesNotHaveExactCount(usize),
}

impl Assertion {
    pub fn to_string(&self, interner: &ThreadedInterner) -> String {
        match self {
            Assertion::Any => "any".to_string(),
            Assertion::Falsy => "falsy".to_string(),
            Assertion::Truthy => "truthy".to_string(),
            Assertion::IsType(atomic) => atomic.get_key(interner),
            Assertion::IsNotType(atomic) => "!".to_string() + &atomic.get_key(interner),
            Assertion::IsEqual(atomic) => "=".to_string() + &atomic.get_key(interner),
            Assertion::IsNotEqual(atomic) => "!=".to_string() + &atomic.get_key(interner),
            Assertion::IsGreaterThan(atomic) => ">".to_string() + &atomic.get_key(interner),
            Assertion::IsGreaterThanOrEqual(atomic) => ">=".to_string() + &atomic.get_key(interner),
            Assertion::IsLessThan(atomic) => "<".to_string() + &atomic.get_key(interner),
            Assertion::IsLessThanOrEqual(atomic) => "<=".to_string() + &atomic.get_key(interner),
            Assertion::IsEqualIsset => "=isset".to_string(),
            Assertion::IsIsset => "isset".to_string(),
            Assertion::IsNotIsset => "!isset".to_string(),
            Assertion::HasStringArrayAccess => "=string-array-access".to_string(),
            Assertion::HasIntOrStringArrayAccess => "=int-or-string-array-access".to_string(),
            Assertion::ArrayKeyExists => "array-key-exists".to_string(),
            Assertion::ArrayKeyDoesNotExist => "!array-key-exists".to_string(),
            Assertion::HasArrayKey(key) => "=has-array-key-".to_string() + key.get_key(interner).as_str(),
            Assertion::DoesNotHaveArrayKey(key) => "!=has-array-key-".to_string() + key.get_key(interner).as_str(),
            Assertion::HasNonnullEntryForKey(key) => {
                "=has-nonnull-entry-for-".to_string() + key.get_key(interner).as_str()
            }
            Assertion::DoesNotHaveNonnullEntryForKey(key) => {
                "!=has-nonnull-entry-for-".to_string() + key.get_key(interner).as_str()
            }
            Assertion::InArray(union) => "=in-array-".to_string() + &union.get_key(interner),
            Assertion::NotInArray(union) => "!=in-array-".to_string() + &union.get_key(interner),
            Assertion::NonEmptyCountable(negatable) => {
                if *negatable {
                    "non-empty-countable".to_string()
                } else {
                    "=non-empty-countable".to_string()
                }
            }
            Assertion::EmptyCountable => "empty-countable".to_string(),
            Assertion::HasExactCount(number) => "has-exactly-".to_string() + &number.to_string(),
            Assertion::DoesNotHaveExactCount(number) => "!has-exactly-".to_string() + &number.to_string(),
        }
    }

    pub fn has_negation(&self) -> bool {
        matches!(
            self,
            Assertion::Falsy
                | Assertion::IsNotType(_)
                | Assertion::IsNotEqual(_)
                | Assertion::IsNotIsset
                | Assertion::NotInArray(..)
                | Assertion::ArrayKeyDoesNotExist
                | Assertion::DoesNotHaveArrayKey(_)
                | Assertion::DoesNotHaveExactCount(_)
                | Assertion::DoesNotHaveNonnullEntryForKey(_)
                | Assertion::EmptyCountable
        )
    }

    pub fn has_isset(&self) -> bool {
        matches!(
            self,
            Assertion::IsIsset | Assertion::ArrayKeyExists | Assertion::HasStringArrayAccess | Assertion::IsEqualIsset
        )
    }

    pub fn has_non_isset_equality(&self) -> bool {
        matches!(
            self,
            Assertion::InArray(_)
                | Assertion::HasIntOrStringArrayAccess
                | Assertion::HasStringArrayAccess
                | Assertion::IsEqual(_)
        )
    }

    pub fn has_equality(&self) -> bool {
        matches!(
            self,
            Assertion::InArray(_)
                | Assertion::HasIntOrStringArrayAccess
                | Assertion::HasStringArrayAccess
                | Assertion::IsEqualIsset
                | Assertion::IsEqual(_)
                | Assertion::IsNotEqual(_)
        )
    }

    pub fn has_literal_string_or_int(&self) -> bool {
        match self {
            Assertion::IsEqual(kind)
            | Assertion::IsNotEqual(kind)
            | Assertion::IsType(kind)
            | Assertion::IsNotType(kind) => {
                matches!(kind, TypeKind::Value(ValueTypeKind::String { .. } | ValueTypeKind::Integer { .. }))
            }
            _ => false,
        }
    }

    pub fn get_type(&self) -> Option<&TypeKind> {
        match self {
            Assertion::IsEqual(kind)
            | Assertion::IsNotEqual(kind)
            | Assertion::IsType(kind)
            | Assertion::IsNotType(kind) => Some(kind),
            _ => None,
        }
    }

    pub fn is_negation_of(&self, other: &Assertion) -> bool {
        match self {
            Assertion::Any => false,
            Assertion::Falsy => matches!(other, Assertion::Truthy),
            Assertion::Truthy => matches!(other, Assertion::Falsy),
            Assertion::IsType(atomic) => match other {
                Assertion::IsNotType(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsNotType(atomic) => match other {
                Assertion::IsType(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsEqual(atomic) => match other {
                Assertion::IsNotEqual(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsNotEqual(atomic) => match other {
                Assertion::IsEqual(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsGreaterThan(atomic) => match other {
                Assertion::IsLessThanOrEqual(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsLessThanOrEqual(atomic) => match other {
                Assertion::IsGreaterThan(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsGreaterThanOrEqual(atomic) => match other {
                Assertion::IsLessThan(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsLessThan(atomic) => match other {
                Assertion::IsGreaterThanOrEqual(other_atomic) => other_atomic == atomic,
                _ => false,
            },
            Assertion::IsEqualIsset => false,
            Assertion::IsIsset => matches!(other, Assertion::IsNotIsset),
            Assertion::IsNotIsset => matches!(other, Assertion::IsIsset),
            Assertion::HasStringArrayAccess => false,
            Assertion::HasIntOrStringArrayAccess => false,
            Assertion::ArrayKeyExists => matches!(other, Assertion::ArrayKeyDoesNotExist),
            Assertion::ArrayKeyDoesNotExist => matches!(other, Assertion::ArrayKeyExists),
            Assertion::HasArrayKey(str) => match other {
                Assertion::DoesNotHaveArrayKey(other_str) => other_str == str,
                _ => false,
            },
            Assertion::DoesNotHaveArrayKey(str) => match other {
                Assertion::HasArrayKey(other_str) => other_str == str,
                _ => false,
            },
            Assertion::HasNonnullEntryForKey(str) => match other {
                Assertion::DoesNotHaveNonnullEntryForKey(other_str) => other_str == str,
                _ => false,
            },
            Assertion::DoesNotHaveNonnullEntryForKey(str) => match other {
                Assertion::HasNonnullEntryForKey(other_str) => other_str == str,
                _ => false,
            },
            Assertion::InArray(union) => match other {
                Assertion::NotInArray(other_union) => other_union == union,
                _ => false,
            },
            Assertion::NotInArray(union) => match other {
                Assertion::InArray(other_union) => other_union == union,
                _ => false,
            },
            Assertion::NonEmptyCountable(negatable) => {
                if *negatable {
                    matches!(other, Assertion::EmptyCountable)
                } else {
                    false
                }
            }
            Assertion::EmptyCountable => matches!(other, Assertion::NonEmptyCountable(true)),
            Assertion::HasExactCount(number) => match other {
                Assertion::DoesNotHaveExactCount(other_number) => other_number == number,
                _ => false,
            },
            Assertion::DoesNotHaveExactCount(number) => match other {
                Assertion::HasExactCount(other_number) => other_number == number,
                _ => false,
            },
        }
    }

    pub fn get_negation(&self) -> Self {
        match self {
            Assertion::Any => Assertion::Any,
            Assertion::Falsy => Assertion::Truthy,
            Assertion::IsType(atomic) => Assertion::IsNotType(atomic.clone()),
            Assertion::IsNotType(atomic) => Assertion::IsType(atomic.clone()),
            Assertion::Truthy => Assertion::Falsy,
            Assertion::IsEqual(atomic) => Assertion::IsNotEqual(atomic.clone()),
            Assertion::IsNotEqual(atomic) => Assertion::IsEqual(atomic.clone()),
            Assertion::IsGreaterThan(atomic) => Assertion::IsLessThanOrEqual(atomic.clone()),
            Assertion::IsLessThanOrEqual(atomic) => Assertion::IsGreaterThan(atomic.clone()),
            Assertion::IsGreaterThanOrEqual(atomic) => Assertion::IsLessThan(atomic.clone()),
            Assertion::IsLessThan(atomic) => Assertion::IsGreaterThanOrEqual(atomic.clone()),
            Assertion::IsIsset => Assertion::IsNotIsset,
            Assertion::IsNotIsset => Assertion::IsIsset,
            Assertion::NonEmptyCountable(negatable) => {
                if *negatable {
                    Assertion::EmptyCountable
                } else {
                    Assertion::Any
                }
            }
            Assertion::EmptyCountable => Assertion::NonEmptyCountable(true),
            Assertion::ArrayKeyExists => Assertion::ArrayKeyDoesNotExist,
            Assertion::ArrayKeyDoesNotExist => Assertion::ArrayKeyExists,
            Assertion::InArray(union) => Assertion::NotInArray(union.clone()),
            Assertion::NotInArray(union) => Assertion::InArray(union.clone()),
            Assertion::HasExactCount(size) => Assertion::DoesNotHaveExactCount(*size),
            Assertion::DoesNotHaveExactCount(size) => Assertion::HasExactCount(*size),
            Assertion::HasArrayKey(str) => Assertion::DoesNotHaveArrayKey(str.clone()),
            Assertion::DoesNotHaveArrayKey(str) => Assertion::HasArrayKey(str.clone()),
            Assertion::HasNonnullEntryForKey(str) => Assertion::DoesNotHaveNonnullEntryForKey(str.clone()),
            Assertion::DoesNotHaveNonnullEntryForKey(str) => Assertion::HasNonnullEntryForKey(str.clone()),
            Assertion::HasStringArrayAccess => Assertion::Any,
            Assertion::HasIntOrStringArrayAccess => Assertion::Any,
            Assertion::IsEqualIsset => Assertion::Any,
        }
    }
}
