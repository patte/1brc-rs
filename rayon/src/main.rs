#![feature(slice_split_once)]

use fixed::types::I48F16;
use memmap2::Mmap;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
type Value = I48F16;

// https://gist.github.com/Lucretiel/b9d8a2f75c445ba62035fd80adb5fd57
// parses a byte slice into a fixed point number eg.
//  123 => 12.3
//  -12 => -1.2
fn fast_parse(input: &[u8]) -> Value {
    let neg = input[0] == b'-';
    let len = input.len();

    let (d1, d2, d3) = match (neg, len) {
        (false, 3) => (0, input[0] - b'0', input[2] - b'0'),
        (false, 4) => (input[0] - b'0', input[1] - b'0', input[3] - b'0'),
        (true, 4) => (0, input[1] - b'0', input[3] - b'0'),
        (true, 5) => (input[1] - b'0', input[2] - b'0', input[4] - b'0'),
        _ => unreachable!(),
    };

    let int = (d1 as i16 * 100) + (d2 as i16 * 10) + (d3 as i16);
    let int = if neg { -int } else { int };

    Value::from_num(int) / Value::from_num(10)
}

fn main() {
    // read stdin into a buffer
    //let mut buffer = Vec::<u8>::with_capacity(4 * 1024 * 1024 * 1024);
    //io::stdin().lock().read_to_end(&mut buffer).unwrap();

    let path = "../measurements-1000000000.txt";
    let file = File::open(path).expect("failed to open file");

    // read the file measure.csv into a vec
    //let mut file_vec = Vec::<u8>::with_capacity(4 * 1024 * 1024 * 1024);
    //let mut file_buffer = BufReader::new(file);
    //file_buffer.read_to_end(&mut file_vec).unwrap();

    // map the file into memory
    let mapped_data = unsafe { Mmap::map(&file) }.expect("failed to create memory map");
    let raw_data = &*mapped_data;
    let raw_data = raw_data.strip_suffix(b"\n").unwrap_or(raw_data);

    // group by station into a hashmap
    let stations_with_values: HashMap<&[u8], Vec<Value>> = raw_data
        .par_split(|&b| b == b'\n')
        .map(|line_bytes| {
            let (station, value_raw) = line_bytes
                .split_once(|&b| b == b';')
                .expect("no ; separator");
            let value = fast_parse(value_raw);
            (station, value)
        })
        .fold(
            || HashMap::new(),
            |mut map, (station, value)| {
                map.entry(station).or_insert_with(Vec::new).push(value);
                map
            },
        )
        .reduce_with(|mut acc, mut map| {
            for (key, value) in map.drain() {
                acc.entry(key).or_insert_with(Vec::new).extend(value);
            }
            acc
        })
        .unwrap();

    // release buffer
    //drop(file_vec);

    let mut result: Vec<(&&[u8], (Value, Value, Value))> = stations_with_values
        .par_iter()
        .map(|(station, values)| {
            let avg = ((values.iter().sum::<Value>() / Value::from_num(values.len()))
                * Value::from_num(10))
            .round()
                / Value::from_num(10);
            let min = values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let max = values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            (station, (avg, *min, *max))
        })
        .collect();

    // sort
    result
        .as_parallel_slice_mut()
        .par_sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // print result
    for (station, (avg, min, max)) in &result {
        println!(
            "{};{}/{}/{}",
            String::from_utf8_lossy(station),
            avg,
            min,
            max
        );
    }

    println!("Number of stations: {}", result.len());
}
