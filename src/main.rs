use sha256::digest;

fn main() {

}

// yay type aliases!
type maybeNode = Option<Box<Node>>;

struct Node{
    leftNode: maybeNode,
    rightNode: maybeNode,
    hash: String,
}


fn build_node(leftNode: maybeNode, rightNode: maybeNode) -> Node{
    let mut newHash;
    let mut newLeft = leftNode;
    let mut newRight = rightNode;
    if !leftNode.is_none() && !rightNode.is_none() {
        newHash = hash(leftNode.unwrap().hash, rightNode.unwrap().hash);
    }
    else if rightNode.is_none(){
        newHash = leftNode.unwrap().hash;
        newRight = None;
    }
    else if leftNode.is_none(){
        newHash = rightNode.unwrap().hash;
        newLeft = None;
    }
    else {
        newHash = "".to_string();
    }

    Node {
        leftNode: newLeft,
        rightNode: newRight,
        hash: newHash,
    }

}
