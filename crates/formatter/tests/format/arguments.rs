use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

#[test]
pub fn test_expand_last_argument() {
    let code = indoc! {r#"
        <?php

        $value = strtr($this->fileExcerpt($trace['file'], $trace['line'], 5), [
            '#DD0000' => 'var(--highlight-string)',
            '#007700' => 'var(--highlight-keyword)',
            '#0000BB' => 'var(--highlight-default)',
            '#FF8000' => 'var(--highlight-comment)',
        ]);
    "#};

    let expected = indoc! {r#"
        <?php

        $value = strtr($this->fileExcerpt($trace['file'], $trace['line'], 5), [
            '#DD0000' => 'var(--highlight-string)',
            '#007700' => 'var(--highlight-keyword)',
            '#0000BB' => 'var(--highlight-default)',
            '#FF8000' => 'var(--highlight-comment)',
        ]);
    "#};

    test_format(code, expected, FormatSettings::default());
}
