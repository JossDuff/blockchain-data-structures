# blockchain-data-structures

Creating a merkle tree and block structure in Rust to test my understanding of basic blockchain structures and to learn Rust.

## Merkle tree

### Account structure

Holds the balance and address of the account both as Strings.

### Node structure

Most nodes have a left and a right node to take the hash of, but sometimes the leaves of the tree aren't an even number which results in a few uneven nodes.  So to be able to handle a node with only a left or only a right node, the node structure needs to be able to handle null values in the event there is no node.  Since Rust doesn't have null values I had to figure out how to use Rust's Option<> enums to hold node types.

Each node also holds the hash of the concatenation of the left node's hash and the right node's hash.

### Merkle Tree Structure

TODO

### Hash

Using [sha256 crate](https://crates.io/crates/sha256) for my hash function that accepts 2 Strings and returns a String.

## Block
Block header consists of:
 - hash of the previous block (0 for genesis block)
 - hash of the root of the merkle tree stored in the current block
 - Unix timestamp
 - difficulty target
 - nonce

After header comes the full list of accounts (looks like the original input file)

## Input

Example input file 'test.txt' is provided.  Input consists of a account address, a space, and an account balance.

## For me:

-[x] put project on github

-[ ] finish readme

-[ ] finish everything else
