<?php

/**
 * @return iterable<int, string>
 *
 * @mago-expect analysis:invalid-yield-key-type
 */
function generator(): iterable
{
    yield 'key' => 'value';
}
