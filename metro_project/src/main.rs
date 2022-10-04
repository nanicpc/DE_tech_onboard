use std::cell::{RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use std::error::Error;
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

impl Transit {
    fn get_str_value(&self) -> String{
        match &*self {
            Transit::Optcost(co) => format!("=> {} €", co),
            Transit::Time(t)  => format!("=> {} minutes", t),
            Transit::LineChange(s)  => format!("=> Line: {}", s),
        }   
    }
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

    fn get_node_id(&self, name: &str) -> usize{
        let id = self.nodes.iter().position(|station| station.borrow().name == name.to_string()).unwrap();
        id
    }

    fn get_edge(&self, from:usize, to: usize) ->Option<&Rc<RefCell<Edge>>>{
        let current= self.edges.iter().find(|&edge| (edge.borrow().from==from)&&(edge.borrow().to==to));
        current
    }

    fn find_all_paths(&self, from: &str, to: &str) -> Result<Vec<Vec<usize>>, Box<dyn Error>>{
        println!("looking for the shortest path from \"{}\" to \"{}\"", from, to);
        let from = self.get_node_id(from);
        let to= self.get_node_id(to);
        println!("nodes {} to {}", from, to);
        let mut all_paths:Vec<Vec<usize>>=Vec::new();
        let mut bfs_queue:VecDeque<(usize,Vec<usize>)> = VecDeque::new();
        bfs_queue.push_back((from,vec![from]));
        let mut already_seen:Vec<usize> = Vec::new();
        while !bfs_queue.is_empty(){
            let (mut current_vertex, mut path) = bfs_queue.pop_front().unwrap();
            already_seen.push(current_vertex);
            for neighbor in self.nodes[current_vertex].borrow().neighbors.iter(){
                let n_id = self.get_node_id(&neighbor.borrow().name);
                if !already_seen.contains(&n_id){
                    let mut new_path = path.clone();
                    new_path.extend(vec![n_id]);
                    if n_id == to{                        
                        all_paths.push(new_path);
                    } else{
                        bfs_queue.push_back((n_id, new_path));
                    }
                }
            }
        }
        return Ok(all_paths)
    }

    fn get_edges_from_path(&self, path: &Vec<usize>)->Vec<&Rc<RefCell<Edge>>>{
        let mut edge_vector:Vec<&Rc<RefCell<Edge>>> = Vec::new();
        for i in 1..path.len(){
            if let Some(x) = self.get_edge(path[i-1], path[i]){
                edge_vector.push(x);
            }
        }        
        edge_vector
    }

    fn print_paths(&self, all_paths:Vec<Vec<usize>>){
        for path in all_paths.iter(){
            let mut curr_str = Vec::new();
            let edge_vector = self.get_edges_from_path(path);
            let mut curr_transit=edge_vector[0].borrow().props.clone();
            for i in 1..edge_vector.len(){
                let edge2 = edge_vector[i].borrow().props.clone();
                let str1 = curr_transit.get_str_value();
                let s= curr_transit+edge2.clone();
                let str_new = s.get_str_value();
                if str_new == str1{
                    curr_str.push(str1);
                    curr_transit = edge2;
                } else{
                    curr_transit = s;
                }                
            }
            let jic = curr_transit.get_str_value();
            if jic != curr_str[curr_str.len()-1]{
                curr_str.push(jic);
            }
            println!("{}", curr_str.join(" "));
        }
    }
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
    let k = graph.add_node("k");
    let l = graph.add_node("l");
    let m = graph.add_node("m");

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
    graph.add_edge(j, Transit::LineChange(String::from("E")), k);
    graph.add_edge(j, Transit::LineChange(String::from("F")), l);
    graph.add_edge(k, Transit::Time(3), m);
    graph.add_edge(l, Transit::Time(5), m);
    let routes = graph.find_all_paths("a", "m").unwrap();
    graph.print_paths(routes);
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

#[test]
fn find_node_id_ok(){
    let mut graph = SummPath::default();
    let a = graph.add_node("a");
    let b = graph.add_node("b");
    assert_eq!(a, graph.get_node_id("a"));
    assert_eq!(b, graph.get_node_id("b"));
}

#[test]
#[should_panic]
fn find_node_id_ko(){
    let mut graph = SummPath::default();
    let _a = graph.add_node("a");
    graph.get_node_id("c");
}