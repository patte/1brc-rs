use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{BufReader, Read};

fn main() {
    // read stdin into a buffer
    //let mut buffer = Vec::<u8>::with_capacity(4 * 1024 * 1024 * 1024);
    //io::stdin().lock().read_to_end(&mut buffer).unwrap();

    // read the file measure.csv into a buffer completely
    let mut buffer = Vec::<u8>::with_capacity(4 * 1024 * 1024 * 1024);
    let mut file = BufReader::new(std::fs::File::open("measurements-1000000000.txt").unwrap());
    file.read_to_end(&mut buffer).unwrap();

    // group by station into a hashmap
    let stations_with_values: HashMap<String, Vec<f32>> = buffer
        .par_split(|&c| c == b'\n')
        .map(|line_bytes| String::from_utf8(line_bytes.to_vec()).unwrap())
        .fold(
            || HashMap::new(),
            |mut acc, line| {
                if line.len() == 0 {
                    return acc;
                }
                let (station, value_raw) = line.split_once(';').unwrap();
                let value = value_raw.parse::<f32>().unwrap();
                acc.entry(station.to_string())
                    .or_insert_with(Vec::new)
                    .push(value);
                acc
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
    drop(buffer);

    let mut result: Vec<(String, (f32, f32, f32))> = stations_with_values
        .par_iter()
        .map(|(station, values)| {
            let avg = ((values.iter().sum::<f32>() / values.len() as f32) * 10.0).round() / 10.0;
            let min = values
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            let max = values
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();
            (station.clone(), (avg, *min, *max))
        })
        .collect();

    // sort
    result
        .as_parallel_slice_mut()
        .par_sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // print result
    for (station, (avg, min, max)) in &result {
        println!("{};{}/{}/{}", station, min, avg, max);
    }

    println!("Number of stations: {}", result.len());
}
//
/* read in chunks

let (tx, rx) = crossbeam_channel::unbounded();
thread::spawn(move || {
    let mut input = io::stdin().lock();
    // read chunks into a buffer and send to the channel, no line splitting yet
    loop {
        let mut buffer = Vec::<u8>::with_capacity(1024 * 1024);
        let n = input.read(&mut buffer).unwrap();
        println!("read {} bytes", n);
        if n == 0 {
            //break;
        }
        tx.send(buffer).unwrap();
    }
});

loop {
    rx.iter().for_each(|buffer| {
        // print buffer
        let mut lines = buffer.split(|&c| c == b'\n');
        println!("{:?}", lines.next().unwrap());
    });
}

// group by station
let stations_with_values: HashMap<String, Vec<f32>> = rx
    .into_iter()
    .par_bridge()
    .fold(

*/
