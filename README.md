# casting

[![CI](https://github.com/npmccallum/casting/workflows/Test/badge.svg)](https://github.com/npmccallum/casting/actions)
[![Crates.io](https://img.shields.io/crates/v/casting.svg)](https://crates.io/crates/casting)
[![Documentation](https://docs.rs/casting/badge.svg)](https://docs.rs/casting)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Generic numeric casting traits mirroring From/Into.

`casting` provides `CastFrom` and `CastInto` traits that mirror the standard
library's `From` and `Into`, but with casting semantics. This enables:

- Generic programming over type conversions (impossible with `as`)
- Extending cast behavior to custom types (newtypes, wrappers)
- Uniform APIs that work consistently across numeric types

## Relationship to From/Into

`CastFrom` and `CastInto` mirror the standard library's `From` and `Into`
traits, but with casting semantics:

- **`From`/`Into`**: Lossless, infallible conversions (e.g., `u8` → `u16`)
- **`CastFrom`/`CastInto`**: Casting conversions that may lose precision or wrap
  (e.g., `f64` → `i32`, `u16` → `u8`)

Like the standard library:

- `CastInto` is automatically implemented for all types that implement
  `CastFrom`
- You should implement `CastFrom` rather than `CastInto` directly
- The blanket implementation ensures consistency and reduces boilerplate

## Installation

```toml
[dependencies]
casting = "0.1"
```

For nightly features (f16 and f128 support):

```toml
[dependencies]
casting = { version = "0.1", features = ["nightly"] }
```

## Quick Start

```rust
use casting::{CastFrom, CastInto};

// Basic usage with CastInto
let x: u8 = 255;
let y: u16 = x.cast_into();
assert_eq!(y, 255u16);

// Or with CastFrom
let z = u16::cast_from(x);
assert_eq!(z, 255u16);

// Generic programming
fn convert_slice<T, U>(input: &[T]) -> Vec<U>
where
    T: CastInto<U> + Copy,
{
    input.iter().map(|&x| x.cast_into()).collect()
}

let bytes: Vec<u8> = vec![1, 2, 3];
let ints: Vec<i32> = convert_slice(&bytes);
assert_eq!(ints, vec![1i32, 2, 3]);

// Extensibility to custom types
#[derive(Copy, Clone)]
struct Kilometers(f32);

impl CastFrom<Kilometers> for f64 {
    fn cast_from(value: Kilometers) -> f64 {
        (value.0 * 1000.0) as f64 // Convert to meters
    }
}

let distance = Kilometers(5.5);
let meters: f64 = distance.cast_into();
assert_eq!(meters, 5500.0);
```

## Supported Types

- **Boolean**: `bool` (casts to integers only)
- **Character**: `char` (casts to integers only, cast from `u8` only)
- **Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `i8`, `i16`, `i32`,
  `i64`, `i128`, `isize`
- **Floats**: `f32`, `f64`, and `f16`/`f128` with the `nightly` feature

All integer and float types can cast between each other. `bool` and `char` have
limited casting support as noted above.

## Features

- **Zero overhead**: Compiles to the same code as `as` casts
- **Type-safe**: Return types are inferred from context
- **no_std**: Works in embedded and bare-metal environments
- **Extensible**: Add casting to your own types

### Cargo Features

- `nightly`: Enables `f16` and `f128` support (requires nightly Rust)

## Use Cases

- Generic code that converts between numeric types
- Libraries with flexible numeric type conversions
- Custom numeric types (newtypes, wrappers) that need casting
- `no_std` environments requiring portable conversions

## Why Not Just Use `as`?

The `as` operator can't be used in generic contexts:

**Generic programming:**

```rust
use casting::CastInto;

fn double_and_convert<T, U>(x: T) -> U
where
    T: CastInto<U> + std::ops::Add<Output = T> + Copy,
{
    (x + x).cast_into()
}
```

**Custom type extensibility:**

```rust
use casting::{CastFrom, CastInto};

#[derive(Copy, Clone)]
struct Degrees(f32);

impl CastFrom<Degrees> for f64 {
    fn cast_from(value: Degrees) -> f64 {
        value.0 as f64
    }
}
```

Neither is possible with the built-in `as` operator.

## License

Licensed under the [MIT License](LICENSE).
