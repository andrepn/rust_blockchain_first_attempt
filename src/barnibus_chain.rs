///Andrew Noel's first custom blockchain Barnibus
///Almost certainly not functional
///But possibly

///“But I don’t want to go among mad people,” Alice remarked.
///“Oh, you can’t help that,” said the Cat: “we’re all mad here. I’m mad. You’re mad.”
///“How do you know I’m mad?” said Alice.
///“You must be,” said the Cat, “or you wouldn’t have come here.”
// TASKS:

// Describe Block struct
// Different Tx enums (Bonds, Tokes)
// Calculate hash
// Info
use std::fmt;

pub struct Block {
    txs: Vec<Transaction>,
    hash: u64,
    data: String,
    nonce: u64,
}

pub trait Transact {
    fn trade(self, other) -> Transaction {}
    fn send(self, Entity) -> Transaction {}
}

// Describe Bond Struct
#[derive(Debug)]
pub struct Bond {
    face_value: u32,
    interest_rate: u8,
    num_blocks_between_payment: u8,
    ID: u64,
    history_of_trades: Vec<Transaction>,
    current_owner: Entity,
    issuer: Entity,
}

// Method for new bond NEED WORK
impl Bond {
    pub fn new() -> Bond {}
}

// Describe Token Struct
#[derive(Debug)]
pub struct Token {
    amount: u64,
    ID: u64,
    history_of_trades: Vec<Transaction>,
    current_owner: entity,
}

//Method for new Token NEED WORK
impl Token {
    pub fn new() -> Token {}
}

pub enum Goods<T: Transact> {}

pub struct Transaction {
    good_one: Goods,
    good_two: Result<Goods, None>,
    trade_description: String,
}

impl Transact for Goods {
    pub fn send(self, entity: Entity) -> Transaction {
        self.owner = entity;

        Transaction {
            good_one: self,
            good_two: None,
            trade_description: String::from("Sent {:?} to {:?}", good_one, self.owner),
        }
    }
    pub fn trade(self, other: Goods) -> Transaction {
        let x = self.owner;
        self.owner = other.owner;
        other.owner = x;

        Transaction {
            good_one: self,
            good_two: other,
            trade_description: String::from(
                "Traded {:?} from {} to {} in exchange for {:?}",
                good_one,
                other.owner,
                self.owner,
                good_two,
            ),
        }
    }
}

// Describe Organization Struct
pub struct Entity {
    name: String,
    permissions: Vec<Permissions>,
    owned_goods: Vec<Goods>,
}

//Describe methods for entity:
//New entity
//Offer bonds for sale
//View offered Bonds
//Bid on bonds
//View owned_goods
//Check permissions NEED WORK
impl Entity {
    pub fn new() -> Entity {}
    pub fn auction_bonds() -> Bonds {}
    pub fn list_offered_bonds() {}
    pub fn place_bid() {}
    pub fn list_owned_goods() {}
    pub fn view_permissions() {}
}

pub enum Permissions {}

// Describe Blockchain struct NEED WORK
pub struct Blockchain {
    chain: Vec<Block>,
}

// Impl method to add block
impl Blockchain {
    pub fn add_block() -> Blockchain {}
}

// Check consensus

// Determine block winner (use Qiskit?) some kind of Proof of

// Open channels to listen for network changes

// Method for
// Error handling
// Mining error
// Didn't win block
// Connectivity error
