<?php

/**
 * @return iterable<int, string>
 *
 * @mago-expect analysis:yield-from-invalid-value-type
 */
function generator(): iterable
{
    yield from [1, 2, 3];
}
