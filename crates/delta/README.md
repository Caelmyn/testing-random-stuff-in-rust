# Delta
Compute a delta between two values and keep the last
value for future updates. The delta computation takes interger
overflow into account.

## Exemples
```rust
 use delta::Delta;

let mut delta = 0u16;

assert_eq!(toto.delta(1), 1);
assert_eq!(toto.delta(1), 0);
assert_eq!(toto.delta(1000), 999);
assert_eq!(toto.delta(u16::MAX), u16::MAX - 1000);
assert_eq!(toto.delta(u16::MAX), 0);

// Overflow!
assert_eq!(toto.delta(0), 1);
```
