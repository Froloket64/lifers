use criterion::{criterion_group, criterion_main, Criterion};
use lifers::{engine::AutomatonBuilder, prelude::*};

#[derive(Default)]
struct Cell {
    is_alive: bool,
}

impl RenderCell<String> for Cell {
    fn render_cell(&self) -> String {
        match self.is_alive {
            true => "██",
            false => "··",
        }
        .into()
    }
}

pub fn conway_bench(c: &mut Criterion) {
    let mut game = prepare_game(Automaton::build((10, 10)));

    c.bench_function("conway [10x10]", |b| b.iter(|| game.step()));

    let mut game1 = prepare_game(Automaton::build((100, 100)));

    c.bench_function("conway [100x100]", |b| b.iter(|| game1.step()));

    let mut game2 = prepare_game(Automaton::build((1000, 1000)));

    c.bench_function("conway [1000x1000]", |b| b.iter(|| game2.step()));
}

fn prepare_game(builder: AutomatonBuilder<Cell, usize>) -> Automaton<Cell, usize> {
    builder
        .init(|_| Cell { is_alive: false })
        .map(|(x, y), _, cells| count_neighbors(cells, (x, y), 1, |c| c.is_alive))
        .run(|(_, _), is_alive, neighbors_n| match is_alive.is_alive {
            true => Cell {
                is_alive: (2..=3).contains(neighbors_n),
            },
            false => Cell {
                is_alive: *neighbors_n == 3,
            },
        })
}

criterion_group!(benches, conway_bench);
criterion_main!(benches);
