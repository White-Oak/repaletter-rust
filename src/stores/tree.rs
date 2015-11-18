struct OctoTree{
    root: Node;
}
struct Node{
    nodes: [Node; 8],
    elements: Vec<Bag>
}
