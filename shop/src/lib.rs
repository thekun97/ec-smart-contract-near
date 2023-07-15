
use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::serde::{Serialize, Deserialize};
use std::clone::Clone;

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
pub struct ShopInfo {
  pub id: String,
  pub name: String,
  pub location: String,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Shop {
  pub owner_id: AccountId,
  pub info: ShopInfo,

  pub shops: UnorderedMap<AccountId, Vec<ShopInfo>>,
}

#[near_bindgen]
impl Shop {

  #[init]
  pub fn new() -> Self {
    let shop_info = ShopInfo {
      id: "Id Shop".to_string(),
      name: "Name".to_string(),
      location: "Location".to_string(),
    };
    
    Self {
      owner_id: env::signer_account_id(),
      info: shop_info,
      shops: UnorderedMap::new(b"shop".to_vec()),
    }
  }

  pub fn add_shop(&mut self, id: String, name: String, location: String) -> ShopInfo {
    let info = ShopInfo {
      id: id.to_string(),
      name: name.to_string(),
      location: location.to_string(),
    };
    
    let mut shops_by_owner = self.shops.get(&env::signer_account_id()).unwrap_or_else(|| Vec::new());
    shops_by_owner.push(info.clone());
    self.shops.insert(&env::signer_account_id(), &shops_by_owner);

    info
  }

  pub fn get_shop_by_owner(&self, account_id: AccountId) -> Option<Vec<ShopInfo>> {
    if let Some(shops) = self.shops.get(&account_id) {
      Some(shops)
    } else {
      None
    }
  }
}