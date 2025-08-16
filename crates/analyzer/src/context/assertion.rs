use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;

use mago_codex::metadata::CodebaseMetadata;

#[derive(Clone, Copy, Debug)]
pub struct AssertionContext<'a> {
    pub resolved_names: &'a ResolvedNames,
    pub interner: &'a ThreadedInterner,
    pub codebase: &'a CodebaseMetadata,
    pub this_class_name: Option<&'a StringIdentifier>,
}
