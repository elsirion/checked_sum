## `checked_sum`

[![Crates.io](https://img.shields.io/crates/v/checked_sum)](https://crates.io/crates/checked_sum)
[![Docs.rs](https://docs.rs/checked_sum/badge.svg)](https://docs.rs/checked_sum)
[![License](https://img.shields.io/crates/l/checked_sum)](LICENSE)

Utility crate for summing up iterators in a safe way. The `CheckedSum`
trait is implemented for any iterator of items implementing `CheckedAdd`,
which in turn is implemented for all integer primitives but can also be
implemented for other types like newtypes wrapping integers.

```rust
use checked_sum::CheckedSum;

// If the sum fits into the type, it is returned
let numbers = vec![1u8, 2, 3, 4, 5];
assert_eq!(numbers.into_iter().checked_sum(), Some(15),);

// If the sum overflows, `None` is returned
let numbers = vec![255u8, 1];
assert_eq!(numbers.into_iter().checked_sum(), None,);
```