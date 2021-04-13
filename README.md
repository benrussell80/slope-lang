# Specification for Language
## Types
Numbers are the only primitive types. Period.
"Types" are manifested as constraints on numbers to be contained in domains (e.g. fields or intervals).
For example, this function
```
add(x: R, y: R): R = x + y
```
only accepts x and y as real numbers. Therefore, imaginary numbers could not be passed.
Real numbers are the default domain annotation. Domain inference would be a future detail.

Fields: Reals, Imaginaries, Integers.
Matrices, Vectors, Tensors, etc.
Functions, constants, ranges, sets, booleans.
Subscripts to hold parts
Factorials, partials, limits
vector: (x, y, z)  # analogous to a tuple in Rust
set: {1, 2, 3}
absolute value/norm: |...|
stepped range: 1 to 10 (inclusive)
compile to latex and wasm
White space should not matter
`in/of` instad of epsilon for element of
// comments
undefined and Infinity (infinity in an input, undefined is an output)
all functions should be pure (and therefore results can be cached)
if a function is impure then it should be marked as such. Or if it calls an impure function then it is impure by association.
Higher order functions
- Set
    - Field
        - Interval (0 <= x < Infinity)

### Basic Tests
- arithmetic and assignment
```rust
radius = 10
pi = 3.14
area = radius ^ 2 * pi
volume = 4 / 3 * pi * r ^ 3

length = 5
width = 6.2
perimeter = 2 * (length + width)
```
- expressions
```rust
2 + 2  // should print out 4
pi = 3.14  // not an expression
pi  // should print out the value of pi
```
- booleans
```rust
t = true
f = false
t and f  // false
t or f  // true
t xor t  // false
```
- typed numbers and imaginaries/complex
```rust
// N, Z, Q, R, C
num: R = 1  // an 'integer' that is actually a real number
comp = 1 + 2i
```

### Function Definitions
```rust
// area of a circle
// constants are defined outside of functions
[
    1  to 20;
    21 to 40;
    41 to 60;
]
let approxPi: R = 3.14
// and can be used inside functions
fn circleArea(r: R): R = approxPi * r^2

// functions cannot have any local variables
// NOT ALLOWED
changePi(v: R) = approxPi = v  // X

// functions must only use global constants, input parameters, or other functions

// fuctions can take infinity as a parameter

// but will not return infinity (will return undefined)
hole(x: R): R = (x + 1) / (x - 2)

hole(3)  // 4
hole(2)  // undefined

// a number defined in one field may be undefined in another
imagSqrt(i: R): Q = i^(-2)
imagSqrt(-1)  // 1i

realSqrt(i: R): R = i^(-2)
realSqrt(-1)  // undefined

// higher order sum function
sum(f in (V) -> T, s in Set[V]) in T -> /* implementation details */

// simple dot product function
// lambda function for the inside of the sum
// type inference?
dot(a in R[n], b in R[n]) in R -> sum(
    (i) -> a[i] * b[i],
    { i in N: 1 <= i and i <= n }
)

// Sets/subsets
let Evens = { i in N: i % 2 == 0 }
let FizzBuzzNumbers = { i in N: i % 3 == 0 or i % 5 == 0 }
BabyBoomerBirthYears = { i in N: 1946 <= i and i <= 1964 }
IdentityMatrices = { i in R[n, n]: det(i) = 1 and inverse(i) = i}

// booleans
// true and false


dot([1, 2, 3], [3, 2, 1]) // 3 + 4 + 3 -> 10 

// Matrix multiplication
// type signature for a matrix
unitTwoByTwoMatrix: R[2, 2] = [
    1, 0;
    0, 1
]

buildMatrix(f as (N, N) -> T[m, n], ) in T[m, n]

inner1(a in R[m, n], k in N) in (i in N, j in N) in R -> sum(

)

set()

matrixMult(a, b) -> sum(
    (k) -> a[i, k] * b[k, j],
    { k: 1 <= k and k <= n }
)

A = [
    1, 2, 3;
    4, 5, 6
]
B = [
    1;
    1;
    1
]

matrixMult(A, B)  // -> R[2, 1] -> [6; 15]
```

<img src="./dotproduct.svg" alt="dot product equation as a summation" style="background: white;" />

```rust
// the set of even numbers
let Evens = { x in N: x % 2 == 0 };

// piecewise fibonacci function with recursion (cached automatically because it is a pure function)
fn fib(i: N) -> N = {
    i if i == 0 or i == 1;
    fib(i - 1) + fib(i - 2) else;
};

// the set of first 20 fibonacci numbers
let Fn = { f(x), x in N: x < 20 };

4 in Fn;  // false
5 in Fn;  // true

// ordered pairs
fn f(x: R) -> R = x^2 + 2*x - 8;

let xs = { x in Z: -10 <= x and x <= 10 }

// f can be called on Z because Z < R
let points = { (x, f(x)) for x in xs };

// plot ordered pairs


/*
    // piecewise function
    <func-name>(<parameters>) = {
        <expression> for <condition>,
        ...,
        <expression> for <condition|else>
    }

    // set notation
    { <identifier> in <domain>: <condition> }
*/


// gaussian noise generator (impure because the same inputs will likely produce different outputs)
// impure functions will not cache
impure gaussian_noise(mean, std) = /* implementation details */
```

## Keywords
- `if`
- `in`
- `and`
- `or`
- `xor`
- `not`
- `undefined`
- `else`

## Operations and Symbols
- Arithmetic: `+, -, *, /, ^, |, <=, =, >=, <, >`
- Indexing: `a[1], b[1, 2]`
- Function body begin: `->`
