# Slope
Slope is a programming language with a syntax matching mathematical notation as closely as reasonably possible.

## Installation and Usage
```
git clone <this repo>
cargo test  # see issues if any tests fail
cargo run --example <file from examples dir>
cargo run
```

## The Basics
Slope is intended to seamlessly bridge the gap between math notation learned in school, and programming syntax. To that end, many opinionated choices have gone into Slope's implementation. That being said, Slope's developers will always be open to hearing what could be done better. Nonetheless, Slope is not a general purpose programming language.

### Values
Numbers, and collections thereof, are the only "types" in Slope; there are no string-like types, pointers, dictionaries, or maps.

Values are stored using the following syntax.

```
let pi = 3.14;
```

This will store the value `3.14` of the type real to `pi`.

All values are constant (immutable). There exists no syntax for updating a value. Therefore, this is not possible:

```
let pi = 3.14;
pi = 3;
```

This program will result in a syntax error.

Values with the same name cannot be re-bound either.

```
let pi = 3.14;
let pi = 3;
```

This program will result in a runtime error.

Currently, the supported "types" are:
- Reals
- Integers
- Booleans
- Functions
- Sets

These types are actually sets. A real number's "type" is the set of real numbers. Just like in math a member of a subset of a set is a member of that set. Or, `if num in A and A <= B then num in B`. In Slope, this idea has been extrapolated to programming. For example, a function with an argument defined to be in the set of real numbers can accept an integer (because the set of integers is a subset of the set of real numbers). Typing in general, however, is still in its infancy.

For information about future types and features see [the future intended work](#The-Future).

### Arithmetic
The basic arithmetic operations are defined syntactically as follows:
- `+`: addition
- `-`: subtraction
- `*`: multiplication
- `/`: division
- `^`: exponentiation
- `%`: modulo

Slope is also comes with many common mathematical operations out of the box not usually found in other programming languages such as:
- `|<number>|`: absolute value
- sets of sets
- `!` notation for factorial
- more to come; see [the future](#The-Future)

### Functions
Functions are defined using the following syntax:
```
fn sqrt(x) = x ^ 0.5;
```

Functions can contain only one expression after `=`. Functions also cannot have locally-bound values. These two choices were deliberately made in order to follow mathematical notation and to promote creativity when writing code.

Functions also receive all inputs by value rather than by reference. Passing by reference would serve no purpose as there is no syntax for mutation in Slope.

### Control Flow
Piecewise functions allow a function to return different values based on certain conditions. For example, the nth fibonacci number can be found using

```
fn fib(i) = {
    1 if i == 0 or i == 1;
    fib(i - 2) + fib(i - 1) else;
};
```

Piecewise blocks are expressions on their own and so can be bound to a value. For example,

```
let x = randomNumberBetween(0, 10);

let x_is_odd = {
    false if x % 2 == 0;
    true else;
};
```

(Note, however, that there currently exists no random number generator in Slope.)

Piecewise blocks are also nestable. For example,

```
let x = randomNumberBetween(0, 10);

let x_is_2 = {
    { true if x == 2; false else; } if x % 2 == 0;
    false else;
};
```

### Equality, Inequality, and Booleans
Equality and inequality in Slope programs are employed as such:
- `<`, `<=`: Less than, less than or equal
- `>`, `>=`: Greather than, greater than or equal
- `==`: Equals
- `=/=`: Not Equals

Each of the above operations will produce one of the boolean values `true` or `false`.

Conditional expressions employing these operations can be strung together using any of `and`, `or`, or `xor` (exclusive or). They evaluate as expected:
- `true and true`: `true`
- `true or true`: `true`
- `true xor true`: `false`

Only booleans can be combined with `and`, `or`, and `xor`; Slope does not allow truth-y evalution of, say, integers. Therefore, the following is not permitted:
- `1 and 0`. This causes a runtime error.

### Undefined
When a function is called on an input outside of its domain then that function will return `undefined` (just as in basic math contexts). For example,

```
fn sqrt(n) = n ^ 0.5;
let i = sqrt(-1);
```

`sqrt(-1)` returns `undefined` because complex numbers and type annotations are not currently implemented.

Other basic operations can return undefined as well, such as:
- `1 / 0`
- `0 / 0`

Piecewise blocks that do not have any arms with conditions evaluating to true will return undefined. For example,

```
let x = 2;
let y = {
    123 if x == 1;
};
```

Piecewise blocks can be prevented from returning undefined by using `else`.

`undefined` is different than undefined in say, JavaScript, where it may denote an uninitialized variable. In Slope, values cannot be uninitialized. If a value name is read and does not exist then that is a `NameError`.

#### Handling Undefined
To check if any value is undefined use the `?` infix operator. The `?` (called question mark or coalescence) operator offers a default for a value when it is undefined. For example,

```
fn line_with_hole(x) = (x - 1) * (x + 2) / (x + 2);
let line_with_hole = line(-2) ? 0;
```

### Sets
As alluded to previously, "types" are manifested as sets in Slope (just as they are in mathematics). Sets can be defined to contain any set of numbers/sets/etc (sets of functions are untested) available in Slope. Set contents must all be the same type, however.

#### Set Literals
Sets can be explicitly created as such:
```
let s = { 1, 2, 3 };
let power_set_of_s = {
    {},
    { 1 }, { 2 }, { 3 },
    { 1, 2 }, { 1, 3 }, { 2, 3 },
    { 1, 2, 3 }
};
```

#### Checking Existence
To determine if a value exists in a set use the `in` infix operator:
```
let perfect_squares = { 1, 4, 9, 16, 25, 36, 49 };
49 in perfect_squares == true;
```

#### Checking If a Set is a Subset of Another Set
In math, often it is important to know if a set is a subset of another set. Slope re-uses `<=` and `<` for the subset and proper subset operators. For example:

```
{}          <  { 1, 2, 3 } == true;
{ 1 }       <= { 1, 2, 3 } == true;
{ 1, 2, 3 } <= { 1, 2, 3 } == true;
{ 1, 2, 3 } <  { 1, 2, 3 } == false;
```

#### Union
```
let a = { 1, 2, 3 };
let b = { 3, 4 };

a \/ b == { 1, 2, 3, 4 };
```

#### Intersection
```
let a = { 1, 2, 3 };
let b = { 3, 4 };

a /\ b == { 3 };
```

#### Difference (Relative Complement)
```
let a = { 1, 2, 3 };
let b = { 3, 4 };

a \ b == { 1, 2 };
```

#### Symmetric Difference
```
let a = { 1, 2, 3 };
let b = { 3, 4 };

a /_\ b == { 1, 2, 4 };
```

### Errors
The current implementation of Slope has two kinds of errors: SyntaxError and RuntimeError. Errors are not currently recoverable/handle-able.

A SyntaxError is exactly what it sounds like.

A RuntimeError has three variants:
- NameError
- OperatorError
- TypeError

A NameError is raised when a value is read before being initialized.

An OperatorError occurs when an operator is used on one or more values that do not have defined behavior for that operation. For example, `undefined ^ 5`.

A TypeError occurs in a few distinct cases:
- When a piecewise arm's condition does not return a boolean
- When values of differing types are added to a set
- When a built-in function is called on a value of the wrong type (e.g. `max(2)`)


<!-- 
#### Set-Builder Notation
Sets can also be created implicitly using set builder notation commonly used in mathematics.
```
let Evens = { i in Z: i % 2 == 0 };
let Odds = { i in Z: i % 2 =/= 0 };
let PerfectSquares = { i in N: i ^ 0.5 % 1 == 0};
let OddPerfectSquares = { i in PefectSquares: i % 2 =/= 0 };
```


Under the hood, set-builders will have a superset or universal set (the Reals, the Integers, the Naturals) and a set of conditions that a potential member must pass in order for `in` to return `true`. This condition-based existence is based on the implementation of Python's `range` function. See [this SO post for inspiration](https://stackoverflow.com/questions/30081275/why-is-1000000000000000-in-range1000000000000001-so-fast-in-python-3).

In the future, notation such as
```
let PerfectSquares = { i ^ 2 for i in N };
```
will be accepted and preferred (this may required invertible functions, however).

#### Complement
```cpp
let a = { 1, 2, 3 };  // set of Z's

\ a == { i in Z: i not in a)};
```

#### Multi-Sets
Currently, sets only allow one instance of any given value. Multi-sets and their implementation are a future feature of Slope. -->

### Reserved Yet Unused Symbols, Symbol Combinations and Keywords
The following
- `->`, `=>`: arrows to possibly be used for function declaration
- `for`: keyword to be used in set-builders
- `where`: keyword to be used in matrix-builders
- `not in`: not in operation used for sets
- `import`, `use`, `export`, `pub`: keywords possibly to be used in modules
- `:`: colon used for type annotations
- `i`: postfix operator used for complex numbers (still okay to use in `let i = 1;`, for example; similar to python's use of `1j`)
<!-- - `R`, `N`, `Z`, `Q`, `C`: the sets of the real, natural, integer, rational, and complex numbers -->
<!-- - `sum`, `product`, `min`, `max`: built-in functions on sets -->

## About
Slope's interpreter is written in Rust, and created by following Thorsten Ball's [book](https://interpreterbook.com/).

### The Future
#### New Types
- [ ] Rationals
- [ ] Naturals
- [ ] Decimals (possibly to replace floats for Reals)
- [ ] Complex numbers
- [x] Set literals
- [ ] Set builders: declarative ways to instantiate sets (e.g. `{ i in N: 0 <= i and i < 10 }`)
- [ ] Named set members (similar to enums)
- [ ] Tuples (e.g. ordered pairs)
- [ ] Vectors and matrices
- [ ] Vector and matrix builders
- [ ] Multi-sets
- [ ] Graphs (nodes and edges)

#### Operations
- [x] Factorial
- [x] Plus-or-minus
- [ ] Minus-or-plus
- [x] Set containment (`in`)
- [x] Set difference
- [x] Set union
- [x] Set intersection
- [x] Set symmetric difference
- [x] Set size/norm (using abs val)
- [x] Subset and proper subset (`<=`, `<` for sets)
- [ ] Type conversion

##### Longshots
These would be amazing but might never happen.
- [ ] Derivatives and integrals of functions
- [ ] Inverse of functions

#### Features
- [ ] Increased test coverage, automated workflows
- [ ] Line numbers in syntax error and backtraces in runtime errors
- [ ] Web-based REPL
- [ ] Distributed extensions for:
    - [ ] Syntax highlighting
    - [ ] Font ligatures for various operations (`≠` for `=/=`, `∪` for `\/`, `∩` for `/\`, `Δ` for `/_\`, `±` for `+/-`)
- [x] Comments
- [ ] Docstrings
- [ ] Export function definitions to LaTeX
- [ ] Language documentation and specification
- [ ] Function literals
- [ ] Type annotations
- [ ] Static typing
- [ ] Undefined safety (with operations such as add, sub, etc.)
- [ ] WebAssembly compilation
- [ ] Modules, imports, and a standard library (possible just importing WebAssembly modules)
- [ ] Increased efficiency, less use of Strings in interpreter