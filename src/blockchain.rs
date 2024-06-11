extern crate serde;
extern crate serde_json;
extern crate sha2;
extern crate time;

use self::sha2::{sha256, Digest2};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amout: f32,
}

#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactrion: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Blocks>,
    curr_transaction: u32,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Chain {
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain: Vec::new(),
            curr_transaction: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block();
        chain
    }
}

pub fn generate_new_block(&mut self) -> bool {
    let header = Blockheader {
        timestamp: time::now().to_timespec().sec,
        nonce: 0,
        pre_hash: self.last_hash(),
        merkle: String::new(),
        difficulty: self.difficulty,
    };

    let reward_trans = Transaction {
        sender: String::from("Root"),
        receiver: self.miner_addr.clone(),
        amout: self.reward,
    };

    let mut block = Block {
        header,
        count: 0,
        transactrion: vec![]
    };

    block.transactrion.push(reward_trans);
    block.transactrion.append(&mut self.curr_transaction);
    block.count = block.transactrion.len() as u32;
    block.header.merkle = self.merkle(&block.transactrion.clone);
    Chain::proof_of_work(&mut block.header);

    println!("{:#?}", &block);
    self.chain.push(block);
    true
}
