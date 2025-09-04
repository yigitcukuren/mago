<?php

/**
 * @template K
 * @template-covariant V
 *
 * @inheritors IteratorAggregate|Iterator
 */
interface Traversable
{
}

/**
 * @template K
 * @template-covariant V
 *
 * @extends Traversable<K, V>
 */
interface IteratorAggregate extends Traversable
{
    /**
     * @return Traversable<K, V>
     *
     * @throws Exception
     */
    public function getIterator(): Traversable;
}

/**
 * @template K
 * @template-covariant V
 *
 * @extends Traversable<K, V>
 */
interface Iterator extends Traversable
{
    /**
     * @return null|V
     */
    public function current(): mixed;

    public function next(): void;

    /**
     * @return null|K
     */
    public function key(): mixed;

    public function valid(): bool;

    public function rewind(): void;
}

interface Throwable
{
}

class Exception implements Throwable
{
    /**
     * @mago-expect analysis:unhandled-thrown-type
     */
    public function __construct(string $message = '')
    {
        throw new Exception($message);
    }
}

/**
 * @return ($value is non-empty-array|non-empty-list ? int<1, max> : int<0, max>)
 *
 * @pure
 */
function count(Countable|array $value): int
{
    return count($value);
}

/**
 * @assert truthy $assertion
 */
function assert(mixed $assertion, Throwable|string|null $description = null): bool
{
    assert($assertion, $description);
}

/**
 * @template K
 * @template V
 *
 * @param Traversable<K, V>|array<K, V> $iterator <p>
 *
 * @return ($preserve_keys is true ? array<K, V> : list<V>)
 */
function iterator_to_array(Traversable|array $iterator, bool $preserve_keys = true): array
{
}

class Row
{
    private array $cells;

    public function __construct(array $cells)
    {
        $this->cells = $cells;
    }

    public function getCell(int $index): mixed
    {
        return $this->cells[$index] ?? null;
    }
}

interface SheetInterface
{
    public function getName(): string;

    /**
     * @return Iterator<Row>
     */
    public function getRowIterator(): Iterator;
}

class ExcelImporter
{
    /**
     * @mago-expect analysis:unhandled-thrown-type
     */
    public function importSheet(SheetInterface $sheet, null|string $parent = null): array
    {
        $iter = $sheet->getRowIterator();
        $rows = iterator_to_array($iter);
        assert(count($rows) >= 1, 'Expected at least one row in the "' . $sheet->getName() . '" sheet');

        $data = [];

        if (count($rows) === 1 && $parent === null) {
            $rows[] = new Row([]);
        }

        for ($i = 2; $i <= count($rows); $i++) {
            $row = $rows[$i];

            $key = $row->getCell(1); // @mago-expect analysis:mixed-assignment
            $value = $row->getCell(2); // @mago-expect analysis:mixed-assignment

            if ($key === null || $value === null) {
                $message = 'Both key and value are required in row ' . $i . ' of the "' . $sheet->getName() . '" sheet';

                throw new Exception($message);
            }
        }

        return $data;
    }
}
