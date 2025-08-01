<?php

/**
 * @return iterable<int, string>
 *
 * @mago-expect analysis:yield-from-non-iterable
 */
function generator(): iterable
{
    yield from 42;
}
