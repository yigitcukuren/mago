<?php

/**
 * <b>Reflector</b> is an interface implemented by all
 * exportable Reflection classes.
 *
 * @link https://php.net/manual/en/class.reflector.php
 */
interface Reflector extends Stringable
{
    /**
     * Returns the string representation of any Reflection object.
     *
     * Please note that since PHP 8.0 this method is absent in this interface
     * and inherits from the {@see Stringable} parent.
     *
     * @return string
     */
    public function __toString();
}
