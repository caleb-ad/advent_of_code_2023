use std::io::Read;

const SCHEMATIC_DIM: (usize, usize) = (141, 140);
const SCHEMATIC_DIM_I32: (i32, i32) = (SCHEMATIC_DIM.0 as i32, SCHEMATIC_DIM.1 as i32);

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day3/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");


    let diagram: Vec<char> = input.chars().collect();
    let mut sum = 0;
    for idx in 0..diagram.len() {
        if diagram[idx] == '*' { sum += get_gear_ratio(&diagram, idx) }
    }
    println!("{:?}", sum);
}


fn get_adjacent(schematic: &Vec<char>, idx: usize) -> [Option<usize>; 8] {
    let offsets: [i32; 8] = [-1, -1 - SCHEMATIC_DIM_I32.0, 0-SCHEMATIC_DIM_I32.0, 1-SCHEMATIC_DIM_I32.0, 1, 1 + SCHEMATIC_DIM_I32.0, SCHEMATIC_DIM_I32.0, -1 + SCHEMATIC_DIM_I32.0];
    offsets.map(|offset| {
        let Ok(aidx) = TryInto::<usize>::try_into(TryInto::<i32>::try_into(idx).expect("") + offset) else {return None; };
        if aidx < schematic.len() {Some(aidx)}
        else {None}
    })
}

fn get_gear_ratio(schematic: &Vec<char>, idx: usize) -> u32 {
    let mut numeric_adjacent = get_adjacent(schematic, idx).map(|val| {if let Some(idx) = val {if schematic[idx].is_numeric() {return Some(idx)}} None});
    if numeric_adjacent[2].is_some() {
        numeric_adjacent[1] = None;
        numeric_adjacent[3] = None;
    }
    if numeric_adjacent[6].is_some() {
        numeric_adjacent[5] = None;
        numeric_adjacent[7] = None;
    }
    let mut numeric_only = numeric_adjacent.iter().flatten();
    match (numeric_only.next(), numeric_only.next(), numeric_only.next()) {
        (Some(idx1), Some(idx2), None) => {
            let gear1 = get_number_from_idx_unchecked(schematic, *idx1);
            let gear2 = get_number_from_idx_unchecked(schematic, *idx2);
            println!("{:?} * {:?}", gear1, gear2);
            gear1 * gear2
        }
        _ => 0,
    }
}

fn get_number_from_idx_unchecked(schematic: &Vec<char>, idx: usize) -> u32 {
    let end = idx + schematic.iter().skip(idx).position(|c| !c.is_numeric()).unwrap();
    let start = 1 + idx - schematic.iter().rev().skip(schematic.len() - 1 - idx).position(|c| !c.is_numeric()).unwrap();
    u32::from_str_radix(schematic[start..end].iter().collect::<String>().as_str(), 10).expect("")
}

fn is_vertically_adjacent_symbol(schematic: &Vec<char>, idx: usize) -> bool {
    if idx >= SCHEMATIC_DIM.0 && is_symbol(&schematic[idx - SCHEMATIC_DIM.0]){ return true }
    if idx < schematic.len() - SCHEMATIC_DIM.0 && is_symbol(&schematic[idx + SCHEMATIC_DIM.0]){ return true }
    false
}

fn check_part_no(schematic: &Vec<char>, idx: &mut usize) -> u32 {
    let mut part_no = false;
    let start = *idx;

    if *idx > 0 && is_symbol(&schematic[*idx - 1]) {part_no = true}
    if !part_no && *idx > 0 { part_no = is_vertically_adjacent_symbol(&schematic, *idx - 1) }

    while schematic[*idx].is_numeric() {
        if !part_no {part_no = is_vertically_adjacent_symbol(schematic, *idx)}
        *idx += 1;
    }

    if !part_no && *idx < schematic.len() && is_symbol(&schematic[*idx]) { part_no = true; }
    if !part_no && *idx < schematic.len() { part_no = is_vertically_adjacent_symbol(&schematic, *idx) }

    if part_no {
        println!("{:?}", &schematic[start..*idx]);
        u32::from_str_radix(schematic[start..*idx].iter().collect::<String>().as_str(), 10).expect("")
    }
    else { 0 }
}

fn is_symbol(c: &char) -> bool {
    c.is_ascii_punctuation() && *c != '.'
}

