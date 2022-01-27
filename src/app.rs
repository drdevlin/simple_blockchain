use chrono::Utc;
use crate::block::Block;
use crate::helpers::*;

#[derive(PartialEq, Debug)]
pub struct App<Block> {
  pub blocks: Vec<Block>,
}

// TODO: handle unwraps properly
impl App<Block> {
  fn new() -> Self {
    Self { blocks: vec![] }
  }

  fn genesis(&mut self) {
    let genesis_block = Block {
      id: 0,
      hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
      previous_hash: String::from("genesis"),
      timestamp: Utc::now().timestamp(),
      data: String::from("genesis!"),
      nonce: 2836,
    };
    self.blocks.push(genesis_block);
  }

  fn add_block(&mut self, block: Block) {
    let tail = &self.blocks.last().unwrap();
    if self.is_block_valid(&block, tail) {
      self.blocks.push(block);
    }
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

  fn is_chain_valid(&self) -> bool {
    let mut blocks = self.blocks.iter();
    blocks.next();
    let mut previous_blocks = self.blocks.iter();

    blocks.all(|block| self.is_block_valid(&block, &previous_blocks.next().unwrap()))
  }

  fn choose_chain(&mut self, remote: &App<Block>) {
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

// TODO: Refactor tests to keep them isolated. I.E. no method calls except for the one tested.
#[test]
fn creates_a_new_app() {
  let expected = App::<Block> { blocks: vec![] };
  let result = App::new();
  assert_eq!(expected, result);
}

#[test]
fn creates_genesis_block() {
  let mut new_app = App::new();
  println!("{:#?}", new_app.blocks.last());
  new_app.genesis();
  println!("{:#?}", new_app.blocks.last());
  assert_eq!(&new_app.blocks[0], &new_app.blocks[0]);
}

#[test]
fn adds_a_valid_block() {
  let mut new_app = App::new();
  let block = Block {
    id: 1,
    hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 24271,
  };
  let expected = block.clone();
  new_app.genesis();
  new_app.add_block(block);
  assert_eq!(&new_app.blocks[1], &expected);
}

#[test]
#[should_panic]
fn add_an_invalid_block() {
  let mut new_app = App::new();
  let invalid_block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  new_app.genesis();
  new_app.add_block(invalid_block);
  let added_block = &new_app.blocks[1];
}

#[test]
fn valid_when_prev_hash_match() {
  let mut new_app = App::new();
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
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_prev_hash_mismatch() {
  let mut new_app = App::new();
  let block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };
  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_when_prefix_match() {
  let mut new_app = App::new();
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
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };

  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_prefix_mismatch() {
  let mut new_app = App::new();
  let block = Block {
    id: 1,
    hash: String::from("ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };

  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_when_next_id() {
  let mut new_app = App::new();
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
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };

  assert!(new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_not_next_id() {
  let mut new_app = App::new();
  let block = Block {
    id: 2,
    hash: String::from("0000ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };

  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn invalid_when_not_a_hash() {
  let mut new_app = App::new();
  let block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  let previous_block = Block {
    id: 0,
    hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    previous_hash: String::from("genesis"),
    timestamp: Utc::now().timestamp(),
    data: String::from("genesis!"),
    nonce: 2836,
  };

  assert!(!new_app.is_block_valid(&block, &previous_block));
}

#[test]
fn valid_chain_when_all_blocks_valid() {
  let mut new_app = App::new();
  let first_block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  new_app.genesis();
  new_app.add_block(first_block);
  assert!(new_app.is_chain_valid());
}

#[test]
fn invalid_chain_when_invalid_block() {
  let mut new_app = App::new();
  let first_block = Block {
    id: 1,
    hash: String::from("0000ff"),
    previous_hash: "not_the_previous_hash".to_string(),
    timestamp: Utc::now().timestamp(),
    data: String::from("next"),
    nonce: 2836,
  };
  new_app.genesis();
  new_app.blocks.push(first_block);
  assert!(!new_app.is_chain_valid());
}

#[test]
fn chooses_the_longest_valid_chain() {
  let mut app1 = App::new();
  let mut app2 = App::new();
  let app1_block = Block {
    id: 1,
    hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    previous_hash: "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43".to_string(),
    timestamp: 1643223669,
    data: String::from("next"),
    nonce: 24271
  };
  let app2_first_block = app1_block.clone();
  let app2_second_block = Block {
    id: 2,
    hash: "00007bdfa20dee7c17ae04ac745ef45a23cdd1d05a4f7299f69d335e3d00c6a7".to_string(),
    previous_hash: "00005ea81511a2a24a25a2055d5fc581879b8cfbedc5ddfb6918caed4917138e".to_string(),
    timestamp: 1643224393,
    data: String::from("second"),
    nonce: 38161
  };
  app1.genesis();
  app2.genesis();
  app1.add_block(app1_block);
  app2.add_block(app2_first_block);
  app2.add_block(app2_second_block);
  app1.choose_chain(&app2);
  assert_eq!(app1.blocks, app2.blocks);
}
