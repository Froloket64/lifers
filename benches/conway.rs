use criterion::{criterion_group, criterion_main, Criterion};
use lifers::{engine::AutomatonBuilder, prelude::*};

pub fn conway_bench(c: &mut Criterion) {
    let mut game = prepare_game(Automaton::build((10, 10)));

    c.bench_function("conway [10x10]", |b| b.iter(|| game.step()));

    let mut game1 = prepare_game(Automaton::build((100, 100)));

    c.bench_function("conway [100x100]", |b| b.iter(|| game1.step()));

    let mut game2 = prepare_game(Automaton::build((1000, 1000)));

    c.bench_function("conway [1000x1000]", |b| b.iter(|| game2.step()));
}

fn prepare_game(builder: AutomatonBuilder) -> Automaton<bool, usize> {
    builder
        .init(|_| false)
        .map(|(x, y), _, cells| count_neighbors(cells, (x, y), 1, |is_alive| *is_alive))
        .run(|(_, _), is_alive, neighbors_n| match is_alive {
            true => (2..=3).contains(neighbors_n),
            false => *neighbors_n == 3,
        })
}

criterion_group!(benches, conway_bench);
criterion_main!(benches);
