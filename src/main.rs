use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;


#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Point {
        Point{x: x, y: y}
    }

    fn dist(&self, other: &Self) -> f32 {
        let x_sq = i64::pow(self.x - other.x, 2);
        let y_sq = i64::pow(self.y - other.y, 2);

        return ((x_sq + y_sq) as f32).sqrt();
    }
}

fn build_distance_map() ->  HashMap<char, HashMap<char, f32>> {
    let keyboard = vec![String::from("1  2  3  4  5  6  7  8  9  0  - "),
                        String::from(" q  w  e  r  t  y  u  i  o  p "),
                        String::from("  a  s  d  f  g  h  j  k  l  ;  ' "),
                        String::from("   z  x  c  v  b  n  m  ,  .  ? ")];
    
    let mut key_positions = HashMap::new();

    for (row_idx, row) in keyboard.iter().enumerate() {
        for (column_idx, c) in row.chars().enumerate() {
            if c == ' ' { continue; }
            key_positions.insert(c, Point::new(column_idx as i64, row_idx as i64));
        }
    }
    let mut distance_map = HashMap::new();

    for (parent_key, parent_position) in key_positions.iter() {
        let new_map: HashMap<char, f32> = HashMap::new();
        distance_map.entry(*parent_key).or_insert(new_map);
        let parent_distance_map = distance_map.get_mut(parent_key).unwrap();

        for (child_key, child_position) in key_positions.iter() {
            parent_distance_map.insert(*child_key, parent_position.dist(child_position));
        }
    }

    return distance_map;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_distance(distance_map: &HashMap<char, HashMap<char, f32>>, word: &String) -> f32 {
    let mut dist: f32 = 0.0;
    for (idx, c) in word.chars().enumerate() {
        if idx == 0 { continue; }
        let prev_char = word.chars().nth(idx-1).unwrap();
        dist += distance_map[&c][&prev_char];
    }
    return dist / word.len() as f32;
}

fn main() {
    let distance_map_start = Instant::now();
    let distance_map = build_distance_map();
    println!("Built distance map in {:.2?}", distance_map_start.elapsed());

    //let mut word_distances: HashMap<String, f32> = HashMap::new();
    let mut biggest_word: String = String::from("");
    let mut biggest_word_distance: f32 = 0.0;

    let biggest_word_start = Instant::now();
    if let Ok(lines) = read_lines("./words_alpha.txt") {
        for result in lines {
            if let Ok(word) = result {
                let distance = get_distance(&distance_map, &word);
                if distance > biggest_word_distance {
                    biggest_word_distance = distance;
                    biggest_word = word;
                }
            }
        }
    }

    println!("Finished calculations in {:.2?}", biggest_word_start.elapsed());
    println!("Biggest Word: {} of length {}", biggest_word, biggest_word_distance);
    
}
