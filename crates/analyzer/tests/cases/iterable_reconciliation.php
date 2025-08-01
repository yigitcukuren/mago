<?php

/**
 * @return ($values is iterable ? true : false)
 *
 * @assert-if-true iterable<mixed, mixed> $value
 *
 * @pure
 */
function is_iterable(mixed $value): bool
{
    return is_iterable($value);
}

/**
 * @return array{mixed, mixed}
 *
 * @mago-expect analysis:mixed-assignment
 */
function get_first_pair(mixed $mixed): array
{
    if (is_iterable($mixed)) {
        foreach ($mixed as $k => $v) {
            return [$k, $v];
        }
    }

    return [null, null];
}
