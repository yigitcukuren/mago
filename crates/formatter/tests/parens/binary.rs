use indoc::indoc;

use mago_formatter::settings::FormatSettings;
use mago_source::error::SourceError;

use crate::test_format;

#[test]
pub fn test_binary_operand_needs_parens() -> Result<(), SourceError> {
    let code = indoc! {r#"
        <?php

        $a = ((@include $file) === (false));
    "#};

    let expected = indoc! {r#"
        <?php

        $a = (@include $file) === false;
    "#};

    test_format(code, expected, FormatSettings::default())
}
