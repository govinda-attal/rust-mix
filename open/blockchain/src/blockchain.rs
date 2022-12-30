use std::collections::HashSet;

use crate::block::check_difficulty;

use super::*;

#[derive(Debug)]
pub enum BlockValidationError {
    MismatchIndex,
    InvalidHash,
    MismatchedPreviousHash,
    TimestampNotInCronologicalOrder,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationError> {
        let i = self.blocks.len();
        if block.index != i as u32 {
            return Err(BlockValidationError::MismatchIndex);
        }
        if !check_difficulty(&block.hash(), block.difficulty) {
            return Err(BlockValidationError::InvalidHash);
        }
        if i == 0 {
            // genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationError::InvalidGenesisBlockFormat);
            }
        } else {
            let prv_block = &self.blocks[i - 1];
            if block.timestamp <= prv_block.timestamp {
                return Err(BlockValidationError::TimestampNotInCronologicalOrder);
            }
            if block.prev_block_hash != prv_block.hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            }
            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();
            let mut total_fee = 0 as u64;

            for tran in transactions {
                let input_hashses = tran.input_hashes();
                if !(&input_hashses - &self.unspent_outputs).is_empty()
                    || !(&input_hashses & &block_spent).is_empty()
                {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_value = tran.input_value();
                let output_value = tran.output_value();

                if output_value > input_value {
                    return Err(BlockValidationError::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashses);
                block_created.extend(tran.output_hashes());
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            } else {
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created)
        }

        self.blocks.push(block);

        Ok(())
    }
}
