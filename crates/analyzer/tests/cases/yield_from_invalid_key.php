<?php

/**
 * @return iterable<string, string>
 *
 * @mago-expect analysis:yield-from-invalid-key-type
 */
function generator(): iterable
{
    yield from [1 => 'value'];
}
