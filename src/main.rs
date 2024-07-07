pub fn main() {
    use polars::prelude::*;

    let mut _vec_expr = [
        // col("one")
        //     .str()
        //     .slice(lit(0), lit(9))
        //     .cast(DataType::Int32)
        //     .alias("col_1"),
        (col("one") * lit(4)).alias("col_1m4"),
        (col("one") * lit(10)).alias("col_1m10"),
        (col("one") * lit(100000)).alias("col_1m100000"),
        // col("l").str().slice(lit(9), lit(10)).alias("col_2"),
        // col("l").str().slice(lit(19), lit(6)).alias("col_3"),
        // col("l").str().slice(lit(25), lit(40)).alias("col_4"),
        col("four").str().slice(lit(0), lit(0)).alias("col_5"),
        col("four").str().slice(lit(1), lit(1)).alias("col_6"),
        col("four").str().slice(lit(2), lit(1)).alias("col_7"),
        col("four").str().slice(lit(3), lit(1)).alias("col_8"),
        col("four").str().slice(lit(4), lit(1)).alias("col_9"),
        col("four").str().slice(lit(5), lit(1)).alias("col_10"),
        col("four").str().slice(lit(6), lit(1)).alias("col_11"),
    ];

    println!("{:?}", _vec_expr);

    use std::time::Instant;
    let now = Instant::now();

    let path = "data/temp_file_fwf.txt";
    // Read with csv reader lazily (if you have comma in the file, change the delimiter)
    let data_ = LazyCsvReader::new(path)
        .with_has_header(true)
        // test 100 first lines
        // .with_n_rows(Some(100))
        .finish()
        .unwrap();

    println!("{:?}", data_.clone().collect());

    // append the polars lazyframe with the expressions generated above
    let data_ = data_.with_columns(_vec_expr);

    // collect
    println!("{:?}", data_.collect());
    println!("Elapsed: {:.2?}", now.elapsed());
}
