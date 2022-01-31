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
  /// Creates a new block by adding a timestamp and mining a hash.
  /// 
  /// # Examples
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # let mut my_blockchain = Blockchain::new();
  /// # let genesis = my_blockchain.genesis();
  /// let new_block = Block::new(
  ///   my_blockchain.blocks[0].id + 1,
  ///   &my_blockchain.blocks[0].hash,
  ///   "new".to_string()
  /// );
  /// assert_eq!(new_block.id, 1);
  /// assert_eq!(new_block.data, "new");
  /// ```
  pub fn new(id: u64, previous_hash: &String, data: String) -> Self {
    let timestamp = Utc::now().timestamp();
    let (nonce, hash) = mine_hash(id, timestamp, previous_hash, &data);
    Self { id, hash, previous_hash: previous_hash.clone(), timestamp, data, nonce }
  }
}

#[test]
fn creates_a_new_block() {
  let block = Block::new(
    69,
    &"0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    "foo".to_string()
  );
  assert_eq!(block.id, 69);
  assert!(!block.hash.is_empty());
  assert_eq!(block.previous_hash, "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string());
  assert!(block.timestamp > 0);
  assert_eq!(block.data, "foo".to_string());
  assert!(block.nonce > 0);
}
