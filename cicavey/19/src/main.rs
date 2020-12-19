use aoc;
//use regex::Regex;
use pcre2::bytes::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let mut r_pat: HashMap<String, String> = HashMap::new();
    let mut u_pat: HashMap<String, Vec<String>> = HashMap::new();

    let mut test = vec![];

    for line in lines {
        if line.contains(":") {
            let s: Vec<&str> = line.split(": ").collect();
            let id = s[0].to_string();
            let mut val = s[1].to_string();

            if val.contains("\"") {
                val = val.replace("\"", "");
                r_pat.insert(id, val);
            } else {
                let tok: Vec<String> = val.split_whitespace().map(|s| s.to_string()).collect();
                u_pat.insert(id, tok);
            }
        } else {
            test.push(line.clone());
        }
    }

    for _ in 0..1000 {
        let mut transfer = HashSet::new();
        transfer.clear();
        for (k, v) in &r_pat {
            for (uk, uv) in &mut u_pat {
                // part 2, skip these and result manually hack tastic
                if uk == "0" || uk == "8" || uk == "11" {
                    continue;
                }

                // replace any occurences of the re
                for i in 0..uv.len() {
                    if uv[i] == *k {
                        uv[i] = v.clone();
                    }
                }

                // Check if patter is completely resolved
                // contains no numbers
                let mut complete = true;
                for v in uv {
                    if v.chars().all(char::is_numeric) {
                        complete = false;
                        break;
                    }
                }

                if complete {
                    transfer.insert(uk.clone());
                }
            }
        }

        for t in transfer {
            let temp = u_pat.remove(&t).unwrap();
            let pat: String = temp.join("");
            r_pat.insert(t, format!("(?:{})", pat)); // non capturing
        }
    }

    dbg!(u_pat);

    // Manual regex manipulation
    // 0: 8 11
    // 8: 42 | 42 8
    //
    // 42+
    //
    // 11: 42 31 | 42 11 31
    //
    // 42 42 42 {n} 31 31 31 {m} where n == m, recursive?
    //
    //
    // 0: (42)+ ((42)+ (31)+)

    // PCRE2 recursive matching ...wat.
    let mut zero = "(?:42+)(42(?1)?31)".to_string();

    let _42 = r_pat.get("42").unwrap().clone();
    let _31 = r_pat.get("31").unwrap().clone();

    dbg!(&_42, &_31);

    zero = zero.replace("42", &_42);
    zero = zero.replace("31", &_31);

    // let zero = r_pat.get("0").unwrap().clone();

    let r = Regex::new(&format!("^{}$", zero)).unwrap();

    dbg!(&zero);

    let count = test
        .iter()
        .filter(|t| r.is_match(t.as_bytes()).unwrap())
        .count();
    dbg!(count);
}
