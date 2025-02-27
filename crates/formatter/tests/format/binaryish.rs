use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

#[test]
pub fn test_binaryish_ops() {
    let code = indoc! {r#"
        <?php

        var_dump($a **  2);
        var_dump($a *  2);
        var_dump($a /  2);
        var_dump($a %  2);
        var_dump($a +  2);
        var_dump($a -  2);
        var_dump($a <<  2);
        var_dump($a >>  2);
        var_dump($a &  2);
        var_dump($a |  2);
    "#};

    let expected = indoc! {r#"
        <?php

        var_dump($a ** 2);
        var_dump($a * 2);
        var_dump($a / 2);
        var_dump($a % 2);
        var_dump($a + 2);
        var_dump($a - 2);
        var_dump($a << 2);
        var_dump($a >> 2);
        var_dump($a & 2);
        var_dump($a | 2);
    "#};

    test_format(code, expected, FormatSettings::default());
}

#[test]
pub fn test_multiple_concat_operations_in_array() {
    let code = indoc! {r#"
        <?php

        yield [
            $this->getType(),
            "array{'name': string, 'articles': vec<array{" .
                "'title': string, " .
                "'content': string, " .
                "'likes': int, " .
                "'comments'?: vec<array{'user': string, 'comment': string}>" .
                '}>}',
        ];
    "#};

    let expected = indoc! {r#"
        <?php

        yield [
            $this->getType(),
            "array{'name': string, 'articles': vec<array{"
                . "'title': string, "
                . "'content': string, "
                . "'likes': int, "
                . "'comments'?: vec<array{'user': string, 'comment': string}>"
                . '}>}',
        ];
    "#};

    test_format(code, expected, FormatSettings { line_before_binary_operator: true, ..Default::default() });

    let expected = indoc! {r#"
        <?php

        yield [
            $this->getType(),
            "array{'name': string, 'articles': vec<array{" .
                "'title': string, " .
                "'content': string, " .
                "'likes': int, " .
                "'comments'?: vec<array{'user': string, 'comment': string}>" .
                '}>}',
        ];
    "#};

    test_format(code, expected, FormatSettings { line_before_binary_operator: false, ..Default::default() });
}

#[test]
pub fn test_nesting_and_wrapping() {
    let code = indoc! {r#"
        <?php

        $data = [
                 'response_body' => false !== $responseBody && '' !== $responseBody && 'GET' !== $request->getMethod() ? $responseBody : null,
             ];
    "#};

    let expected_super_wide = indoc! {r#"
        <?php

        $data = [
            'response_body' => false !== $responseBody && '' !== $responseBody && 'GET' !== $request->getMethod() ? $responseBody : null,
        ];
    "#};

    test_format(code, expected_super_wide, FormatSettings { print_width: 200, ..Default::default() });

    let expected_wide = indoc! {r#"
        <?php

        $data = [
            'response_body' =>
                false !== $responseBody && '' !== $responseBody && 'GET' !== $request->getMethod() ? $responseBody : null,
        ];
    "#};

    test_format(code, expected_wide, FormatSettings { print_width: 120, ..Default::default() });

    let expected_narrow = indoc! {r#"
        <?php

        $data = [
            'response_body' =>
                false !== $responseBody && '' !== $responseBody && 'GET' !== $request->getMethod()
                    ? $responseBody
                    : null,
        ];
    "#};

    test_format(code, expected_narrow, FormatSettings { print_width: 105, ..Default::default() });

    let expected_more_narrow = indoc! {r#"
        <?php

        $data = [
            'response_body' =>
                false !== $responseBody &&
                '' !== $responseBody &&
                'GET' !== $request->getMethod()
                    ? $responseBody
                    : null,
        ];
    "#};

    test_format(code, expected_more_narrow, FormatSettings { print_width: 80, ..Default::default() });

    let expected_super_narrow = indoc! {r#"
        <?php

        $data = [
            'response_body' =>
                false !== $responseBody &&
                '' !== $responseBody &&
                'GET' !== $request->getMethod()
                    ? $responseBody
                    : null,
        ];
    "#};

    test_format(code, expected_super_narrow, FormatSettings { print_width: 40, ..Default::default() });
}

#[test]
pub fn test_binary_wrapping() {
    let code = indoc! {r#"
        <?php

        if (null === $crudControllerFqcn || null === $routeName = $thisssssss->findRouteName($dashboardControllerFqcndashboardControllerFqcndashboardControllerFqcn)) {
        }

        if ($this->adminRouteGenerator->usesPrettyUrls() && null !== $entityFqcnOrCrudControllerFqcn = $request->query->get(EA::CRUD_CONTROLLER_FQCN)) {
        }
    "#};

    let expected = indoc! {r#"
        <?php

        if (
            null === $crudControllerFqcn ||
            null ===
                ($routeName = $thisssssss->findRouteName(
                    $dashboardControllerFqcndashboardControllerFqcndashboardControllerFqcn,
                ))
        ) {
        }

        if (
            $this->adminRouteGenerator->usesPrettyUrls() &&
            null !== ($entityFqcnOrCrudControllerFqcn = $request->query->get(EA::CRUD_CONTROLLER_FQCN))
        ) {
        }
    "#};

    test_format(code, expected, FormatSettings::default());
}
#[test]
pub fn test_logical_within_parens() {
    let code = indoc! {r#"
        <?php

        $limit = 10;
        $position = 0;
        while (1 < $limit && null !== $position) {
            $limit--;
            $position++;
        }
    "#};

    let expected = indoc! {r#"
        <?php

        $limit = 10;
        $position = 0;
        while (1 < $limit && null !== $position) {
            $limit--;
            $position++;
        }
    "#};

    test_format(code, expected, FormatSettings::default());
}
