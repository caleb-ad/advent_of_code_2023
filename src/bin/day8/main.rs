use std::cell::RefCell;
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
        };
    }

    let mut current = nodes.get("AAA").unwrap().clone();
    let mut steps: usize = 0;
    while current.borrow().name != "ZZZ" {
        // if steps < 100 { println!("{:#?}", current.borrow()); }
        // println!("{:?}", steps);
        current = match directions.next() {
            Some(b'L') => current.borrow().L.as_ref().unwrap().clone(),
            Some(b'R') => current.borrow().R.as_ref().unwrap().clone(),
            _ => panic!("invalid direction")
        };
        steps += 1;
    }

    println!("steps: {:?}", steps);
}

fn get_or_insert<'a>(h: &mut HashMap<&'a str, Rc<RefCell<Node>>>, key: &'a str) -> Rc<RefCell<Node>> {
    if let Some(v) = h.get(key) { v.clone() }
    else {
        let tmp = Rc::new(RefCell::new(Node {name: String::from(key), L: None, R: None }));
        h.insert(key, tmp.clone());
        tmp
    }
}

