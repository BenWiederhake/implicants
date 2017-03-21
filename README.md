# implicants

> Enumerates (prime) implicants of an arbitrary function.

Provides a highly efficient way to enumerate all implicants / prime implicants
of an arbitrary binary function.

This library provides:
- Rust interface
- Rust examples (see `tests/`)
- C interface (see `include/implicants.h`)
- C and C++ example (see `examples/print.c`)

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [TODOs](#todos)
- [Contribute](#contribute)

## Background

Examples will be given with binary functions,
but all of this obviously applies to arbitrary arity.

### Implicants

Let's say you have an arbitrary binary function `f`, i.e.,
something that takes several bits of input and spits out a boolean.
Then the term *implicant* means any partial input such that
`f` will output `true` for whatever you fill into the "gaps".

Exmples, where `M` is such a "gap":
- `MM` is an implicant for the constant `true` function.
- `1M` is an implicant for the binary `or` function.
- So is `M1` and `11`.
- The only implicant of the binary `and` function is `11`.
- The constant `false` function has no implicants.

### Prime implicants

A prime implicant is an implicant which has a maximal amount of "gaps".
In other words, if the partial input gets any more unspecific,
it is no longer an implicant.

Examples:
- `MM` is a prime implicant of the constant `true` function.
- The binary `or` function has exactly two implicants: `1M` and `M1`, but *not* `11`.

## Install

### Rust

Add at an appropriate position to your `Cargo.toml`:

```TOML
[dependencies]
implicants = { git = "https://github.com/BenWiederhake/implicants.git" }
```

<!-- FIXME: Test this. -->

That should be it.

In case you don't want libc to be pulled, just disable the feature `c-abi`.
(Note that, obviously, the C ABI won't be available then.
I don't know how to conditionally remove cratetypes.)

### Additional step for best performance

For best performance, you should allow `rustc` (or in this case, LLVM actually)
to use special instructions that can speed up execution even more.
Specifically, this library makes extensive use of `u32::count_ones()`,
which could be compiled to the single special-purpose instruction `popcnt`.

To enable this instruction, add this to your `.cargo/config` file
[somewhere up the tree](http://doc.crates.io/config.html#hierarchical-structure):

```TOML
[build]
rustflags = ["-C", "target-feature=+popcnt"]
#rustflags = ["-C", "target-cpu=native"]
```

Feel free to be even more specific about the target architecture.
I only highlighted this singular instruction, as it is available
on all common architectures, and has the most impact, as far as the
current benchmarks are concerned.

<!--
  Assuming that the processor doesn't already recognize the pattern and
  optimize on its own.  In this case, `popcnt` might still be of advantage
  because of the limited instruction cache.
  The "bitcount hack" is pretty long!
-->

### From C

Provide the header file in `include/` when compiling, like this

```
$ gcc my_compilation_unit.c -o my_compilation_unit.o -I../include/
```

And link against it, plus all transitive dependencies (which you can see
[when compiling, currently](https://github.com/rust-lang/rust/issues/25820)).

```
$ gcc -o my_artifact $MY_OBJECTS -Lpath/to/implicants/target/release/ \
      -limplicants -lutil -ldl -lrt -lpthread -lgcc_s -lc -lm -lrt -lutil
```

<!--
  It appears as you only really need `-limplicants -lpthreads -ldl`,
  which also appears to reduce binary size by a few KiB.  Do this only
  if you know what you're doing.
  But hey, you're the one reading HTML comments in a Markdown file,
  so go ahead and have fun ;-)
-->

## Usage

### Semantics

The core (and only) API is the function `generate` which
essentially only takes two callbacks of types:
```
sampling_fn: &'a Fn(u32) -> bool,
report_fn: &'b mut FnMut(u32, u32, bool),
```

That's a lot syntax.

#### The "sampling" callback

Obviously one needs access to `f` in order to enumerate its implicants.
I think that an intuitive "format" for this is to enable the user to
provide a literal function object.

This must be a `Fn`, so the callback must be immutable.
This may be overly strict, but as this is meant to be a "pure" function,
it matches the intuition behind this.

Note that Rust doesn't support runtime-sized integers out of the box,
and in this context BigIntegers don't make too much sense anyway.
(Remember, exponential running time!)
So `u32` should usually fit all your input bits.
The exact arity (i.e., number of bits your function cares about)
is given as the `arity` argument to `generate`.

This callback is called exactly `2^arity` times.

#### The "report" callback

This callback is called once for each implicant found, but no order is guaranteed.

<!--
    Technically, *some* order *is* guaranteed:
    When you see an implicant with some `k == mask_gap.count_ones()`,
    then it's currently guaranteed that future implicants have at least
    this many bits set in the `mask_gap`, too.
    It's very unlikely that this ever changes in future versions.
    If so, I'll call it a breaking change, and continue versioning from 2.0.0.
-->

It is allowed to mutate it's own state; so closures that access a `&mut` are perfectly fine.
In `tests/collect.rs` you see an example for exactly this.

The arguments to your callback must be:

```
mask_gap: u32, value: u32, is_prime: bool
```

`is_prime` just indicates whether the reported implicant is actually a prime implicant.
Each implicant is only reported once.
As an implicant is actually a ternary value (`0`, `M`, `1`),
this needs some little thought to store:
- `M` becomes `1` in `mask_gap` and `0` in `value`
- `0` and `1` become `0` and `1` in `value` respectively, and `mask_gap` is `0` in both cases.

The order and position correspond exactly to whatever `sample_fn` defines it to be.

### From Rust

Just use it!

```Rust
extern crate implicants;

let my_fn = |x: u32| (x % 3) == 0;
let mut print_it = |mask_gap: u32, value: u32, is_prime: bool| {
    println!("{:032b}/{:032b} is a{} implicant.",
             mask_gap,
             value,
             if is_prime { " prime" } else { "n" });
};

println!("Hello world!");
implicants::generate(&my_fn, &mut print_it, 3);
println!("That's it.");
```

The only "surprise" lies in that the closure-borrow needs to be mutable,
as the report function (here: `print_it`) will usually mutate things
outside it's closure environment.

### From C

Just call it:

```C
#include <stdio.h>
#include <implicants.h>

int my_fn( / * args */ ) { /* ... */ }
#define MY_FNS_ARITY 3

void print_it( /* args */ ) { /* ... */ }

int main(int argc, char** argv) {
    implicants_enumerate(my_fn, NULL, print_it, NULL, MY_FNS_ARITY);
    return 0;
}
```

What ends up being `NULL` pointers in the above example is actually a
"context" void pointer that is passed as-is to your callback.
This makes it easy to avoid needing global state.

## TODOs

- Check whether `cdylib` makes more sense.
- Filter implicants, early abort, and other weird things.
  Not sure whether I actually need that, so I'll first wait.
- Think about optimizations, if necessary:
  * Measure performance, use FNV HashMap if reasonable.
  * Transfer implicants in bulk to avoid cache trashing.
    (After all, `report_0n` needs to read from "all over" a chunk!)
  * Handle all-ones chunks more efficiently. (Might not have any impact.)
  * In `build_rank_n`, check several subchunks for existence?

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/implicants/issues/new) or submit PRs.
