<?php

// Start of standard v.5.3.2-0.dotdeb.1

use JetBrains\PhpStorm\ArrayShape;
use JetBrains\PhpStorm\Deprecated;
use JetBrains\PhpStorm\ExpectedValues;
use JetBrains\PhpStorm\Internal\LanguageLevelTypeAware;
use JetBrains\PhpStorm\Internal\PhpStormStubsElementAvailable;
use JetBrains\PhpStorm\Internal\TentativeType;
use JetBrains\PhpStorm\Pure;

final class __PHP_Incomplete_Class
{
    /**
     * @var string
     */
    public $__PHP_Incomplete_Class_Name;
}

class php_user_filter
{
    #[LanguageLevelTypeAware(['8.1' => 'string'], default: '')]
    public $filtername;

    #[LanguageLevelTypeAware(['8.1' => 'mixed'], default: '')]
    public $params;
    public $stream;

    /**
     * @link https://php.net/manual/en/php-user-filter.filter.php
     * @param resource $in <p> is a resource pointing to a <i>bucket brigade</i< which contains one or more <i>bucket</i> objects containing data to be filtered.</p>
     * @param resource $out <p>is a resource pointing to a second bucket brigade into which your modified buckets should be placed.</p>
     * @param int &$consumed <p>which must <i>always</i> be declared by reference, should be incremented by the length of the data which your filter reads in and alters. In most cases this means you will increment consumed by <i>$bucket->datalen</i> for each <i>$bucket</i>.</p>
     * @param bool $closing <p>If the stream is in the process of closing (and therefore this is the last pass through the filterchain), the closing parameter will be set to <b>TRUE</b>
     * @return int <p>
     * The <b>filter()</b> method must return one of
     * three values upon completion.
     * </p><table>
     *
     * <thead>
     * <tr>
     * <th>Return Value</th>
     * <th>Meaning</th>
     * </tr>
     *
     * </thead>
     *
     * <tbody class="tbody">
     * <tr>
     * <td><b>PSFS_PASS_ON</b></td>
     * <td>
     * Filter processed successfully with data available in the
     * <code class="parameter">out</code> <em>bucket brigade</em>.
     * </td>
     * </tr>
     *
     * <tr>
     * <td><b>PSFS_FEED_ME</b></td>
     * <td>
     * Filter processed successfully, however no data was available to
     * return. More data is required from the stream or prior filter.
     * </td>
     * </tr>
     *
     * <tr>
     * <td><b>PSFS_ERR_FATAL</b> (default)</td>
     * <td>
     * The filter experienced an unrecoverable error and cannot continue.
     * </td>
     * </tr>
     */
    #[TentativeType]
    public function filter(
        $in,
        $out,
        &$consumed,
        #[LanguageLevelTypeAware(['8.0' => 'bool'], default: '')]  $closing,
    ): int {
    }

    /**
     * @link https://php.net/manual/en/php-user-filter.oncreate.php
     * @return bool
     */
    #[TentativeType]
    public function onCreate(): bool
    {
    }

    /**
     * @link https://php.net/manual/en/php-user-filter.onclose.php
     */
    #[TentativeType]
    public function onClose(): void
    {
    }
}

/**
 * @since 8.4
 */
final class StreamBucket
{
    public $bucket;
    public string $data;
    public int $datalen;
    public int $dataLength;
}

/**
 * Instances of Directory are created by calling the dir() function, not by the new operator.
 */
class Directory
{
    /**
     * @var string The directory that was opened.
     * @since 8.1
     */
    public readonly string $path;

    /**
     * @var resource Can be used with other directory functions such as {@see readdir()}, {@see rewinddir()} and {@see closedir()}.
     * @since 8.1
     */
    public readonly mixed $handle;

    /**
     * Close directory handle.
     * Same as closedir(), only dir_handle defaults to $this.
     * @param resource $dir_handle [optional]
     * @link https://secure.php.net/manual/en/directory.close.php
     */
    #[TentativeType]
    public function close(#[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]  $dir_handle = null): void
    {
    }

    /**
     * Rewind directory handle.
     * Same as rewinddir(), only dir_handle defaults to $this.
     * @param resource $dir_handle [optional]
     * @link https://secure.php.net/manual/en/directory.rewind.php
     */
    #[TentativeType]
    public function rewind(#[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]  $dir_handle = null): void
    {
    }

    /**
     * Read entry from directory handle.
     * Same as readdir(), only dir_handle defaults to $this.
     * @param resource $dir_handle [optional]
     * @return string|false
     * @link https://secure.php.net/manual/en/directory.read.php
     */
    #[TentativeType]
    public function read(#[PhpStormStubsElementAvailable(from: '5.3', to: '7.4')]  $dir_handle = null): string|false
    {
    }
}

/**
 * Returns the value of a constant
 * @link https://php.net/manual/en/function.constant.php
 * @param string $name <p>
 * The constant name.
 * </p>
 * @return mixed the value of the constant.
 * @throws Error If the constant is not defined
 */
#[Pure(true)]
function constant(string $name): mixed
{
}

/**
 * Convert binary data into hexadecimal representation
 * @link https://php.net/manual/en/function.bin2hex.php
 * @param string $string <p>
 * A string.
 * </p>
 * @return string the hexadecimal representation of the given string.
 */
#[Pure]
function bin2hex(string $string): string
{
}

/**
 * Delays the program execution for the given number of seconds
 * @link https://php.net/manual/en/function.sleep.php
 * @param int<0,max> $seconds <p>
 * Halt time in seconds (must be greater than or equal to 0).
 * </p>
 * @return int Returns zero on success.
 * <p>
 * If the call was interrupted by a signal, sleep() returns a
 * non-zero value. On Windows, this value will always be 192
 * (the value of the WAIT_IO_COMPLETION constant within the Windows API).
 * On other platforms, the return value will be the
 * number of seconds left to sleep.
 * </p>
 * <p>
 * As of PHP 8.0, if the specified number of seconds is negative,
 * this function will throw a ValueError.
 * Before PHP 8.0, an E_WARNING was raised instead, and the function returned false.
 * </p>
 */
#[LanguageLevelTypeAware(['8.0' => 'int'], default: 'int|false')]
function sleep(int $seconds)
{
}

/**
 * Delay execution in microseconds
 * @link https://php.net/manual/en/function.usleep.php
 * @param int<0,max> $microseconds <p>
 * Halt time in micro seconds. A micro second is one millionth of a
 * second.
 * </p>
 * @return void
 */
function usleep(int $microseconds): void
{
}

/**
 * Delay for a number of seconds and nanoseconds
 * @link https://php.net/manual/en/function.time-nanosleep.php
 * @param positive-int $seconds <p>
 * Must be a positive integer.
 * </p>
 * @param positive-int $nanoseconds <p>
 * Must be a positive integer less than 1 billion.
 * </p>
 * @return bool|array true on success or false on failure.
 * <p>
 * If the delay was interrupted by a signal, an associative array will be
 * returned with the components:
 * seconds - number of seconds remaining in
 * the delay
 * nanoseconds - number of nanoseconds
 * remaining in the delay
 * </p>
 */
#[ArrayShape(['seconds' => 'int', 'nanoseconds' => 'int'])]
function time_nanosleep(int $seconds, int $nanoseconds): array|bool
{
}

/**
 * Make the script sleep until the specified time
 * @link https://php.net/manual/en/function.time-sleep-until.php
 * @param float $timestamp <p>
 * The timestamp when the script should wake.
 * </p>
 * @return bool true on success or false on failure.
 */
function time_sleep_until(float $timestamp): bool
{
}

/**
 * Parse a time/date generated with <function>strftime</function>
 * @link https://php.net/manual/en/function.strptime.php
 * @param string $timestamp <p>
 * The string to parse (e.g. returned from strftime)
 * </p>
 * @param string $format <p>
 * The format used in date (e.g. the same as
 * used in strftime).
 * </p>
 * <p>
 * For more information about the format options, read the
 * strftime page.
 * </p>
 * @return array|false an array or false on failure.
 * <p>
 * <table>
 * The following parameters are returned in the array
 * <tr valign="top">
 * <td>parameters</td>
 * <td>Description</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_sec"</td>
 * <td>Seconds after the minute (0-61)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_min"</td>
 * <td>Minutes after the hour (0-59)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_hour"</td>
 * <td>Hour since midnight (0-23)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_mday"</td>
 * <td>Day of the month (1-31)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_mon"</td>
 * <td>Months since January (0-11)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_year"</td>
 * <td>Years since 1900</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_wday"</td>
 * <td>Days since Sunday (0-6)</td>
 * </tr>
 * <tr valign="top">
 * <td>"tm_yday"</td>
 * <td>Days since January 1 (0-365)</td>
 * </tr>
 * <tr valign="top">
 * <td>"unparsed"</td>
 * <td>the date part which was not
 * recognized using the specified format</td>
 * </tr>
 * </table>
 * </p>
 * @deprecated 8.1
 */
#[ArrayShape([
    'tm_sec' => 'int',
    'tm_min' => 'int',
    'tm_hour' => 'int',
    'tm_mday' => 'int',
    'tm_mon' => 'int',
    'tm_year' => 'int',
    'tm_wday' => 'int',
    'tm_yday' => 'int',
    'unparsed' => 'string',
])]
function strptime(string $timestamp, string $format): array|false
{
}

function flush(): void
{
}

/**
 * @pure
 */
function wordwrap(string $string, int $width = 75, string $break = "\n", bool $cut_long_words = false): string
{
}

/**
 * @pure
 */
function htmlspecialchars(
    string $string,
    int $flags = ENT_QUOTES | ENT_SUBSTITUTE,
    null|string $encoding = null,
    bool $double_encode = true,
): string {
}

/**
 * @pure
 */
function htmlentities(
    string $string,
    int $flags = ENT_QUOTES | ENT_SUBSTITUTE,
    null|string $encoding,
    bool $double_encode = true,
): string {
}

/**
 * @pure
 */
function html_entity_decode(string $string, int $flags = ENT_QUOTES | ENT_SUBSTITUTE, null|string $encoding): string
{
}

/**
 * @pure
 */
function htmlspecialchars_decode(string $string, int $flags = ENT_QUOTES | ENT_SUBSTITUTE): string
{
}

/**
 * @pure
 */
function get_html_translation_table(
    int $table = 0,
    int $flags = ENT_QUOTES | ENT_SUBSTITUTE,
    string $encoding = 'UTF-8',
): array {
}

/**
 * @pure
 */
function sha1(string $string, bool $binary = false): string
{
}

function sha1_file(string $filename, bool $binary = false): string|false
{
}

/**
 * @pure
 */
function md5(string $string, bool $binary = false): string
{
}

function md5_file(string $filename, bool $binary = false): string|false
{
}

/**
 * @pure
 */
function crc32(string $string): int
{
}

/**
 * @pure
 */
function iptcparse(string $iptc_block): array|false
{
}

function iptcembed(string $iptc_data, string $filename, int $spool = 0): string|bool
{
}

/**
 * Get the size of an image
 * @link https://php.net/manual/en/function.getimagesize.php
 * @param string $filename <p>
 * This parameter specifies the file you wish to retrieve information
 * about. It can reference a local file or (configuration permitting) a
 * remote file using one of the supported streams.
 * </p>
 * @param array &$image_info [optional] <p>
 * This optional parameter allows you to extract some extended
 * information from the image file. Currently, this will return the
 * different JPG APP markers as an associative array.
 * Some programs use these APP markers to embed text information in
 * images. A very common one is to embed
 * IPTC information in the APP13 marker.
 * You can use the iptcparse function to parse the
 * binary APP13 marker into something readable.
 * </p>
 * @return array|false an array with 7 elements.
 * <p>
 * Index 0 and 1 contains respectively the width and the height of the image.
 * </p>
 * <p>
 * Some formats may contain no image or may contain multiple images. In these
 * cases, getimagesize might not be able to properly
 * determine the image size. getimagesize will return
 * zero for width and height in these cases.
 * </p>
 * <p>
 * Index 2 is one of the IMAGETYPE_XXX constants indicating
 * the type of the image.
 * </p>
 * <p>
 * Index 3 is a text string with the correct
 * height="yyy" width="xxx" string that can be used
 * directly in an IMG tag.
 * </p>
 * <p>
 * mime is the correspondant MIME type of the image.
 * This information can be used to deliver images with correct the HTTP
 * Content-type header:
 * getimagesize and MIME types
 * </p>
 * <p>
 * channels will be 3 for RGB pictures and 4 for CMYK
 * pictures.
 * </p>
 * <p>
 * bits is the number of bits for each color.
 * </p>
 * <p>
 * For some image types, the presence of channels and
 * bits values can be a bit
 * confusing. As an example, GIF always uses 3 channels
 * per pixel, but the number of bits per pixel cannot be calculated for an
 * animated GIF with a global color table.
 * </p>
 * <p>
 * On failure, false is returned.
 * </p>
 */
#[ArrayShape([
    0 => 'int',
    1 => 'int',
    2 => 'int',
    3 => 'string',
    'bits' => 'int',
    'channels' => 'int',
    'mime' => 'string',
])]
function getimagesize(string $filename, &$image_info): array|false
{
}

/**
 * Get Mime-Type for image-type returned by getimagesize, exif_read_data, exif_thumbnail, exif_imagetype
 * @link https://php.net/manual/en/function.image-type-to-mime-type.php
 * @param int $image_type <p>
 * One of the IMAGETYPE_XXX constants.
 * </p>
 * @return string The returned values are as follows
 * <table>
 * Returned values Constants
 * <tr valign="top">
 * <td>imagetype</td>
 * <td>Returned value</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_GIF</td>
 * <td>image/gif</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_JPEG</td>
 * <td>image/jpeg</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_PNG</td>
 * <td>image/png</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_SWF</td>
 * <td>application/x-shockwave-flash</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_PSD</td>
 * <td>image/psd</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_BMP</td>
 * <td>image/bmp</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_TIFF_II (intel byte order)</td>
 * <td>image/tiff</td>
 * </tr>
 * <tr valign="top">
 * <td>
 * IMAGETYPE_TIFF_MM (motorola byte order)
 * </td>
 * <td>image/tiff</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_JPC</td>
 * <td>application/octet-stream</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_JP2</td>
 * <td>image/jp2</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_JPX</td>
 * <td>application/octet-stream</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_JB2</td>
 * <td>application/octet-stream</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_SWC</td>
 * <td>application/x-shockwave-flash</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_IFF</td>
 * <td>image/iff</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_WBMP</td>
 * <td>image/vnd.wap.wbmp</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_XBM</td>
 * <td>image/xbm</td>
 * </tr>
 * <tr valign="top">
 * <td>IMAGETYPE_ICO</td>
 * <td>image/vnd.microsoft.icon</td>
 * </tr>
 * </table>
 */
#[Pure]
function image_type_to_mime_type(int $image_type): string
{
}

/**
 * Get file extension for image type
 * @link https://php.net/manual/en/function.image-type-to-extension.php
 * @param int $image_type <p>
 * One of the IMAGETYPE_XXX constant.
 * </p>
 * @param bool $include_dot [optional] <p>
 * Removed since 8.0.
 * Whether to prepend a dot to the extension or not. Default to true.
 * </p>
 * @return string|false A string with the extension corresponding to the given image type, or false on failure.
 */
#[Pure]
function image_type_to_extension(int $image_type, bool $include_dot = true): string|false
{
}

function phpinfo(int $flags = INFO_ALL): bool
{
}

/**
 * @pure
 */
function phpversion(null|string $extension): string|false
{
}

function phpcredits(int $flags = CREDITS_ALL): bool
{
}

/**
 * @return 'cli'|'phpdbg'|'embed'|'apache'|'apache2handler'|'cgi-fcgi'|'cli-server'|'fpm-fcgi'|'litespeed'|false
 * @pure
 */
function php_sapi_name(): string|false
{
}

/**
 * @pure
 */
function php_uname(string $mode = 'a'): string
{
}

/**
 * @pure
 */
function php_ini_scanned_files(): string|false
{
}

/**
 * @pure
 */
function php_ini_loaded_file(): string|false
{
}

/**
 * @pure
 */
function strnatcmp(string $string1, string $string2): int
{
}

/**
 * @pure
 */
function strnatcasecmp(string $string1, string $string2): int
{
}

/**
 * @return int<0,max>
 *
 * @pure
 */
function substr_count(string $haystack, string $needle, int $offset = 0, null|int $length): int
{
}

/**
 * @pure
 */
function strspn(string $string, string $characters, int $offset = 0, null|int $length): int
{
}

/**
 * @pure
 */
function strcspn(string $string, string $characters, int $offset = 0, null|int $length): int
{
}

/**
 * @pure
 */
function strtok(string $string, null|string $token = null): string|false
{
}
