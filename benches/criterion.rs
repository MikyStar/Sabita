use criterion::{criterion_group, criterion_main, Criterion};
use sabita::core::grid::Grid;

use std::hint::black_box;

////////////////////////////////////////

pub fn generate(c: &mut Criterion) {
    c.bench_function("generate", |b| b.iter(|| Grid::generate(black_box(None))));
}

pub fn solve_10(c: &mut Criterion) {
    let mut grid = Grid::generate(None);
    grid.remove_random_values(10);

    c.bench_function("solve 10", |b| b.iter(|| grid.solve()));
}

pub fn solve_30(c: &mut Criterion) {
    let mut grid = Grid::generate(None);
    grid.remove_random_values(30);

    c.bench_function("solve 30", |b| b.iter(|| grid.solve()));
}

pub fn solve_50(c: &mut Criterion) {
    let mut grid = Grid::generate(None);
    grid.remove_random_values(50);

    c.bench_function("solve 50", |b| b.iter(|| grid.solve()));
}

pub fn solve_64(c: &mut Criterion) {
    let mut grid = Grid::generate(None);
    grid.remove_random_values(64);

    c.bench_function("solve 64", |b| b.iter(|| grid.solve()));
}

////////////////////////////////////////

criterion_group!(benches, generate, solve_10, solve_30, solve_50, solve_64);

criterion_main!(benches);
