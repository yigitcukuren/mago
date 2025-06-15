<?php

enum PropertyHookType implements BackedEnum, UnitEnum
{
    case Get = 'get';
    case Set = 'set';

    public static function cases(): array
    {
    }

    public static function from(int|string $value): static
    {
    }

    public static function tryFrom(int|string $value): null|static
    {
    }
}
