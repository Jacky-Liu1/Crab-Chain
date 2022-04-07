use blockchainlib::*;

fn main() {
    let mut block = Block::new(0, now(), vec![0;32], 0x00000fffffffffffffffffffffffffff);

    println!("{:?}", &block);

    let h = block.hash();

    println!("{:?}", &block);

    block.mine();

    println!("{:?}", &block);

    block.hash = h;
}
