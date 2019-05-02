/* Kurze Einleitung,
in Rust gibt es kein NULL sondern sogenannte Options,
eine Option ist ein Enum der den Wert “Some” oder “None” annehmen kann
wenn eine Option Some ist dann hat sie auch einen Wert x der auf den Optionalen
Wert zeigt, None ist also äquivalent zu NULL.  Wenn man nun auf so eine Option
zugreifen will dann macht man das meistens mit dem “match” Operator der ähnlich
wie “swicht” funktioniert nur wesentlich vielseitiger ist.

Weiteres existiert in Rust das Konzept “Ownership” was so viel heißt wie jeder Wert
kann nur von einer Variable besessen werden oder anders gesagt man kann nicht wie in
C mit mehreren Pointern auf dasselbe Objekt zeigen. Das bedeutet, wenn ich einen Wert
per Funktion übergebe muss ich diesen auch wieder zurückgeben falls ich ihn nicht verwerfen
will oder ich verwende den “Borrow” Operator der durch “&” dargestellt wird.

Variablen in Rust sind Default nicht mutable, das bedeutet man kann sie nicht verändern,
wenn man das aber doch will muss man, wenn man die Variable vereinbart “mut” davor schreiben.

Variablen in Rust werden folgendermaßen vereinbart:
let i = 1.0;

i ist in diesem Fall ein f64(64bit float) der Typ wird automatisch vergeben,

Man kann den Typ aber auch vereinbaren und zwar so:

let i:f32 = 1.0;

Hier ist i ein f32, wenn ich ihn nun auch noch mutable machen will sieht das so aus:

let mut i:f32 = 1.0;
*/

use std::env;
use std::fs;

use std::string::String;

// Box ist das c Äquivalent zum Pointer
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    // new gibt ein Node Struct zurück und setzt den Wert v auf value und left/right auf None
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

    /* Ein Wert wird eingefügt, wenn der root-value kleiner null ist,
    gehe ich davon aus, dass der zu einzufügende Wert den momentanen
    Node gehört*/
    pub fn insert(&mut self, v: i32) {
        if self.value < 0 {
            self.value = v;
            return;
        }
        /* wenn der einzufügende Wert kleiner als der momentane Nodewert ist,
        gehen wir in den linken Zwieg schauen ob dieser Some ist wenn ja => rekursion,
        wenn nein wird links eine neue Node erstellt
        wenn der Wert größer ist passiert das selbe rechts*/
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

    /*print_bal gibt die balanceFaktoren der einzelnen Zweige aus
    und gibt ein tupel der momentanen höhe und einen boll mit dem
    avlViolation status zurück*/
    fn print_bal(&self) -> (i16, bool) {
        // ich inizialisiere zwei (i16,bool) Tupel
        let mut lh = (0, false);
        let mut rh = (0, false);

        // wenn rechts/links Some ist dann Rekursion
        match &self.right {
            Some(x) => rh = x.print_bal(),
            None => (),
        }
        match &self.left {
            Some(x) => lh = x.print_bal(),
            None => (),
        }

        //ich ziehe die linke höhe von der rechten ab um den Balancefaktor zu erhalten
        let bal = rh.0 - lh.0;

        print!("bal({}) = {}", self.value, bal);

        let mut avl;
        // wenn der bal Wert größer als 1 oder kleiner als -1 ist dann wird avl auf false gesetzt
        if bal > 1 || bal < -1 {
            println!(" (AVL violation!)");
            avl = false;
        } else {
            println!();
            avl = true;
        }
        // wenn einer der vorherigen bools true ist wird avl auch auf true gesetzt
        if lh.1 || rh.1 {
            avl = true;
        }

        //der größere Wert wird zurückgegeben und um eins erhöht
        if lh <= rh {
            return (1 + rh.0, avl);
        } else {
            return (1 + lh.0, avl);
        }
    }

    /*Wir gehen so lange left/right bis wir auf None stoßen dann haben wir
    den min/max Wert gefunden*/
    fn print_min(&self) {
        match &self.left {
            Some(x) => x.print_min(),
            None => print!("min: {}", self.value),
        }
    }
    fn print_max(&self) {
        match &self.right {
            Some(x) => x.print_max(),
            None => print!(", max: {}", self.value),
        }
    }

    /* put_vec schreibt alle Werte im Baum in einen Vector
    wenn Some dann wird der Vector in der Rekursion weitergeben*/
    fn put_vec(&self, v: &mut Vec<i32>) {
        v.push(self.value);
        match &self.left {
            Some(x) => x.put_vec(v),
            None => (),
        }
        match &self.right {
            Some(x) => x.put_vec(v),
            None => (),
        }
    }

    /* put_vec wird aufgerufen, dann werden alle Werte aufsummiert
    und durch die Länge dividiert*/
    fn print_avg(&self) {
        let mut v: Vec<i32> = Vec::new();
        self.put_vec(&mut v);

        let mut sum = 0;

        for i in &v {
            sum += i;
        }

        print!(", avg: {}", (sum as f32) / (v.len() as f32));
    }
    pub fn print_stats(&self) {
        if self.print_bal().1 {
            println!("AVL: no");
        } else {
            println!("AVL: yes");
        }
        self.print_min();
        self.print_max();
        self.print_avg();
    }
}

fn main() {
    let a = 1.0;
    let mut arg;

    //argument an der Stelle 1 wird in String arg geschrieben
    {
        let args: Vec<String> = env::args().collect();
        arg = args.get(1).unwrap().clone();
    }

    arg.insert_str(0, "./res/");

    let contents = fs::read_to_string(arg).expect("Something went wrong reading the file");

    let mut root = Node::new(-1);

    for s in contents.lines() {
        root.insert(s.parse::<i32>().unwrap());
    }

    root.print_stats();
}
