use indoc::indoc;

use mago_formatter::settings::FormatSettings;

use crate::test_format;

#[test]
pub fn test_print_leading_with_missing_prefix() {
    let code = indoc! {r#"
        <?php

        /*
          This is missing prefix
         */
        class Foo
        {
            public function bar(): void
            {
                if (baz()) {
                    /*
                      If allow_reload is configured and the client requests "Cache-Control: no-cache",
                      reload the cache by fetching a fresh response and caching it (if possible).
                     */
                    echo 'Hello, world!';
                }
            }
        }
    "#};

    let expected = indoc! {r#"
        <?php

        /*
         * This is missing prefix
         */
        class Foo
        {
            public function bar(): void
            {
                if (baz()) {
                    /*
                     * If allow_reload is configured and the client requests "Cache-Control: no-cache",
                     * reload the cache by fetching a fresh response and caching it (if possible).
                     */
                    echo 'Hello, world!';
                }
            }
        }
    "#};

    test_format(code, expected, FormatSettings::default())
}
