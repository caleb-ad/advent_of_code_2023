use std::cell::RefCell;
use std::vec;
use std::{io::Read, rc::Rc};
use std::collections::HashMap;

struct Node {
    name: String,
    L: Option<Rc<RefCell<Node>>>,
    R: Option<Rc<RefCell<Node>>>
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {:?}, L: {:?}, R: {:?}",
            self.name,
            if let Some(l) = &self.L {l.borrow().name.clone()}
            else {String::from("None")},
            if let Some(r) = &self.R {r.borrow().name.clone()}
            else {String::from("None")})
    }
}

#[derive(Debug, Clone, Copy)]
struct Cycle {
    offset: usize,
    len: usize,
}

fn main() {
    let mut input = String::from("\n");
    let _ = std::fs::File::open("src/bin/day8/input.txt")
        .expect("failed to open file")
        .read_to_string(&mut input)
        .expect("failed to read file");

    let mut lines = input.lines();
    lines.next();
    let mut directions = lines.next().unwrap().bytes().cycle();
    let mut nodes: HashMap<&str, Rc<RefCell<Node>>> = HashMap::new();

    lines.next();

    let mut ghosts = vec![];
    for line in lines {
        let mut symbols = line
            .split(['=', '(', ')', ',', ' '])
            .filter(|s| s.len() > 0);

        let (node, left, right) = (symbols.next().unwrap(), symbols.next().unwrap(), symbols.next().unwrap());
        let left = get_or_insert(&mut nodes, left);
        let right = get_or_insert(&mut nodes, right);

        if let Some(n) = nodes.get(node) {
            n.borrow_mut().L = Some(left);
            n.borrow_mut().R = Some(right);
        } else {
            nodes.insert(node, Rc::new(RefCell::new(Node{name: String::from(node), L: Some(left), R: Some(right)})));
        }

        if node.chars().nth(2).unwrap() == 'A' { ghosts.push(nodes.get(node).unwrap().clone()) }
    }

    for g in ghosts {
        println!("{:?}", get_cycle(&mut directions.clone(), g));
    }
}

//iterator must start at beginning
//assume each start leads to exactly one Z
fn get_cycle(d: &mut impl Iterator<Item = u8>, mut s: Rc<RefCell<Node>>) -> Cycle {
    let offset = advance_to_z(d, &mut s);
    s = advance(d.next(), &s);
    let len = advance_to_z(d, &mut s) + 1;
    Cycle{offset, len}
}

fn advance_to_z(d: &mut impl Iterator<Item = u8>, s: &mut Rc<RefCell<Node>>) -> usize {
    let mut steps = 0;
    while s.borrow().name.chars().nth(2).unwrap() != 'Z' {
        *s = advance(d.next(), s);
        steps += 1;
    }
    steps
}

fn advance(d: Option<u8>, s: &Rc<RefCell<Node>>) -> Rc<RefCell<Node>> {
    match d {
        Some(b'L') => s.borrow().L.as_ref().unwrap().clone(),
        Some(b'R') => s.borrow().R.as_ref().unwrap().clone(),
        _ => panic!("invalid direction")
    }
}

fn finished(pos: &Vec<Rc<RefCell<Node>>>) -> bool {
    pos.iter().find(|a| a.borrow().name.chars().nth(2).unwrap() != 'Z').is_none()
}

fn get_or_insert<'a>(h: &mut HashMap<&'a str, Rc<RefCell<Node>>>, key: &'a str) -> Rc<RefCell<Node>> {
    if let Some(v) = h.get(key) { v.clone() }
    else {
        let tmp = Rc::new(RefCell::new(Node {name: String::from(key), L: None, R: None }));
        h.insert(key, tmp.clone());
        tmp
    }
}

