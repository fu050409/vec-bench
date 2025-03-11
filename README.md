# Rust Vec Benchmarks

This is a benchmark to compare the performance of `Vec` and its alternatives in Rust.

## Run the benchmarks

```shell
cargo bench
```

## Results

Benchmarks based on `criterion-rs`.

In this benchmark, we compare the performance of `Vec`, `SmallVec`, and `EcoVec` in both scenarios
with a small and large number of elements.

Here's a summary of the performance differences between `Vec`, `SmallVec`, and `EcoVec` based on various operations:

| Operation               | Vec           | SmallVec      | EcoVec        |
| ----------------------- | ------------- | ------------- | ------------- |
| **Push Small**          | 545.93 ns     | **191.09 ns** | 1.0947 µs     |
| **Push Large**          | 26.142 µs     | **20.923 µs** | 58.836 µs     |
| **Random Access Small** | 8.6471 ns     | **8.5620 ns** | 8.5393 ns     |
| **Random Access Large** | **8.8973 ns** | 8.6310 ns     | 8.7271 ns     |
| **Remove Small**        | 2.8257 µs     | **2.3943 µs** | 3.4883 µs     |
| **Remove Large**        | 12.387 ms     | **12.325 ms** | 12.364 ms     |
| **Clone Small**         | 31.911 ns     | 481.43 ns     | **27.158 ns** |
| **Clone Large**         | 3.8817 µs     | 7.6483 µs     | **27.154 ns** |

### Observations and Characteristics

- **Push Operation**
  - For small data, `SmallVec` is the fastest, about **2.85x faster** than `Vec`, while `EcoVec` is significantly slower.
  - For large data, `SmallVec` remains faster than `Vec`, while `EcoVec` performs the worst.
- **Random Access**
  - For small data, `SmallVec` is slightly faster, but the difference with `Vec` and `EcoVec` is negligible.
  - For large data, `Vec` is the fastest, followed by `EcoVec`, with `SmallVec` being the slowest.
- **Remove Operation**
  - For small data, `SmallVec` performs the best, followed by `Vec`, with `EcoVec` being the slowest.
  - For large data, `SmallVec` still slightly outperforms `Vec`, while `EcoVec` has similar performance.
- **Clone Operation**
  - For small data, `EcoVec` is the fastest, followed by `Vec`, with `SmallVec` being the slowest.
  - For large data, `EcoVec` is dramatically faster than both `Vec` and `SmallVec`, likely due to internal optimizations.

### Conclusion

- **`SmallVec` is ideal for small data** since it performs best in push and remove operations.
- **`Vec` is better for handling large data**, especially in random access and large push operations.
- **`EcoVec` excels at cloning**, but its push and remove performance are significantly weaker.

## Detailed Results

The results below are from a Arch Linux laptop machine with an Intel i9-13900H CPU.

Ignore the performance changes, these changes are due to the benchmarking environment.
No code changes between the two benchmark results.

```console
$ cargo bench
    Finished `bench` profile [optimized] target(s) in 0.21s
     Running unittests src/main.rs (target/release/deps/vec_bench-293bbd1569452586)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running benches/vector_bench.rs (target/release/deps/vector_bench-4a10436f03261850)
Gnuplot not found, using plotters backend
Vec push small          time:   [545.93 ns 548.38 ns 550.93 ns]
                        change: [+1.8678% +2.6091% +3.3218%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) low mild

Vec push large          time:   [26.142 µs 27.056 µs 27.978 µs]
                        change: [+1.2020% +2.7898% +4.5940%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 13 outliers among 100 measurements (13.00%)
  1 (1.00%) high mild
  12 (12.00%) high severe

SmallVec push small     time:   [191.09 ns 192.12 ns 193.37 ns]
                        change: [-0.4556% +0.1861% +0.8986%] (p = 0.63 > 0.05)
                        No change in performance detected.
Found 6 outliers among 100 measurements (6.00%)
  4 (4.00%) high mild
  2 (2.00%) high severe

SmallVec push large     time:   [20.923 µs 20.992 µs 21.062 µs]
                        change: [+4.2706% +4.8220% +5.3662%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  4 (4.00%) high mild

EcoVec push small       time:   [1.0947 µs 1.0969 µs 1.0997 µs]
                        change: [+5.0746% +5.4087% +5.8262%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) low severe
  2 (2.00%) low mild
  6 (6.00%) high mild
  4 (4.00%) high severe

EcoVec push large       time:   [58.836 µs 59.066 µs 59.389 µs]
                        change: [+0.7037% +1.2523% +2.0659%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

Vec random access small time:   [8.6471 ns 8.8226 ns 9.0343 ns]
                        change: [-5.9355% -4.6739% -3.2814%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 12 outliers among 100 measurements (12.00%)
  3 (3.00%) high mild
  9 (9.00%) high severe

Vec random access large time:   [8.8973 ns 8.9921 ns 9.0812 ns]
                        change: [+2.5902% +3.6440% +4.7211%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high severe

SmallVec random access small
                        time:   [8.5620 ns 8.5798 ns 8.6064 ns]
                        change: [+0.3176% +1.1094% +2.4671%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) low mild
  3 (3.00%) high severe

SmallVec random access large
                        time:   [8.6310 ns 8.8351 ns 9.0962 ns]
                        change: [-6.1774% -5.0287% -3.4832%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high severe

EcoVec random access small
                        time:   [8.5393 ns 8.5475 ns 8.5554 ns]
                        change: [-0.8572% -0.3052% +0.1405%] (p = 0.26 > 0.05)
                        No change in performance detected.
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) low mild
  1 (1.00%) high severe

EcoVec random access large
                        time:   [8.7271 ns 8.7752 ns 8.8359 ns]
                        change: [+3.9921% +4.6974% +5.3090%] (p = 0.00 < 0.05)
                        Performance has regressed.

Vec remove small        time:   [2.8257 µs 2.8297 µs 2.8363 µs]
                        change: [-4.3310% -4.2161% -4.1038%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) low mild
  3 (3.00%) high mild
  2 (2.00%) high severe

Vec remove large        time:   [12.387 ms 12.497 ms 12.632 ms]
                        change: [+0.6086% +1.5819% +2.6797%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 28 outliers among 100 measurements (28.00%)
  11 (11.00%) low severe
  3 (3.00%) low mild
  14 (14.00%) high severe

SmallVec remove small   time:   [2.3943 µs 2.3981 µs 2.4020 µs]
                        change: [+0.0706% +0.3157% +0.6093%] (p = 0.02 < 0.05)
                        Change within noise threshold.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

SmallVec remove large   time:   [12.325 ms 12.354 ms 12.400 ms]
                        change: [-0.0206% +0.2387% +0.6352%] (p = 0.17 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  1 (1.00%) low mild
  2 (2.00%) high mild
  5 (5.00%) high severe

EcoVec remove small     time:   [3.4883 µs 3.4931 µs 3.4996 µs]
                        change: [+4.8839% +5.1646% +5.5133%] (p = 0.00 < 0.05)
                        Performance has regressed.

EcoVec remove large     time:   [12.364 ms 12.372 ms 12.381 ms]
                        change: [-0.0562% +0.0093% +0.0820%] (p = 0.80 > 0.05)
                        No change in performance detected.
Found 25 outliers among 100 measurements (25.00%)
  22 (22.00%) low severe
  3 (3.00%) high severe

Vec clone small         time:   [31.911 ns 31.942 ns 31.976 ns]
                        change: [+0.2406% +0.5650% +1.0530%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  1 (1.00%) high mild
  1 (1.00%) high severe

Vec clone large         time:   [3.8817 µs 3.8829 µs 3.8844 µs]
                        change: [-5.6435% -5.1620% -4.8268%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

SmallVec clone small    time:   [481.43 ns 481.87 ns 482.28 ns]
                        change: [-0.1164% +0.0172% +0.1510%] (p = 0.80 > 0.05)
                        No change in performance detected.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

SmallVec clone large    time:   [7.6483 µs 7.6625 µs 7.6777 µs]
                        change: [-1.2131% +0.0180% +0.9937%] (p = 0.97 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  2 (2.00%) high mild

EcoVec clone small      time:   [27.158 ns 27.166 ns 27.176 ns]
                        change: [+0.0829% +0.1673% +0.2401%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 6 outliers among 100 measurements (6.00%)
  6 (6.00%) high mild

EcoVec clone large      time:   [27.154 ns 27.165 ns 27.177 ns]
                        change: [-0.0895% -0.0011% +0.0628%] (p = 0.98 > 0.05)
                        No change in performance detected.
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe
```
