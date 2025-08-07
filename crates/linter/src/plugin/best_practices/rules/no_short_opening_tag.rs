use indoc::indoc;

use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct NoShortOpeningTag;

impl Rule for NoShortOpeningTag {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("No Short Opening Tag", Level::Warning)
            .with_description(indoc! {"
                Disallows the use of short opening tags (`<?`).

                While `<?` is a valid PHP opening tag, its availability depends on the
                `short_open_tag` directive in `php.ini`. If this setting is disabled on a
                server, any code within the short tags will be exposed as plain text to the
                browser, which is a significant security risk.

                Using the full `<?php` opening tag is the only guaranteed portable way to
                ensure your code is always interpreted as PHP.
            "})
            .with_example(RuleUsageExample::invalid(
                "Using a short opening tag",
                indoc! {r#"
                    <?

                    echo "Hello, World!";
                "#},
            ))
            .with_example(RuleUsageExample::valid(
                "Using the full opening tag",
                indoc! {r#"
                    <?php

                    echo "Hello, World!";
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::ShortOpeningTag(opening_tag) = node else {
            return LintDirective::default();
        };

        let issue =
            Issue::new(context.level(), "Avoid using the short opening tag `<?`")
                .with_annotation(
                    Annotation::primary(opening_tag.span()).with_message("Short opening tag used here"),
                )
                .with_note(
                    "This tag's behavior depends on the `short_open_tag` setting in `php.ini`.",
                )
                .with_note(
                    "If disabled on the server, the enclosed PHP code will be exposed as plain text, creating a security vulnerability.",
                )
                .with_help("Always use the full `<?php` opening tag for portability and security.");

        context.propose(issue, |fix| {
            fix.replace(opening_tag.span.to_range(), "<?php", SafetyClassification::Safe);
        });

        LintDirective::Prune
    }
}
