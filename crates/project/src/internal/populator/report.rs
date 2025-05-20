use mago_interner::ThreadedInterner;
use mago_reflection::Reflection;
use mago_reflection::class_like::ClassLikeReflection;
use mago_reflection::identifier::Name;
use mago_reporting::Annotation;
use mago_reporting::Issue;
use mago_span::HasSpan;

const CODE: &str = "reflection";

#[inline]
pub fn report_missing_parent_class(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    class_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let class_name_value = interner.lookup(&class_name.value);

    reflection.issues.push(
        Issue::error(format!("{class_like_kind} `{class_like_name}` extends undefined class `{class_name_value}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(class_name.span)
                    .with_message(format!("Class `{class_name_value}` does not exist.")),
            )
            .with_help(format!("Ensure the class `{class_name_value}` is defined or imported before extending it.")),
    );
}

#[inline]
pub fn report_parent_not_class(interner: &ThreadedInterner, reflection: &mut ClassLikeReflection, class_name: &Name) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let class_name_value = interner.lookup(&class_name.value);

    reflection.issues.push(
        Issue::error(format!("{class_like_kind} `{class_like_name}` extends non-class `{class_name_value}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(class_name.span).with_message(format!("`{class_name_value}` is not a class.")),
            )
            .with_help(format!("Ensure the class `{class_name_value}` is defined or imported before extending it.")),
    );
}

#[inline]
pub fn report_parent_class_is_final(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    class_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let class_name_value = interner.lookup(&class_name.value);

    reflection.issues.push(
        Issue::error(format!("{class_like_kind} `{class_like_name}` extends final class `{class_name_value}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(class_name.span).with_message(format!("Class `{class_name_value}` is final.")),
            )
            .with_help(format!("Ensure the class `{class_name_value}` is not final or remove the `extends` clause.")),
    );
}

#[inline]
pub fn report_parent_class_is_readonly(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    parent_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_name = reflection.name.get_key(interner);
    let parent_name_value = interner.lookup(&parent_name.value);

    reflection.issues.push(
        Issue::error(format!(
            "Extending readonly class `{parent_name_value}` from non-readonly class `{class_like_name}`."
        ))
        .with_code(CODE)
        .with_annotation(
            Annotation::primary(parent_name.span).with_message(format!("Class `{parent_name_value}` is readonly.")),
        )
        .with_annotation(
            Annotation::secondary(reflection.name.span())
                .with_message(format!("Class `{class_like_name}` is not readonly.")),
        )
        .with_help(format!("Mark the class `{class_like_name}` as readonly or remove the `extends` clause.")),
    );
}

#[inline]
pub fn report_parent_class_is_not_readonly(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    parent_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_name = reflection.name.get_key(interner);
    let parent_name_value = interner.lookup(&parent_name.value);

    reflection.issues.push(
        Issue::error(format!(
            "Cannot extend non-readonly class `{parent_name_value}` from readonly class `{class_like_name}`."
        ))
        .with_code(CODE)
        .with_annotation(
            Annotation::primary(parent_name.span).with_message(format!("Class `{parent_name_value}` is not readonly.")),
        )
        .with_annotation(
            Annotation::secondary(reflection.name.span())
                .with_message(format!("Class `{class_like_name}` is readonly.")),
        )
        .with_help(format!("Ensure the class `{parent_name_value}` is readonly or remove the `extends` clause.")),
    );
}

pub fn report_parent_class_circular_reference(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    parent_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_name_str = reflection.name.get_key(interner);
    let parent_name_str = interner.lookup(&parent_name.value);

    reflection.issues.push(
        Issue::error(format!("Circular inheritance detected between `{class_name_str}` and `{parent_name_str}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(parent_name.span)
                    .with_message(format!("Class `{parent_name_str}` already extends `{class_name_str}`.")),
            )
            .with_help(format!(
                "Ensure there is no circular inheritance between `{class_name_str}` and `{parent_name_str}`."
            )),
    );
}

#[inline]
pub fn report_missing_parent_interface(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    interface_name: &Name,
    implemented: bool,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let interface_name_value = interner.lookup(&interface_name.value);

    reflection.issues.push(
        Issue::error(format!(
            "{} `{}` {} undefined interface `{}`.",
            class_like_kind,
            class_like_name,
            if implemented { "implements" } else { "extends" },
            interface_name_value
        ))
        .with_code(CODE)
        .with_annotation(
            Annotation::primary(interface_name.span)
                .with_message(format!("Interface `{interface_name_value}` does not exist.")),
        )
        .with_help(format!(
            "Ensure the interface `{}` is defined or imported before {} it.",
            interface_name_value,
            if implemented { "implementing" } else { "extending" }
        )),
    );
}

#[inline]
pub fn report_parent_not_interface(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    interface_name: &Name,
    implemented: bool,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let interface_name_value = interner.lookup(&interface_name.value);

    reflection.issues.push(
        Issue::error(format!(
            "{} `{}` {} non-interface `{}`.",
            class_like_kind,
            class_like_name,
            if implemented { "implements" } else { "extends" },
            interface_name_value
        ))
        .with_code(CODE)
        .with_annotation(
            Annotation::primary(interface_name.span)
                .with_message(format!("`{interface_name_value}` is not an interface.")),
        )
        .with_help(format!(
            "Ensure the interface `{}` is defined or imported before {} it.",
            interface_name_value,
            if implemented { "implementing" } else { "extending" }
        )),
    );
}

#[inline]
pub fn report_parent_interface_circular_reference(
    interner: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    interface_name: &Name,
    implemented: bool,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_name_str = reflection.name.get_key(interner);
    let interface_name_str = interner.lookup(&interface_name.value);

    reflection.issues.push(
        Issue::error(format!("Circular inheritance detected between `{class_name_str}` and `{interface_name_str}`."))
            .with_code(CODE)
            .with_annotation(Annotation::primary(interface_name.span).with_message(format!(
                "Interface `{}` already {} `{}`.",
                interface_name_str,
                if implemented { "implements" } else { "extends" },
                class_name_str
            )))
            .with_help(format!(
                "Ensure there is no circular inheritance between `{class_name_str}` and `{interface_name_str}`."
            )),
    );
}

#[inline]
pub fn report_missing_trait(interner: &ThreadedInterner, reflection: &mut ClassLikeReflection, trait_name: &Name) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let trait_name_value = interner.lookup(&trait_name.value);

    reflection.issues.push(
        Issue::error(format!("{class_like_kind} `{class_like_name}` uses undefined trait `{trait_name_value}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(trait_name.span)
                    .with_message(format!("Trait `{trait_name_value}` does not exist.")),
            )
            .with_help(format!("Ensure the trait `{trait_name_value}` is defined or imported before using it.")),
    );
}

#[inline]
pub fn report_not_trait(interner: &ThreadedInterner, reflection: &mut ClassLikeReflection, trait_name: &Name) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_like_kind = reflection.name.get_kind();
    let class_like_name = reflection.name.get_key(interner);
    let trait_name_value = interner.lookup(&trait_name.value);

    reflection.issues.push(
        Issue::error(format!("{class_like_kind} `{class_like_name}` uses non-trait `{trait_name_value}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(trait_name.span).with_message(format!("`{trait_name_value}` is not a trait.")),
            )
            .with_help(format!("Ensure the trait `{trait_name_value}` is defined or imported before using it.")),
    );
}

#[inline]
pub fn report_trait_circular_reference(
    inter: &ThreadedInterner,
    reflection: &mut ClassLikeReflection,
    trait_name: &Name,
) {
    if !reflection.is_user_defined() {
        return;
    }

    let class_name_str = reflection.name.get_key(inter);
    let trait_name_str = inter.lookup(&trait_name.value);

    reflection.issues.push(
        Issue::error(format!("Circular inheritance detected between `{class_name_str}` and `{trait_name_str}`."))
            .with_code(CODE)
            .with_annotation(
                Annotation::primary(trait_name.span)
                    .with_message(format!("Trait `{trait_name_str}` already uses `{class_name_str}`.")),
            )
            .with_help(format!(
                "Ensure there is no circular inheritance between `{class_name_str}` and `{trait_name_str}`."
            )),
    );
}
