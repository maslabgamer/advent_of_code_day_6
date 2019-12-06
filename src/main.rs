use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let mut orbits: HashMap<String, Vec<(String, i32)>> = HashMap::new();
    let mut orbiters: HashMap<String, String> = HashMap::new();
    let orbital_pairs = read_input("input.txt");

    for pair in orbital_pairs {
        let being_orbited = &pair[0];
        let orbiting = &pair[1];

        match orbits.entry(being_orbited.to_string()) {
            Entry::Vacant(e) => { e.insert(vec![(orbiting.to_string(), 0)]); }
            Entry::Occupied(mut e) => { e.get_mut().push((orbiting.to_string(), 0)); }
        }
        match orbiters.entry(orbiting.to_string()) {
            Entry::Vacant(e) => { e.insert(being_orbited.to_string()); }
            Entry::Occupied(e) => { println!("Found entry {:?} already!", e); }
        }
    }

    let total_orbits = get_total_orbits(&mut orbits);
    println!("Total orbits is {}.", total_orbits);

    let required_transfers = get_transfers(&orbiters);
    println!("Required transfers is {}.", required_transfers);

    let end = Instant::now();
    println!("Runtime was {:?}.", end.duration_since(start));
}

fn get_transfers(orbiters: &HashMap<String, String>) -> usize {
    let mut you_path = get_com_path(&orbiters, "YOU");
    let mut san_path = get_com_path(&orbiters, "SAN");
    loop {
        let you_current = you_path.last();
        let san_current = san_path.last();
        if let (Some(you), Some(san)) = (you_current, san_current) {
            if you == san {
                you_path.pop();
                san_path.pop();
            } else {
                break;
            }
        }
    }
    you_path.len() + san_path.len()
}

fn get_com_path(orbiters: &HashMap<String, String>, current_key: &str) -> Vec<String> {
    let mut current_key = current_key;
    let mut full_path: Vec<String> = Vec::new();
    while let Some(v) = orbiters.get(current_key) {
        current_key = v;
        full_path.push(current_key.to_string());
    }
    full_path
}

fn get_total_orbits(orbits: &mut HashMap<String, Vec<(String, i32)>>) -> i32 {
    let mut orbiter_stack: Vec<(String, i32)> = vec![("COM".to_string(), 0)];
    let mut total_orbits = 0;
    while let Some(mut check_orbiter) = orbiter_stack.pop() {
        total_orbits += check_orbiter.1;
        if let Some(orbiters) = orbits.get_mut(&check_orbiter.0) {
            check_orbiter.1 += 1;
            let mut orbiters: Vec<(String, i32)> = orbiters.iter().map(|x| (x.0.clone(), x.1 + check_orbiter.1)).collect();
            orbiter_stack.append(orbiters.as_mut());
        }
    }
    total_orbits
}

fn read_input(filename: &str) -> Vec<Vec<String>> {
    let file = File::open(filename).expect("File not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|x| x.expect("Could not parse line").split(")").map(|s| s.to_string()).collect())
        .collect()
}
