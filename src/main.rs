use std::fmt::Debug;
use std::hash::Hash;
use std::iter::Iterator;

pub trait Graph
    where Self: for<'graph> GraphPredecessors<'graph, Item=<Self as Graph>::Node>,
          Self: for<'graph> GraphSuccessors<'graph, Item=<Self as Graph>::Node>
{
    type Node: NodeIndex;

    fn num_nodes(&self) -> usize;
    fn start_node(&self) -> Self::Node;
    fn predecessors<'graph>(&'graph self, node: Self::Node)
                            -> <Self as GraphPredecessors<'graph>>::Iter;
    fn successors<'graph>(&'graph self, node: Self::Node)
                            -> <Self as GraphSuccessors<'graph>>::Iter;
}

pub trait GraphPredecessors<'graph> {
    type Item;
    type Iter: Iterator<Item=Self::Item>;
}

pub trait GraphSuccessors<'graph> {
    type Item;
    type Iter: Iterator<Item=Self::Item>;
}

pub trait NodeIndex: Copy + Debug + Eq + Ord + Hash + Into<usize> + From<usize> {
    fn as_usize(self) -> usize {
        self.into()
    }
}

#[derive(Debug)]
struct SuperMir {
    nodes: Vec<usize>,
    edges: Vec<(usize, usize)>,
}

struct SuccIter<'a> {
    idx: usize,
    target: usize,
    data: &'a Vec<(usize, usize)>,
}

struct PredIter<'a> {
    idx: usize,
    target: usize,
    data: &'a Vec<(usize,usize)>,
}

impl<'a> Iterator for SuccIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            if self.idx == self.data.len() {
                return None;
            }
            if self.data[self.idx].0 == self.target {
                let rv = Some(self.data[self.idx].1);
                self.idx += 1;
                return rv;
            } else {
                self.idx += 1;
            }
        }
    }
}
impl<'a> Iterator for PredIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        loop {
            if self.idx == self.data.len() {
                return None;
            }
            if self.data[self.idx].1 == self.target {
                let rv = Some(self.data[self.idx].0);
                self.idx += 1;
                return rv;
            } else {
                self.idx += 1;
            }
        }
    }
}

impl Graph for SuperMir {
    type Node = usize;

    fn num_nodes(&self) -> usize
    {
        self.nodes.len()
    }
    fn start_node(&self) -> Self::Node
    {
        if self.nodes.len() > 0 {
            self.nodes[0]
        } else {
            panic!("no nodes!");
        }
    }
    fn predecessors<'graph>(&'graph self, node: usize)
                            -> <Self as GraphPredecessors<'graph>>::Iter
    {
        //PredIter { idx: 0, data: &self.edges, target: node }
        self.edges.into_iter().filter(|&(from, to)| {to == node })
        .map(|(from, to)| { from })
    }
    fn successors<'graph>(&'graph self, node: usize)
                            -> <Self as GraphSuccessors<'graph>>::Iter
    {
        //SuccIter { idx: 0, data: &self.edges, target: node }
        self.edges.into_iter().filter(|&(from, to)| {from == node })
        .map(|(_, to)| { to })
    }
}

impl SuperMir {
    fn add_edge(&mut self, from: usize, to: usize) {
        self.edges.push((from, to));
    }
    fn add_node(&mut self, node: usize) {
        self.nodes.push(node);
    }
}

impl<'g> GraphPredecessors<'g> for SuperMir {
    type Item = usize;
    //type Iter = PredIter<'g>;
    //type Iter = std::vec<(usize,usize)>::Iterator;
    //type Iter = std::iter::Iterator;
    type Iter = std::iter::Iterator<vec<(usize,usize)>>;
}

impl<'g>  GraphSuccessors<'g> for SuperMir {
    type Item = usize;
    //type Iter = SuccIter<'g>;
    //type Iter = std::vec<(usize,usize)>::Iterator;
    type Iter = std::iter::Iterator<vec<(usize,usize)>>;
}

impl NodeIndex for usize {
    fn as_usize(self) -> usize {
        self
    }
}

fn main() {
    println!("hello world!");
    let mut g = SuperMir { nodes: vec![], edges: vec![] };
    g.add_node(1);
    g.add_node(2);
    g.add_node(3);
    g.add_node(4);
    g.add_edge(1,3);
    g.add_edge(2,3);
    g.add_edge(1,4);
    println!("1's succs: ");
    for p in g.successors(1) {
        println!("{}", p);
    }
    println!("3's preds: ");
    for p in g.predecessors(3) {
        println!("{}", p);
    }
}
