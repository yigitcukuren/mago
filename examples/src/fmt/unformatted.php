<?php

$a = function () use (
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    // This is a comment
) {
    return $aaaaaaaaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaaaaaaaa;
};

$a = function () use (
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
) {
    return $aaaaaaaaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaaaaaaaa;
};

$a = function () use (
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
    $aaaaaaaaaaaaaaaaaaaaaaaa,
) {
    return $aaaaaaaaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaaaaaaaa;
};

$a = function () use ($aaaaaaaaaaaaaaaaaaaaaaaa, $aaaaaaaaaaaaaaaaaaaaaaaa, $aaaaaaaaaaaaaaaaaaaaaaaa) {
    return $aaaaaaaaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaa + $aaaaaaaaaaaaaaaaaaaaaaaa;
};

require
    'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
require_once
    'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
include
    'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
include_once
    'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';

require
    'ffffffffffffffffffffffffffffffffffffffffffffffff' . 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
require_once
    'ffffffffffffffffffffffffffffffffffffffffffffffff' . 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
include
    'ffffffffffffffffffffffffffffffffffffffffffffffff' . 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';
include_once
    'ffffffffffffffffffffffffffffffffffffffffffffffff' . 'fffffffffffffffffffffffffffffffffffffffffffffffffffffffffff';

class Talker
{
    use A, B {
        B::smallTalk insteadof A;
        A::bigTalk insteadof B;
    }
}

class Talker
{
    use A, B {
        B::smallTalk insteadof A; // space
        A::bigTalk insteadof B, C, D, E;

        // This is a comment
    }
}

class Talker
{
    use A, B {
    }

    use A, B {
        // This is a comment
    }

    private static $instance;
    private $name;
    private $age;

    public static function getInstance(): self
    {
        if (self::$instance === null) {
            self::$instance = new self();
        }

        return self::$instance;
    }

    public function getName(): string
    {
        return $this->name;
    }

    public function setName(string $name): void
    {
        $this->name = $name;
    }

    // This is a comment
}
