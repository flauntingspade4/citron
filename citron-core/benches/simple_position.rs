use chesty_core::{explore_line, hash, Board, Position};
use criterion::{criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("benches");

    let board =
        Board::from_fen("r2q1rk1/1p3p1p/1b4p1/pPp2b2/3pn1P1/P2Q4/B1P1NP1P/R1B2RK1 b - - 0 30")
            .unwrap();

    let hash = hash(&board);

    let expected_best_move = (Position::new(4, 3), Position::new(5, 1));

    group.bench_function("depth 3", |b| {
        b.iter(|| {
            let table = board.iterative_deepening(3);

            let best = table.get(&hash).unwrap();
            let best_move = best.best_move;

            assert_eq!(best_move, expected_best_move);
        })
    });

    group.sample_size(10);

    group.bench_function("depth 4", |b| {
        b.iter(|| {
            let table = board.iterative_deepening(4);

            let best = table.get(&hash).unwrap();
            let best_move = best.best_move;

            if best_move != expected_best_move {
                // assert_eq!(best_move, expected_best_move);
                explore_line(board.clone(), &table);
                panic!(
                    "({}) ({}) {}",
                    best_move.0,
                    best_move.1,
                    best.evaluation.into_inner()
                )
            }
        })
    });

    group.bench_function("depth 5", |b| {
        b.iter(|| {
            let table = board.iterative_deepening(5);

            let best = table.get(&hash).unwrap();
            let best_move = best.best_move;

            if best_move != expected_best_move {
                // assert_eq!(best_move, expected_best_move);
                explore_line(board.clone(), &table);
                panic!(
                    "({}) ({}) {}",
                    best_move.0,
                    best_move.1,
                    best.evaluation.into_inner()
                )
            }
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
