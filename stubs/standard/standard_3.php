<?php

function getlastmod(): int|false
{
}

/**
 * @pure
 */
function base64_decode(string $string, bool $strict = false): string|false
{
}

/**
 * @pure
 */
function base64_encode(string $string): string
{
}

/**
 * @pure
 */
function convert_uuencode(string $string): string
{
}

/**
 * @pure
 */
function convert_uudecode(string $string): string|false
{
}

/**
 * @pure
 */
function abs(int|float $num): int|float
{
}

/**
 * @pure
 */
function ceil(int|float $num): float
{
}

/**
 * @pure
 */
function floor(int|float $num): float
{
}

/**
 * @pure
 */
function round(int|float $num, int $precision = 0, RoundingMode|int $mode = 0): float
{
}

/**
 * @pure
 */
function sin(float $num): float
{
}

/**
 * @pure
 */
function cos(float $num): float
{
}

/**
 * @pure
 */
function tan(float $num): float
{
}

/**
 * @pure
 */
function asin(float $num): float
{
}

/**
 * @pure
 */
function acos(float $num): float
{
}

/**
 * @pure
 */
function atan(float $num): float
{
}

/**
 * @pure
 */
function atanh(float $num): float
{
}

/**
 * @pure
 */
function atan2(float $y, float $x): float
{
}

/**
 * @pure
 */
function sinh(float $num): float
{
}

/**
 * @pure
 */
function cosh(float $num): float
{
}

/**
 * @pure
 */
function tanh(float $num): float
{
}

/**
 * @pure
 */
function asinh(float $num): float
{
}

/**
 * @pure
 */
function acosh(float $num): float
{
}

/**
 * @pure
 */
function expm1(float $num): float
{
}

/**
 * @pure
 */
function log1p(float $num): float
{
}

/**
 * @pure
 */
function pi(): float
{
}

/**
 * @pure
 */
function is_finite(float $num): bool
{
}

/**
 * @pure
 */
function is_nan(float $num): bool
{
}

/**
 * @pure
 *
 * @throws DivisionByZeroError
 * @throws ArithmeticError
 */
function intdiv(int $num1, int $num2): int
{
}

/**
 * @pure
 */
function is_infinite(float $num): bool
{
}

/**
 * @pure
 */
function pow(mixed $num, mixed $exponent): object|int|float
{
}

/**
 * @pure
 */
function exp(float $num): float
{
}

/**
 * @pure
 */
function log(float $num, float $base = M_E): float
{
}

/**
 * @pure
 */
function log10(float $num): float
{
}

/**
 * @pure
 */
function sqrt(float $num): float
{
}

/**
 * @pure
 */
function hypot(float $x, float $y): float
{
}

/**
 * @pure
 */
function deg2rad(float $num): float
{
}

/**
 * @pure
 */
function rad2deg(float $num): float
{
}

/**
 * @pure
 */
function bindec(string $binary_string): int|float
{
}

/**
 * @pure
 */
function hexdec(string $hex_string): int|float
{
}

/**
 * @pure
 */
function octdec(string $octal_string): int|float
{
}

/**
 * @pure
 */
function decbin(int $num): string
{
}

/**
 * @pure
 */
function decoct(int $num): string
{
}

/**
 * @pure
 */
function dechex(int $num): string
{
}

/**
 * @pure
 */
function base_convert(string $num, int $from_base, int $to_base): string
{
}

/**
 * @pure
 */
function number_format(
    float $num,
    int $decimals = 0,
    null|string $decimal_separator = '.',
    null|string $thousands_separator = ',',
): string {
}

/**
 * @pure
 */
function fmod(float $num1, float $num2): float
{
}

/**
 * @pure
 */
function fdiv(float $num1, float $num2): float
{
}

/**
 * @pure
 */
function inet_ntop(string $ip): string|false
{
}

/**
 * @pure
 */
function inet_pton(string $ip): string|false
{
}

/**
 * @pure
 */
function ip2long(string $ip): int|false
{
}

/**
 * @pure
 */
function long2ip(int $ip): string
{
}

/**
 * @return ($name is null ? array<string, string> : string|false)
 */
function getenv(null|string $name = null, bool $local_only = false): array|string|false
{
}

function putenv(string $assignment): bool
{
}

/**
 * @param string $short_options
 * @param list<string> $long_options
 *
 * @return array<string, string>|false
 */
function getopt(string $short_options, array $long_options = [], null|int &$rest_index = null): array|false
{
}

function sys_getloadavg(): array|false
{
}

/**
 * @return ($as_float is true ? float : ($as_float is false ? string : string|float))
 */
function microtime(bool $as_float = false): string|float
{
}

/**
 * @return ($as_float is true ? float : (
 *   $ast_float is false ? array{sec: int, usec: int, minuteswest: int, dsttime: int} : array{sec: int, usec: int, minuteswest: int, dsttime: int}|float
 * ))
 */
function gettimeofday(bool $as_float = false): array|float
{
}

/**
 * @return array<string, scalar>|false
 */
function getrusage(int $mode = 0): array|false
{
}

/**
 * @return non-empty-string
 */
function uniqid(string $prefix = '', bool $more_entropy = false): string
{
}

/**
 * @pure
 */
function quoted_printable_decode(string $string): string
{
}

/**
 * @pure
 */
function quoted_printable_encode(string $string): string
{
}

function get_current_user(): string
{
}

function set_time_limit(int $seconds): bool
{
}

/**
 * @pure
 */
function get_cfg_var(string $option): array|string|false
{
}

/**
 * @deprecated
 */
function get_magic_quotes_runtime(): int
{
}

function error_log(
    string $message,
    int $message_type = 0,
    null|string $destination,
    null|string $additional_headers,
): bool {
}
