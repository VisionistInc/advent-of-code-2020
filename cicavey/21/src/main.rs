use aoc;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn main() {
    let lines = aoc::lines_from_file("input.txt");


    let mut a_to_i: HashMap<String, HashSet<String>> = HashMap::new();

    let mut ing_hist: HashMap<String, u64> = HashMap::new();

    for mut line in lines {
        // trim off that pesky railing paren
        line.pop();
        let mut s = line.split("(contains ");
        
        let ingredients: HashSet<String> = s.next().unwrap().split_whitespace().map(|s| s.to_owned()).collect();

        for i in &ingredients {
            *ing_hist.entry(i.clone()).or_default() += 1;
        }

        let alergens : HashSet<String> = s.next().unwrap().split(", ").map(|s| s.to_owned()).collect();


        // keep intersecting ingredients by alergen to distill posible mappings. Resolve it in the next phase
        for alergen in alergens {

            if a_to_i.contains_key(&alergen) {
                // wtf. I hate rust.
                a_to_i.entry(alergen).and_modify(|e| *e = e.intersection(&ingredients).map(|q| q.clone()).collect());
            } else {
                a_to_i.insert(alergen, ingredients.clone());   
            }
        }
    }

    let mut resolved: HashMap<String, String> = HashMap::new();

     // this is just like day 16 now
    loop {
        let mut unresolved_count = 0;

       
        for (k, v) in a_to_i.iter_mut() {

            // Found a resolved mapping! 
            if v.len() == 1 {
                resolved.insert(v.iter().next().unwrap().clone(), k.clone());
                v.clear();
            }

            unresolved_count += v.len();
        }


        // clean up all resolved items
        for (_, v) in a_to_i.iter_mut() {
            for r in resolved.keys() {
                v.remove(&r.clone());
            }
        }

        if unresolved_count == 0 {
            break;
        }
    }

    // TODO filter_map?
    let sum: u64 = ing_hist.iter().filter(|(k,_)| !resolved.contains_key(k.clone())).map(|(_, v)| v).sum();
    dbg!(sum);

    // cheap sorting hack
    let mut resolved_inv_sort: BTreeMap<String, String> = BTreeMap::new();
    for (k, v) in resolved {
        resolved_inv_sort.insert(v, k);
    }
    for v in resolved_inv_sort.values() {
        print!("{},", v);
    }
    println!();
}
