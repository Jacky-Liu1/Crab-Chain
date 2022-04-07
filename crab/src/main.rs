use blockchainlib::*;

fn main() {
    let difficulty = 0x0000afffffffffffffffffffffffffff;
    let mut block = Block::new(0, now(), vec![0;32], difficulty);

    println!("{:?}", &block);

    block.mine();

    println!("Genesis block {:?}", &block);

    let mut last_hash = block.hash.clone();

    let mut blockchain = Blockchain { blocks: vec![block], };
   
    for i in 1..=10 {
        let mut block = Block::new(i, now(), last_hash, difficulty);


        last_hash = block.hash.clone();
        block.mine();
        println!("Mined block {:?}", &block);
        blockchain.blocks.push(block);


    }


}
