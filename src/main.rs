
// Fragen ? https://doc.rust-lang.org/1.25.0/book/second-edition/index.html

use std::env;
use std::fs;

use std::string::String;

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    pub fn new(v: i32) -> Node {
        Node {
            value: v,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, l: Node) {
        self.left = Some(Box::new(l));
    }
    pub fn set_right(&mut self, r: Node) {
        self.right = Some(Box::new(r));
    }

    pub fn insert(&mut self, v: i32) {
        if self.value < 0 {
            self.value = v;
            return;
        }

        if self.value > v {
            match &mut self.left {
                Some(x) => x.insert(v),
                None => self.set_left(Node::new(v)),
            }
        } else {
            match &mut self.right {
                Some(x) => x.insert(v),
                None => self.set_right(Node::new(v)),
            }
        }
    }

    pub fn print_tree(&self) {
        println!("{}", self.value);
        match &self.left {
            Some(x) => x.print_child(String::from("l")),
            None => (),
        }
        match &self.right {
            Some(x) => x.print_child(String::from("r")),
            None => (),
        }
    }

    fn print_child(&self, d: String) {
        println!("{} {}", d, self.value);

        match &self.left {
            Some(x) => {
                let mut l = d.clone();
                l.push('l');
                x.print_child(l)
            }
            None => (),
        }
        match &self.right {
            Some(x) => {
                let mut l = d.clone();
                l.push('r');
                x.print_child(l)
            }
            None => (),
        }
    }

    pub fn print_stats(&self) {

        let mut b=0;
        self.print_bal(&mut b);
        self.print_minmax();

    }

    fn print_bal(&self,b:&mut i32) {

        match &self.left {
            Some(x) => {
                *b-=1;
                x.print_bal(b);
            },
            None => ()
        }
        match &self.right {
            Some(x) => {
                *b+=1;
                x.print_bal(b);
            },
            None => ()
        }

        println!("bal({}) = {}",self.value,b);

    }

    fn print_minmax(&self)
    {

    }
}

fn main() {
    let mut arg;

    {
        let args: Vec<String> = env::args().collect();
        arg = args.get(1).unwrap().clone();
    }

    arg.insert_str(0, "./src/");

    println!("{}", arg);
    let contents = fs::read_to_string(arg).expect("Something went wrong reading the file");

    let mut root = Node::new(-1);

    for s in contents.lines() {
        root.insert(s.parse::<i32>().unwrap());
    }
    root.print_tree();
    root.print_stats();
}
