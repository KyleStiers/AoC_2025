type Vertex = u32;
type Edge = (u32,u32);
pub struct Node {
    id: u32,
    children: Vec<Node>
}
type Tree = Vec<Node>;

// depth-first search of trees
pub fn dfs(node: &Node) {
    println!("Visiting node with id: {}", node.id);
    for child in &node.children {
        println!("Child: {}", child.id);
        dfs(child);
    }
}

// demo graph creation and test
pub fn create_graph() -> Node {
    let mut root = Node { id: 0, children: Vec::new() };
    root
}