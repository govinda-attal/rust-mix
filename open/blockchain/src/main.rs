use blockchain::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0; 32], 0, "genesis".to_owned());

    let hash = block.hash();
    block.hash = hash;

    println!("{:?}", &block);
}
