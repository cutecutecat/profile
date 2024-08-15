use cases::method1::asymmetric_binary_dot_product as method1;
use cases::method2::asymmetric_binary_dot_product as method2;
use cases::method3::asymmetric_binary_dot_product_16 as method3_16;
use cases::method3::asymmetric_binary_dot_product_120 as method3_120;
use cases::method3::asymmetric_binary_dot_product_200 as method3_200;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::{thread_rng, Rng};
use std::hint::black_box;



fn criterion_benchmark_short(c: &mut Criterion) {
    let mut left = [0u64; 16];
    let mut right = [0u64; 64];
    thread_rng().fill(&mut left[..]);
    thread_rng().fill(&mut right[..]);
    c.bench_function("dim_16_method_c", |b| {
        b.iter(|| method1(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_16_method_rs", |b| {
        b.iter(|| method2(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_16_method_c_static", |b| {
        b.iter(|| method3_16(black_box(&left), black_box(&right)))
    });
}

fn criterion_benchmark_mid(c: &mut Criterion) {
    let mut left = [0u64; 120];
    let mut right = [0u64; 480];
    thread_rng().fill(&mut left[..]);
    thread_rng().fill(&mut right[..]);
    c.bench_function("dim_120_method_c", |b| {
        b.iter(|| method1(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_120_method_rs", |b| {
        b.iter(|| method2(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_120_method_c_static", |b| {
        b.iter(|| method3_120(black_box(&left), black_box(&right)))
    });
}

fn criterion_benchmark_long(c: &mut Criterion) {
    let mut left = [0u64; 200];
    let mut right = [0u64; 800];
    thread_rng().fill(&mut left[..]);
    thread_rng().fill(&mut right[..]);
    c.bench_function("dim_200_method_c", |b| {
        b.iter(|| method1(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_200_method_rs", |b| {
        b.iter(|| method2(black_box(&left), black_box(&right)))
    });
    c.bench_function("dim_200_method_c_static", |b| {
        b.iter(|| method3_200(black_box(&left), black_box(&right)))
    });
}

criterion_group!{
    name = benches;
    config = Criterion::default();
    targets = criterion_benchmark_short, criterion_benchmark_mid, criterion_benchmark_long
}
criterion_main!(benches);
