lifers
------
A Rust crate that aims to generalize cellular automata creation. Current features include:
- [x] Easy creation using the builder pattern
- [x] Fast simulation engine
- [ ] Interface to create custom frontends
  - [x] [ASCII frontend](github.com/Froloket64/lifers-ascii)
  - [ ] Raylib frontend

## Usage
An example illustrating Conway's Game of Life implementation in `lifers`:
```rust
use lifers::prelude::*;

fn main() {
    // Use a 100x100 grid
    let mut game = Automaton::build(100, 100)
        // Initialize all cells with random states (alive or dead)
        .init(|_| random::<bool>())
        // Count neighbors in radius of 1 for each cell
        .map(|(x, y), _, cells| lifers::count_neighbors(cells, (x, y), 1, |b| *b))
        // Change cells' state depending on the number of neighbors
        .run(|(_, _), is_alive, neighbors_n| match is_alive {
            true => (2..=3).contains(neighbors_n),
            false => *neighbors_n == 3,
        });
    
    // Compute the next generation
    game.step();
}
```
