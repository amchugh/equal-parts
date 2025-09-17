use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use equal_parts::{EqualParts, IntoEqualParts};

fn bench_equal_parts_slice(c: &mut Criterion) {
    let mut group = c.benchmark_group("equal_parts_slice");
    
    // Test different data sizes
    for size in [100, 1000, 10000, 100000].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        // Test different numbers of parts
        for num_parts in [2, 4, 8, 16, 32].iter() {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), num_parts),
                num_parts,
                |b, &num_parts| {
                    b.iter(|| {
                        let slice = data.as_slice();
                        let parts: Vec<_> = black_box(slice.equal_parts(num_parts).collect());
                        black_box(parts)
                    });
                },
            );
        }
    }
    group.finish();
}

fn bench_equal_parts_vec(c: &mut Criterion) {
    let mut group = c.benchmark_group("equal_parts_vec");
    
    // Test different data sizes
    for size in [100, 1000, 10000, 100000].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        // Test different numbers of parts
        for num_parts in [2, 4, 8, 16, 32].iter() {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), num_parts),
                num_parts,
                |b, &num_parts| {
                    b.iter(|| {
                        let parts: Vec<_> = black_box(data.equal_parts(num_parts).collect());
                        black_box(parts)
                    });
                },
            );
        }
    }
    group.finish();
}

fn bench_into_equal_parts(c: &mut Criterion) {
    let mut group = c.benchmark_group("into_equal_parts");
    
    // Test different data sizes
    for size in [100, 1000, 10000, 100000].iter() {
        // Test different numbers of parts
        for num_parts in [2, 4, 8, 16, 32].iter() {
            group.bench_with_input(
                BenchmarkId::new(format!("size_{}", size), num_parts),
                num_parts,
                |b, &num_parts| {
                    b.iter(|| {
                        let data: Vec<i32> = (0..*size).collect();
                        let parts: Vec<_> = black_box(data.into_equal_parts(num_parts).collect());
                        black_box(parts)
                    });
                },
            );
        }
    }
    group.finish();
}

fn bench_iterator_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("iterator_overhead");
    
    let data: Vec<i32> = (0..10000).collect();
    
    group.bench_function("slice_creation_only", |b| {
        b.iter(|| {
            let slice = data.as_slice();
            let iter = black_box(slice.equal_parts(8));
            black_box(iter)
        });
    });
    
    group.bench_function("full_iteration", |b| {
        b.iter(|| {
            let slice = data.as_slice();
            let parts: Vec<_> = black_box(slice.equal_parts(8).collect());
            black_box(parts)
        });
    });
    
    group.finish();
}

fn bench_edge_cases(c: &mut Criterion) {
    let mut group = c.benchmark_group("edge_cases");
    
    // Empty slice
    let empty_data: Vec<i32> = vec![];
    group.bench_function("empty_slice", |b| {
        b.iter(|| {
            let slice = empty_data.as_slice();
            let parts: Vec<_> = black_box(slice.equal_parts(5).collect());
            black_box(parts)
        });
    });
    
    // Single element
    let single_data = vec![42];
    group.bench_function("single_element", |b| {
        b.iter(|| {
            let slice = single_data.as_slice();
            let parts: Vec<_> = black_box(slice.equal_parts(5).collect());
            black_box(parts)
        });
    });
    
    // More parts than elements
    let small_data = vec![1, 2, 3];
    group.bench_function("more_parts_than_elements", |b| {
        b.iter(|| {
            let slice = small_data.as_slice();
            let parts: Vec<_> = black_box(slice.equal_parts(10).collect());
            black_box(parts)
        });
    });
    
    // Single part (no splitting)
    let data: Vec<i32> = (0..1000).collect();
    group.bench_function("single_part", |b| {
        b.iter(|| {
            let slice = data.as_slice();
            let parts: Vec<_> = black_box(slice.equal_parts(1).collect());
            black_box(parts)
        });
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_equal_parts_slice,
    bench_equal_parts_vec,
    bench_into_equal_parts,
    bench_iterator_overhead,
    bench_edge_cases
);
criterion_main!(benches);