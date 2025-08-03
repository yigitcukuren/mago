<?php

/**
 * @psalm-assert-if-true int $value
 */
function is_int($value): bool
{
    return is_int($value);
}

function strlen(string $val): int
{
    return strlen($val);
}

function conditional_type_narrowing(int|string $a): int
{
    return is_int($a) ? ($a + 1) : strlen($a);
}
