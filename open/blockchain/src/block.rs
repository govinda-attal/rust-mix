use crate::*;
use std::fmt::{self, Debug, Formatter};

use crate::hashable::Hashable;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub prev_block_hash: BlockHash,
    pub hash: BlockHash,
    pub nounce: u64,
    pub payload: String,
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: BlockHash,
        nounce: u64,
        payload: String,
    ) -> Self {
        Self {
            index,
            timestamp,
            prev_block_hash,
            hash: vec![0; 32],
            nounce,
            payload,
        }
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Block [{}]: {} at: {} with: {}",
            &self.index,
            &hex::encode(&self.hash),
            &self.timestamp,
            &self.payload
        )
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nounce));
        bytes.extend(self.payload.as_bytes());

        bytes
    }
}

impl Block {}
