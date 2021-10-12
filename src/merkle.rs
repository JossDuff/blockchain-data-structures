use sha256::digest;

//fn main() {

    //read accounts from file into accounts object array
    //turn each account into a node object and store in node object array
    //create merkletree object using node object array
    //print tree
//}

//hash function taking two objects, turning them into strings, and returning the hash of the two
fn hash(a: String, b: String) -> String{
    let mut c = String::new();
    c = c + &a + &b;
    let to_hash = digest(c);
    return to_hash;
}


//account struct
    //constructor setting address and balance

    //hash function that returns the hash of the address and balance (a leaf)

struct Account{
    addr: String,
    bal: String,
}

//makes the hash function callable on an account struct
impl Account{
    fn hash(&self) -> String {
        return hash(self.addr.clone(), self.bal.clone());
    }
}

//function to create an account object
fn build_account(addr: String, bal: String) -> Account{
    Account {
        addr: addr,
        bal: bal,
    }
}



//node class
    //constructor that hashes takes the left and right branches of the tree and hashes them together
        //or hashes just the right or left side if it is uneven

    //function from_object does something

// yay type aliases!
type MaybeNode = Option<Box<Node>>;

struct Node{
    leftNode: MaybeNode,
    rightNode: MaybeNode,
    hash: String,
}


fn build_node(leftNode: MaybeNode, rightNode: MaybeNode) -> Node{
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

//function takes an array (vector?) of account objects and returns an array (vector?) of nodes
fn accts_to_nodes(accts: Vec<Account>){
    let mut nodes = Vec::new();
    let mut node;
    for x in &accts {
        node = build_node(None, None);
        node.hash = accts[x].hash(); //this will probably break everything
        nodes.push()
    }
}



//merkletree class
    //constructor initializing root

    //build function that adds each layer of the tree.  argument nodes object array
        //divides each layer of nodes by 2 by hashing neighboring nodes until there is only one node left

    //print function that prints the merkle tree

struct merkleTree{
    root: String,
}

//takes parameter of array of nodes, which are initially leaves
fn build_tree(){

}
