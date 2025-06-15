<?php

use JetBrains\PhpStorm\ArrayShape;
use JetBrains\PhpStorm\Deprecated;
use JetBrains\PhpStorm\ExpectedValues;
use JetBrains\PhpStorm\Internal\LanguageLevelTypeAware;
use JetBrains\PhpStorm\Internal\PhpStormStubsElementAvailable;
use JetBrains\PhpStorm\Pure;

/**
 * @pure
 */
function strtoupper(string $string): string
{
}

/**
 * @pure
 */
function strtolower(string $string): string
{
}

/**
 * @param int<0, max> $offset
 *
 * @return int<0, max>|false
 *
 * @pure
 */
function strpos(string $haystack, string $needle, int $offset = 0): int|false
{
}

/**
 * @return int<0, max>|false
 *
 * @pure
 */
function stripos(string $haystack, string $needle, int $offset = 0): int|false
{
}

/**
 * @return int<0, max>|false
 *
 * @pure
 */
function strrpos(string $haystack, string $needle, int $offset = 0): int|false
{
}

/**
 * @return int<0, max>|false
 *
 * @pure
 */
function strripos(string $haystack, string $needle, int $offset = 0): int|false
{
}

/**
 * @return ($string is non-empty-string ? non-empty-string : string)
 *
 * @pure
 */
function strrev(string $string): string
{
}

/**
 * @pure
 */
function hebrev(string $string, int $max_chars_per_line = 0): string
{
}

/**
 * @pure
 */
function nl2br(string $string, bool $use_xhtml = true): string
{
}

/**
 * @pure
 */
function basename(string $path, string $suffix = ''): string
{
}

/**
 * @pure
 */
function dirname(string $path, #[PhpStormStubsElementAvailable(from: '7.0')] int $levels = 1): string
{
}

/**
 * Returns information about a file path
 * @link https://php.net/manual/en/function.pathinfo.php
 * @param string $path <p>
 * The path being checked.
 * </p>
 * @param int $flags [optional] <p>
 * You can specify which elements are returned with optional parameter
 * options. It composes from
 * PATHINFO_DIRNAME,
 * PATHINFO_BASENAME,
 * PATHINFO_EXTENSION and
 * PATHINFO_FILENAME. It
 * defaults to return all elements.
 * </p>
 * @return string|array{dirname: string, basename: string, extension: string, filename: string} The following associative array elements are returned:
 * dirname, basename,
 * extension (if any), and filename.
 * </p>
 * <p>
 * If options is used, this function will return a
 * string if not all elements are requested.
 */
#[ArrayShape(['dirname' => 'string', 'basename' => 'string', 'extension' => 'string', 'filename' => 'string'])]
function pathinfo(
    string $path,
    #[ExpectedValues(flags: [
        PATHINFO_DIRNAME,
        PATHINFO_BASENAME,
        PATHINFO_EXTENSION,
        PATHINFO_FILENAME,
    ])]
    int $flags = PATHINFO_ALL,
): array|string {
}

/**
 * @pure
 */
function stripslashes(string $string): string
{
}

/**
 * @pure
 */
function stripcslashes(string $string): string
{
}

/**
 * @pure
 */
function strstr(string $haystack, string $needle, bool $before_needle = false): string|false
{
}

/**
 * @pure
 */
function stristr(string $haystack, string $needle, bool $before_needle = false): string|false
{
}

/**
 * @pure
 */
function strrchr(
    string $haystack,
    string $needle,
    #[PhpStormStubsElementAvailable(from: '8.3')] bool $before_needle = false,
): string|false {
}

/**
 * @template T as string
 *
 * @param T $string
 *
 * @return (T is non-empty-string ? non-empty-string : string)
 *
 * @pure
 */
function str_shuffle(string $string): string
{
}

/**
 * @template T as 0|1|2
 *
 * @param T $format
 *
 * @return (T is 0 ? int<0, max> : (
 *   T is 1 ? list<non-empty-string> : (
 *     T is 2 ? array<int<0, max>, non-empty-string> : (
 *       int<0, max>|list<non-empty-string>|array<int<0, max>, non-empty-string>
 *     )
 *   )
 * ))
 *
 * @pure
 */
function str_word_count(string $string, int $format = 0, null|string $characters): array|int
{
}

/**
 * @param int<1, max> $length
 *
 * @return list<string>
 *
 * @pure
 */
function str_split(string $string, int $length = 1): array
{
}

/**
 * @pure
 */
function strpbrk(string $string, string $characters): string|false
{
}

/**
 * @pure
 */
function substr_compare(
    string $haystack,
    string $needle,
    int $offset,
    null|int $length,
    bool $case_insensitive = false,
): int {
}

/**
 * @pure
 */
function strcoll(string $string1, string $string2): int
{
}

/**
 * @param string $string
 * @param null|int<0, max> $length
 *
 * @pure
 */
function substr(string $string, int $offset, null|int $length = null): string
{
}

/**
 * @template K as array-key
 *
 * @param string|array<K, string> $string
 * @param string|array<string> $replace
 * @param int|array<int> $offset
 * @param null|int<0, max>|array<int<0, max>> $length
 *
 * @return ($string is string ? string : array<K, string>)
 *
 * @pure
 */
function substr_replace(
    array|string $string,
    array|string $replace,
    array|int $offset,
    array|int|null $length = null,
): array|string {
}

/**
 * @pure
 */
function quotemeta(string $string): string
{
}

/**
 * @pure
 */
function ucfirst(string $string): string
{
}

/**
 * @pure
 */
function lcfirst(string $string): string
{
}

/**
 * @pure
 */
function ucwords(string $string, string $separators = " \t\r\n\f\v"): string
{
}

/**
 * @pure
 */
function strtr(string $string, string $from, string $to): string
{
}

/**
 * @pure
 */
function strtr(string $str, array $replace_pairs): string
{
}

/**
 * @pure
 */
function addslashes(string $string): string
{
}

/**
 * @pure
 */
function addcslashes(string $string, string $characters): string
{
}

/**
 * @pure
 */
function rtrim(string $string, string $characters = " \n\r\t\v\0"): string
{
}

/**
 * @param string|array<string> $search
 * @param string|array<string> $replace
 * @param string|array<string> $subject
 *
 * @return ($subject is string ? string : (
 *   $subject is array<string> ? array<string> : string|array<string>
 * ))
 *
 * @pure
 */
function str_replace(
    array|string $search,
    array|string $replace,
    array|string $subject,
    null|int &$count = null,
): array|string {
}

/**
 * @param string|array<string> $search
 * @param string|array<string> $replace
 * @param string|array<string> $subject
 *
 * @return ($subject is string ? string : (
 *   $subject is array<string> ? array<string> : string|array<string>
 * ))
 *
 * @pure
 */
function str_ireplace(
    array|string $search,
    array|string $replace,
    array|string $subject,
    null|int &$count = null,
): array|string {
}

/**
 * @pure
 */
function str_repeat(string $string, int $times): string
{
}

/**
 * @pure
 */
function count_chars(string $string, int $mode = 0): array|string
{
}

/**
 * @pure
 */
function chunk_split(string $string, int $length = 76, string $separator = "\r\n"): string
{
}

/**
 * @pure
 */
function trim(string $string, string $characters = " \n\r\t\v\0"): string
{
}

/**
 * @pure
 */
function ltrim(string $string, string $characters = " \n\r\t\v\0"): string
{
}

/**
 * @param array<string>|string|null $allowed_tags
 *
 * @pure
 */
function strip_tags(string $string, string|array|null $allowed_tags = null): string
{
}

/**
 * @pure
 */
function similar_text(string $string1, string $string2, &$percent): int
{
}

/**
 * @return list<non-empty-string>
 *
 * @pure
 */
function explode(string $separator, string $string, int $limit = PHP_INT_MAX): array
{
}

/**
 * Join array elements with a string
 * @link https://php.net/manual/en/function.implode.php
 * @param array|string  $separator [optional]<p>
 * Defaults to an empty string. This is not the preferred usage of
 * implode as glue would be
 * the second parameter and thus, the bad prototype would be used.
 * </p>
 * @param array|null $array <p>
 * The array of strings to implode.
 * </p>
 * @return string a string containing a string representation of all the array
 * elements in the same order, with the glue string between each element.
 */
#[Pure]
function implode(array|string $separator = '', null|array $array): string
{
}

/**
 * Alias:
 * {@see implode}
 * @link https://php.net/manual/en/function.join.php
 * @param array|string  $separator [optional] <p>
 * Defaults to an empty string. This is not the preferred usage of
 * implode as glue would be
 * the second parameter and thus, the bad prototype would be used.
 * </p>
 * @param array|null $array <p>
 * The array of strings to implode.
 * </p>
 * @return string a string containing a string representation of all the array
 * elements in the same order, with the glue string between each element.
 */
#[Pure]
function join(array|string $separator = '', null|array $array): string
{
}

/**
 * Set locale information
 * @link https://php.net/manual/en/function.setlocale.php
 * @param int $category <p>
 * <em>category</em> is a named constant specifying the
 * category of the functions affected by the locale setting:
 * </p><ul>
 * <li>
 * <b>LC_ALL</b> for all of the below
 * </li>
 * <li>
 * <b>LC_COLLATE</b> for string comparison, see
 * {@see strcoll()}
 * </li>
 * <li>
 * <b>LC_CTYPE</b> for character classification and conversion, for
 * example {@see strtoupper()}
 * </li>
 * <li>
 * <b>LC_MONETARY</b> for {@see localeconv()}
 * </li>
 * <li>
 * <b>LC_NUMERIC</b> for decimal separator (See also
 * {@see localeconv()})
 * </li>
 * <li>
 * <b>LC_TIME</b> for date and time formatting with
 * {@see strftime()}
 *
 * </li>
 * <li>
 * <b>LC_MESSAGES</B> for system responses (available if PHP was compiled with
 * <em>libintl</em>)
 *
 * </li>
 * </ul>
 * @param string|string[]|int $locales <p>
 * If locale is null or the empty string
 * "", the locale names will be set from the
 * values of environment variables with the same names as the above
 * categories, or from "LANG".
 * </p>
 * <p>
 * If locale is "0",
 * the locale setting is not affected, only the current setting is returned.
 * </p>
 * <p>
 * If locale is an array or followed by additional
 * parameters then each array element or parameter is tried to be set as
 * new locale until success. This is useful if a locale is known under
 * different names on different systems or for providing a fallback
 * for a possibly not available locale.
 * </p>
 * @param string|string[] ...$rest
 * @return string|false <p>the new current locale, or false if the locale functionality is
 * not implemented on your platform, the specified locale does not exist or
 * the category name is invalid.
 * </p>
 * <p>
 * An invalid category name also causes a warning message. Category/locale
 * names can be found in RFC 1766
 * and ISO 639.
 * Different systems have different naming schemes for locales.
 * </p>
 * <p>
 * The return value of setlocale depends
 * on the system that PHP is running. It returns exactly
 * what the system setlocale function returns.</p>
 */
function setlocale(
    #[ExpectedValues([LC_ALL, LC_COLLATE, LC_CTYPE, LC_MONETARY, LC_NUMERIC, LC_TIME, LC_MESSAGES])] int $category,
    #[PhpStormStubsElementAvailable(from: '8.0')]  $locales,
    #[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]  $rest,
    ...$rest,
): string|false {
}

/**
 * Get numeric formatting information
 * @link https://php.net/manual/en/function.localeconv.php
 * @return array localeconv returns data based upon the current locale
 * as set by setlocale. The associative array that is
 * returned contains the following fields:
 * <tr valign="top">
 * <td>Array element</td>
 * <td>Description</td>
 * </tr>
 * <tr valign="top">
 * <td>decimal_point</td>
 * <td>Decimal point character</td>
 * </tr>
 * <tr valign="top">
 * <td>thousands_sep</td>
 * <td>Thousands separator</td>
 * </tr>
 * <tr valign="top">
 * <td>grouping</td>
 * <td>Array containing numeric groupings</td>
 * </tr>
 * <tr valign="top">
 * <td>int_curr_symbol</td>
 * <td>International currency symbol (i.e. USD)</td>
 * </tr>
 * <tr valign="top">
 * <td>currency_symbol</td>
 * <td>Local currency symbol (i.e. $)</td>
 * </tr>
 * <tr valign="top">
 * <td>mon_decimal_point</td>
 * <td>Monetary decimal point character</td>
 * </tr>
 * <tr valign="top">
 * <td>mon_thousands_sep</td>
 * <td>Monetary thousands separator</td>
 * </tr>
 * <tr valign="top">
 * <td>mon_grouping</td>
 * <td>Array containing monetary groupings</td>
 * </tr>
 * <tr valign="top">
 * <td>positive_sign</td>
 * <td>Sign for positive values</td>
 * </tr>
 * <tr valign="top">
 * <td>negative_sign</td>
 * <td>Sign for negative values</td>
 * </tr>
 * <tr valign="top">
 * <td>int_frac_digits</td>
 * <td>International fractional digits</td>
 * </tr>
 * <tr valign="top">
 * <td>frac_digits</td>
 * <td>Local fractional digits</td>
 * </tr>
 * <tr valign="top">
 * <td>p_cs_precedes</td>
 * <td>
 * true if currency_symbol precedes a positive value, false
 * if it succeeds one
 * </td>
 * </tr>
 * <tr valign="top">
 * <td>p_sep_by_space</td>
 * <td>
 * true if a space separates currency_symbol from a positive
 * value, false otherwise
 * </td>
 * </tr>
 * <tr valign="top">
 * <td>n_cs_precedes</td>
 * <td>
 * true if currency_symbol precedes a negative value, false
 * if it succeeds one
 * </td>
 * </tr>
 * <tr valign="top">
 * <td>n_sep_by_space</td>
 * <td>
 * true if a space separates currency_symbol from a negative
 * value, false otherwise
 * </td>
 * </tr>
 * <td>p_sign_posn</td>
 * <td>
 * 0 - Parentheses surround the quantity and currency_symbol
 * 1 - The sign string precedes the quantity and currency_symbol
 * 2 - The sign string succeeds the quantity and currency_symbol
 * 3 - The sign string immediately precedes the currency_symbol
 * 4 - The sign string immediately succeeds the currency_symbol
 * </td>
 * </tr>
 * <td>n_sign_posn</td>
 * <td>
 * 0 - Parentheses surround the quantity and currency_symbol
 * 1 - The sign string precedes the quantity and currency_symbol
 * 2 - The sign string succeeds the quantity and currency_symbol
 * 3 - The sign string immediately precedes the currency_symbol
 * 4 - The sign string immediately succeeds the currency_symbol
 * </td>
 * </tr>
 * </p>
 * <p>
 * The p_sign_posn, and n_sign_posn contain a string
 * of formatting options. Each number representing one of the above listed conditions.
 * </p>
 * <p>
 * The grouping fields contain arrays that define the way numbers should be
 * grouped. For example, the monetary grouping field for the nl_NL locale (in
 * UTF-8 mode with the euro sign), would contain a 2 item array with the
 * values 3 and 3. The higher the index in the array, the farther left the
 * grouping is. If an array element is equal to CHAR_MAX,
 * no further grouping is done. If an array element is equal to 0, the previous
 * element should be used.
 */
#[ArrayShape([
    'decimal_point' => 'string',
    'thousands_sep' => 'string',
    'grouping' => 'array',
    'int_curr_symbol' => 'string',
    'currency_symbol' => 'string',
    'mon_decimal_point' => 'string',
    'mon_thousands_sep' => 'string',
    'mon_grouping' => 'string',
    'positive_sign' => 'string',
    'negative_sign' => 'string',
    'int_frac_digits' => 'string',
    'frac_digits' => 'string',
    'p_cs_precedes' => 'bool',
    'p_sep_by_space' => 'bool',
    'n_cs_precedes' => 'bool',
    'n_sep_by_space' => 'bool',
    'p_sign_posn' => 'int',
    'n_sign_posn' => 'int',
])]
#[Pure(true)]
function localeconv(): array
{
}
