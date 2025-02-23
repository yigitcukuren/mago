use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

pub mod arguments;
pub mod assignment;
pub mod binaryish;
pub mod control_structure;
pub mod expression;
pub mod statement;
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

#[test]
pub fn test_inline_php() {
    let code = indoc! {r#"
        <div class="mx-2 mb-1 mt-<?= $marginTop ?>">
            <span class="px-1 bg-<?= $bgColor ?> text-<?= $fgColor ?>"><?= $title ?></span>
            <span class="<?= $title ? 'ml-1' : '' ?>">
                <?= htmlspecialchars($content) ?>
            </span>
        </div>
    "#};

    let expected = indoc! {r#"
        <div class="mx-2 mb-1 mt-<?= $marginTop ?>">
            <span class="px-1 bg-<?= $bgColor ?> text-<?= $fgColor ?>"><?= $title ?></span>
            <span class="<?= $title ? 'ml-1' : '' ?>">
                <?= htmlspecialchars($content) ?>
            </span>
        </div>
    "#};

    test_format(code, expected, FormatSettings { print_width: 120, ..FormatSettings::default() });
}

#[test]
pub fn test_inline_echo_statements() {
    let code = indoc! {r#"
        <?php if ($foo) { ?>
            <div id="trace-html-<?= $prefix . '-' . $i; ?>" class="trace-code sf-toggle-content">
            <?= strtr($this->fileExcerpt($trace['file'], $trace['line'], 5), [
                '#DD0000' => 'var(--highlight-string)',
                '#007700' => 'var(--highlight-keyword)',
                '#0000BB' => 'var(--highlight-default)',
                '#FF8000' => 'var(--highlight-comment)',
            ]); ?>
            </div>
        <?php } ?>
    "#};

    let expected = indoc! {r#"
        <?php if ($foo) { ?>
            <div id="trace-html-<?= $prefix . '-' . $i; ?>" class="trace-code sf-toggle-content">
            <?= strtr($this->fileExcerpt($trace['file'], $trace['line'], 5), [
                '#DD0000' => 'var(--highlight-string)',
                '#007700' => 'var(--highlight-keyword)',
                '#0000BB' => 'var(--highlight-default)',
                '#FF8000' => 'var(--highlight-comment)',
            ]); ?>
            </div>
        <?php }
    "#};

    test_format(code, expected, FormatSettings::default());
}

#[test]
pub fn test_mixed_html_php() {
    let code = indoc! {r#"
        <?php if ($condition1): ?>
            <div class="outer-div">
                <?php if ($condition2): ?>
                    <p>Some text inside nested condition.</p>
                    <?php for ($i = 0; $i < 5; $i++): ?>
                        <span>Item <?= $i ?></span>
                        <?php if ($i % 2 == 0): ?>
                            <img src="image<?= $i ?>.jpg" alt="Image <?= $i ?>">
                        <?php else: ?>
                            <?php if ($condition3): ?>
                                <a href="link<?= $i ?>">Link <?= $i ?></a>
                            <?php endif; ?>
                        <?php endif; ?>
                    <?php endfor; ?>
                <?php else: ?>
                    <ul>
                      <?php $items = ["one", "two", "three"]; ?>
                      <?php foreach ($items as $item): ?>
                        <li><?= strtoupper($item) ?></li>
                      <?php endforeach; ?>
                    </ul>
                <?php endif; ?>
            </div>
        <?php endif; ?>
    "#};

    let expected = indoc! {r#"
        <?php if ($condition1): ?>
            <div class="outer-div">
                <?php if ($condition2): ?>
                    <p>Some text inside nested condition.</p>
                    <?php for ($i = 0; $i < 5; $i++): ?>
                        <span>Item <?= $i ?></span>
                        <?php if (($i % 2) == 0): ?>
                            <img src="image<?= $i ?>.jpg" alt="Image <?= $i ?>">
                        <?php else: ?>
                            <?php if ($condition3): ?>
                                <a href="link<?= $i ?>">Link <?= $i ?></a>
                            <?php endif; ?>
                        <?php endif; ?>
                    <?php endfor; ?>
                <?php else: ?>
                    <ul>
                      <?php $items = ['one', 'two', 'three']; ?>
                      <?php foreach ($items as $item): ?>
                        <li><?= strtoupper($item) ?></li>
                      <?php endforeach; ?>
                    </ul>
                <?php endif; ?>
            </div>
        <?php endif;
    "#};

    test_format(code, expected, FormatSettings::default());
}

#[test]
pub fn test_php_85_constant_attributes() {
    let code = indoc! {r#"
        <?php

        #[Deprecated]
        const FOO = 'foo';
    "#};

    let expected = indoc! {r#"
        <?php

        #[Deprecated]
        const FOO = 'foo';
    "#};

    test_format(code, expected, FormatSettings::default())
}
