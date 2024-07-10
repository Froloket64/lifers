use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::HashMap;

use crate::{
    engine::{ExecutionState, Pos},
    impl_builder_misc,
};

// TODO:
// - Generalize the `Grid` to allow any `BuildHasher`
// - Implement custom Hasher for performance
// - Bench

pub type Grid<S> = FxHashMap<Pos, S>;
pub type DataFn<S, D> = fn(Pos, Option<&S>, &Grid<S>) -> D;
pub type StepFn<S, D> = fn(Pos, Option<S>, D) -> Option<S>;

/// A (Game of) Life-like automaton.
///
/// This means that it only considers the cells that are currently alive
/// and the ones around them in a fixed radius.
///
/// The name might feel a bit misleading because the automaton actually
/// allows for much more advanced features than GoL has (e.g. position-based
/// logic, multiple states instead of only two, etc.).
pub struct Automaton<S, D = ()> {
    cells: Grid<S>,
    radius: u8,
    generations_left: Option<u32>,
    step_fn: StepFn<S, D>,
    data_fn: DataFn<S, D>,
}

impl<S, D> Automaton<S, D> {
    pub fn new(
        cells: Grid<S>,
        radius: u8,
        generations_left: Option<u32>,
        step_fn: StepFn<S, D>,
        data_fn: DataFn<S, D>,
    ) -> Self {
        Self {
            cells,
            radius,
            generations_left,
            step_fn,
            data_fn,
        }
    }

    /// Computes the next generation.
    pub fn step(&mut self) -> ExecutionState {
        let radius = self.radius as isize;
        // NOTE: The number of synthetic cells with always be
        // `self.cells.len() * f(RADIUS)` (where f(x) - see below)
        // TODO: Generalize (`9`)
        let mut synthetic_cells = Vec::with_capacity(9 * self.cells.len());

        // Create synthetic cells
        self.cells.keys().for_each(|(x, y)| {
            // OPTIM: Use a macro to expand to all `(dx, dy)`'s
            (-radius..=radius)
                .cartesian_product(-radius..=radius)
                .for_each(|(dx, dy)| {
                    x.checked_add_signed(dx).and_then(|x| {
                        y.checked_add_signed(dy).map(|y| {
                            if !self.cells.contains_key(&(x, y)) {
                                synthetic_cells.push((x, y))
                            }
                        })
                    });
                });
        });

        // OPTIM: Get rid of these calls
        // IDEA: Use a binary tree instead of `Vec`
        synthetic_cells.sort();
        synthetic_cells.dedup();

        // Collect data for each cell (both real and synthetic)
        // OPTIM: Don't collect, evaluate `data_fn` in-place.
        let mut cells_data: HashMap<Pos, D> = self
            .cells
            .iter()
            .map(|(pos, state)| (*pos, Some(state)))
            .chain(synthetic_cells.iter().map(|pos| (*pos, None)))
            .map(|(pos, state)| (pos, (self.data_fn)(pos, state, &self.cells)))
            .collect();

        // Evaluate `step_fn()` for each cell and collect new states
        // (if alive or became alive)
        // NOTE: We have iterated over all elements of `self.cells`
        // and wrote corresponding `cells_data`, so it must always
        // exist.
        let cells: HashMap<_, _> = self.cells.drain().collect();

        cells.into_iter().for_each(|(pos, state)| {
            let new_state = (self.step_fn)(pos, Some(state), cells_data.remove(&pos).unwrap());

            match new_state {
                None => (),
                Some(s) => {
                    self.cells.insert(pos, s);
                }
            };
        });

        synthetic_cells.iter().for_each(|pos| {
            // NOTE: See above for **safety**
            if let Some(s) = (self.step_fn)(*pos, None, cells_data.remove(pos).unwrap()) {
                self.cells.insert(*pos, s);
            }
        });

        // Return proper execution state signal
        self.generations_left.map_or(ExecutionState::Infinite, |x| {
            x.checked_sub(1).map_or(ExecutionState::Finished, |y| {
                self.generations_left = Some(y);
                ExecutionState::Remaining(y)
            })
        })
    }

    pub fn cells(&self) -> &Grid<S> {
        &self.cells
    }

    pub fn is_finished(&self) -> bool {
        self.generations_left.map_or(false, |x| x == 0)
    }
}

pub struct AutomatonBuilder {
    radius: u8,
}

impl AutomatonBuilder {
    pub fn new(radius: u8) -> Self {
        Self { radius }
    }

    // NOTE: Should there be a `self` parameter?
    pub fn init<F, S>(self, mut init_fn: F) -> InitBuilder<S>
    where
        F: FnMut() -> Grid<S>,
    {
        InitBuilder {
            radius: self.radius,
            cells: init_fn(),
            generations_limit: None,
        }
    }
}

pub struct InitBuilder<S> {
    radius: u8,
    cells: Grid<S>,
    generations_limit: Option<u32>,
}

impl<S> InitBuilder<S> {
    pub fn run(self, step_fn: StepFn<S, ()>) -> Automaton<S, ()> {
        Automaton {
            radius: self.radius,
            cells: self.cells,
            generations_left: self.generations_limit,
            data_fn: |_, _, _| (),
            step_fn,
        }
    }

    pub fn map<D>(self, data_fn: DataFn<S, D>) -> MappedBuilder<S, D> {
        MappedBuilder {
            radius: self.radius,
            cells: self.cells,
            generations_limit: self.generations_limit,
            data_fn,
        }
    }

    impl_builder_misc! { Self }
}

pub struct MappedBuilder<S, D> {
    radius: u8,
    cells: Grid<S>,
    data_fn: DataFn<S, D>,
    generations_limit: Option<u32>,
}

impl<S, D> MappedBuilder<S, D> {
    pub fn run(self, step_fn: StepFn<S, D>) -> Automaton<S, D> {
        Automaton {
            radius: self.radius,
            cells: self.cells,
            generations_left: self.generations_limit,
            data_fn: self.data_fn,
            step_fn,
        }
    }

    impl_builder_misc! { Self }
}

/// Counts all neighbors using a hashmap containing only real cells.
pub fn count_neighbors<S>(pos: Pos, radius: u8, grid: &Grid<S>) -> u32 {
    let radius = radius as isize;

    (-radius..=radius)
        .cartesian_product(-radius..=radius)
        .filter_map(|(dx, dy)| {
            // OPTIM: Remove the `.flatten()`
            (!(dx == 0 && dy == 0))
                .then(|| {
                    pos.0
                        .checked_add_signed(dx)
                        .and_then(|x| pos.1.checked_add_signed(dy).map(|y| (x, y)))
                })
                .flatten()
        })
        .map(|pos| grid.contains_key(&pos))
        .map(u32::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{AutomatonBuilder, Grid};

    #[test]
    fn builder() {
        let _ = AutomatonBuilder::new(1)
            .init(|| Grid::default())
            .run(|_, _: Option<()>, _| None);

        let _ = AutomatonBuilder::new(1)
            .init(|| Grid::default())
            // .map(|_, _: Option<()>, _| ())
            .run(|_, _: Option<()>, _| None);
    }
}
