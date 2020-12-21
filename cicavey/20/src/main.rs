use aoc;
use std::collections::HashMap;

fn rev(s: String) -> String {
    s.chars().rev().collect::<String>()
}

fn line_to_id(l: &String) -> (u64, u64) {
    let mut temp = l.replace(".", "0").replace("#", "1").to_string();

    // Bookend the string with bits to ensure that leading/trailing zero bits count
    temp.insert(0, '1');
    temp.push('1');

    // kinda like a hash of the border in bother directions
    let forward = u64::from_str_radix(&temp, 2).unwrap();
    let reverse = u64::from_str_radix(&rev(temp), 2).unwrap();

    (forward, reverse)
}

type TileId = u64;

#[derive(Debug, PartialEq)]
enum TileKind {
    Interior,
    Edge,
    Corner,
}

// A struct with two fields
#[derive(Debug)]
struct Tile {
    id: TileId,
    data: Vec<String>,
    kind: TileKind,
    borders: Vec<u64>,
}

impl Tile {
    fn new(id: u64, data: Vec<String>) -> Tile {
        // Use the data to dervice the border ids

        let top_id = data.get(0).unwrap().to_string();
        let bot_id = data.last().unwrap().to_string();

        let mut left = "".to_string();
        let mut right = "".to_string();
        for tl in &data {
            left.push(tl.chars().next().unwrap());
            right.push(tl.chars().rev().next().unwrap());
        }

        let left_id = left.to_string();
        let right_id = right.to_string();

        // Given strings, creates binary ids for all borders including reversed versions
        let b = generate_borders(top_id, bot_id, left_id, right_id);

        Tile {
            id: id,
            data: data.clone(),
            kind: TileKind::Interior, // default to interior until we can figure out the types
            borders: b,
        }
    }
}

fn generate_borders(
    // borders: &mut HashMap<u64, u64>,
    top_id: String,
    bot_id: String,
    left_id: String,
    right_id: String,
) -> Vec<u64> {
    let mut all = vec![];

    let ids = line_to_id(&top_id);
    all.push(ids.0);
    all.push(ids.1);

    let ids = line_to_id(&bot_id);
    all.push(ids.0);
    all.push(ids.1);

    let ids = line_to_id(&left_id);
    all.push(ids.0);
    all.push(ids.1);

    let ids = line_to_id(&right_id);
    all.push(ids.0);
    all.push(ids.1);

    all
}

fn main() {
    let lines = aoc::lines_from_file("input.txt");

    let mut id = 0;
    let mut cur_tile: Vec<String> = vec![];
    let mut tiles: HashMap<TileId, Tile> = HashMap::new();
    let mut borders: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        if line.starts_with("Tile") {
            let tok: Vec<&str> = line.split_whitespace().collect();
            let ids = tok[1];
            id = ids[0..ids.len() - 1].to_string().parse::<u64>().unwrap();
            continue;
        }

        if line.is_empty() {
            tiles.insert(id, Tile::new(id, cur_tile.clone()));
            cur_tile.clear();
            continue;
        }

        cur_tile.push(line);
    }

    // Create a histogram of all borders
    for (_, t) in &tiles {
        for b in &t.borders {
            *borders.entry(*b).or_insert(0) += 1;
        }
    }

    // Refine the tile types now by checking the borders
    for (tid, tile) in &mut tiles {
        // get border counts for the current borders, both types
        let ss = tile
            .borders
            .iter()
            .map(|b| borders.get(b).unwrap())
            .fold(0, |acc, v| acc + v);

        // Update tile kind based on number of non-unique edges
        match ss {
            14 => tile.kind = TileKind::Edge,
            12 => tile.kind = TileKind::Corner,
            _ => {}
        }
    }

    // multiple all corner tile ids together
    let prod = tiles
        .iter()
        .filter(|(_, v)| v.kind == TileKind::Corner)
        .fold(1, |acc, (_, t)| acc * t.id);

    dbg!(prod); //20913499394191
}
