/// Takes an iterator over a [`Grid<T>`](crate::engine::Grid). Maps the given
/// function (`f: fn((usize, usize), T) -> L`) on each cell in the grid. Expands to
/// a nested iterator over `f`'s return type.
///
/// If `f` take more arguments, they have to be passed in the `args` rule.
#[macro_export]
macro_rules! grid_map {
    ($grid_it:expr, $f:expr) => {
        $grid_it.enumerate().map(|(y, xs)| {
            xs.into_iter()
                .enumerate()
                .map(|(x, state)| ($f)((x, y), state))
                .collect()
        })
    };
    ($grid_it:expr, $f:expr, $( $args:expr ),+) => {
        $grid_it.enumerate().map(|(y, xs)| {
            xs.into_iter()
                .enumerate()
                .map(|(x, state)| ($f)((x, y), state, $( $args ),+))
                .collect()
        })
    };
}
