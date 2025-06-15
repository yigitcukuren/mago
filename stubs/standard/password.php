<?php

const PASSWORD_DEFAULT = '2y';

const PASSWORD_BCRYPT_DEFAULT_COST = 12;

const PASSWORD_BCRYPT = '2y';

const PASSWORD_ARGON2I = 'argon2i';

const PASSWORD_ARGON2ID = 'argon2id';

const PASSWORD_ARGON2_DEFAULT_MEMORY_COST = 65536;

const PASSWORD_ARGON2_DEFAULT_TIME_COST = 4;

const PASSWORD_ARGON2_DEFAULT_THREADS = 1;

const PASSWORD_ARGON2_PROVIDER = 'standard';

/**
 * @return array{
 *   'algo': non-empty-string,
 *   'algoName': non-empty-string,
 *   'options': array{'salt'?: int, 'cost'?: int, 'memory_cost'?: int, 'time_cost'?: int, 'threads'?: int, ...},
 * }
 *
 * @no-named-arguments
 * @pure
 */
function password_get_info(string $hash): array
{
}

/**
 * @param array{'salt'?: int, 'cost'?: int, 'memory_cost'?: int, 'time_cost'?: int, 'threads'?: int, ...} $options
 *
 * @return non-empty-string
 *
 * @no-named-arguments
 * @pure
 */
function password_hash(string $password, string|int|null $algo, array $options = []): string
{
}

/**
 * @param array{'salt'?:int, 'cost'?: int, 'memory_cost'?: int, 'time_cost'?: int, 'threads'?: int, ...} $options
 *
 * @no-named-arguments
 * @pure
 */
function password_needs_rehash(string $hash, string|int|null $algo, array $options = []): bool
{
}

/**
 * @no-named-arguments
 * @pure
 */
function password_verify(string $password, string $hash): bool
{
}

/**
 * @return list<non-empty-string>
 *
 * @no-named-arguments
 * @pure
 */
function password_algos(): array
{
}
