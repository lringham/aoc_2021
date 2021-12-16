use super::helpers;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn day12() {
    let lines = helpers::read_file("inputs/day12.txt");

    // build cave_system
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

    // find paths
    let start = &cave_system[cave_index_map[&("start".to_string())]];

    let paths = find_all_paths(&start, &cave_system, false);
    println!("# part 1 - Ans: {}", paths.len());

    let paths = find_all_paths(&start, &cave_system, true);
    println!("# part 2 - Ans: {}", paths.len());
}

fn find_all_paths(
    cave: &Cave,
    cave_system: &Vec<Cave>,
    allow_multi_visit: bool,
) -> Vec<Vec<String>> {
    let mut paths: Vec<Vec<String>> = Vec::new();
    let mut path: Vec<String> = Vec::new();
    let mut double_visit_name = if allow_multi_visit {
        "".to_string()
    } else {
        "disabled".to_string()
    };
    find_path_helper(
        cave,
        &mut path,
        cave_system,
        &mut paths,
        &mut double_visit_name,
    );
    paths
}

// part 2
fn find_path_helper(
    cave: &Cave,
    path: &mut Vec<String>,
    cave_system: &Vec<Cave>,
    previous_paths: &mut Vec<Vec<String>>,
    double_visit_name: &mut String,
) {
    path.push(cave.name.clone());

    if cave.name.eq("end") {
        previous_paths.push(path.clone());
    } else {
        for adj_cave_idx in &cave.adjacent_caves {
            let adj_cave = &cave_system[*adj_cave_idx];
            let mut can_visit = !adj_cave.is_small || !path.contains(&adj_cave.name);

            if !can_visit {
                if (*double_visit_name).eq("")
                    && !adj_cave.name.eq("start")
                    && !adj_cave.name.eq("end")
                {
                    *double_visit_name = adj_cave.name.clone();
                    can_visit = true;
                }
            }

            if can_visit {
                find_path_helper(
                    &adj_cave,
                    path,
                    cave_system,
                    previous_paths,
                    double_visit_name,
                );
                path.pop();
                if (*double_visit_name).eq(&adj_cave.name) {
                    *double_visit_name = "".to_string();
                }
            }
        }
    }
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
