#[derive(Debug)]
pub struct Block {
    pub tag: String,
}


impl Block {
    pub fn new(tag: &str) -> Block {
	Block {
	    tag: String::from(tag),
	}
    }
}
