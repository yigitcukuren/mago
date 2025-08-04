<?php

/**
 * @template K as array-key
 * @template V
 * @template S
 * @template U
 *
 * @param (callable(V): U)|(callable(V, S): U)|null $callback
 * @param array<K, V> $array
 * @param array<S> ...$arrays
 *
 * @return array<K, U>
 */
function array_map(null|callable $callback, array $array, array ...$arrays): array
{
    return array_map($callback, $array, ...$arrays);
}

function test(string $type, int $zero = 0): int
{
    return $zero;
}

array_map(test(...), ['a', 'b']);
