use mago_ast::Program;
use mago_interner::ThreadedInterner;
use mago_parser::error::ParseError;
use mago_source::error::SourceError;
use mago_source::SourceIdentifier;
use mago_source::SourceManager;

#[derive(Debug)]
pub struct AstService {
    interner: ThreadedInterner,
    source_manager: SourceManager,
}

impl AstService {
    pub fn new(interner: ThreadedInterner, source_manager: SourceManager) -> Self {
        Self { interner, source_manager }
    }

    ///  Parse the given bytes into an AST.
    pub async fn parse(&self, source: SourceIdentifier) -> Result<(Program, Option<ParseError>), SourceError> {
        let source = self.source_manager.load(source)?;

        Ok(mago_parser::parse_source(&self.interner, &source))
    }
}
