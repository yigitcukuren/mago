<?php

const MB_CASE_UPPER = 0;

const MB_CASE_LOWER = 1;

const MB_CASE_TITLE = 2;

const MB_CASE_FOLD = 3;

const MB_CASE_UPPER_SIMPLE = 4;

const MB_CASE_LOWER_SIMPLE = 5;

const MB_CASE_TITLE_SIMPLE = 6;

const MB_CASE_FOLD_SIMPLE = 7;

const MB_ONIGURUMA_VERSION = '6.9.9';

/**
 * @pure
 */
function mb_convert_case(string $string, int $mode, null|string $encoding): string
{
}

/**
 * @pure
 */
function mb_strtoupper(string $string, null|string $encoding): string
{
}

/**
 * @pure
 */
function mb_strtolower(string $string, null|string $encoding): string
{
}

function mb_language(null|string $language): string|bool
{
}

function mb_internal_encoding(null|string $encoding): string|bool
{
}

/**
 * @param 'G'|'P'|'C'|'S'|'L'|'I'|null $type
 *
 * @return ($type is 'G'|'P'|'C'|'S'|'L'|null ? string|false : list<string>|false)
 */
function mb_http_input(null|string $type): array|string|false
{
}

function mb_http_output(null|string $encoding): string|bool
{
}

/**
 * @return bool|list<string>
 */
function mb_detect_order(array|string|null $encoding = null): array|true
{
}

/**
 * Set/Get substitution character
 * @link https://php.net/manual/en/function.mb-substitute-character.php
 * @param string|int|null $substitute_character [optional] <p>
 * Specify the Unicode value as an integer,
 * or as one of the following strings:</p><ul>
 * <li>"none" : no output</li>
 * <li>"long": Output character code value (Example: U+3000, JIS+7E7E)</li>
 * <li>"entity": Output character entity (Example: Ȁ)</li>
 * </ul>
 * @return bool|int|string If substchar is set, it returns true for success,
 * otherwise returns false.
 * If substchar is not set, it returns the Unicode value,
 * or "none" or "long".
 */
function mb_substitute_character(string|int|null $substitute_character = null): string|int|bool
{
}

/**
 * Parse GET/POST/COOKIE data and set global variable
 * @link https://php.net/manual/en/function.mb-parse-str.php
 * @param string $string <p>
 * The URL encoded data.
 * </p>
 * @param array &$result [optional] <p>
 * An array containing decoded and character encoded converted values.
 * </p>
 * @return bool true on success or false on failure.
 */
#[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]
function mb_parse_str(string $string, &$result): bool
{
}

/**
 * Parse GET/POST/COOKIE data and set global variable
 * @link https://php.net/manual/en/function.mb-parse-str.php
 * @param string $string <p>
 * The URL encoded data.
 * </p>
 * @param array &$result <p>
 * An array containing decoded and character encoded converted values.
 * </p>
 * @return bool true on success or false on failure.
 */
#[PhpStormStubsElementAvailable(from: '8.0')]
function mb_parse_str(string $string, &$result): bool
{
}

/**
 * Callback function converts character encoding in output buffer
 * @link https://php.net/manual/en/function.mb-output-handler.php
 * @param string $string <p>
 * The contents of the output buffer.
 * </p>
 * @param int $status <p>
 * The status of the output buffer.
 * </p>
 * @return string The converted string.
 */
#[Pure]
function mb_output_handler(string $string, int $status): string
{
}

/**
 * @pure
 */
function mb_preferred_mime_name(string $encoding): string|false
{
}

/**
 * @return int<0, max>
 *
 * @pure
 */
function mb_strlen(string $string, null|string $encoding = null): int
{
}

/**
 * @return int<0,max>|false
 *
 * @pure
 */
function mb_strpos(string $haystack, string $needle, int $offset = 0, null|string $encoding): int|false
{
}

/**
 * @return int<0,max>|false
 *
 * @pure
 */
function mb_strrpos(string $haystack, string $needle, int $offset = 0, null|string $encoding): int|false
{
}

/**
 * @return int<0,max>|false
 *
 * @pure
 */
function mb_stripos(string $haystack, string $needle, int $offset = 0, null|string $encoding): int|false
{
}

/**
 * @return int<0,max>|false
 *
 * @pure
 */
function mb_strripos(string $haystack, string $needle, int $offset = 0, null|string $encoding): int|false
{
}

/**
 * Finds first occurrence of a string within another
 * @link https://php.net/manual/en/function.mb-strstr.php
 * @param string $haystack <p>
 * The string from which to get the first occurrence
 * of needle
 * </p>
 * @param string $needle <p>
 * The string to find in haystack
 * </p>
 * @param bool $before_needle [optional] <p>
 * Determines which portion of haystack
 * this function returns.
 * If set to true, it returns all of haystack
 * from the beginning to the first occurrence of needle.
 * If set to false, it returns all of haystack
 * from the first occurrence of needle to the end,
 * </p>
 * @param string|null $encoding [optional] <p>
 * Character encoding name to use.
 * If it is omitted, internal character encoding is used.
 * </p>
 * @return string|false the portion of haystack,
 * or false if needle is not found.
 */
#[Pure]
function mb_strstr(string $haystack, string $needle, bool $before_needle = false, null|string $encoding): string|false
{
}

/**
 * @pure
 */
function mb_strrchr(string $haystack, string $needle, bool $before_needle = false, null|string $encoding): string|false
{
}

/**
 * @pure
 */
function mb_stristr(string $haystack, string $needle, bool $before_needle = false, null|string $encoding): string|false
{
}

/**
 *@pure
 */
function mb_strrichr(string $haystack, string $needle, bool $before_needle = false, null|string $encoding): string|false
{
}

/**
 * @pure
 */
function mb_substr_count(string $haystack, string $needle, null|string $encoding): int
{
}

/**
 * @pure
 */
function mb_substr(string $string, int $start, null|int $length, null|string $encoding): string
{
}

/**
 * @pure
 */
function mb_strcut(string $string, int $start, null|int $length, null|string $encoding): string
{
}

/**
 * @pure
 */
function mb_strwidth(string $string, null|string $encoding): int
{
}

/**
 * @pure
 */
function mb_strimwidth(string $string, int $start, int $width, string $trim_marker = '', null|string $encoding): string
{
}

/**
 * @param string|array<string> $string
 * @param string|array<string>|null $from_encoding
 *
 * @return ($string is string ? string|false : array|false)
 *
 * @pure
 */
function mb_convert_encoding(
    array|string $string,
    string $to_encoding,
    array|string|null $from_encoding = null,
): array|string|false {
}

/**
 * @param string|array<string>|null $encodings
 *
 * @pure
 */
function mb_detect_encoding(string $string, array|string|null $encodings = null, bool $strict = false): string|false
{
}

/**
 * @return list<string>
 *
 * @pure
 */
function mb_list_encodings(): array
{
}

/**
 * @return list<string>
 *
 * @pure
 */
function mb_encoding_aliases(string $encoding): array
{
}

/**
 * @pure
 */
function mb_convert_kana(string $string, string $mode = 'KV', null|string $encoding): string
{
}

/**
 * Encode string for MIME header
 * @link https://php.net/manual/en/function.mb-encode-mimeheader.php
 * @param string $string <p>
 * The string being encoded.
 * </p>
 * @param string|null $charset [optional] <p>
 * charset specifies the name of the character set
 * in which str is represented in. The default value
 * is determined by the current NLS setting (mbstring.language).
 * mb_internal_encoding should be set to same encoding.
 * </p>
 * @param string|null $transfer_encoding [optional] <p>
 * transfer_encoding specifies the scheme of MIME
 * encoding. It should be either "B" (Base64) or
 * "Q" (Quoted-Printable). Falls back to
 * "B" if not given.
 * </p>
 * @param string $newline [optional] <p>
 * linefeed specifies the EOL (end-of-line) marker
 * with which mb_encode_mimeheader performs
 * line-folding (a RFC term,
 * the act of breaking a line longer than a certain length into multiple
 * lines. The length is currently hard-coded to 74 characters).
 * Falls back to "\r\n" (CRLF) if not given.
 * </p>
 * @param int $indent <p>
 * Indentation of the first line (number of characters in the header
 * before str).
 * </p>
 * @return string A converted version of the string represented in ASCII.
 */
#[Pure]
function mb_encode_mimeheader(
    string $string,
    null|string $charset,
    null|string $transfer_encoding,
    string $newline = "\r\n",
    int $indent = 0,
): string {
}

/**
 * Decode string in MIME header field
 * @link https://php.net/manual/en/function.mb-decode-mimeheader.php
 * @param string $string <p>
 * The string being decoded.
 * </p>
 * @return string The decoded string in internal character encoding.
 */
#[Pure]
function mb_decode_mimeheader(string $string): string
{
}

/**
 * Convert character code in variable(s)
 * @link https://php.net/manual/en/function.mb-convert-variables.php
 * @param string $to_encoding <p>
 * The encoding that the string is being converted to.
 * </p>
 * @param string|string[] $from_encoding <p>
 * from_encoding is specified as an array
 * or comma separated string, it tries to detect encoding from
 * from-coding. When from_encoding
 * is omitted, detect_order is used.
 * </p>
 * @param string|array|object &$var var is the reference to the variable being converted.
 * @param string|array|object &...$vars <p>
 * vars is the other references to the
 * variables being converted. String, Array and Object are accepted.
 * mb_convert_variables assumes all parameters
 * have the same encoding.
 * </p>
 * @return string|false The character encoding before conversion for success,
 * or false for failure.
 */
function mb_convert_variables(
    string $to_encoding,
    array|string $from_encoding,
    #[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]  &$vars,
    #[PhpStormStubsElementAvailable(from: '8.0')] mixed &$var,
    mixed &...$vars,
): string|false {
}

/**
 * Encode character to HTML numeric string reference
 * @link https://php.net/manual/en/function.mb-encode-numericentity.php
 * @param string $string <p>
 * The string being encoded.
 * </p>
 * @param int[] $map <p>
 * convmap is array specifies code area to
 * convert.
 * </p>
 * @param null|string $encoding
 * @param bool $hex [optional]
 * @return string The converted string.
 */
#[Pure]
function mb_encode_numericentity(string $string, array $map, null|string $encoding = null, bool $hex = false): string
{
}

/**
 * Decode HTML numeric string reference to character
 * @link https://php.net/manual/en/function.mb-decode-numericentity.php
 * @param string $string <p>
 * The string being decoded.
 * </p>
 * @param int[] $map <p>
 * convmap is an array that specifies
 * the code area to convert.
 * </p>
 * @param null|string $encoding
 * @param bool $is_hex [optional] <p>
 * this parameter is not used.
 * </p>
 * @return string|false|null The converted string.
 */
#[Pure]
#[LanguageLevelTypeAware(['8.0' => 'string'], default: 'string|false|null')]
function mb_decode_numericentity(
    string $string,
    array $map,
    null|string $encoding = null,
    #[PhpStormStubsElementAvailable(from: '7.2', to: '7.4')]  $is_hex = false,
) {
}

/**
 * Send encoded mail
 * @link https://php.net/manual/en/function.mb-send-mail.php
 * @param string $to <p>
 * The mail addresses being sent to. Multiple
 * recipients may be specified by putting a comma between each
 * address in to.
 * This parameter is not automatically encoded.
 * </p>
 * @param string $subject <p>
 * The subject of the mail.
 * </p>
 * @param string $message <p>
 * The message of the mail.
 * </p>
 * @param string|array $additional_headers <p>
 * String or array to be inserted at the end of the email header. <br/>
 * Since 7.2.0 accepts an array. Its keys are the header names and its values are the respective header values.<br/>
 * This is typically used to add extra
 * headers. Multiple extra headers are separated with a
 * newline ("\n").
 * </p>
 * @param string|null $additional_params [optional] <p>
 * additional_parameter is a MTA command line
 * parameter. It is useful when setting the correct Return-Path
 * header when using sendmail.
 * </p>
 * @return bool true on success or false on failure.
 */
function mb_send_mail(
    string $to,
    string $subject,
    string $message,
    array|string $additional_headers = [],
    null|string $additional_params,
): bool {
}

/**
 * Get internal settings of mbstring
 * @link https://php.net/manual/en/function.mb-get-info.php
 * @param string $type [optional] <p>
 * If type isn't specified or is specified to
 * "all", an array having the elements "internal_encoding",
 * "http_output", "http_input", "func_overload", "mail_charset",
 * "mail_header_encoding", "mail_body_encoding" will be returned.
 * </p>
 * <p>
 * If type is specified as "http_output",
 * "http_input", "internal_encoding", "func_overload",
 * the specified setting parameter will be returned.
 * </p>
 * @return array|string|int|false An array of type information if type
 * is not specified, otherwise a specific type.
 */
#[Pure]
#[ArrayShape([
    'internal_encoding' => 'string',
    'http_input' => 'string',
    'http_output' => 'string',
    'http_output_conv_mimetypes' => 'string',
    'mail_charset' => 'string',
    'mail_header_encoding' => 'string',
    'mail_body_encoding' => 'string',
    'illegal_chars' => 'string',
    'encoding_translation' => 'string',
    'language' => 'string',
    'detect_order' => 'string',
    'substitute_character' => 'string',
    'strict_detection' => 'string',
])]
#[LanguageLevelTypeAware(['8.2' => 'array|string|int|false|null'], default: 'array|string|int|false')]
function mb_get_info(string $type = 'all')
{
}

/**
 * Check if the string is valid for the specified encoding
 * @link https://php.net/manual/en/function.mb-check-encoding.php
 * @param string|string[]|null $value [optional] <p>
 * The byte stream to check. If it is omitted, this function checks
 * all the input from the beginning of the request.
 * </p>
 * @param string|null $encoding [optional] <p>
 * The expected encoding.
 * </p>
 * @return bool true on success or false on failure.
 * @since 5.1.3
 */
#[Pure]
function mb_check_encoding(array|string|null $value = null, null|string $encoding): bool
{
}

/**
 * Returns current encoding for multibyte regex as string
 * @link https://php.net/manual/en/function.mb-regex-encoding.php
 * @param string|null $encoding [optional]
 * @return bool|string If encoding is set, then Returns TRUE on success
 * or FALSE on failure. In this case, the internal character encoding
 * is NOT changed. If encoding is omitted, then the current character
 * encoding name for a multibyte regex is returned.
 */
function mb_regex_encoding(null|string $encoding): string|bool
{
}

/**
 * Set/Get the default options for mbregex functions
 * @link https://php.net/manual/en/function.mb-regex-set-options.php
 * @param string|null $options [optional] <p>
 * The options to set.
 * </p>
 * @return string The previous options. If options is omitted,
 * it returns the string that describes the current options.
 */
function mb_regex_set_options(null|string $options): string
{
}

/**
 * Regular expression match with multibyte support
 * @link https://php.net/manual/en/function.mb-ereg.php
 * @param string $pattern <p>
 * The search pattern.
 * </p>
 * @param string $string <p>
 * The search string.
 * </p>
 * @param string[] &$matches [optional] <p>
 * Contains a substring of the matched string.
 * </p>
 * @return bool
 */
function mb_ereg(string $pattern, string $string, &$matches): bool
{
}

/**
 * Regular expression match ignoring case with multibyte support
 * @link https://php.net/manual/en/function.mb-eregi.php
 * @param string $pattern <p>
 * The regular expression pattern.
 * </p>
 * @param string $string <p>
 * The string being searched.
 * </p>
 * @param string[] &$matches [optional] <p>
 * Contains a substring of the matched string.
 * </p>
 * @return bool|int
 */
#[LanguageLevelTypeAware(['8.0' => 'bool'], default: 'false|int')]
function mb_eregi(string $pattern, string $string, &$matches): bool
{
}

/**
 * Replace regular expression with multibyte support
 * @link https://php.net/manual/en/function.mb-ereg-replace.php
 * @param string $pattern <p>
 * The regular expression pattern.
 * </p>
 * <p>
 * Multibyte characters may be used in pattern.
 * </p>
 * @param string $replacement <p>
 * The replacement text.
 * </p>
 * @param string $string <p>
 * The string being checked.
 * </p>
 * @param string|null $options Matching condition can be set by option
 * parameter. If i is specified for this
 * parameter, the case will be ignored. If x is
 * specified, white space will be ignored. If m
 * is specified, match will be executed in multiline mode and line
 * break will be included in '.'. If p is
 * specified, match will be executed in POSIX mode, line break
 * will be considered as normal character. If e
 * is specified, replacement string will be
 * evaluated as PHP expression.
 * <p>PHP 7.1: The <i>e</i> modifier has been deprecated.</p>
 * @return string|false|null The resultant string on success, or false on error.
 */
#[Pure]
function mb_ereg_replace(
    string $pattern,
    string $replacement,
    string $string,
    null|string $options = null,
): string|false|null {
}

/**
 * Perform a regular expresssion seach and replace with multibyte support using a callback
 * @link https://secure.php.net/manual/en/function.mb-ereg-replace-callback.php
 * @param string $pattern <p>
 * The regular expression pattern.
 * </p>
 * <p>
 * Multibyte characters may be used in <b>pattern</b>.
 * </p>
 * @param callable $callback <p>
 * A callback that will be called and passed an array of matched elements
 * in the  <b>subject</b> string. The callback should
 * return the replacement string.
 * </p>
 * <p>
 * You'll often need the <b>callback</b> function
 * for a <b>mb_ereg_replace_callback()</b> in just one place.
 * In this case you can use an anonymous function to
 * declare the callback within the call to
 * <b>mb_ereg_replace_callback()</b>. By doing it this way
 * you have all information for the call in one place and do not
 * clutter the function namespace with a callback function's name
 * not used anywhere else.
 * </p>
 * @param string $string <p>
 * The string being checked.
 * </p>
 * @param string $options <p>
 * Matching condition can be set by <em><b>option</b></em>
 * parameter. If <em>i</em> is specified for this
 * parameter, the case will be ignored. If <em>x</em> is
 * specified, white space will be ignored. If <em>m</em>
 * is specified, match will be executed in multiline mode and line
 * break will be included in '.'. If <em>p</em> is
 * specified, match will be executed in POSIX mode, line break
 * will be considered as normal character. Note that <em>e</em>
 * cannot be used for <b>mb_ereg_replace_callback()</b>.
 * </p>
 * @return string|false|null <p>
 * The resultant string on success, or <b>FALSE</b> on error.
 * </p>
 * @since 5.4.1
 */
function mb_ereg_replace_callback(
    string $pattern,
    callable $callback,
    string $string,
    null|string $options = null,
): string|false|null {
}

/**
 * Replace regular expression with multibyte support ignoring case
 * @link https://php.net/manual/en/function.mb-eregi-replace.php
 * @param string $pattern <p>
 * The regular expression pattern. Multibyte characters may be used. The case will be ignored.
 * </p>
 * @param string $replacement <p>
 * The replacement text.
 * </p>
 * @param string $string <p>
 * The searched string.
 * </p>
 * @param string|null $options option has the same meaning as in
 * mb_ereg_replace.
 * <p>PHP 7.1: The <i>e</i> modifier has been deprecated.</p>
 * @return string|false|null The resultant string or false on error.
 */
#[Pure]
function mb_eregi_replace(
    string $pattern,
    string $replacement,
    string $string,
    #[PhpStormStubsElementAvailable(from: '7.0')] null|string $options = null,
): string|false|null {
}

/**
 * Split multibyte string using regular expression
 * @link https://php.net/manual/en/function.mb-split.php
 * @param string $pattern <p>
 * The regular expression pattern.
 * </p>
 * @param string $string <p>
 * The string being split.
 * </p>
 * @param int $limit [optional] If optional parameter limit is specified,
 * it will be split in limit elements as
 * maximum.
 * @return string[]|false The result as an array.
 */
#[Pure]
function mb_split(string $pattern, string $string, int $limit = -1): array|false
{
}

/**
 * Regular expression match for multibyte string
 * @link https://php.net/manual/en/function.mb-ereg-match.php
 * @param string $pattern <p>
 * The regular expression pattern.
 * </p>
 * @param string $string <p>
 * The string being evaluated.
 * </p>
 * @param string|null $options [optional] <p>
 * </p>
 * @return bool
 */
#[Pure]
function mb_ereg_match(string $pattern, string $string, null|string $options): bool
{
}

/**
 * @pure
 */
function mb_ereg_search(null|string $pattern, null|string $options): bool
{
}

/**
 * @return list<int>|false
 *
 * @pure
 */
function mb_ereg_search_pos(null|string $pattern, null|string $options): array|false
{
}

/**
 * @return list<string>|false
 *
 * @pure
 */
function mb_ereg_search_regs(null|string $pattern, null|string $options): array|false
{
}

function mb_ereg_search_init(string $string, null|string $pattern, null|string $options): bool
{
}

/**
 * @return list<string>|false
 *
 * @pure
 */
function mb_ereg_search_getregs(): array|false
{
}

/**
 * @pure
 */
function mb_ereg_search_getpos(): int
{
}

/**
 * @pure
 */
function mb_ereg_search_setpos(int $offset): bool
{
}

/**
 * @pure
 */
function mb_chr(int $codepoint, null|string $encoding): string|false
{
}

/**
 * @return int<0, max>
 *
 * @pure
 */
function mb_ord(string $string, null|string $encoding): int|false
{
}

/**
 * @pure
 * @deprecated
 */
function mb_scrub(string $string, null|string $encoding): string
{
}

/**
 * @pure
 * @deprecated
 */
function mbereg_search_setpos($position)
{
}

/**
 * @return ($string is non-empty-string ? list<non-empty-string> : list<string>)
 *
 * @pure
 */
function mb_str_split(string $string, int $length = 1, null|string $encoding): array
{
}

function mb_str_pad(
    string $string,
    int $length,
    string $pad_string = ' ',
    int $pad_type = STR_PAD_RIGHT,
    null|string $encoding = null,
): string {
}

function mb_ucfirst(string $string, null|string $encoding = null): string
{
}

function mb_lcfirst(string $string, null|string $encoding = null): string
{
}

function mb_trim(string $string, null|string $characters = null, null|string $encoding = null): string
{
}

function mb_ltrim(string $string, null|string $characters = null, null|string $encoding = null): string
{
}

function mb_rtrim(string $string, null|string $characters = null, null|string $encoding = null): string
{
}
