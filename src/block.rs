use serde::{ Serialize, Deserialize };
use chrono::Utc;
use crate::helpers::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Block {
  pub id: u64,
  pub hash: String,
  pub previous_hash: String,
  pub timestamp: i64,
  pub data: String,
  pub nonce: u64,
}

impl Block {
  pub fn new(id: u64, previous_hash: String, data: String) -> Self {
    let timestamp = Utc::now().timestamp();
    let (nonce, hash) = mine_hash(id, timestamp, &previous_hash, &data);
    Self { id, hash, previous_hash, timestamp, data, nonce }
  }
}

#[test]
fn creates_a_new_block() {
  let block = Block::new(
    69,
    "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    "foo".to_string()
  );
}
