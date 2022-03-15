The iterator to enumerate every combinaison.

## Exemple
```rust
use kombini::Kombini;

let komb = Kombini::from([1, 2, 3]);
let expected = [
    [1, 2, 3],
    [2, 1, 3],
    [2, 3, 1],
    [3, 2, 1],
    [3, 1, 2],
    [1, 3, 2],
];

// Iterates over all the possible combinasion of the
// array [1, 2, 3]
komb.enumerate().for_each(|(index, combinaison)| {
    assert_eq!(combinaison, expected[index])
})
```
