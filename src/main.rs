pub fn main() {
    use polars::prelude::*;

    // let mut _vec_expr = [
    //     col("l")
    //         .str()
    //         .slice(lit(0), lit(9))
    //         .cast(DataType::Int32)
    //         .alias("col_1"),
    //     // (col("l").str().slice(lit(0), lit(9)).cast(DataType::Int32) * lit(4)).alias("col_1m4"),
    //     // (col("l").str().slice(lit(0), lit(9)).cast(DataType::Int32) * lit(10)).alias("col_1m10"),
    //     col("l").str().slice(lit(9), lit(10)).alias("col_2"),
    //     col("l").str().slice(lit(19), lit(6)).alias("col_3"),
    //     col("l").str().slice(lit(25), lit(40)).alias("col_4"),
    //     col("l").str().slice(lit(25), lit(1)).alias("col_5"),
    //     col("l").str().slice(lit(26), lit(1)).alias("col_6"),
    //     col("l").str().slice(lit(27), lit(1)).alias("col_7"),
    //     col("l").str().slice(lit(28), lit(1)).alias("col_8"),
    //     col("l").str().slice(lit(29), lit(1)).alias("col_9"),
    //     col("l").str().slice(lit(30), lit(1)).alias("col_10"),
    //     col("l").str().slice(lit(31), lit(1)).alias("col_11"),
    // ];

    // println!("{:?}", _vec_expr);

    use std::time::Instant;
    let now = Instant::now();

    let path = "data/temp_file_fwf.txt";
    // Read with csv reader lazily (if you have comma in the file, change the delimiter)
    let data_ = LazyCsvReader::new(path)
        // read just one column named "l" for line
        .with_schema(Some(Arc::new(
            Schema::from_iter(vec![Field::new("l", DataType::String)]).into(),
        )))
        .with_has_header(false)
        // test 100 first lines
        // .with_n_rows(Some(100))
        .finish()
        .unwrap();

    println!("{:?}", data_.clone().collect());

    // append the polars lazyframe with the expressions generated above
    let data_ = data_
        .with_columns([col("l")
            .str()
            .extract_groups(r"^(?<col_1>.{9})(?<col_2>.{10})(?<col_3>.{6})(?<col_4>.*)")
            .unwrap()])
        .unnest(["l"].into_iter());

    // collect
    println!("{:?}", data_.collect());
    println!("Elapsed: {:.2?}", now.elapsed());
}
