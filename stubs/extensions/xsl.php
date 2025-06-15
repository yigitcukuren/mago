<?php

class XSLTProcessor
{
    /**
     * @template T as DOMDocument|SimpleXMLElement
     *
     * @param T $stylesheet
     */
    public function importStylesheet(object $stylesheet): bool
    {
    }

    /**
     * @template T of object
     *
     * @param T $document
     * @param null|class-string<T> $returnClass
     * @return T|false
     */
    public function transformToDoc(object $document, string|null $returnClass = null): object|false
    {
    }

    public function transformToUri(object $document, string $uri): int
    {
    }

    public function transformToXml(object $document): string|false|null
    {
    }

    public function setParameter(string $namespace, array|string $name, string|null $value = null): bool
    {
    }

    public function getParameter(string $namespace, string $name): string|false
    {
    }

    public function removeParameter(string $namespace, string $name): bool
    {
    }

    public function hasExsltSupport(): bool
    {
    }

    public function registerPHPFunctions(array|string|null $functions = null): void
    {
    }

    public function setProfiling(string|null $filename)
    {
    }

    public function setSecurityPrefs(int $preferences): int
    {
    }

    public function getSecurityPrefs(): int
    {
    }
}

const XSL_CLONE_AUTO = 0;

const XSL_CLONE_NEVER = -1;

const XSL_CLONE_ALWAYS = 1;

const XSL_SECPREF_NONE = 0;

const XSL_SECPREF_READ_FILE = 2;

const XSL_SECPREF_WRITE_FILE = 4;

const XSL_SECPREF_CREATE_DIRECTORY = 8;

const XSL_SECPREF_READ_NETWORK = 16;

const XSL_SECPREF_WRITE_NETWORK = 32;

const XSL_SECPREF_DEFAULT = 44;

const LIBXSLT_VERSION = 10128;

const LIBXSLT_DOTTED_VERSION = '1.1.28';

const LIBEXSLT_VERSION = 817;

const LIBEXSLT_DOTTED_VERSION = '1.1.28';
