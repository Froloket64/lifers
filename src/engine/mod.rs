pub mod generic;
pub mod life_like;

pub type Pos = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_enums)]
pub enum ExecutionState {
    Finished,
    Remaining(u32),
    Infinite,
}
