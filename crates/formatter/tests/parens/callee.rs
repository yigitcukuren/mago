use indoc::indoc;

use mago_formatter::settings::FormatSettings;
use mago_source::error::SourceError;

use crate::test_format;

#[test]
pub fn test_callee_needs_parens() -> Result<(), SourceError> {
    let code = indoc! {r#"
        <?php

        $a = (foo())->bar();
        $c = (foo())->bar;
        $d = (foo())::bar();
        $e = (foo())[0];
        $f = new (foo())();
    "#};

    let expected = indoc! {r#"
        <?php

        $a = foo()->bar();
        $c = foo()->bar;
        $d = foo()::bar();
        $e = foo()[0];
        $f = new (foo())();
    "#};

    test_format(code, expected, FormatSettings::default())
}
