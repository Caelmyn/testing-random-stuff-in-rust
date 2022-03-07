use criterion::{criterion_main, criterion_group, BenchmarkId, Criterion};
use grid::{Grid, Area};

/* ---------- */

fn manually<T>(grid: &Grid<T>, top: usize, left: usize, bot: usize, right: usize) {
    for y in top..bot {
        for x in left..right {
            grid.get(x, y);
        }
    }
}

/* ---------- */

pub fn from_areas(c: &mut Criterion) {
    let mut grid = Grid::<u8>::new_square_with_default(1000);
    let mut group = c.benchmark_group("from_areas");

    group.significance_level(0.1).sample_size(500);
    group.bench_with_input(BenchmarkId::from_parameter("Iter Over"), &Area::new(0, 0, 999, 999), |b, area| {
        b.iter(|| grid.iter_over_mut(*area).count());
    });
    group.bench_with_input(BenchmarkId::from_parameter("Iter Mut"), &Area::new(0, 0, 999, 999), |b, _| {
        b.iter(|| grid.iter_mut().count());
    });
    group.bench_with_input(BenchmarkId::from_parameter("Manually"), &Area::new(0, 0, 999, 999), |b, area| {
        b.iter(|| (manually(&grid, criterion::black_box(area.top), criterion::black_box(area.left), criterion::black_box(area.bottom), criterion::black_box(area.right))));
    });

    group.finish();
}

/* ---------- */

criterion_group!(benches, from_areas);
criterion_main!(benches);
