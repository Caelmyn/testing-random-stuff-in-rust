The iterator to enumerate partition of fixed addends of a given number.

## Exemple
```rust
use partitions::Generator;

let mut gen = Generator::<2>::new(10);

assert_eq!(gen.next(), Some([9, 1]));
assert_eq!(gen.next(), Some([8, 2]));
assert_eq!(gen.next(), Some([7, 3]));
assert_eq!(gen.next(), Some([6, 4]));
assert_eq!(gen.next(), Some([5, 5]));
```
