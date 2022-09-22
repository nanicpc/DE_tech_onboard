use std::cell::{RefCell};
use std::rc::Rc;
// use std::env::args;
use std::ops::{Add, Deref};
use std::fmt;
use std::mem;

struct Station {
    name: String,
    id: usize,
    neighbors: Vec<Rc<RefCell<Station>>>
}

impl Station {
    fn new(name: &str, id: usize) -> Station{
        Station {name: name.to_string(), id: id, neighbors: Vec::new()}
    }
}

impl fmt::Display for Station {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Station {} :{}", self.id, self.name)
    }

}

impl Deref for Station{
    type Target= usize;
    fn deref(&self) -> &Self::Target {        
        &self.id
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Transit {
   Optcost(f64),
   Time(isize),
   LineChange(String),
}

struct Edge {
    from: usize,
    to: usize,
    props: Transit
}

impl Edge{
    fn new(from:usize, props: Transit, to:usize) -> Rc<RefCell<Edge>> {
        Rc::new(RefCell::new(Edge {from, to, props}))
    }
}

impl fmt::Display for Transit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Transit::Optcost(co) => write!(f, "{}", co),
            Transit::Time(t) => write!(f, "{}", t),
            Transit::LineChange(s) => write!(f, "{}", s),
        }        
    }

}
impl Add for Transit {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if mem::discriminant(&self)==mem::discriminant(&rhs){
            match self {
                Transit::Optcost(co) => {
                    let mut new_val = co;
                    if let Transit::Optcost(sec) = rhs{
                        new_val += sec;
                    }
                    Transit::Optcost(new_val)},
                Transit::Time(t) => {
                    let mut new_val = t;
                    if let Transit::Time(sec) = rhs{
                        new_val += sec;
                    }
                    Transit::Time(new_val)},
                Transit::LineChange(s) => {
                    let mut tempstr = s.clone();
                    if let Transit::LineChange(sec) = rhs{
                        tempstr.push_str(" -> ");
                        tempstr.push_str(&sec);
                    }                    
                    Transit::LineChange(tempstr)
                },
            }    
        } else {
            self
        }
    }
}

#[derive(Default)]
struct SummPath {
    nodes: Vec<Rc<RefCell<Station>>>,
    edges: Vec<Rc<RefCell<Edge>>>
}

impl SummPath {
    fn add_node(&mut self, name: &str) -> usize {
        // under the hypothesis that stations cannot be removed
        let id = self.nodes.len();
        let node = Rc::new(RefCell::new(Station::new(name, id)));
        self.nodes.push(node.clone());
        id
    }

    fn add_edge(&mut self, from: usize, value: Transit, to: usize) {
        let edge = Edge::new(from, value, to);
        self.edges.push(edge.clone());
        self.nodes[from].borrow_mut().neighbors.push(self.nodes[to].clone());
        // to.neighbors.push(from);
    }

   /*  fn shortest_path(self, from: usize, to:usize) {
        let mut neigh_paths:Vec<Vec<usize>> = Vec::new();
        let mut neighs = Vec::new();
        neighs.push(from);
        fn find_neigh (curr_neigh: Vec<usize>, goal: usize, &neighs: Vec<usize>) -> Vec<usize>{
            let mut chaine = neighs.clone();
            for n in curr_neigh{
                chaine.push(n);
                if n != goal {
                    find_neigh(n, goal, &chaine);
                } else {
                    break;
                }
            }
            neighs 
        };
        println!("{:?}", find_neigh(from, to, neighs));
    } */
}

fn main() {
    let mut graph = SummPath::default();
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    let c = graph.add_node("c");
    let d = graph.add_node("d");
    let e = graph.add_node("e");
    let f = graph.add_node("f");
    let g = graph.add_node("g");
    let h = graph.add_node("h");
    let i = graph.add_node("i");
    let j = graph.add_node("j");

    graph.add_edge(a, Transit::LineChange(String::from("A")), b);
    graph.add_edge(a, Transit::LineChange(String::from("C")), c);
    graph.add_edge(b, Transit::Time(8), d);
    graph.add_edge(c, Transit::Time(6), f);
    graph.add_edge(c, Transit::LineChange(String::from("D")), g);
    graph.add_edge(d, Transit::Time(6), e);
    graph.add_edge(d, Transit::LineChange(String::from("B")), h);
    graph.add_edge(f, Transit::Time(5), i);
    graph.add_edge(g, Transit::Time(4), i);
    graph.add_edge(e, Transit::Time(3), j);
    graph.add_edge(h, Transit::Optcost(1.5), j);
}


#[test]
fn string_works(){
    let test1 = Transit::LineChange(String::from("A"));
    let test2 = Transit::LineChange(String::from("B"));
    let su = test1.clone()+test2;
    assert_eq!(su, Transit::LineChange(String::from("A -> B")));
}

#[test]
fn cost_works(){
    let test1 = Transit::Optcost(3.2);
    let test2 = Transit::Optcost(1.2);
    let su = test1+test2;
    assert_eq!(su, Transit::Optcost(4.4));
}

#[test]
fn time_works(){
    let test1 = Transit::Time(5);
    let test2 = Transit::Time(4);
    let su = test1+test2;
    assert_eq!(su, Transit::Time(9));
}

#[test]
fn mix_eq_first(){
    let test1 = Transit::LineChange(String::from("A"));
    let test2 = Transit::Time(5);
    let su = test1.clone()+test2;
    assert_eq!(su, test1);
}