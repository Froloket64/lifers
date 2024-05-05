//! Frontend specification and helper traits and functions

use std::fmt::Display;

/// A simple helper function to quickly display a grid
pub fn render_grid_str<T: Display>(cells: &[Vec<T>]) -> String {
    let mut output = String::new();

    for xs in cells {
        output += &xs
            .iter()
            .map(|state| format!("{state}"))
            .collect::<Box<[_]>>()
            .join("");
        output += "\n";
    }

    output
}

pub trait RenderCell<T> {
    fn render_cell(&self) -> T;
}
