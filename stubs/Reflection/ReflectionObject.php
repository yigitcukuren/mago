<?php

use JetBrains\PhpStorm\Deprecated;
use JetBrains\PhpStorm\Internal\LanguageLevelTypeAware;

/**
 * The <b>ReflectionObject</b> class reports
 * information about an object.
 *
 * @link https://php.net/manual/en/class.reflectionobject.php
 */
class ReflectionObject extends ReflectionClass
{
    /**
     * Constructs a ReflectionObject
     *
     * @link https://php.net/manual/en/reflectionobject.construct.php
     * @param object $object An object instance.
     */
    public function __construct(#[LanguageLevelTypeAware(['8.0' => 'object'], default: '')]  $object) {}
}
