use criterion::{Criterion, black_box, criterion_group, criterion_main};
use ecow::EcoVec;
use rand::prelude::*;
use smallvec::SmallVec;

const SMALL_SIZE: usize = 100;
const LARGE_SIZE: usize = 10_000;

fn vec_push_small(c: &mut Criterion) {
    c.bench_function("Vec push small", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..SMALL_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn vec_push_large(c: &mut Criterion) {
    c.bench_function("Vec push large", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..LARGE_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn vec_random_access_small(c: &mut Criterion) {
    let mut vec = Vec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("Vec random access small", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..SMALL_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn vec_random_access_large(c: &mut Criterion) {
    let mut vec = Vec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("Vec random access large", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..LARGE_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn vec_remove_small(c: &mut Criterion) {
    c.bench_function("Vec remove small", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..SMALL_SIZE {
                vec.push(i);
            }
            for _ in 0..SMALL_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn vec_remove_large(c: &mut Criterion) {
    c.bench_function("Vec remove large", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..LARGE_SIZE {
                vec.push(i);
            }
            for _ in 0..LARGE_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn vec_clone_small(c: &mut Criterion) {
    let mut vec = Vec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    c.bench_function("Vec clone small", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

fn vec_clone_large(c: &mut Criterion) {
    let mut vec = Vec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    c.bench_function("Vec clone large", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

// ===== SmallVec 基准测试 =====

fn smallvec_push_small(c: &mut Criterion) {
    c.bench_function("SmallVec push small", |b| {
        b.iter(|| {
            let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
            for i in 0..SMALL_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn smallvec_push_large(c: &mut Criterion) {
    c.bench_function("SmallVec push large", |b| {
        b.iter(|| {
            let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
            for i in 0..LARGE_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn smallvec_random_access_small(c: &mut Criterion) {
    let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("SmallVec random access small", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..SMALL_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn smallvec_random_access_large(c: &mut Criterion) {
    let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("SmallVec random access large", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..LARGE_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn smallvec_remove_small(c: &mut Criterion) {
    c.bench_function("SmallVec remove small", |b| {
        b.iter(|| {
            let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
            for i in 0..SMALL_SIZE {
                vec.push(i);
            }
            for _ in 0..SMALL_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn smallvec_remove_large(c: &mut Criterion) {
    c.bench_function("SmallVec remove large", |b| {
        b.iter(|| {
            let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
            for i in 0..LARGE_SIZE {
                vec.push(i);
            }
            for _ in 0..LARGE_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn smallvec_clone_small(c: &mut Criterion) {
    let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    c.bench_function("SmallVec clone small", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

fn smallvec_clone_large(c: &mut Criterion) {
    let mut vec: SmallVec<[usize; 1024]> = SmallVec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    c.bench_function("SmallVec clone large", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

// ===== EcoVec =====

fn ecovec_push_small(c: &mut Criterion) {
    c.bench_function("EcoVec push small", |b| {
        b.iter(|| {
            let mut vec: EcoVec<usize> = EcoVec::new();
            for i in 0..SMALL_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn ecovec_push_large(c: &mut Criterion) {
    c.bench_function("EcoVec push large", |b| {
        b.iter(|| {
            let mut vec: EcoVec<usize> = EcoVec::new();
            for i in 0..LARGE_SIZE {
                vec.push(black_box(i));
            }
        })
    });
}

fn ecovec_random_access_small(c: &mut Criterion) {
    let mut vec: EcoVec<usize> = EcoVec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("EcoVec random access small", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..SMALL_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn ecovec_random_access_large(c: &mut Criterion) {
    let mut vec: EcoVec<usize> = EcoVec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    let mut rng = rand::rng();
    c.bench_function("EcoVec random access large", |b| {
        b.iter(|| {
            let idx = rng.random_range(0..LARGE_SIZE);
            black_box(vec[idx]);
        })
    });
}

fn ecovec_remove_small(c: &mut Criterion) {
    c.bench_function("EcoVec remove small", |b| {
        b.iter(|| {
            let mut vec: EcoVec<usize> = EcoVec::new();
            for i in 0..SMALL_SIZE {
                vec.push(i);
            }
            for _ in 0..SMALL_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn ecovec_remove_large(c: &mut Criterion) {
    c.bench_function("EcoVec remove large", |b| {
        b.iter(|| {
            let mut vec: EcoVec<usize> = EcoVec::new();
            for i in 0..LARGE_SIZE {
                vec.push(i);
            }
            for _ in 0..LARGE_SIZE {
                vec.remove(black_box(0));
            }
        })
    });
}

fn ecovec_clone_small(c: &mut Criterion) {
    let mut vec: EcoVec<usize> = EcoVec::new();
    for i in 0..SMALL_SIZE {
        vec.push(i);
    }
    c.bench_function("EcoVec clone small", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

fn ecovec_clone_large(c: &mut Criterion) {
    let mut vec: EcoVec<usize> = EcoVec::new();
    for i in 0..LARGE_SIZE {
        vec.push(i);
    }
    c.bench_function("EcoVec clone large", |b| {
        b.iter(|| {
            let cloned = vec.clone();
            black_box(cloned);
        })
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // Push
    vec_push_small(c);
    vec_push_large(c);
    smallvec_push_small(c);
    smallvec_push_large(c);
    ecovec_push_small(c);
    ecovec_push_large(c);


    // Random Access
    vec_random_access_small(c);
    vec_random_access_large(c);
    smallvec_random_access_small(c);
    smallvec_random_access_large(c);
    ecovec_random_access_small(c);
    ecovec_random_access_large(c);

    // Remove
    vec_remove_small(c);
    vec_remove_large(c);
    smallvec_remove_small(c);
    smallvec_remove_large(c);
    ecovec_remove_small(c);
    ecovec_remove_large(c);

    // Clone
    vec_clone_small(c);
    vec_clone_large(c);
    smallvec_clone_small(c);
    smallvec_clone_large(c);
    ecovec_clone_small(c);
    ecovec_clone_large(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
