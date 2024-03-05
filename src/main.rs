mod blocks;
use blocks::block::{Block};
use blocks::primitives::{Primitive};

fn main() {

    let b = Block::new("Nice tag!");
    println!("{}", b.tag);


    let p = Primitive::new(2);
    println!("{}", p.n_qubits);

    
    println!("Hallo, Rust here!")
}
