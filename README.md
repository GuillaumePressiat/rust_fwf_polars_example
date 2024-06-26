# Slower parsing of a file in Rust 1.81?

Repo support for issue:
https://github.com/rust-lang/rust/issues/126937

## Sample code to illustrate a difference in behaviour between rust 1.79 and rust 1.81 with polars expression


The file is in `data`. It's a fixed widths file of 500k rows.

```shell
% head data/temp_file_fwf.txt 
000000001Strawberryfieldsforever
000000002Strawberryfieldsforever
000000003Strawberryfieldsforever
000000004Strawberryfieldsforever
000000005Strawberryfieldsforever
% cat data/temp_file_fwf.txt | wc -l 
  500000
```

## Summary

With the exact same code parsing of file takes:

- ~ 42ms with Rust 1.79
- ~ 97ms with Rust 1.81

On a mac M2 pro.

I observe longer run time with rust 1.81 than rust 1.79 which is concentrated on this polars expression vector:

```rust 
    let mut _vec_expr = [
        col("l")
            .str()
            .slice(lit(0), lit(9))
            .cast(DataType::Int32)
            .alias("col_1"),
        col("l").str().slice(lit(9), lit(10)).alias("col_2"),
        col("l").str().slice(lit(19), lit(6)).alias("col_3"),
        col("l").str().slice(lit(25), lit(40)).alias("col_4"),
        col("l").str().slice(lit(25), lit(1)).alias("col_5"),
        col("l").str().slice(lit(26), lit(1)).alias("col_6"),
        col("l").str().slice(lit(27), lit(1)).alias("col_7"),
        col("l").str().slice(lit(28), lit(1)).alias("col_8"),
        col("l").str().slice(lit(29), lit(1)).alias("col_9"),
        col("l").str().slice(lit(30), lit(1)).alias("col_10"),
        col("l").str().slice(lit(31), lit(1)).alias("col_11"),
    ];
```


### Rust 1.79

#### Build and run

```shell
cargo +1.79.0 build --release
```

```shell
./target/release/rust_fwf_polars
[col("l").str.slice([dyn int: 0, dyn int: 9]).cast(Int32).alias("col_1"), col("l").str.slice([dyn int: 9, dyn int: 10]).alias("col_2"), col("l").str.slice([dyn int: 19, dyn int: 6]).alias("col_3"), col("l").str.slice([dyn int: 25, dyn int: 40]).alias("col_4"), col("l").str.slice([dyn int: 25, dyn int: 1]).alias("col_5"), col("l").str.slice([dyn int: 26, dyn int: 1]).alias("col_6"), col("l").str.slice([dyn int: 27, dyn int: 1]).alias("col_7"), col("l").str.slice([dyn int: 28, dyn int: 1]).alias("col_8"), col("l").str.slice([dyn int: 29, dyn int: 1]).alias("col_9"), col("l").str.slice([dyn int: 30, dyn int: 1]).alias("col_10"), col("l").str.slice([dyn int: 31, dyn int: 1]).alias("col_11")]

Ok(shape: (500_000, 12)
┌─────────────────────────────────┬───────┬────────────┬────────┬───┬───────┬───────┬────────┬────────┐
│ l                               ┆ col_1 ┆ col_2      ┆ col_3  ┆ … ┆ col_8 ┆ col_9 ┆ col_10 ┆ col_11 │
│ ---                             ┆ ---   ┆ ---        ┆ ---    ┆   ┆ ---   ┆ ---   ┆ ---    ┆ ---    │
│ str                             ┆ i32   ┆ str        ┆ str    ┆   ┆ str   ┆ str   ┆ str    ┆ str    │
╞═════════════════════════════════╪═══════╪════════════╪════════╪═══╪═══════╪═══════╪════════╪════════╡
│ 000000001Strawberryfieldsforev… ┆ 1     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000002Strawberryfieldsforev… ┆ 2     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000003Strawberryfieldsforev… ┆ 3     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000004Strawberryfieldsforev… ┆ 4     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000005Strawberryfieldsforev… ┆ 5     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ …                               ┆ …     ┆ …          ┆ …      ┆ … ┆ …     ┆ …     ┆ …      ┆ …      │
│ 000000001Strawberryfieldsforev… ┆ 1     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000002Strawberryfieldsforev… ┆ 2     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000003Strawberryfieldsforev… ┆ 3     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000004Strawberryfieldsforev… ┆ 4     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000005Strawberryfieldsforev… ┆ 5     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
└─────────────────────────────────┴───────┴────────────┴────────┴───┴───────┴───────┴────────┴────────┘)
Elapsed: 48.91ms
```

#### Benchmark for 1.79

```shell
% bench ./target/release/rust_fwf_polars
benchmarking ./target/release/rust_fwf_polars
time                 43.08 ms   (42.35 ms .. 43.88 ms)
                     0.998 R²   (0.996 R² .. 1.000 R²)
mean                 42.78 ms   (42.28 ms .. 43.33 ms)
std dev              1.089 ms   (868.1 μs .. 1.308 ms)
```


### Rust 1.81

#### Build and run

```shell
cargo --version
cargo 1.81.0-nightly (bc89bffa5 2024-06-22)
cargo build --release
```

```shell
./target/release/rust_fwf_polars
[col("l").str.slice([dyn int: 0, dyn int: 9]).cast(Int32).alias("col_1"), col("l").str.slice([dyn int: 9, dyn int: 10]).alias("col_2"), col("l").str.slice([dyn int: 19, dyn int: 6]).alias("col_3"), col("l").str.slice([dyn int: 25, dyn int: 40]).alias("col_4"), col("l").str.slice([dyn int: 25, dyn int: 1]).alias("col_5"), col("l").str.slice([dyn int: 26, dyn int: 1]).alias("col_6"), col("l").str.slice([dyn int: 27, dyn int: 1]).alias("col_7"), col("l").str.slice([dyn int: 28, dyn int: 1]).alias("col_8"), col("l").str.slice([dyn int: 29, dyn int: 1]).alias("col_9"), col("l").str.slice([dyn int: 30, dyn int: 1]).alias("col_10"), col("l").str.slice([dyn int: 31, dyn int: 1]).alias("col_11")]

┌─────────────────────────────────┬───────┬────────────┬────────┬───┬───────┬───────┬────────┬────────┐
│ l                               ┆ col_1 ┆ col_2      ┆ col_3  ┆ … ┆ col_8 ┆ col_9 ┆ col_10 ┆ col_11 │
│ ---                             ┆ ---   ┆ ---        ┆ ---    ┆   ┆ ---   ┆ ---   ┆ ---    ┆ ---    │
│ str                             ┆ i32   ┆ str        ┆ str    ┆   ┆ str   ┆ str   ┆ str    ┆ str    │
╞═════════════════════════════════╪═══════╪════════════╪════════╪═══╪═══════╪═══════╪════════╪════════╡
│ 000000001Strawberryfieldsforev… ┆ 1     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000002Strawberryfieldsforev… ┆ 2     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000003Strawberryfieldsforev… ┆ 3     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000004Strawberryfieldsforev… ┆ 4     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000005Strawberryfieldsforev… ┆ 5     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ …                               ┆ …     ┆ …          ┆ …      ┆ … ┆ …     ┆ …     ┆ …      ┆ …      │
│ 000000001Strawberryfieldsforev… ┆ 1     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000002Strawberryfieldsforev… ┆ 2     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000003Strawberryfieldsforev… ┆ 3     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000004Strawberryfieldsforev… ┆ 4     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
│ 000000005Strawberryfieldsforev… ┆ 5     ┆ Strawberry ┆ fields ┆ … ┆ e     ┆ v     ┆ e      ┆ r      │
└─────────────────────────────────┴───────┴────────────┴────────┴───┴───────┴───────┴────────┴────────┘)
Elapsed: 109.79ms
```

#### Benchmark for 1.81

```shell
% bench ./target/release/rust_fwf_polars
benchmarking ./target/release/rust_fwf_polars
time                 94.80 ms   (91.95 ms .. 96.55 ms)
                     0.998 R²   (0.993 R² .. 1.000 R²)
mean                 97.26 ms   (95.58 ms .. 99.10 ms)
std dev              2.838 ms   (1.982 ms .. 3.961 ms)
```

## Rust 1.80

### 2/ no problem with rust 1.80

On my mac silicon, I've installed rust 1.80 nightly and it runs in correct times:

```sh
# nightly-2024-06-07-aarch64-apple-darwin installed - rustc 1.80.0-nightly (98489f248 2024-06-06)
cargo +nightly-2024-06-07 build --release
% bench ./target/release/rust_fwf_polars
benchmarking ./target/release/rust_fwf_polars
time                 42.37 ms   (40.73 ms .. 43.81 ms)
                     0.996 R²   (0.992 R² .. 0.998 R²)
mean                 43.12 ms   (41.77 ms .. 46.17 ms)
std dev              3.807 ms   (1.810 ms .. 6.321 ms)
variance introduced by outliers: 34% (moderately inflated)
```

## Regression between 21/06 and 22/06 nightly builds?

```sh
# rustup install nightly-2024-06-22
cargo +nightly-2024-06-22 run --release 
% bench ./target/release/rust_fwf_polars   
benchmarking ./target/release/rust_fwf_polars
time                 94.54 ms   (82.58 ms .. 105.6 ms)
                     0.970 R²   (0.916 R² .. 0.994 R²)
mean                 100.5 ms   (92.48 ms .. 113.0 ms)
std dev              16.04 ms   (10.54 ms .. 21.82 ms)
variance introduced by outliers: 53% (severely inflated)

# rustup install nightly-2024-06-21
cargo +nightly-2024-06-21 run --release 
% bench ./target/release/rust_fwf_polars 
benchmarking ./target/release/rust_fwf_polars
time                 42.77 ms   (41.82 ms .. 43.96 ms)
                     0.997 R²   (0.994 R² .. 0.999 R²)
mean                 42.45 ms   (41.52 ms .. 43.25 ms)
std dev              1.729 ms   (1.277 ms .. 2.614 ms)
variance introduced by outliers: 13% (moderately inflated)
```

### Analysis of what is slow


![](assets/image1.png)

To pin down which column leads to the regression and know the dtype involved if there is one specific.

```bash
# So I did it: timings are done with bench)
# Just l (no slice expressions) : 11ms
l + col_1 : col_1 > Int32 : 27ms
l + col_1 + col_2 : col_2 > String : 39ms
l + col_1 + col_2 + col_3 : col_3 String : 50ms
l + col_1 + col_2 + col_3 + col_4 : col_4 String : 60ms
l + col_1 + col_2 + col_3 + col_4 + col_5 + col_6 : String : 70ms
....
l + col_1 + col_2 + col_3 + col_4 + col_5 + ... + col_11 : String : 94ms
```
the cost of slices is important

whereas if I multiply col_1 by a litteral (4 or 10), the cost is lower

```bash
col_1 and col_1m4 col_1 * lit(4) : 33ms
col_1 and col_1m4 and col_1m10, col_1 * lit(10) : 35ms
```
![](assets/image2.png)

Done all of this with nightly build before regression:

![](assets/image3.png)

Measures are in microseconds.
In both case (integer or string), the cost of adding a new column is bigger in regression 06/22 compared to 06/21.


