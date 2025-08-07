#![allow(dead_code)]

use mago_codex::metadata::CodebaseMetadata;
use mago_codex::reference::ReferenceSource;
use mago_codex::ttype::resolution::TypeResolutionContext;
use mago_collector::Collector;
use mago_database::file::File;
use mago_docblock::document::Document;
use mago_interner::StringIdentifier;
use mago_interner::ThreadedInterner;
use mago_names::ResolvedNames;
use mago_names::scope::NamespaceScope;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_reporting::IssueCollection;
use mago_span::HasSpan;
use mago_span::Span;
use mago_syntax::ast::Trivia;
use mago_syntax::comments;

use crate::analysis_result::AnalysisResult;
use crate::artifacts::AnalysisArtifacts;
use crate::code::Code;
use crate::context::assertion::AssertionContext;
use crate::context::block::BlockContext;
use crate::settings::Settings;

pub mod assertion;
pub mod block;
pub mod scope;

#[derive(Debug)]
pub struct Context<'a> {
    pub(super) interner: &'a ThreadedInterner,
    pub(super) codebase: &'a CodebaseMetadata,
    pub(super) source_file: &'a File,
    pub(super) resolved_names: &'a ResolvedNames,
    pub(super) type_resolution_context: TypeResolutionContext,
    pub(super) comments: &'a [Trivia],
    pub(super) settings: &'a Settings,
    pub(super) scope: NamespaceScope,
    pub(super) collector: Collector<'a>,
    pub(super) statement_span: Span,
}

impl<'a> Context<'a> {
    pub fn new(
        interner: &'a ThreadedInterner,
        codebase: &'a CodebaseMetadata,
        source: &'a File,
        resolved_names: &'a ResolvedNames,
        settings: &'a Settings,
        statement_span: Span,
        comments: &'a [Trivia],
        collector: Collector<'a>,
    ) -> Self {
        Self {
            interner,
            codebase,
            source_file: source,
            resolved_names,
            type_resolution_context: TypeResolutionContext::new(),
            comments,
            settings,
            scope: NamespaceScope::default(),
            statement_span,
            collector,
        }
    }

    pub fn get_assertion_context_from_block<'b>(&'b self, block_context: &'a BlockContext<'_>) -> AssertionContext<'b> {
        self.get_assertion_context(
            block_context.scope.get_class_like_name(),
            block_context.scope.get_reference_source(),
            block_context.inside_loop,
        )
    }

    #[inline]
    pub fn get_assertion_context<'b>(
        &'b self,
        this_class_name: Option<&'a StringIdentifier>,
        reference_source: Option<ReferenceSource>,
        inside_loop: bool,
    ) -> AssertionContext<'b> {
        AssertionContext {
            resolved_names: self.resolved_names,
            interner: self.interner,
            codebase: self.codebase,
            this_class_name,
            type_resolution_context: &self.type_resolution_context,
            reference_source,
            settings: self.settings,
            in_loop: inside_loop,
        }
    }

    pub fn get_docblock(&self) -> Option<&'a Trivia> {
        comments::docblock::get_docblock_before_position(
            self.source_file,
            self.comments,
            self.statement_span.start.offset,
        )
    }

    pub fn get_parsed_docblock(&mut self) -> Option<Document> {
        let trivia = self.get_docblock()?;

        match mago_docblock::parse_trivia(self.interner, trivia) {
            Ok(document) => Some(document),
            Err(error) => {
                let error_span = error.span();

                let mut issue = Issue::error(error.to_string())
                    .with_annotation(
                        Annotation::primary(error_span).with_message("This part of the docblock has a syntax error"),
                    )
                    .with_note(error.note());

                if trivia.span != error_span {
                    issue = issue.with_annotation(
                        Annotation::secondary(trivia.span).with_message("The error is within this docblock"),
                    );
                }

                issue = issue.with_annotation(
                    Annotation::secondary(self.statement_span)
                        .with_message("This docblock is associated with the following statement"),
                );

                issue = issue.with_help(error.help());

                self.collector.report_with_code(Code::INVALID_DOCBLOCK, issue);

                None
            }
        }
    }

    pub fn record<T>(&mut self, callback: impl FnOnce(&mut Context<'a>) -> T) -> (T, IssueCollection) {
        self.collector.start_recording();
        let result = callback(self);
        let issues = self.collector.finish_recording().unwrap_or_default();

        (result, issues)
    }

    pub fn finish(self, artifacts: AnalysisArtifacts, analysis_result: &mut AnalysisResult) {
        analysis_result.issues.extend(self.collector.finish());
        analysis_result.symbol_references.extend(artifacts.symbol_references);
    }
}
