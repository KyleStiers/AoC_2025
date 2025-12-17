use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

// Create a tree with root node and the ability to insert children
#[derive(Debug)]
struct Node {
    id: u32,
    children: Vec<Node>,
}
impl Node {
    fn new(id: u32) -> Self {
        Node { id, children: Vec::new() }
    }

    fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }

}

fn dfs_count_leaves(node: &Node) -> usize {
    if node.children.is_empty() {
        return 1;
    }
    let mut leaf_count = 0;
    for child in &node.children {
        leaf_count += dfs_count_leaves(child);
    }
    leaf_count
}

fn get_char_at(char_matrix: Vec<String>, coordinate: (usize, usize)) -> char {
    let (row, col) = coordinate;
    char_matrix[row].chars().nth(col).unwrap()
}

fn main() -> io::Result<()> {
    // Specify the path to your text file
    let file_path = Path::new("test.txt");
    
    // Open the file
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Set up some variables to store things in
    let mut total :usize = 0;

    // Initialize matrix with same dims as input
    let mut lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let rows = lines.len() as usize;
    let cols = lines[0].len() as usize;

    let mut root = Node::new(0);
    let child1 = Node::new(1);
    let child2 = Node::new(2);
    root.add_child(child1);
    root.add_child(child2);
    let grandchild1 = Node::new(3);
    root.children[0].add_child(grandchild1);
    root.children[0].add_child(Node::new(4));
    root.children[1].add_child(Node::new(5));
    root.children[1].add_child(Node::new(6));

    println!("Tree structure: {:?}", root);
    println!("Counting leaves in the tree...");
    let leaf_count = dfs_count_leaves(&root);
    println!("Total leaf nodes: {}", leaf_count);

    Ok(())
}