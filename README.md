# AABEL
A Rust crate for different base functionalities.

[![Rust](https://github.com/veminovici/aabel-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/veminovici/aabel-rs/actions/workflows/rust.yml)
![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/veminovici/aabel-rs)

- [Collections](https://github.com/veminovici/aabel-rs#1-collections)
  - [CountedBag](https://github.com/veminovici/aabel-rs#11-countedbag)
  - [Shringles](https://github.com/veminovici/aabel-rs#12-shingles)
  - [Permutations](#13-permutations)
- [Distances](https://github.com/veminovici/aabel-rs#2-distances)
  - [Jaccard](https://github.com/veminovici/aabel-rs#21-jaccard-distance)
  - [Euclidean](https://github.com/veminovici/aabel-rs#22-euclidean-distance)
  - [Manhattan](https://github.com/veminovici/aabel-rs#23-manhattan-distance)
- [Bits](https://github.com/veminovici/aabel-rs#3-bitwise-operations)
- [Tests](https://github.com/veminovici/aabel-rs#4-tests)
  - [Test Coverage](https://github.com/veminovici/aabel-rs#41-test-coverage)
  - [Property Testting](https://github.com/veminovici/aabel-rs#42-property-based-testing)

## 1. Collections

### 1.1. CountedBag
**CountedBag** is a data structure which is counting the number of occurences of a given key inside a collection of keys.
It is implemented in the **aabel_rs::collections::CountedBag** module. 
You can find the implementation at [counted_bag.rs](./src/collections/counted_bag.rs)

```rust
use aabel_rs::collections::CountedBag;

let xs = [('a', 1), ('b', 1), ('c', 20), ('d', 30)];
let xs = CountedBag::<char>::from_iter(xs);

let ys = [('a', 2), ('b', 1), ('x', 10)];
let ys = CountedBag::<char>::from_iter(ys);

let intersection = xs.intersection(&ys);
let iter = intersection.into_iter();
assert_eq!(iter.count(), 2);
```

### 1.2 Shingles
**Shingles** is an iterator over a slice which returns the shingles of a given size.
It is implemented in the **aabel_rs::collections::Shingles** module. 
You can find the implementation at [shingles.rs](./src/collections/shingles.rs)

```rust
use aabel_rs::collections::{shingles, Shingles};
let source = vec![1, 2, 3];

let is_start = |_: &i32| true;
let mut ss = shingles(source.as_slice(), 2, is_start);

assert_eq!(Some([1, 2].as_slice()), ss.next());
assert_eq!(Some([2, 3].as_slice()), ss.next());
assert_eq!(None, ss.next());
```

### 1.3. Permutations
**Permutations** can generate all permutations for a given set.
It is implemented in the **aabel_rs::colllections::Permutations**.
You can find the implementation at [permutations.rs](./src/collections/permutations.rs).

```rust
use aabel_rs::collections::Permutations;

let source = &mut [1, 2, 3, 4];
let mut permutations = Permutations::new(4, source);

let results = permutations.generate();
assert_eq!(24, results.len());
```

## 2. Distances
The distances are implemented by the **aabel_rs::distances::Distance** trait.

### 2.1 Jaccard distance
You can find the implementation at [jaccard.rs](./src/distances/jaccard.rs).

```rust
use aabel_rs::distances::Distance;
let xs = ['a','b', 'b', 'c', 'c', 'c'];
let ys = ['b', 'c', 'c', 'd', 'd', 'd'];
let it = xs.into_iter().jaccard1(ys);
assert_eq!(it, 0.25);
```

### 2.2. Euclidean distance
You can find the implementation at [euclid.rs](./src/distances/euclid.rs).

```rust
use aabel_rs::distances::Distance;
let it = [3., 4.].into_iter().euclid([0., 0.]);
assert_eq!(5., it)
```

### 2.3. Manhattan distance
You can find the implementation at [manhattan.rs](./src/distances/manhattan.rs).

```rust
use aabel_rs::distances::Distance;
let it = [3., 4.].into_iter().manhattan([0., 0.]);
assert_eq!(7., it)
```

## 3. Bitwise Operations
Implementations for bit-wise manipulation as well for a vector of bits.

## 4. Tests

### 4.1. Test Coverage
To get the test coverage, I use the [grcov](https://github.com/mozilla/grcov#how-to-get-grcov).
See the instructions [steps](https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project).

```bash
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./coverage/aabel-%p-%m.profraw"
cargo build
cargo test
grcov ./coverage -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
open ./target/debug/coverage/index.html
```

### 4.2. Property Based Testing
The library is using property based testing. It uses the [quickcheck](https://docs.rs/quickcheck/latest/quickcheck/) crate.