<?php

/**
 * @template K of array-key
 * @template V
 *
 * @param array<K, V> $array
 * @param-out ($array is list ? list<V> : array<K, V>) $array
 *
 * @return V|null
 *
 * @pure
 */
function array_shift(array &$array): mixed
{
    return array_shift($array);
}

/**
 * @template K as array-key
 * @template V
 *
 * @param array<K, V> $array
 * @param V $filter_value
 * @param bool $strict
 *
 * @return list<K>
 *
 * @no-named-arguments
 * @pure
 */
function array_keys(array $array, mixed $filter_value = null, bool $strict = false): array
{
    return array_keys($array, $filter_value, $strict);
}

/**
 * @return int<0, max>
 *
 * @pure
 */
function count(Countable|array $value): int
{
    return count($value);
}

#[Attribute(Attribute::TARGET_METHOD)]
final class Override
{
    public function __construct() {}
}

#[Attribute(Attribute::TARGET_CLASS)]
final class Attribute
{
    public const TARGET_CLASS = 1;
    public const TARGET_FUNCTION = 2;
    public const TARGET_METHOD = 4;
    public const TARGET_PROPERTY = 8;
    public const TARGET_CLASS_CONSTANT = 16;
    public const TARGET_PARAMETER = 32;
    public const TARGET_ALL = 63;
    public const IS_REPEATABLE = 64;

    public int $flags;

    public function __construct(int $flags = self::TARGET_ALL) {}
}

interface Stringable
{
    public function __toString(): string;
}

interface Throwable extends Stringable
{
    public function getMessage(): string;

    /**
     * @return int|string
     */
    public function getCode();

    public function getFile(): string;

    public function getLine(): int;

    public function getTrace(): array;

    public function getTraceAsString(): string;

    public function getPrevious(): Throwable|null;

    /**
     * @return string
     */
    public function __toString();
}

class Exception implements Throwable
{
    protected $message;

    protected $code;

    protected string $file;

    protected int $line;

    /**
     * @pure
     */
    public function __construct(string $message = '', int $code = 0, null|Throwable $previous = null) {}

    /**
     * @mutation-free
     */
    final public function getMessage(): string
    {
    }

    /**
     * @return int|string
     *
     * @mutation-free
     */
    final public function getCode()
    {
    }

    /**
     * @mutation-free
     */
    final public function getFile(): string
    {
    }

    /**
     * @mutation-free
     */
    final public function getLine(): int
    {
    }

    /**
     * @mutation-free
     */
    final public function getTrace(): array
    {
    }

    /**
     * @mutation-free
     */
    final public function getPrevious(): null|Throwable
    {
    }

    /**
     * @mutation-free
     */
    final public function getTraceAsString(): string
    {
    }

    public function __toString(): string
    {
    }

    private function __clone(): void
    {
    }

    public function __wakeup(): void
    {
    }
}

class RuntimeException extends Exception
{
}

class UnderflowException extends RuntimeException
{
}

interface Countable
{
    public function count(): int;
}

/**
 * Returns the largest element of the given list, or null if the
 * list is empty.
 *
 * @template T of int|float
 *
 * @param list<T> $numbers
 *
 * @return ($numbers is non-empty-list<T> ? T : null)
 *
 * @pure
 */
function max_value(array $numbers): null|int|float
{
    return max_value($numbers);
}

/**
 * An interface representing a queue data structure ( FIFO ).
 *
 * @template T
 */
interface QueueInterface extends Countable
{
    /**
     * Adds a node to the queue.
     *
     * @param T $node
     */
    public function enqueue(mixed $node): void;

    /**
     * Retrieves, but does not remove, the node at the head of this queue,
     * or returns null if this queue is empty.
     *
     * @return null|T
     */
    public function peek(): mixed;

    /**
     * Retrieves and removes the node at the head of this queue,
     * or returns null if this queue is empty.
     *
     * @return null|T
     */
    public function pull(): mixed;

    /**
     * Retrieves and removes the node at the head of this queue.
     *
     * @return T
     */
    public function dequeue(): mixed;

    /**
     * Count the nodes in the queue.
     *
     * @return int<0, max>
     */
    #[Override]
    public function count(): int;
}

/**
 * @template T
 *
 * @extends QueueInterface<T>
 */
interface PriorityQueueInterface extends QueueInterface
{
    /**
     * Adds a node to the queue.
     *
     * @param T $node
     */
    #[Override]
    public function enqueue(mixed $node, int $priority = 0): void;
}

/**
 * @template T
 *
 * @implements PriorityQueueInterface<T>
 */
final class PriorityQueue implements PriorityQueueInterface
{
    /**
     * @var array<int, non-empty-list<T>>
     */
    private array $queue = [];

    /**
     * Adds a node to the queue.
     *
     * @param T $node
     *
     * @psalm-external-mutation-free
     */
    #[Override]
    public function enqueue(mixed $node, int $priority = 0): void
    {
        $nodes = $this->queue[$priority] ?? [];
        $nodes[] = $node;

        $this->queue[$priority] = $nodes;
    }

    /**
     * Retrieves, but does not remove, the node at the head of this queue,
     * or returns null if this queue is empty.
     *
     * @return null|T
     *
     * @psalm-mutation-free
     */
    #[Override]
    public function peek(): mixed
    {
        if (0 === $this->count()) {
            return null;
        }

        $keys = array_keys($this->queue);

        // Retrieve the highest priority.
        $priority = max_value($keys) ?? 0;

        // Retrieve the list of nodes with the priority `$priority`.
        $nodes = $this->queue[$priority] ?? [];

        // Retrieve the first node of the list.
        return $nodes[0] ?? null;
    }

    /**
     * Retrieves and removes the node at the head of this queue,
     * or returns null if this queue is empty.
     *
     * @return null|T
     *
     * @psalm-external-mutation-free
     */
    #[Override]
    public function pull(): mixed
    {
        try {
            return $this->dequeue();
        } catch (UnderflowException) {
            return null;
        }
    }

    /**
     * Dequeues a node from the queue.
     *
     * @throws UnderflowException If the queue is empty.
     *
     * @return T
     *
     * @psalm-external-mutation-free
     */
    #[Override]
    public function dequeue(): mixed
    {
        if (0 === $this->count()) {
            throw new UnderflowException('Cannot dequeue a node from an empty queue.');
        }

        /**
         * retrieve the highest priority.
         *
         * @var int
         */
        $priority = max_value(array_keys($this->queue));

        /**
         * retrieve the list of nodes with the priority `$priority`.
         */
        $nodes = $this->queue[$priority];

        /**
         * shift the first node out.
         */
        $node = array_shift($nodes);

        /**
         * If the list contained only this node, remove the list of nodes with priority `$priority`.
         */
        if ([] === $nodes) {
            unset($this->queue[$priority]);

            return $node;
        }

        $this->queue[$priority] = $nodes;

        return $node;
    }

    /**
     * Count the nodes in the queue.
     *
     * @return int<0, max>
     *
     * @psalm-mutation-free
     */
    #[Override]
    public function count(): int
    {
        $count = 0;
        foreach ($this->queue as $list) {
            $count += count($list);
        }

        /** @var int<0, max> */
        return $count;
    }
}
