use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

pub mod assignment;
pub mod binaryish;
pub mod control_structure;
pub mod expression;
pub mod string;

#[test]
pub fn test_shebang() {
    let code = indoc! {r#"
        #!/usr/bin/env php
        <?php

        echo 'Hello, world!';
    "#};

    let expected = indoc! {r#"
        #!/usr/bin/env php
        <?php

        echo 'Hello, world!';
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_html() {
    let code = indoc! {r#"
        <?php

        $a = 1;
        $b = 2;
        $c = $a + $b;

        ?>
        <a>a + b = <?= $c ?></a>
        <?php

        echo 'Hello, world!';
    "#};

    let expected = indoc! {r#"
        <?php

        $a = 1;
        $b = 2;
        $c = $a + $b;

        ?>
        <a>a + b = <?= $c ?></a>
        <?php

        echo 'Hello, world!';
    "#};

    test_format(code, expected, FormatSettings::default())
}
