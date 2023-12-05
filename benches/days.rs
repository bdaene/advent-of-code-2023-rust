use std::fs;

use criterion::{black_box, Criterion, criterion_group, criterion_main};

use advent_of_code_2023_rust::get_puzzle;

fn benchmark_day(criterion: &mut Criterion, day: u8) {
    let day_name = format!("day_{:0>2}", day);
    let data = fs::read_to_string(&format!("data/inputs/{day_name}.txt")).unwrap();
    let puzzle = get_puzzle(day, &data);

    criterion.bench_function(&format!("{day_name}_data"),
                             |bencher| bencher.iter(|| get_puzzle(day, black_box(&data))));

    criterion.bench_function(&format!("{day_name}_part_1"),
                             |bencher| bencher.iter(|| puzzle.part_1()));

    criterion.bench_function(&format!("{day_name}_part_2"),
                             |bencher| bencher.iter(|| puzzle.part_2()));
}

fn benchmark(criterion: &mut Criterion) {
    (1..=5).for_each(|day| benchmark_day(criterion, day));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
