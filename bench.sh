RUSTFLAGS="-C target-cpu=core-avx2" cargo build --release
RUSTFLAGS="-C target-cpu=core-avx2" cargo bench --bench benchmark --  --noplot