//! `lifers` is an advanced cellular automata creation framework.
//!
//! It consists of the following parts:
//! - **Engine:** Logic and algorithms behing the automata;
//! - **API:** An interface for creating and interacting with created automata;
//! - **Frontend(-s):** External interfaces that allow to represent simulation data
//! in a different way (graphical, ASCII, etc.).
//!
//! # Examples
//! This is how one can implement Conway's Game of Life with `lifers`:
//!
//! ```rust
//! // Use a 100x100 grid
//! let mut game = Automaton::build((100, 100))
//!     // Initialize all cells with random states (alive or dead)
//!     .init(|_| random::<bool>())
//!     // Count neighbors in radius of 1 for each cell
//!     .map(|(x, y), _, cells| lifers::count_neighbors(cells, (x, y), 1, |b| *b))
//!     // Change cells' state depending on the number of neighbors
//!     .run(|(_, _), is_alive, neighbors_n| match is_alive {
//!         true => (2..=3).contains(neighbors_n),
//!         false => *neighbors_n == 3,
//!     });
//!
//! // Compute the next generation
//! game.step();
//! ```

// TODO:
// Ecosystem:
// - [x] Engine
//   -  [ ] Optimization
// - [x] API (builder, stepping, frontend creation..)
// - [ ] Frontend
//   - [ ] Raylib Frontend
//   - [-] ASCII Frontend

pub mod engine;
pub mod frontend;
mod macros;

/// Helper module with all common imports
pub mod prelude {
    pub use crate::{
        engine::{count_neighbors, iter_grid, Automaton, ExecutionState},
        frontend::RenderCell,
    };
}
