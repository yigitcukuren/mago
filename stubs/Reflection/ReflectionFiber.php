<?php

final class ReflectionFiber
{
    public function __construct(Fiber $fiber) {}

    public function getFiber(): Fiber
    {
    }

    public function getExecutingFile(): null|string
    {
    }

    public function getExecutingLine(): null|int
    {
    }

    public function getCallable(): callable
    {
    }

    public function getTrace(int $options = DEBUG_BACKTRACE_PROVIDE_OBJECT): array
    {
    }
}
