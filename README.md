# AABEL

## 1. Collections


### 1.1. CountedBag
**CountedBag** is a data structure which is counting the number of occurences of a given key inside a collection of keys.
It is implemented in the **rust_aabel::counted_bag** module.

```rust
use rust_aabel::counted_bag::*;

let xs = [('a', 1), ('b', 1), ('c', 20), ('d', 30)];
let xs = CountedBag::<char>::from_iter(xs);

let ys = [('a', 2), ('b', 1), ('x', 10)];
let ys = CountedBag::<char>::from_iter(ys);

let intersection = xs.intersection(&ys);
let iter = intersection.into_iter();
assert_eq!(iter.count(), 2);
```

### 1.2 Shingles
**Shingles** is am iterator over a slice which returns the shingles of a given size.
It is implemented in the *8rust_aabel::shingles** module.

```rust
use rust_aabel::shingles::*;
let source = vec![1, 2, 3];

let is_start = |_: &i32| true;
let mut ss = shingles(source.as_slice(), 2, is_start);

assert_eq!(Some([1, 2].as_slice()), ss.next());
assert_eq!(Some([2, 3].as_slice()), ss.next());
assert_eq!(None, ss.next());
```

## 2. Distances
The distances are implemented by the **rust_aabel::distances::Distance** trait.

### 2.1 Jaccard distance
```rust
use rust_aabel::distances::Distance;
let xs = ['a','b', 'b', 'c', 'c', 'c'];
let ys = ['b', 'c', 'c', 'd', 'd', 'd'];
let it = xs.into_iter().jaccard1(ys);
assert_eq!(it, 0.25);
```

### 2.2. Euclidean distance
```rust
use rust_aabel::distances::Distance;
let it = [3., 4.].into_iter().euclid([0., 0.]);
assert_eq!(5., it)
```

### 2.3. Manhattan distance
```rust
use rust_aabel::distances::Distance;
let it = [3., 4.].into_iter().manhattan([0., 0.]);
assert_eq!(7., it)
```

## Test Coverage
To get the test coverage, I use the [grcov](https://github.com/mozilla/grcov#how-to-get-grcov).
See the instructions [steps](https://github.com/mozilla/grcov#example-how-to-generate-source-based-coverage-for-a-rust-project).

```bash
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="./coverage/aabel-%p-%m.profraw"
cargo build
cargo test
grcov ./coverage -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
open ./target/debug/coverage
```