use mago_php_version::PHPVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Settings {
    pub version: PHPVersion,
    pub find_unused_expressions: bool,
    pub find_unused_definitions: bool,
    pub analyze_dead_code: bool,
    pub allow_include: bool,
    pub allow_eval: bool,
    pub allow_empty: bool,
    pub memoize_properties: bool,
    pub trigger_error_exists: bool,
    pub allow_possibly_undefined_array_keys: bool,
    pub diff: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new(PHPVersion::LATEST)
    }
}

impl Settings {
    pub fn new(version: PHPVersion) -> Self {
        Self {
            version,
            find_unused_expressions: false,
            find_unused_definitions: false,
            analyze_dead_code: false,
            allow_include: true,
            allow_eval: true,
            allow_empty: true,
            memoize_properties: true,
            trigger_error_exists: false,
            allow_possibly_undefined_array_keys: true,
            diff: false,
        }
    }
}
