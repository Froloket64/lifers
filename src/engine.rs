//! Automata engine components

use crate::grid_map;

// TODO:
// - Allow multiple `DataFn`s in `Automaton`

pub type Pos = (usize, usize);
pub type Grid<T> = Vec<Vec<T>>;
pub type StepFn<S, D> = fn(Pos, S, D) -> S;
pub type DataFn<S, D> = fn(Pos, &S, &Grid<S>) -> D;

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub enum ExecutionState {
    Finished,
    Remaining(u32),
    Infinite,
}

/// The main struct that contains the state of an automaton.
///
/// It's recommended to use [`Automaton::build()`] for easier creation process.
pub struct Automaton<S, D = ()> {
    cells: Grid<S>,
    generations_left: Option<u32>,
    step_fn: StepFn<S, D>,
    data_fn: DataFn<S, D>,
}

impl Automaton<(), ()> {
    /// Returns a builder to help construct an automaton.
    pub const fn build(grid_size: (usize, usize)) -> AutomatonBuilder {
        AutomatonBuilder { grid_size }
    }
}

impl<S, D> Automaton<S, D> {
    /// Computes the next generation.
    pub fn step(&mut self) -> ExecutionState {
        // Get cells data
        let mut cells_data: Vec<Vec<D>> =
            grid_map!(self.cells.iter(), self.data_fn, &self.cells).collect();

        // Run step function
        self.cells = std::mem::take(&mut self.cells)
            .into_iter()
            .enumerate()
            .map(|(y, xs)| {
                xs.into_iter()
                    .enumerate()
                    .map(|(x, state)| {
                        // NOTE: `cells_data` is never read from, so we can zero
                        // out its elements
                        #[allow(unsafe_code, clippy::mem_replace_with_uninit)]
                        unsafe {
                            (self.step_fn)(
                                (x, y),
                                state,
                                std::mem::replace(&mut cells_data[y][x], std::mem::zeroed()),
                            )
                        }
                    })
                    .collect()
            })
            .collect();

        // Return proper execution state signal
        self.generations_left.map_or(ExecutionState::Infinite, |x| {
            x.checked_sub(1).map_or(ExecutionState::Finished, |y| {
                self.generations_left = Some(y);
                ExecutionState::Remaining(y)
            })
        })
    }

    /// Returns an immutable reference to the cell grid.
    pub const fn cells(&self) -> &Grid<S> {
        &self.cells
    }

    /// Returns the cells grid dimensions.
    pub fn grid_size(&self) -> (usize, usize) {
        self.cells
            .first()
            .map(Vec::len)
            .map_or((0, 0), |l| (l, self.cells.len()))
    }

    /// Like [`grid_size()`](Self::grid_size), but doesn't perform bounds checks.
    ///
    /// # Safety
    /// This function will panic if the grid's width is 0.
    pub unsafe fn grid_size_unchecked(&self) -> (usize, usize) {
        (self.cells[0].len(), self.cells.len())
    }
}

/// A helper struct for building an [`Automaton`].
pub struct AutomatonBuilder {
    grid_size: Pos,
}

impl AutomatonBuilder {
    /// Creates a builder with the given grid size.
    pub const fn new(grid_size: Pos) -> Self {
        Self { grid_size }
    }

    /// Initializes all cells in the grid using the given function that returns
    /// a value of type `S`.
    ///
    /// Returns an [`InitBuilder<S>`].
    pub fn init<S, F: Fn(Pos) -> S>(self, f: F) -> InitBuilder<S> {
        let grid: Vec<Vec<_>> = (0..self.grid_size.0)
            .map(|y| (0..self.grid_size.1).map(|x| (f)((x, y))).collect())
            .collect();

        InitBuilder { grid }
    }
}

/// An initialized automaton builder.
pub struct InitBuilder<S> {
    grid: Grid<S>,
}

impl<S> InitBuilder<S> {
    /// Converts `self` to a [`MappedBuilder<S, ()>`] directly, setting a
    /// `()`-returning data function.
    pub fn to_mapped(self) -> MappedBuilder<S, ()> {
        MappedBuilder {
            grid: self.grid,
            data_fn: |_, _, _| (),
            generations_limit: None,
        }
    }

    /// Returns a [`MappedBuilder<S, D>`] with a given data collection function
    /// returning a value of type `D`.
    pub fn map<D>(self, f: DataFn<S, D>) -> MappedBuilder<S, D> {
        MappedBuilder {
            grid: self.grid,
            data_fn: f,
            generations_limit: None,
        }
    }
}

/// A builder with a grid and a data collection function attached.
///
/// This is the final stage of automaton creation, so it also contains some
/// other parameters.
pub struct MappedBuilder<S, D> {
    /// The cell grid.
    grid: Grid<S>,
    /// Data collection function.
    data_fn: DataFn<S, D>,
    /// Number of generations that the automaton will be limited to.
    generations_limit: Option<u32>,
}

impl<S, D> MappedBuilder<S, D> {
    /// Returns an [`Automaton<S, D>`] using the information contained in `self`.
    pub fn run(self, f: StepFn<S, D>) -> Automaton<S, D> {
        Automaton {
            cells: self.grid,
            // cells_data: vec![],
            generations_left: self.generations_limit,
            data_fn: self.data_fn,
            step_fn: f,
        }
    }

    /// Set a limit on the  number of generations that the automaton will produce.
    pub const fn generations(mut self, generations_limit: u32) -> Self {
        self.generations_limit = Some(generations_limit);

        self
    }
}

pub fn iter_grid<T>(grid: &[Vec<T>]) -> impl Iterator<Item = (usize, usize, &T)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, xs)| xs.iter().enumerate().map(move |(x, val)| (x, y, val)))
}

/// Counts the number of neighbors a cell has.
///
/// Extracts information on whether a cell is alive using an `extractor` function.
///
/// **Note:** If `radius >= 0b1000_0000`, this will result in UB on 8-bit
/// architectures.
pub fn count_neighbors<S, F>(grid: &Grid<S>, pos: (usize, usize), radius: u8, is_alive: F) -> usize
where
    F: Fn(&S) -> bool,
{
    #[allow(clippy::shadow_reuse)]
    let radius = radius as isize;

    #[allow(clippy::indexing_slicing, clippy::arithmetic_side_effects)]
    (-radius..=radius)
        .flat_map(|x| {
            (-radius..=radius)
                .map(move |y| (pos.0.wrapping_add_signed(x), pos.1.wrapping_add_signed(y)))
        })
        .map(|(x, y)| {
            x < grid[1].len()
                && y < grid.len()
                && !(x == pos.0 && y == pos.1)
                // SAFETY: This may panic
                && is_alive(&grid[y][x])
        })
        .map(usize::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::{engine::AutomatonBuilder, prelude::ExecutionState};

    use super::Automaton;

    const DEFAULT_GRID_SIZE: (usize, usize) = (10, 10);
    const DEFAULT_INIT_FN: fn((usize, usize)) -> bool = |(x, y)| x > y;
    const DEFAULT_STEP_FN: fn((usize, usize), bool, ()) -> bool = |(_x, _y), state, _| state;

    fn default_game() -> Automaton<bool, ()> {
        AutomatonBuilder::new(DEFAULT_GRID_SIZE)
            .init(DEFAULT_INIT_FN)
            .to_mapped()
            .generations(2)
            .run(DEFAULT_STEP_FN)
    }

    #[test]
    fn builder() {
        let _game = Automaton::build(DEFAULT_GRID_SIZE)
            .init(DEFAULT_INIT_FN)
            .to_mapped()
            .generations(5)
            .run(DEFAULT_STEP_FN);

        let _game = AutomatonBuilder::new(DEFAULT_GRID_SIZE);

        let _game = default_game();
    }

    #[test]
    fn generations_limit() {
        let mut game = default_game();

        assert_eq!(game.step(), ExecutionState::Remaining(1));
        assert_eq!(game.step(), ExecutionState::Remaining(0));
        assert_eq!(game.step(), ExecutionState::Finished);

        let mut game = AutomatonBuilder::new(DEFAULT_GRID_SIZE)
            .init(DEFAULT_INIT_FN)
            .to_mapped()
            .run(DEFAULT_STEP_FN);

        assert_eq!(game.step(), ExecutionState::Infinite);
    }
}
