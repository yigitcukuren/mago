use mago_ast::*;
use mago_fixer::SafetyClassification;
use mago_reporting::*;
use mago_span::HasSpan;
use mago_walker::Walker;

use crate::context::LintContext;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct ReadonlyClassPromotionRule;

impl Rule for ReadonlyClassPromotionRule {
    fn get_name(&self) -> &'static str {
        "readonly-class-promotion"
    }

    fn get_default_level(&self) -> Option<Level> {
        Some(Level::Warning)
    }
}

impl<'a> Walker<LintContext<'a>> for ReadonlyClassPromotionRule {
    fn walk_in_class(&self, class: &Class, context: &mut LintContext<'a>) {
        // Check if the class is already marked readonly
        if class.modifiers.contains_readonly() {
            return;
        }

        let mut all_properties_readonly = true;
        let mut property_count = 0;
        for member in class.members.iter() {
            if let ClassLikeMember::Property(property) = member {
                property_count += 1;
                if !property.modifiers().contains_readonly() {
                    all_properties_readonly = false;
                    break;
                }
            }
        }

        if !all_properties_readonly || property_count == 0 {
            return;
        }

        let annotations = class
            .members
            .iter()
            .filter_map(|member| {
                if let ClassLikeMember::Property(property) = member {
                    property.modifiers().get_readonly().map(|modifier| {
                        Annotation::secondary(modifier.span()).with_message("property is marked as readonly")
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        // Prepare fix plan
        let issue = Issue::new(context.level(), "promote class to readonly")
            .with_annotations(annotations)
            .with_annotation(Annotation::primary(class.span()).with_message("class has all readonly properties"))
            .with_note("classes with all readonly properties can be marked readonly themselves.")
            .with_help("add the `readonly` modifier to the class and remove `readonly` from all properties");

        // Determine safety classification
        let safety = if class.extends.is_some() {
            SafetyClassification::Unsafe
        } else if class.modifiers.contains_final() {
            SafetyClassification::Safe
        } else {
            SafetyClassification::PotentiallyUnsafe
        };

        context.report_with_fix(issue, |plan| {
            // Remove readonly from all properties
            for member in class.members.iter() {
                if let ClassLikeMember::Property(property) = member {
                    if let Some(readonly) = property.modifiers().get_readonly() {
                        plan.delete(readonly.span().to_range(), safety);
                    }
                }
            }

            // Add readonly keyword to the class
            plan.insert(class.class.span.start_position().offset, "readonly ", safety);
        });
    }
}
