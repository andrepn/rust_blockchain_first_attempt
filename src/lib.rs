use chrono::prelude::*;
extern crate crypto;
use crypto::digest::Digest;
use crypto::sha2::Sha256;
use num_bigint::BigUint;
use num_traits::One;
use std::error;
use std::fmt;

const DIFFICULTY: usize = 5;
const MAX_NONCE: u64 = 1_000_000;

//Define Blocks
const HASH_BYTE_SIZE: usize = 32;

pub type Sha256Hash = [u8; HASH_BYTE_SIZE];

#[derive(Debug)]
pub struct Block {
    // Headers.
    timestamp: timestamp(),
    prev_block_hash: Sha256Hash,
    nonce: u64,

    // Body.
    // Instead of transactions, blocks contain data.
    data: Vec<u8>,
}

impl Block {
    // Creates a new block.
    pub fn new(data: &str, prev_hash: Sha256Hash) -> Result<Self, MiningError> {
        let mut s = Self {
            prev_block_hash: prev_hash,
            nonce: 0,
            data: data.to_owned().into(),
        };

        s.try_hash()
            .ok_or(MiningError::Iteration)
            .and_then(|nonce| {
                s.nonce = nonce;

                Ok(s)
            })
    }

    fn try_hash(&self) -> Option<u64> {
        // The target is a number we compare the hash to. It is a 256bit binary with DIFFICULTY
        // leading zeroes.
        let target = BigUint::one() << (256 - 4 * DIFFICULTY);

        for nonce in 0..MAX_NONCE {
            let hash = calculate_hash(&block, nonce);
            let hash_int = BigUint::from_bytes_be(&hash);

            if hash_int < target {
                return Some(nonce);
            }
        }

        None
    }

    pub fn calculate_hash(block: &Block, nonce: u64) -> Sha256Hash {
        let mut headers = block.headers();
        headers.extend_from_slice(convert_u64_to_u8_array(nonce));

        let mut hasher = Sha256::new();
        hasher.input(&headers);
        let mut hash = Sha256Hash::default();

        hasher.result(&mut hash);

        hash
    }

    pub fn headers(&self) -> Vec<u8> {
        let mut vec = Vec::new();

        vec.extend(&util::convert_u64_to_u8_array(self.timestamp as u64));
        vec.extend_from_slice(&self.prev_block_hash);

        vec
    }

    // This transforms a u64 into a little endian array of u8
    pub fn convert_u64_to_u8_array(val: u64) -> [u8; 8] {
        return [
            val as u8,
            (val >> 8) as u8,
            (val >> 16) as u8,
            (val >> 24) as u8,
            (val >> 32) as u8,
            (val >> 40) as u8,
            (val >> 48) as u8,
            (val >> 56) as u8,
        ];
    }

    // Creates a genesis block, which is a block with no parent.
    //
    // The `prev_block_hash` field is set to all zeroes.
    pub fn genesis() -> Result<Self, MiningError> {
        Self::new("Genesis block", Sha256Hash::default())
    }
}

#[derive(Debug)]
pub enum MiningError {
    Iteration,
    NoParent,
}

impl fmt::Display for MiningError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MiningError::Iteration => write!(f, "could not mine block, hit iteration limit"),
            MiningError::NoParent => write!(f, "block has no parent"),
        }
    }
}

impl error::Error for MiningError {
    fn description(&self) -> &str {
        match *self {
            MiningError::Iteration => "could not mine block, hit iteration limit",
            MiningError::NoParent => "block has no parent",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    // Initializes a new blockchain with a genesis block.
    pub fn new() -> Result<Self, MiningError> {
        let blocks = Block::genesis()?;

        Ok(Self {
            blocks: vec![blocks],
        })
    }

    // Adds a newly-mined block to the chain.
    pub fn add_block(&mut self, data: &str) -> Result<(), MiningError> {
        let block: Block;
        {
            match self.blocks.last() {
                Some(prev) => {
                    block = Block::new(data, prev.hash())?;
                }
                // Adding a block to an empty blockchain is an error, a genesis block needs to be
                // created first.
                None => return Err(MiningError::NoParent),
            }
        }

        self.blocks.push(block);

        Ok(())
    }

    // A method that iterates over the blockchain's blocks and prints out information for each.
    pub fn traverse(&self) {
        for (i, block) in self.blocks.iter().enumerate() {
            println!("block: {}", i);
            println!("hash: {:?}", block.pretty_hash());
            println!("parent: {:?}", block.pretty_parent());
            println!("data: {:?}", block.pretty_data());
            println!()
        }
    }
}
