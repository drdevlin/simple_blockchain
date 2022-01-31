use sha2::{Sha256, Digest};

pub const PREFIX: &str = "00";

pub fn binary_string_of(hash: &String) -> String {
  hex::decode(hash)
      .unwrap()
      .into_iter()
      .map(|num| format!("{:b}", num))
      .collect::<String>()
}

pub fn calculate_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str, nonce: u64) -> String {
  let content = serde_json::json!({
    "id": id,
    "timestamp": timestamp,
    "previous_hash": previous_hash,
    "data": data,
    "nonce": nonce
  });
  let mut hasher = Sha256::new();
  hasher.update(content.to_string().as_bytes());
  hex::encode(hasher.finalize().as_slice().to_owned())
}

pub fn mine_hash(id: u64, timestamp: i64, previous_hash: &str, data: &str) -> (u64, String) {
  let mut nonce = 0;

  loop {
    let hash = calculate_hash(id, timestamp, previous_hash, data, nonce);
    let binary_hash = binary_string_of(&hash);
    if binary_hash.starts_with(PREFIX) {
      return (nonce, hash);
    }
    nonce += 1;
  }
}

#[test]
fn converts_hash_to_binary_string() {
  let hash = String::from("ff");

  assert_eq!(binary_string_of(&hash), String::from("11111111"));
}

#[test]
fn calculates_hash() {
  let hash = calculate_hash(
    69,
    1643220097,
    "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43",
    "foo",
    9386
  );
  assert_eq!(hash, "00007751f1b92a8ac1bdc88407e7a85b4c0dd59313d8fa78ae2208dbcaaad604".to_string());
}

#[test]
fn mines_hash() {
  let (nonce, hash) = mine_hash(
    69,
    1643220097,
    "0000f816a87f806bb0073dcf026a64fb40c946b5abee2573702828694d5b4c43",
    "foo"
  );
  assert_eq!(nonce, 9386);
  assert_eq!(hash, "00007751f1b92a8ac1bdc88407e7a85b4c0dd59313d8fa78ae2208dbcaaad604".to_string());
}
