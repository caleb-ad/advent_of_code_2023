use std::io::Read;

const SCHEMATIC_DIM: (usize, usize) = (141, 140);

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day3/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");


    let diagram: Vec<char> = input.chars().collect();
    let mut sum = 0;
    let mut idx = 0;
    while idx < diagram.len() {
        if diagram[idx].is_numeric() { sum += check_part_no(&diagram, &mut idx) }
        idx += 1;
    }
    println!("{:?}", sum);
}

fn is_symbol(c: &char) -> bool {
    c.is_ascii_punctuation() && *c != '.'
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

