lifers
------
A Rust crate that aims to generalize cellular automata creation. Current features include:
- [x] Easy creation using the builder pattern
- [x] Fast simulation engine
  - [ ] SIMD potential
- [ ] Very ergonomic design for creating simulations
  - [x] Full support for the builder pattern
  - [ ] Multiple data collection functions for an automaton
- [x] Arbitrary types support for cells' state and associated data
- [x] Interface to create custom frontends _(WIP)_
  - [x] [ASCII frontend](https://crates.io/crates/lifers-ascii)
  - [x] [Raylib frontend](https://crates.io/crates/lifers-raylib)

## Usage
An example illustrating Conway's Game of Life implementation in `lifers`:
```rust
use lifers::prelude::*;
use rand::random;

fn main() {
    // Use a 100x100 grid
    let mut game = Automaton::build((100, 100))
        // Initialize all cells with random states (alive or dead)
        .init(|_| random::<bool>())
        // Count neighbors in radius of 1 for each cell
        .map(|(x, y), _, cells| count_neighbors(cells, (x, y), 1, |b| *b))
        // Change cells' state depending on the number of neighbors
        .run(|_, is_alive, neighbors_n| match is_alive {
            true => (2..=3).contains(neighbors_n),
            false => *neighbors_n == 3,
        });
    
    // Compute the next generation
    game.step();
}
```
