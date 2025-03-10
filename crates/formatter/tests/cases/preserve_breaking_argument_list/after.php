<?php

foo(
    1,
    2,
);

return in_array(
    $name,
    $caseNames,
    strict: true,
); /** @phpstan-ignore-line function.impossibleType ( prevent to always evaluate to true/false as in enum context the result is predictable ) */
