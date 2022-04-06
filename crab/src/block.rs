use std::fmt::{self, Debug, Formatter};
use super::*;

pub struct Block{
  pub index: u32,
  pub timestamp: u128,
  pub hash: Hash,
  pub prev_block_hash: Hash,
  pub nonce: u64,
  pub difficulty: u128,
}

impl Block{
  pub fn new(index: u32, timestamp: u128, prev_block_hash: Hash,  difficulty: u128) -> Self{
    Block{
      index,
      timestamp,
      hash: vec![0; 32],
      prev_block_hash,
      nonce: 0,
      difficulty
    }
  }
}

impl Debug for Block { // Block format when printed
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "Block[{}]: {} at: {} nonce: {}", &self.index, &hex::encode(&self.hash), &self.timestamp, &self.nonce)
  }
}

impl Hashable for Block {
  fn bytes (&self) -> Vec<u8> {
    let mut bytes = vec![];

    bytes.extend(&u32_bytes(&self.index));
    bytes.extend(&u128_bytes(&self.timestamp));
    bytes.extend(&self.prev_block_hash);
    bytes.extend(&u64_bytes(&self.nonce));
    //bytes.extend(self.transaction.iter().flat_map(|transaction| transaction.bytes()).collect::<Vec<u8>>());

    bytes.extend(&u128_bytes(&self.difficulty));

    bytes
  }
}