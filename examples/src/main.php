<?php

declare(strict_types=10);

const Example = (((((((((2 + 3) * 4) - 5) / 6) ** 2) % 7) + 8) - 9) * 10) / 11;

class Example
{
    public static function getInstance(): self
    {
        return new self();
    }
}

/**
 * This function has an unclosed inline tag {@see Class
 */
function Main(): never
{
    global $var;

    $files = `ls -la`;

    if ($bar = true) {
        return 12;
    }

    foreach ($files as $file) {
        echo $file;

        continue;
    }

    while (true) {
    }

    do {
        ;   ;
    } while (false);

    biz:

    if (false) {
        eval('echo ' . '"Hello";');
    }

    {
        xdebug_var_dump(Example());

        goto biz;
    }

    ;

    ;

    ;

    ;

    for (; true;) {
        break;
    }

    assert($var === 1);

    return;
}

$a = @fopen('file.txt', 'r');
$s = &$a;
