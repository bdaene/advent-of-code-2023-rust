use std::fs;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use advent_of_code_2023_rust::{get_puzzle, solve_all_puzzles};

const DAYS: u8 = 18;

fn benchmark_all_days(criterion: &mut Criterion) {
    let all_data = (1..=DAYS).into_iter().map(|day| {
        let day_name = format!("day_{:0>2}", day);
        let data = fs::read_to_string(&format!("data/inputs/{day_name}.txt")).unwrap();
        let puzzle = get_puzzle(day, &data);

        criterion.bench_function(&format!("{day_name}_data"),
                                 |bencher| bencher.iter(|| get_puzzle(day, black_box(&data))));

        criterion.bench_function(&format!("{day_name}_part_1"),
                                 |bencher| bencher.iter(|| puzzle.part_1()));

        criterion.bench_function(&format!("{day_name}_part_2"),
                                 |bencher| bencher.iter(|| puzzle.part_2()));

        data
    })
        .collect();

    criterion.bench_function("day_all",
                             |bencher| bencher.iter(|| solve_all_puzzles(&all_data)));
}



criterion_group!(benches, benchmark_all_days);
criterion_main!(benches);
