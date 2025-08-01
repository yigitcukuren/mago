<?php

namespace {
    /**
     * @template K
     * @template V
     */
    interface Traversable
    {
    }

    interface Countable
    {
    }

    function is_countable(mixed $_value): bool
    {
        return false;
    }

    /**
     * @return int<0, max>
     */
    function count(mixed $_value): int
    {
        return 0;
    }
}

namespace Example {
    use function Core\count;
    use function Core\is_countable;

    /**
     * @template T
     *
     * @param iterable<T> $iterable
     *
     * @return int<0, max>
     */
    function count_elements(iterable $iterable): int
    {
        if (is_countable($iterable)) {
            return count($iterable);
        }

        $count = 0;
        foreach ($iterable as $_) {
            ++$count;
        }

        return $count;
    }
}
