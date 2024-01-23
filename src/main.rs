use rayon::prelude::*;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // read stdin into a buffer (14GB capacity)
    let mut buffer = Vec::<u8>::with_capacity(14 * 1024 * 1024 * 1024);
    io::stdin().lock().read_to_end(&mut buffer).unwrap();

    // group by station into a hashmap
    let stations_with_values: HashMap<String, Vec<f64>> = buffer
        .par_split(|&c| c == b'\n')
        .fold(
            || HashMap::new(),
            |mut acc, line_bytes| {
                let line = String::from_utf8(line_bytes.to_vec()).unwrap();
                if line.len() == 0 {
                    return acc;
                }
                let mut parts = line.split(';');
                let station = parts.next().unwrap().to_string();
                let value = parts.next().unwrap().parse::<f64>().unwrap();
                acc.entry(station).or_insert_with(Vec::new).push(value);
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

    // avg,min,max for each station
    let result: HashMap<String, (f64, f64, f64)> = stations_with_values
        .par_iter()
        .map(|(station, values)| {
            let avg = ((values.iter().sum::<f64>() / values.len() as f64) * 10.0).round() / 10.0;
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

    // print result
    for (station, (avg, min, max)) in &result {
        println!("{};{},{},{}", station, avg, min, max);
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
let stations_with_values: HashMap<String, Vec<f64>> = rx
    .into_iter()
    .par_bridge()
    .fold(

*/
