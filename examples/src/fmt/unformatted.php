<?php

(function () {
    echo 'Hello, world!';
})();

$foo->bar()
    ->baz()
    ->qux()
    ->quux(static fn() => [
        'foo',
        'bar',
        'baz',
        'qux',
        'quux',
        'corge',
        'grault',
        'garply',
        'waldo',
    ])
    ->corge(
        static fn() => [
            'foo',
            'bar',
            'baz',
            'qux',
            'quux',
            'corge',
            'grault',
            'garply',
            'waldo',
        ],
        [],
    )
    ->grault(
        #[bar] static fn() => [
            'foo',
            'bar',
            'baz',
            'qux',
            'quux',
            'corge',
            'grault',
            'garply',
            'waldo',
        ],
        [],
    )
    ->garply(
        #[bar] static fn() => [
            'foo',
            'bar',
            'baz',
            'qux',
            'quux',
            'corge',
            'grault',
            'garply',
            'waldo',
        ],
        [],
        [],
    )
    ->waldo(
        #[foo]
        #[bar]
        static fn() => [
            'foo',
            'bar',
            'baz',
            'qux',
            'quux',
            'corge',
            'grault',
            'garply',
            'waldo',
        ],
        [],
        [],
    )
    ->fred()
    ->plugh()
    ->xyzzy()
    ->thud();

$a = new class {
};

$a = new class {
    public function foo(): void
    {
        echo 'Hello, world!';
    }
};

$a = new class {
    public function foo(): void
    {
        echo 'Hello, world!';
    }
};

$a = new class implements
    Foo,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar,
    Bar {
    public function foo(): void
    {
        echo 'Hello, world!';
    }
};

$util->setLogger(new
#[Foo]
#[baz]
readonly class('[DEBUG]') {
    public function __construct(
        private string $prefix,
    ) {
    }

    public function log($msg): void
    {
        echo $this->prefix . ' ' . $msg;
    }
});

class Foo
{
    public static $bar = 1;
}

use Foo, Bar, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe, Baz, qwe;
use Foo,
    Bar,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe,
    Baz,
    qwe
;
use const Foo\Bar\{
    Foo,
    Bar,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Bar
    // This is a comment
};
use const Foo\Bar\{
    Foo,
    Bar,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Baz,
    Bar,
    Baz,
    Bar,
    Baz,
    Bar
};
use const Foo\Bar\{Foo, Bar, Baz};
use const Foo\Bar\{a};
use Foo\Bar\{Foo, Bar, Baz};
use Foo\Bar\{Foo, Bar, Baz};
use Q\{A, B, C};
use function Foo\Bar\{foo, bar, baz};
use const Foo\Bar\{FOO, BAR, BAZ};
use Foo\Bar\{function foo, function bar, function baz};
use Foo\Bar\{const FOO, const BAR, const BAZ};

class ClassName extends ParentClass implements
    ArrayAccess,
    Countable,
    Serializable,
    IteratorAggregate,
    JsonSerializable,
    Traversable,
    SeekableIterator
{
    #[Attribute]
    public const A = 1;

    public const A = 1, B = 1;

    #[Attribute]
    public const
        AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        B = 1,
        C = 1
    ;

    #[Attribute]
    public const int A = 1, AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1;

    public const int A = 1, B = 1;

    #[Attribute(1, 2)]
    public var int $A = 1;

    public mixed
        $C = 12,
        $W = function (): void {
            echo 'Hello, world!';
        }
    ;

    public mixed $W = function (): void {
        echo 'Hello, world!';
    };

    #[Attribute(1, 2)]
    public int
        $AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        $AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        $AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        $AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
        $B = 1,
        $C = 1
    ;

    #[Attribute]
    public int $A = 1, $AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1;

    public int $A = 1, $B = 1;
}

interface Collection extends
    ArrayAccess,
    Countable,
    Serializable,
    IteratorAggregate,
    JsonSerializable,
    Traversable,
    SeekableIterator
{
    // constants, properties, methods
}

$a = isset($a);
$b = isset($a, $b);
$c = isset(
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
    $b,
    $c,
);

unset($a);
unset($a, $b);
unset(
    aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
    $b,
    $c,
);

global $a;
global $a, $b;
global
    $aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa,
    $b,
    $c
;

static $a = 1;
static $a = 1, $b = 1;
static
    $aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa = 1,
    $b = 1,
    $c = 1
;

const A = 1;
const A = 1, B = 1;
const
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
    B = 1,
    C = 1
;
const
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA = 1,
    B = 1,
    C = 1,
    AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA =
        'hello, world',
    D = 1
;

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

switch ($i) {
    case 0:
        echo 'i equals 0';
        break;
    case 1:
        echo 'i equals 1';
        break;
    case 2:
        echo 'i equals 2';
        break;
    default:
        echo 'i is not equal to 0, 1 or 2';
}

switch ($i):
    case 0:
        echo 'i equals 0';
        break;
    case 1:
        echo 'i equals 1';
        break;
    case 2:
        echo 'i equals 2';
        break;
    default:
        echo 'i is not equal to 0, 1 or 2';
endswitch;

switch ($i) {
    case 0:
        echo 'i equals 0';
        break;
    case 1:
        echo 'i equals 1';
        break;
    case 2:
        echo 'i equals 2';
        break;
    default:
        echo 'i is not equal to 0, 1 or 2';
    // Heeeheee
}

match ($i) {
    0 => 'i equals 0',
    1 => 'i equals 1',
    2 => 'i equals 2',
    default => 'i is not equal to 0, 1 or 2',
    // Haaahaaaa
};

switch ($i):
    case 0:
        echo 'i equals 0';
        break;
    case 1:
        echo 'i equals 1';
        break;
    case 2:
        echo 'i equals 2';
        break;
    default:
        echo 'i is not equal to 0, 1 or 2';
    // Haaahaaaa
endswitch;

$a = 'hello, world';
$b = 'hello, world';
$c = "hello, 'world'";
$d = 'hello, \'\' "world"';

// This is a comment

/**
 * Dangling ..
 */
