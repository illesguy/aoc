use std::fmt;
use std::str::Chars;
use std::collections::HashMap;
use commons::file_loading;


#[derive(Clone)]
struct Tree {
    root: usize,
    nodes: HashMap<usize, Node>
}

#[derive(Clone, Debug)]
struct Node {
    parent: Option<usize>,
    ntype: NodeType
}

#[derive(Clone, Debug)]
enum NodeType {
    Leaf(u32),
    Inner(usize, usize)
}


impl Node {
    fn add(&mut self, value: u32) {
        match self.ntype {
            NodeType::Leaf(v) => self.ntype = NodeType::Leaf(v + value),
            _ => ()
        }
    }
}


impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn to_string(node_index: usize, nodes: &HashMap<usize, Node>) -> String {
            match nodes[&node_index].ntype {
                NodeType::Leaf(v) => format!("{}", v),
                NodeType::Inner(left, right) => format!("[{},{}]", to_string(left, nodes), to_string(right, nodes))
            }
        }
        write!(f, "{}", to_string(self.root, &self.nodes))
    }
}


impl Tree {
    fn from_string(numbers_str: &str) -> Self {
        fn build_nodes(number_chars: &mut Chars, nodes: &mut HashMap<usize, Node>) -> usize {
            let c1 = number_chars.next().unwrap();
            let p1 = if c1 == '[' {
                build_nodes(number_chars, nodes)
            } else {
                let leaf = Node {
                    parent: None,
                    ntype: NodeType::Leaf(c1.to_digit(10).unwrap())
                };
                let leaf_index = nodes.len();
                nodes.insert(leaf_index, leaf);
                leaf_index
            };
            
            number_chars.next();
    
            if let Some(c2) = number_chars.next() {
                let p2 = if c2 == '[' {
                    build_nodes(number_chars, nodes)
                } else {
                    let leaf = Node {
                        parent: None,
                        ntype: NodeType::Leaf(c2.to_digit(10).unwrap())
                    };
                    let leaf_index = nodes.len();
                    nodes.insert(leaf_index, leaf);
                    leaf_index
                };
                
                number_chars.next();
                let node_index = nodes.len();
                nodes.get_mut(&p1).unwrap().parent = Some(node_index);
                nodes.get_mut(&p2).unwrap().parent = Some(node_index);
                let node = Node {
                    parent: None,
                    ntype: NodeType::Inner(p1, p2)
                };
                nodes.insert(node_index, node);
                node_index
            } else {
                p1
            }
        }

        let mut nodes = HashMap::new();
        let root = build_nodes(&mut numbers_str.chars(), &mut nodes);
        Tree {root, nodes}
    }


    fn calculate_magnitude(&self) -> u32 {
        fn magnitude(node_index: usize, nodes: &HashMap<usize, Node>) -> u32 {
            match nodes[&node_index].ntype {
                NodeType::Leaf(v) => v,
                NodeType::Inner(left, right) => 3 * magnitude(left, nodes) + 2 * magnitude(right, nodes)
            }
        }
        magnitude(self.root, &self.nodes)
    }

    fn add(self, other: Tree) -> Tree {
        let mut new_nodes = self.nodes.clone();
        let highest_key_so_far = new_nodes.keys().max().unwrap() + 1;
        
        for (index, node) in other.nodes.into_iter() {
            match (node.parent, node.ntype) {
                (Some(parent), NodeType::Inner(left, right)) => {
                    new_nodes.insert(index + highest_key_so_far, Node {
                        parent: Some(parent + highest_key_so_far),
                        ntype: NodeType::Inner(left + highest_key_so_far, right + highest_key_so_far)
                    });
                },
                (Some(parent), NodeType::Leaf(value)) => {
                    new_nodes.insert(index + highest_key_so_far, Node {
                        parent: Some(parent + highest_key_so_far),
                        ntype: NodeType::Leaf(value)
                    });
                },
                (None, NodeType::Inner(left, right)) => {
                    new_nodes.insert(index + highest_key_so_far, Node {
                        parent: None,
                        ntype: NodeType::Inner(left + highest_key_so_far, right + highest_key_so_far)
                    });
                },
                (None, NodeType::Leaf(value)) => {
                    new_nodes.insert(index + highest_key_so_far, Node {
                        parent: None,
                        ntype: NodeType::Leaf(value)
                    });
                },
            }
        }

        let new_root = Node {
            parent: None,
            ntype: NodeType::Inner(self.root, other.root + highest_key_so_far)
        };

        let new_root_index = new_nodes.keys().max().unwrap() + 1;
        new_nodes.get_mut(&self.root).unwrap().parent = Some(new_root_index);
        new_nodes.get_mut(&(other.root + highest_key_so_far)).unwrap().parent = Some(new_root_index);
        new_nodes.insert(new_root_index, new_root);

        let mut result = Tree {
            root: new_root_index,
            nodes: new_nodes
        };
        result.reduce();
        result
    }

    fn reduce(&mut self) {
        loop {
            loop {
                match self.find_explode_candidate() {
                    Some(candidate) => self.explode(candidate),
                    None => break
                }
            }

            match self.find_split_candidate() {
                Some(candidate) => self.split(candidate),
                None => break
            }
        }
    }
    
    fn explode(&mut self, candidate: usize) {
        if let Some(node) = self.nodes.get(&candidate) {
            match node.ntype {
                NodeType::Inner(left, right) => {
                    let left_node = self.nodes.get(&left).unwrap();
                    let right_node = self.nodes.get(&right).unwrap();
                    match (left_node.ntype.clone(), right_node.ntype.clone()) {
                        (NodeType::Leaf(lv), NodeType::Leaf(rv)) => {
                            if let Some(left_n) = self.find_left_neighbour(candidate).and_then(|n| self.nodes.get_mut(&n)) {
                                left_n.add(lv);
                            }
                            if let Some(right_n) = self.find_right_neighbour(candidate).and_then(|n| self.nodes.get_mut(&n)) {
                                right_n.add(rv)
                            }
                        },
                        _ => ()
                    }
                    self.nodes.remove(&left);
                    self.nodes.remove(&right);
                },
                _ => ()
            }
            
            self.nodes.get_mut(&candidate).unwrap().ntype = NodeType::Leaf(0);
        }
    }
    
    fn find_left_neighbour(&self, start: usize) -> Option<usize> {
        fn find_first_left(current: usize, nodes: &HashMap<usize, Node>) -> Option<usize> {
            let current_parent = nodes.get(&current).and_then(|n| n.parent).and_then(|n| nodes.get(&n));
            match current_parent {
                Some(Node { parent: _, ntype: NodeType::Inner(left, right)}) => {
                    if *right == current {
                        Some(*left)
                    } else {
                        find_first_left(nodes.get(&current).and_then(|n| n.parent).unwrap(), nodes)
                    }
                }
                _ => None
            }
        }
        
        fn find_rightmost_child(current: usize, nodes: &HashMap<usize, Node>) -> usize {
            match nodes.get(&current) {
                Some(Node{ parent: _, ntype: NodeType::Inner(_, r)}) => find_rightmost_child(*r, nodes),
                _ => current
            }
        }
        
        let first_left = find_first_left(start, &self.nodes);
        first_left.map(|n| find_rightmost_child(n, &self.nodes))
    }
    
    fn find_right_neighbour(&self, start: usize) -> Option<usize> {
        fn find_first_right(current: usize, nodes: &HashMap<usize, Node>) -> Option<usize> {
            let current_parent = nodes.get(&current).and_then(|n| n.parent).and_then(|n| nodes.get(&n));
            match current_parent {
                Some(Node { parent: _, ntype: NodeType::Inner(left, right)}) => {
                    if *left == current {
                        Some(*right)
                    } else {
                        find_first_right(nodes.get(&current).and_then(|n| n.parent).unwrap(), nodes)
                    }
                }
                _ => None
            }
        }

        fn find_leftmost_child(current: usize, nodes: &HashMap<usize, Node>) -> usize {
            match nodes.get(&current) {
                Some(Node{ parent: _, ntype: NodeType::Inner(l, _)}) => find_leftmost_child(*l, nodes),
                _ => current
            }
        }
        
        let first_right = find_first_right(start, &self.nodes);
        first_right.map(|n| find_leftmost_child(n, &self.nodes))
    }

    fn split(&mut self, candidate: usize) {
        if let Some(node) = self.nodes.get(&candidate) {
            match node.ntype {
                NodeType::Leaf(value) => {
                    let left_child = Node {
                        parent: Some(candidate),
                        ntype: NodeType::Leaf(value / 2)
                    };
                    let right_child = Node {
                        parent: Some(candidate),
                        ntype: NodeType::Leaf((value + 1) / 2)
                    };
                    
                    let highest_current_index = self.nodes.keys().max().unwrap();
                    let left_index = highest_current_index + 1;
                    let right_index = highest_current_index + 2;
                    self.nodes.insert(left_index, left_child);
                    self.nodes.insert(right_index, right_child);
                    self.nodes.get_mut(&candidate).unwrap().ntype = NodeType::Inner(left_index, right_index);
                },
                _ => ()
            }
        }
    }

    fn find_explode_candidate(&self) -> Option<usize> {
        fn candidate(current: usize, nodes: &HashMap<usize, Node>, depth: u32) -> Option<usize> {
            match (nodes.get(&current).map(|n| n.ntype.clone()), depth >= 4) {
                (Some(NodeType::Inner(_, _)), true) => Some(current),
                (Some(NodeType::Inner(left, right)), false) => {
                    match candidate(left, nodes, depth + 1) {
                        Some(c) => Some(c),
                        None => candidate(right, nodes, depth + 1)
                    }
                }
                _ => None
            }
        }
        candidate(self.root, &self.nodes, 0)
    }

    fn find_split_candidate(&self) -> Option<usize> {
        fn candidate(current: usize, nodes: &HashMap<usize, Node>) -> Option<usize> {
            match nodes.get(&current).map(|n| n.ntype.clone()) {
                Some(NodeType::Inner(left, right)) => {
                    match candidate(left, nodes) {
                        Some(c) => Some(c),
                        None => candidate(right, nodes)
                    }
                }
                Some(NodeType::Leaf(v)) => {
                    if v >= 10 {
                        Some(current)
                    } else {
                        None
                    }
                },
                None => None
            }
        }
        candidate(self.root, &self.nodes)
    }
}


fn assert_sfn_parsing(input_line: &str) {
    assert_eq!(input_line, format!("{}", Tree::from_string(input_line)));
}


fn assert_magnitude_calculation() {
    assert_eq!(143, Tree::from_string("[[1,2],[[3,4],5]]").calculate_magnitude());
    assert_eq!(1384, Tree::from_string("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").calculate_magnitude());
}


fn assert_child_parent_relationships(tree: &Tree) {
    for (index, node) in tree.nodes.iter() {
        match node.ntype {
            NodeType::Inner(left, right) => {
                assert_eq!(Some(*index), tree.nodes[&left].parent);
                assert_eq!(Some(*index), tree.nodes[&right].parent);
            },
            _ => ()
        }
    }
}


pub fn main() {
    let content = file_loading::read_file_to_string(2021, 18).unwrap();
    content.split("\n").for_each(assert_sfn_parsing);
    assert_magnitude_calculation();
    
    let trees = content.split("\n").map(Tree::from_string).collect::<Vec<Tree>>();

    let tree_sum = trees.clone().into_iter().reduce(|l, r| l.add(r)).unwrap();
    assert_child_parent_relationships(&tree_sum);
    
    let magnitude = tree_sum.calculate_magnitude();
    println!("{}", magnitude);
    
    let mut magnitudes: Vec<u32> = Vec::new();
    for i in 0..trees.len() {
        for j in 0..trees.len() {
            if i != j {
                magnitudes.push(trees[i].clone().add(trees[j].clone()).calculate_magnitude())
            }
        }
    }

    let max_magnitude = magnitudes.iter().max().unwrap();
    println!("{}", max_magnitude);
}
