<?php

for (;;) {
    print 1;
    // for loop without initialisation, condition and increment
}

for ($i = 0; $i < 10; $i++) {
    print $i;
}

for ($i = 0;; $i++) {
    print $i;

    break;
    // for loop without conditions
}

for ($i = 0; $i < 10;) {
    print $i;

    $i++;
    // for loop without increment
}

for (
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
    aaaaaaaaaaaaaaaaaaaaaaaaaaaa, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa, aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
) {
    // for loop that barely fits before the expressions are split
}

for (
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
    aaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
        aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
) {
    // for loop where the expressions are split
}

#[Attribute]
class A
{
}

#[
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
    Attribute,
]
class B
{
}

#[Attribute, Attribute([
    'foo',
    'bar',
    'baz',
])]
class C
{
}

#[Attribute, Attribute([
    'foo',
    'bar',
    'baz',
])]
class C
{
}

class Author
{
    #[Assert\IsTrue(message: 'The password cannot match your first name')]
    public function isPasswordSafe(): bool
    {
        // ... return true or false
    }
}

class User
{
    #[Assert\All([
        new Assert\NotBlank,
        new Assert\Length(min: 5),
    ])]
    protected array $favoriteColors = [];
}

class Place
{
    #[Assert\Sequentially([
        new Assert\NotNull,
        new Assert\Type('string'),
        new Assert\Length(min: 10),
        new Assert\Regex(Place::ADDRESS_REGEX),
        new AcmeAssert\Geolocalizable,
    ])]
    public string $address;
}

class Discount
{
    #[Assert\GreaterThan(0)]
    #[Assert\When(expression: 'this.getType() == "percent"', constraints: [
        new Assert\LessThanOrEqual(100, message: 'The value should be between 1 and 100!'),
    ])]
    private null|int $value;

    // ...
}

class Author
{
    #[Assert\Collection(
        fields: [
            'personal_email' => new Assert\Email,
            'short_bio' => [
                new Assert\NotBlank,
                new Assert\Length(max: 100, maxMessage: 'Your short bio is too long!'),
            ],
        ],
        allowMissingFields: true,
    )]
    protected array $profileData = [
        'personal_email' => '...',
        'short_bio' => '...',
    ];
}

#[Assert\GroupSequence(['User', 'Strict'])]
class User implements UserInterface
{
    #[Assert\NotBlank]
    private string $username;

    #[Assert\NotBlank]
    private string $password;

    #[Assert\IsTrue(message: 'The password cannot match your username', groups: ['Strict'])]
    public function isPasswordSafe(): bool
    {
        return $this->username !== $this->password;
    }
}

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
