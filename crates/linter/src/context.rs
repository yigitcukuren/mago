use mago_collector::Collector;
use mago_database::file::File;
use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;
use mago_php_version::PHPVersion;
use mago_span::HasPosition;

use crate::scope::ScopeStack;

#[derive(Debug)]
pub struct LintContext<'a> {
    pub php_version: PHPVersion,
    pub interner: &'a ThreadedInterner,
    pub source_file: &'a File,
    pub resolved_names: &'a ResolvedNames,
    pub collector: Collector<'a>,
    pub scope: ScopeStack<'a>,
}

impl<'a> LintContext<'a> {
    pub fn new(
        php_version: PHPVersion,
        interner: &'a ThreadedInterner,
        source_file: &'a File,
        resolved_names: &'a ResolvedNames,
        collector: Collector<'a>,
    ) -> Self {
        Self { php_version, interner, source_file, resolved_names, collector, scope: ScopeStack::new() }
    }

    /// Retrieves the string associated with a given identifier.
    ///
    /// # Panics
    ///
    /// Panics if the identifier is not found in the interner.
    pub fn lookup(&self, id: &StringIdentifier) -> &'a str {
        self.interner.lookup(id)
    }

    /// Checks if a name at a given position is imported.
    pub fn is_name_imported(&self, position: &impl HasPosition) -> bool {
        self.resolved_names.is_imported(&position.position())
    }

    /// Retrieves the name associated with a given position in the code.
    ///
    /// # Panics
    ///
    /// Panics if no name is found at the specified position.
    pub fn lookup_name(&self, position: &impl HasPosition) -> &'a str {
        let name_id = self.resolved_names.get(&position.position());

        self.lookup(name_id)
    }
}
