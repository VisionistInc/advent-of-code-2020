use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::HashMap;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

type Mask = (u64, u64);

fn parse_mask(raw_mask : &str) -> Mask {
    let set_mask = u64::from_str_radix(&raw_mask.replace('X', "0"), 2).unwrap();
    let clr_mask = u64::from_str_radix(&raw_mask.replace('X', "1"), 2).unwrap();
    (set_mask, clr_mask)
}

fn apply_mask(v : u64, m: Mask) -> u64 {
    // fml - need the parens for order of operations
    (v | m.0) & m.1
}

fn write(addr_mask : String, v: u64, mem: &mut Mem) {
    if addr_mask.contains('X') {
        // pick an X, set it to 1 or 0, call write again
        write(addr_mask.replacen('X', "0", 1), v, mem);
        write(addr_mask.replacen('X', "1", 1), v, mem);
    } else {
        // no X, convert to u64 and write
        let real_addr = u64::from_str_radix(&addr_mask, 2).unwrap();
        // dbg!(real_addr, v);
        mem.insert(real_addr, v);
    }
}

type Mem = HashMap<u64, u64>;

fn main() {
    let lines = lines_from_file("input.txt");

    let mut mask: Mask = (0,0);
    let mut mem : Mem = HashMap::new();

    for line in &lines {
        match &line[..3] {
            "mas" => { // mask = 01110X1X1XX0100011X0111X11001X1X1001
                mask = parse_mask(line.split(" = ").skip(1).next().unwrap());
            }
            "mem" => { //mem[16001] = 2818333
                // sneaky split
                let parts : Vec<String> = line.split("] = ").map(|s| s.to_string()).collect();
                let addr = &parts[0][4..].parse::<u64>().unwrap();
                let val = &parts[1].parse::<u64>().unwrap();

                

                // Use the mask to write memory
                mem.insert(*addr, apply_mask(*val, mask));
            }
            _ => {}
        }
    }

    dbg!(mem.values().sum::<u64>());

    mem.clear();

    let lines = lines_from_file("input.txt");

    let mut addr_mask: String = "".to_string();

    let mut i = 0;
    for line in &lines {
        match &line[..3] {
            "mas" => { // mask = 01110X1X1XX0100011X0111X11001X1X1001
                addr_mask = line.split(" = ").skip(1).next().unwrap().to_string();
                mask = parse_mask(&addr_mask);
            }
            "mem" => { //mem[16001] = 2818333
                // sneaky split
                let parts : Vec<String> = line.split("] = ").map(|s| s.to_string()).collect();
                let addr = parts[0][4..].parse::<u64>().unwrap();
                let val = &parts[1].parse::<u64>().unwrap();

                // Recreate addr with don't care bits
                let mod_addr = format!("{:036b}", addr | mask.0);

                // Recreate new mask and propagate forward floating bits
                let temp = addr_mask.chars().rev().zip(mod_addr.chars().rev()).map(|(a,v)| if a == 'X' { a } else { v }).collect::<String>();
                let mod_addr = temp.chars().rev().collect::<String>();

                // println!("{}\n{:036b} {} addr\n{:036b} set_mask\n{:036b}\n  {} -> {}", addr_mask, addr, addr, mask.0, addr | mask.0, mod_addr, val);
          
                // Use the mask to write memory
                write(mod_addr, *val, &mut mem);

                // if i == 3 {
                //     break;
                // }
            }
            _ => {}
        }
        i += 1;
    }

    //dbg!(mem);
    dbg!(mem.values().map(|&v| v as u128).sum::<u128>());
}
