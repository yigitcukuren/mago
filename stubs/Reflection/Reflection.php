<?php

use JetBrains\PhpStorm\Deprecated;
use JetBrains\PhpStorm\Internal\LanguageLevelTypeAware;
use JetBrains\PhpStorm\Internal\TentativeType;

/**
 * The reflection class.
 *
 * @link https://php.net/manual/en/class.reflection.php
 */
class Reflection
{
    /**
     * Gets modifier names
     *
     * @link https://php.net/manual/en/reflection.getmodifiernames.php
     * @param int $modifiers Bitfield of the modifiers to get.
     * @return string[] An array of modifier names.
     */
    #[TentativeType]
    public static function getModifierNames(#[LanguageLevelTypeAware(['8.0' => 'int'], default: '')]  $modifiers): array
    {
    }
}
