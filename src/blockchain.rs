use crate::block::Block;
use crate::helpers::*;
use crate::error::{ BlockchainError, BlockchainError::* };

#[derive(PartialEq, Debug)]
pub struct Blockchain<Block> {
  pub blocks: Vec<Block>
}

impl Blockchain<Block> {
  /// Creates a new, empty blockchain.
  /// 
  /// # Examples
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// assert_eq!(Blockchain::new(), Blockchain::<Block> { blocks: vec![] });
  /// ```
  pub fn new() -> Self {
    Self { blocks: vec![] }
  }

  /// Initializes the blockchain with a genesis block.
  /// 
  /// # Examples
  /// ```
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # let mut my_blockchain = Blockchain::new();
  /// assert_eq!(my_blockchain.genesis(), Ok(()));
  /// assert!(my_blockchain.blocks.len() == 1);
  /// ```
  /// 
  /// # Errors
  /// Returns [`BlockchainError::InvalidChainLength`] if the blockchain is not empty.
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # use simple_blockchain::error::BlockchainError;
  /// # let mut my_blockchain = Blockchain::new();
  /// # my_blockchain.blocks.push(Block::new(0, &"genesis".to_string(), "genesis!".to_string()));
  /// assert!(my_blockchain.blocks.len() > 0);
  /// assert_eq!(my_blockchain.genesis(), Err(BlockchainError::InvalidChainLength));
  /// ```
  pub fn genesis(&mut self)  -> Result<(), BlockchainError> {
    if self.blocks.len() > 0 { return Err(InvalidChainLength) };
    let genesis_block = Block::new(0, &"genesis".to_string(), "genesis!".to_string());
    self.blocks.push(genesis_block);
    Ok(())
  }

  fn is_block_valid(&self, block: &Block, previous_block: &Block) -> bool {
    let hash = calculate_hash(block.id, block.timestamp, &block.previous_hash, &block.data, block.nonce);
    if block.hash != hash {
      return false;
    } else if block.previous_hash != previous_block.hash {
      return false;
    } else if !binary_string_of(&block.hash).starts_with(PREFIX) {
      return false;
    } else if block.id != (previous_block.id + 1) {
      return false;
    }
    true
  }

  /// Adds a valid block to the chain.
  /// 
  /// # Examples
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// let mut my_blockchain = Blockchain::new();
  /// let genesis = my_blockchain.genesis();
  /// if genesis.is_ok() {
  ///   let next_block = Block::new(
  ///     my_blockchain.blocks[0].id + 1,
  ///     &my_blockchain.blocks[0].hash,
  ///     "next".to_string()
  ///   );
  ///   assert_eq!(my_blockchain.add_block(next_block), Ok(()));
  /// }
  /// ```
  /// 
  /// # Errors
  /// Returns [`BlockchainError`] if blockchain is empty or block is invalid.
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # use simple_blockchain::error::BlockchainError;
  /// let mut my_blockchain = Blockchain::new();
  /// let next_block = Block::new(1, &"hash".to_string(), "data".to_string());
  /// 
  /// assert_eq!(my_blockchain.add_block(next_block), Err(BlockchainError::InvalidChainLength));
  /// 
  /// let genesis = my_blockchain.genesis();
  /// if genesis.is_ok() {
  ///   let next_block = Block::new(
  ///     my_blockchain.blocks[0].id + 1,
  ///     &"not_the_previous_hash".to_string(),
  ///     "next".to_string()
  ///   );
  /// 
  ///   assert_eq!(my_blockchain.add_block(next_block), Err(BlockchainError::InvalidBlock));
  /// }
  /// ```
  pub fn add_block(&mut self, block: Block) -> Result<(), BlockchainError> {
    match &self.blocks.last() {
      Some(tail) => if self.is_block_valid(&block, tail) {
        self.blocks.push(block);
        Ok(())
      } else {
        Err(InvalidBlock)
      },
      None => Err(InvalidChainLength)
    }
  }

  /// Returns `true` if all blocks in the blockchain are valid.
  /// Returns `false` otherwise, including if no blocks beyond genesis have been added.
  /// 
  /// # Examples
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # let mut my_blockchain = Blockchain::new();
  /// let genesis = my_blockchain.genesis();
  /// if genesis.is_ok() {
  ///   assert_eq!(my_blockchain.is_chain_valid(), false);
  ///   let valid_block = Block::new(
  ///     my_blockchain.blocks[0].id + 1,
  ///     &my_blockchain.blocks[0].hash,
  ///     "next".to_string()
  ///   );
  ///   my_blockchain.add_block(valid_block);
  ///   assert_eq!(my_blockchain.is_chain_valid(), true);
  /// }
  /// ```
  pub fn is_chain_valid(&self) -> bool {
    if self.blocks.len() <= 1 { return false };

    let mut blocks = self.blocks.iter();
    blocks.next();
    let mut previous_blocks = self.blocks.iter();

    blocks.all(|block| self.is_block_valid(&block, &previous_blocks.next().unwrap()))
  }

  /// Chooses the longest chain between itself and a remote blockchain.
  /// 
  /// Examples
  /// ```
  /// # use simple_blockchain::block::Block;
  /// # use simple_blockchain::blockchain::Blockchain;
  /// # let mut local_chain = Blockchain::new();
  /// # local_chain.genesis();
  /// # local_chain.add_block(Block::new(
  /// #   local_chain.blocks[0].id + 1,
  /// #   &local_chain.blocks[0].hash,
  /// #   "first".to_string()
  /// # ));
  /// # let mut remote_chain = Blockchain::new();
  /// # remote_chain.genesis();
  /// # remote_chain.add_block(Block::new(
  /// #   remote_chain.blocks[0].id + 1,
  /// #   &remote_chain.blocks[0].hash,
  /// #   "first".to_string()
  /// # ));
  /// # remote_chain.add_block(Block::new(
  /// #   remote_chain.blocks[1].id + 1,
  /// #   &remote_chain.blocks[1].hash,
  /// #   "second".to_string()
  /// # ));
  /// assert!(local_chain.blocks.len() == 2);
  /// assert!(remote_chain.blocks.len() == 3);
  /// local_chain.choose_chain(&remote_chain);
  /// assert!(local_chain.blocks.len() == 3);
  pub fn choose_chain(&mut self, remote: &Blockchain<Block>) {
    let is_local_valid = self.is_chain_valid();
    let is_remote_valid = remote.is_chain_valid();

    if is_local_valid
    && is_remote_valid
    && remote.blocks.len() > self.blocks.len() {
      self.blocks = remote.blocks.clone();
    }
    
    if is_remote_valid
    && !is_local_valid {
      self.blocks = remote.blocks.clone();
    }
  }
}

#[test]
fn creates_a_new_app() {
  let expected = Blockchain::<Block> { blocks: vec![] };
  let result = Blockchain::new();
  assert_eq!(expected, result);
}

#[test]
fn creates_genesis_block() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let result = new_app.genesis();
  assert!(result.is_ok());
  assert!(new_app.blocks.len() == 1);
}

#[test]
fn cant_genesis_more_than_once() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let first_result = new_app.genesis();
  assert!(first_result.is_ok());
  let mut again = new_app;
  let second_result = again.genesis();
  assert_eq!(second_result, Err(InvalidChainLength));
}

#[test]
fn adds_a_valid_block() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let genesis_block = Block {
    id: 0,
    hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    previous_hash: "genesis".to_string(),
    timestamp: 1643223000,
    data: "genesis!".to_string(),
    nonce: 44475
  };
  new_app.blocks.push(genesis_block);
  let block = Block {
    id: 1,
    hash: "0000cc07887fb749c99974e8e93debb64e205086f6d0962ef17bf6f0bb295f3e".to_string(),
    previous_hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 236492,
  };
  let expected = block.clone();
  let result = new_app.add_block(block);
  assert!(result.is_ok());
  assert_eq!(&new_app.blocks[1], &expected);
}

#[test]
fn errs_when_adding_invalid_block() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let genesis_block = Block {
    id: 0,
    hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    previous_hash: "genesis".to_string(),
    timestamp: 1643223000,
    data: "genesis!".to_string(),
    nonce: 44475
  };
  new_app.blocks.push(genesis_block);
  let invalid_block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  let result = new_app.add_block(invalid_block);
  assert!(result.is_err());
}

#[test]
fn valid_when_prev_hash_match() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 24271,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_prev_hash_mismatch() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_when_prefix_match() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 24271,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_prefix_mismatch() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: String::from("ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_when_next_id() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 24271,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_not_next_id() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 2,
    hash: String::from("0000ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_not_a_hash() {
  let new_app = Blockchain::<Block> { blocks: vec![] };
  let block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: 1643223669,
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_chain_when_all_blocks_valid() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let genesis_block = Block {
    id: 0,
    hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    previous_hash: "genesis".to_string(),
    timestamp: 1643223000,
    data: "genesis!".to_string(),
    nonce: 44475
  };
  new_app.blocks.push(genesis_block);
  let first_block = Block {
    id: 1,
    hash: "0000cc07887fb749c99974e8e93debb64e205086f6d0962ef17bf6f0bb295f3e".to_string(),
    previous_hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 236492,
  };
  new_app.add_block(first_block);
  assert!(new_app.is_chain_valid());
}

#[test]
fn invalid_chain_when_invalid_block() {
  let mut new_app = Blockchain::<Block> { blocks: vec![] };
  let genesis_block = Block {
    id: 0,
    hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    previous_hash: "genesis".to_string(),
    timestamp: 1643223000,
    data: "genesis!".to_string(),
    nonce: 44475
  };
  new_app.blocks.push(genesis_block);
  let first_block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 2836,
  };
  new_app.blocks.push(first_block);
  assert!(!new_app.is_chain_valid());
}

#[test]
fn chooses_the_longest_valid_chain() {
  let mut app1 = Blockchain::<Block> { blocks: vec![] };
  let mut app2 = Blockchain::<Block> { blocks: vec![] };
  let app1_genesis_block = Block {
    id: 0,
    hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    previous_hash: "genesis".to_string(),
    timestamp: 1643223000,
    data: "genesis!".to_string(),
    nonce: 44475
  };
  let app2_genesis_block = app1_genesis_block.clone();
  app1.blocks.push(app1_genesis_block);
  app2.blocks.push(app2_genesis_block);
  let app1_block = Block {
    id: 1,
    hash: "0000cc07887fb749c99974e8e93debb64e205086f6d0962ef17bf6f0bb295f3e".to_string(),
    previous_hash: "0000dbeb9e573d5382c63fd9a222c3720a4341b06416348fc5bbc0d19380a248".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 236492,
  };
  let app2_first_block = app1_block.clone();
  let app2_second_block = Block {
    id: 2,
    hash: "0000602c49108087d9878af09bb17b107eca531b635ab3f83d3381ddd5c9002b".to_string(),
    previous_hash: "0000cc07887fb749c99974e8e93debb64e205086f6d0962ef17bf6f0bb295f3e".to_string(),
    timestamp: 1643224393,
    data: String::from("second"),
    nonce: 39308
  };
  app1.blocks.push(app1_block);
  app2.blocks.push(app2_first_block);
  app2.blocks.push(app2_second_block);
  app1.choose_chain(&app2);
  assert_eq!(app1.blocks, app2.blocks);
}
