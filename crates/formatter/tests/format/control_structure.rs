use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

#[test]
pub fn test_inline_if() {
    let code = indoc! {r#"
        <html>
            <?php if ($foo): ?>
                Foo
            <?php elseif ($bar): ?>
                Bar
            <?php else: ?>
                Neither
            <?php endif; ?>
        </html>
    "#};

    let expected = indoc! {r#"
        <html>
            <?php if ($foo): ?>
                Foo
            <?php elseif (
                $bar
            ): ?>
                Bar
            <?php else: ?>
                Neither
            <?php endif; ?>
        </html>
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_if_no_indent() {
    let code = indoc! {r#"
        <?php if ($foo): ?>
            Foo
        <?php elseif ($bar): ?>
            Bar
        <?php else: ?>
            Neither
        <?php endif; ?>
    "#};

    let expected = indoc! {r#"
        <?php if ($foo): ?>
            Foo
        <?php elseif ($bar): ?>
            Bar
        <?php else: ?>
            Neither
        <?php endif;
    "#};

    test_format(code, expected, FormatSettings::default())
}

// ---------------------
// WHILE
// ---------------------

#[test]
pub fn test_inline_while() {
    let code = indoc! {r#"
        <html>
            <?php while ($i < 10): ?>
                <?php $i++; ?>
            <?php endwhile; ?>
        </html>
    "#};

    let expected = indoc! {r#"
        <html>
            <?php while ($i < 10): ?>
                <?php $i++; ?>
            <?php endwhile; ?>
        </html>
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_while_no_indent() {
    let code = indoc! {r#"
        <?php while ($i < 10): ?>
            <?php $i++; ?>
        <?php endwhile; ?>
    "#};

    let expected = indoc! {r#"
        <?php while ($i < 10): ?>
            <?php $i++; ?>
        <?php endwhile;
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_foreach() {
    let code = indoc! {r#"
        <html>
            <?php foreach ($items as $item): ?>
                <p><?php echo $item; ?></p>
            <?php endforeach; ?>
        </html>
    "#};

    let expected = indoc! {r#"
        <html>
            <?php foreach ($items as $item): ?>
                <p><?php echo $item; ?></p>
            <?php endforeach; ?>
        </html>
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_foreach_no_indent() {
    let code = indoc! {r#"
        <?php foreach ($items as $item): ?>
            <p><?php echo $item; ?></p>
        <?php endforeach; ?>
    "#};

    let expected = indoc! {r#"
        <?php foreach ($items as $item): ?>
            <p><?php echo $item; ?></p>
        <?php endforeach;
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_for() {
    let code = indoc! {r#"
        <html>
            <?php for ($i = 0; $i < 10; $i++): ?>
                <p><?php echo $i; ?></p>
            <?php endfor; ?>
        </html>
    "#};

    let expected = indoc! {r#"
        <html>
            <?php for ($i = 0; $i < 10; $i++): ?>
                <p><?php echo $i; ?></p>
            <?php endfor; ?>
        </html>
    "#};

    test_format(code, expected, FormatSettings::default())
}

#[test]
pub fn test_inline_for_no_indent() {
    let code = indoc! {r#"
        <?php for ($i = 0; $i < 10; $i++): ?>
            <p><?php echo $i; ?></p>
        <?php endfor; ?>
    "#};

    let expected = indoc! {r#"
        <?php for ($i = 0; $i < 10; $i++): ?>
            <p><?php echo $i; ?></p>
        <?php endfor;
    "#};

    test_format(code, expected, FormatSettings::default())
}
