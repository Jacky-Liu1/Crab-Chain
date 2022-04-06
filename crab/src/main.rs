use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0;32], 10);

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", h);

    block.hash = h;
}
