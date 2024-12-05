use fennec_ast::*;
use fennec_reflection::constant::ConstantReflection;
use fennec_reflection::identifier::Name;
use fennec_span::*;

use crate::internal::context::Context;

pub fn reflect_constant<'ast>(constant: &'ast Constant, context: &'ast mut Context<'_>) -> Vec<ConstantReflection> {
    let mut reflections = vec![];
    for item in constant.items.iter() {
        let name = context.semantics.names.get(&item.name);

        reflections.push(ConstantReflection {
            name: Name::new(*name, item.name.span),
            type_reflection: fennec_typing::infere(context.interner, context.semantics, &item.value),
            item_span: item.span(),
            definition_span: constant.span(),
            is_populated: false,
        });
    }

    reflections
}
