The iterator to enumerate every combinaison.

## Exemple
```rust
use kombini::Kombini;

let komb = Kombini::from([1, 2, 3]);

// Iterates over all the possible combinasion of the array [1, 2, 3]
komb.for_each(|combinaison| {
    // Something nice and cool with the combinaisons.
})
```
