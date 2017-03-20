# implicants

> Enumerates (prime) implicants of an arbitrary function.

Provides a highly efficient way to enumerate all implicants / prime implicants
of an arbitrary binary function.

This library provides:
- a "raw" interface to do it, with as little overhead as possible.
- a C header for the "raw" interface.

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
masked_permute = { git = "https://github.com/BenWiederhake/implicants.git" }
```

<!-- FIXME: Test this. -->

That should be it.
Although `implicants` has some dependencies (`fixedbitset`), I hope this will remain easy.

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

Include the header file in `include/`, and there's
[something weird](http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/)
going on when executing:

```
$ gcc main.c -L ../target -lstringtools-261cf0fc14ce408c -o main
$ LD_LIBRARY_PATH=../target ./main
```

FIXME: Write this.

## Usage

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

Uhh, not sure, but I guess something like:

```C
#include <stdio.h>
#include <implicants.h>

int my_fn(x) {
    return (x % 3) == 0;
}

void print_it(uint32_t mask_gap, uint32_t value, int is_prime) {
    printf("%08x}/%08x is a%s implicant.", mask_gap, value,
        is_prime ? " prime" : "n");
}

int main(int argc, char** argv) {
    printf("Hello world!");
    implicants_enumerate(6, my_fn, print_it);
    printf("That's it.");
    return 0;
}
```

FIXME: Test this.

## TODOs

- Find out how to be accessible from C
- Check binary size, and do `[nostdlib]` magic if necessary
- Measure performance, use FNV HashMap if reasonable

## Contribute

Feel free to dive in! [Open an issue](https://github.com/BenWiederhake/implicants/issues/new) or submit PRs.
