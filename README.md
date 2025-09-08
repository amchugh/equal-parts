# equal-parts

## Overview

The `equal-parts` crate provides the `EqualParts` trait, which allows you to split slices and vectors into a specified number of approximately equal-sized parts. Approximately means that the difference in size between any two parts is at most one element. When the total number of elements doesn't divide evenly, the larger parts appear first.

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
equal-parts = "1.0.0"
```

### Basic Example

```rust
use equal_parts::EqualParts;

let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let mut iter = data.equal_parts(4);

assert_eq!(iter.next(), Some([1, 2, 3].as_slice()));  // 3 elements
assert_eq!(iter.next(), Some([4, 5, 6].as_slice()));  // 3 elements
assert_eq!(iter.next(), Some([7, 8].as_slice()));     // 2 elements
assert_eq!(iter.next(), Some([9, 10].as_slice()));    // 2 elements
assert_eq!(iter.next(), None);
```

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](https://www.apache.org/licenses/LICENSE-2.0) file for details.
