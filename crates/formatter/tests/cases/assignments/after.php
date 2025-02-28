<?php

$a = 1;

class A
{
    public const CONSTANT = 2;
}

const GLOBAL_CONSTANT = 3;

enum Status: int
{
    case Active = 1;
}

class B
{
    public int $property = 4;
}

$arr = [
    'key1' => 'value1',
    'key2' => 'value2',
];
