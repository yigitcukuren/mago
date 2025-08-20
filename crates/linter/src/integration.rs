use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Ord, Copy, Clone, Hash, PartialOrd, Deserialize, Serialize)]
pub struct IntegrationSet(u8);

#[derive(Debug, PartialEq, Eq, Ord, Copy, Clone, Hash, PartialOrd, Serialize)]
#[repr(u8)]
pub enum Integration {
    Psl,
    Symfony,
    Laravel,
    PHPUnit,
}

impl IntegrationSet {
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn only(integration: Integration) -> Self {
        let mut s = Self::empty();
        s.0 |= 1 << (integration as u8);
        s
    }

    #[inline]
    pub fn insert(&mut self, integration: Integration) {
        self.0 |= 1 << (integration as u8);
    }

    #[inline]
    pub const fn contains(&self, lib: Integration) -> bool {
        (self.0 & (1 << (lib as u8))) != 0
    }

    #[inline]
    pub const fn is_superset_of(&self, other: IntegrationSet) -> bool {
        (self.0 & other.0) == other.0
    }

    #[inline]
    pub const fn from_slice(xs: &[Integration]) -> Self {
        let mut s = IntegrationSet::empty();
        let mut i = 0;
        while i < xs.len() {
            s.0 |= 1 << (xs[i] as u8);
            i += 1;
        }

        s
    }
}

impl std::str::FromStr for Integration {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.replace('_', "-").to_lowercase().as_str() {
            "psl" | "php-standard-library" => Ok(Integration::Psl),
            "symfony" => Ok(Integration::Symfony),
            "laravel" => Ok(Integration::Laravel),
            "phpunit" | "php-unit" => Ok(Integration::PHPUnit),
            _ => Err("unknown integration, expected one of `Psl`, `Symfony`, `Laravel`, `PHPUnit`"),
        }
    }
}

impl<'de> Deserialize<'de> for Integration {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Visitor;

        struct IntegrationVisitor;

        impl<'de> Visitor<'de> for IntegrationVisitor {
            type Value = Integration;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(
                    "a string representing an Integration, such as 'psl', 'symfony', 'laravel', or 'phpunit'",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                std::str::FromStr::from_str(v).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_str(IntegrationVisitor)
    }
}

impl std::fmt::Display for Integration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Integration::Psl => write!(f, "Psl"),
            Integration::Symfony => write!(f, "Symfony"),
            Integration::Laravel => write!(f, "Laravel"),
            Integration::PHPUnit => write!(f, "PHPUnit"),
        }
    }
}
