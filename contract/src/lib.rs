use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedSet};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ProductMetadata {
  pub name: String,
  pub category: String,
  pub price: u64,
}

// pub trait ImplementECommerce {
//   fn create_shop();
//   fn create_product();
//   fn view_all_products();
//   fn view_all_products_per_shop();
//   fn view_product_by_id();
//   fn payment(); // Payment -> Product decrement total_supply;
// }

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Product {
  pub owner_id: AccountId,
  pub shop_id: String,
  pub products_by_shop: LookupMap<String, UnorderedSet<String>>,
  pub products_by_owner: LookupMap<AccountId, UnorderedSet<String>>,
  pub metadata: ProductMetadata,
}

#[near_bindgen]
impl Product {
  #[init]
  pub fn new(name: String, category: String, price: u64, shop_id: String) -> Self {
    let metadata = ProductMetadata {
      name: name.to_string(),
      category: category.to_string(),
      price: price,
    };

    Self {
      owner_id: env::signer_account_id(),
      shop_id,
      metadata,
      products_by_shop: LookupMap::new(b"products_by_shop".to_vec()),
      products_by_owner: LookupMap::new(b"products_by_owner".to_vec()),
    }
  }


  pub fn view_products_per_shop() {}

  pub fn view_products_per_owner() {}

  pub fn view_product_by_id() {}

  pub fn payment() {}
}