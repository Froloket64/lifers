//! Frontend specification and helper traits and functions

use std::fmt::{Display, Write};

/// A simple helper function to quickly display a grid
#[allow(clippy::arithmetic_side_effects)]
pub fn render_grid_str<T: Display>(cells: &[Vec<T>]) -> String {
    let mut output = String::new();

    for xs in cells {
        output += &xs.iter().fold(String::new(), |mut acc, cell| {
            let () = write!(acc, "{cell}").unwrap();
            acc
        });

        output += "\n";
    }

    output
}

pub trait RenderCell<T> {
    fn render_cell(&self) -> T;
}
