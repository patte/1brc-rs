use polars::prelude::*;
use polars::{
    datatypes::DataType,
    lazy::frame::{LazyCsvReader, LazyFileListReader},
};
use std::time::Instant;

fn main() {
    // get filename from first arg
    let filename = std::env::args()
        .nth(1)
        .unwrap_or("../measurements-1000000000.txt".into());

    let now = Instant::now();

    let mut schema: Schema = Schema::new();
    schema.with_column("station".into(), DataType::String);
    schema.with_column("value".into(), DataType::Float64);

    let q = LazyCsvReader::new(filename)
        .has_header(false)
        .with_schema(Some(Arc::new(schema)))
        .with_separator(b';')
        .finish()
        .unwrap()
        .group_by(vec![col("station")])
        .agg([col("value")])
        .select(&[
            col("station"),
            col("value").list().min().name().suffix("_min"),
            col("value").list().mean().round(1).name().suffix("_avg"),
            col("value").list().max().name().suffix("_max"),
        ])
        .select(&[
            col("station"),
            concat_str([col("value_min"), col("value_avg"), col("value_max")], ","),
        ])
        .sort("station", Default::default())
        .with_streaming(true);

    let mut df = q.collect().unwrap();
    //println!("{}", df);
    println!("num stations: {}", df.height());

    CsvWriter::new(std::io::stdout())
        .include_header(false)
        .with_separator(b';')
        .finish(&mut df)
        .unwrap();

    println!(
        "Time={} μs - {} s",
        now.elapsed().as_micros(),
        now.elapsed().as_millis() as f64 / 1000.0
    );
}
