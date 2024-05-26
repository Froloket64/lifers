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

#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::allow_attributes_without_reason,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::clone_on_copy,
    clippy::clone_on_ref_ptr,
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::default_numeric_fallback,
    clippy::default_union_representation,
    clippy::exhaustive_enums,
    clippy::expect_used,
    clippy::format_push_string,
    clippy::if_then_some_else_none
)]
#![deny(clippy::perf)]
#![allow(
    clippy::implicit_return,
    clippy::missing_inline_in_public_items,
    clippy::as_conversions,
    clippy::must_use_candidate,
    clippy::return_self_not_must_use,
    clippy::missing_docs_in_private_items,
    clippy::std_instead_of_core,
    clippy::pub_use
)]

// TODO:
// Ecosystem:
// - [x] Engine
//   - [x] Optimization
//   - [ ] Custom types for different kinds of automata (e.g. Life-like, etc.)
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
