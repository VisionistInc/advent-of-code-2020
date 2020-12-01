use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    let file = File::open("input.txt").expect("Could not open file");
    let v = read(file).expect("Could not read lines");

    'outer: 
    for i in &v {
        for j in &v {
            if i + j == 2020 {
                println!("{}", i*j);
                break 'outer;
            }
        }
    }

    'outer2: 
    for i in &v {
        for j in &v {
            for k in &v {
                if i + j + k  == 2020 {
                    println!("{}", i*j*k);
                    break 'outer2;
                }
            }
        }
    }
}
