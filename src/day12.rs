use super::helpers;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day12() {
    let lines = helpers::read_file("inputs/day12.txt");

    let mut cave_names: HashSet<String> = HashSet::new();
    let mut connections: Vec<(String, String)> = Vec::new();
    for line in lines {
        let connection: Vec<&str> = line.split('-').collect::<Vec<&str>>();
        connections.push((connection[0].to_string(), connection[1].to_string()));
        cave_names.insert(connection[0].to_string());
        cave_names.insert(connection[1].to_string());
    }

    let mut cave_index_map: HashMap<String, usize> = HashMap::new();
    let mut cave_system: Vec<Cave> = Vec::new();
    for cave_name in cave_names {
        cave_index_map.insert(cave_name.clone(), cave_system.len());
        cave_system.push(Cave::new(cave_name));
    }

    for connection in &connections {
        cave_system[cave_index_map[&connection.0]]
            .adjacent_caves
            .push(cave_index_map[&connection.1]);
        cave_system[cave_index_map[&connection.1]]
            .adjacent_caves
            .push(cave_index_map[&connection.0]);
    }

    let start = &cave_system[cave_index_map[&("start".to_string())]];
    let paths = find_all_paths(&start, &cave_system);
    println!("# Ans {:?}", paths.len());
}

fn find_all_paths(
    start : &Cave, 
    cave_system: &Vec<Cave>,
) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    find_path(
        start,
        cave_system,
        &mut paths,
    );
    paths
}

fn find_path(
    cave: &Cave,
    cave_system: &Vec<Cave>,
    previous_paths: &mut Vec<Vec<String>>,
) {
    let mut path: Vec<String> = Vec::new();
    let mut can_double_visit_name = "no_cave".to_string();
    find_path_helper(
        cave,
        &mut path,
        cave_system,
        previous_paths,
        &mut can_double_visit_name,
    );
}

fn find_path_helper(
    cave: &Cave,
    path: &mut Vec<String>,
    cave_system: &Vec<Cave>,
    previous_paths: &mut Vec<Vec<String>>,
    can_double_visit_name : &mut String,
) -> bool {
    
    if cave.is_small && path.contains(&cave.name) {
        if !(*can_double_visit_name).eq("no_cave") || cave.name.eq("start") || cave.name.eq("end") {
            return false;
        } else {
            *can_double_visit_name = cave.name.clone();
        }
    }
    
    path.push(cave.name.clone());
    
    if cave.name.eq("end") {
        previous_paths.push(path.clone());
        return true;
    }

    for adj_cave in &cave.adjacent_caves {
        let next_cave = &cave_system[*adj_cave];
        if find_path_helper(
            &next_cave,
            path,
            cave_system,
            previous_paths,
            can_double_visit_name,
        )  {
            path.pop();
            if (*can_double_visit_name).eq(&next_cave.name) {
                *can_double_visit_name = "no_cave".to_string();
            }
        }
    }

    return true;
}

struct Cave {
    pub name: String,
    pub is_small: bool,
    pub adjacent_caves: Vec<usize>,
}

impl Cave {
    pub fn new(name: String) -> Self {
        let is_small = name.to_ascii_lowercase().eq(&name);
        Cave {
            name,
            is_small,
            adjacent_caves: Vec::new(),
        }
    }
}
