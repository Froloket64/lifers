use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use lifers::engine::{generic, life_like};

// TODO: Add cells to the grid, since currently the comparison benchmark is
// pretty pointless
pub fn conway_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("Conway");

    for size in [(10_usize, 10_usize), (100, 100), (1000, 1000)] {
        let mut generic = prepare_game_generic(generic::Automaton::build(size));
        group.bench_with_input(
            BenchmarkId::new("Generic", format_pos(size)),
            &(),
            |b, _| b.iter(|| generic.step()),
        );

        let mut life_like_game = life_like::AutomatonBuilder::new(1)
            .init(|| life_like::Grid::default())
            .map(|(x, y), _, cells| life_like::count_neighbors((x, y), 1, &cells))
            .run(|_, is_alive, neighbors_n| match is_alive {
                Some(_) => (2..=3).contains(&neighbors_n).then_some(()),
                None => (neighbors_n == 3).then_some(()),
            });

        group.bench_with_input(
            BenchmarkId::new("Life-like", format_pos(size)),
            &(),
            |b, _| b.iter(|| life_like_game.step()),
        );
    }
}

fn format_pos(pos: (usize, usize)) -> String {
    format!("({}, {})", pos.0.to_string(), pos.1.to_string())
}

fn prepare_game_generic(builder: generic::AutomatonBuilder) -> generic::Automaton<bool, usize> {
    builder
        .init(|_| false)
        .map(|(x, y), _, cells| generic::count_neighbors(cells, (x, y), 1, |is_alive| *is_alive))
        .run(|_, is_alive, neighbors_n| match is_alive {
            true => (2..=3).contains(&neighbors_n),
            false => neighbors_n == 3,
        })
}

criterion_group!(benches, conway_bench);
criterion_main!(benches);
