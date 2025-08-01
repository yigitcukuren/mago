<?php

/**
 * @return iterable<int, string>
 *
 * @mago-expect analysis:invalid-yield-value-type
 */
function generator(): iterable
{
    yield 1 => 42;
}
