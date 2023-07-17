use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault};
use near_sdk::env::signer_account_id;

// pub trait ImplementECommerce {
//   fn create_shop();
//   fn create_product();
//   fn view_all_products();
//   fn view_all_products_per_shop();
//   fn view_product_by_id();
//   fn payment(); // Payment -> Product decrement total_supply;
// }

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct ProductMetadata {
  pub id_product: String,
  pub name: String,
  pub category: String,
  pub price: u64,
  pub total_supply: u64,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Product {
  pub owner_id: AccountId,
  pub shop_id: String,
  pub metadata: ProductMetadata,

  pub products_by_shop: UnorderedMap<String, Vec<ProductMetadata>>,
  pub products_by_owner: UnorderedMap<AccountId, Vec<ProductMetadata>>,

  pub products: Vector<ProductMetadata>,
}

#[near_bindgen]
impl Product {
  #[init]
  pub fn new(id: String, name: String, category: String, price: u64, shop_id: String, 
    total_supply: u64) -> Self {
    let metadata = ProductMetadata {
      id_product: id,
      name: name.to_string(),
      category: category.to_string(),
      price,
      total_supply,
    };

    Self {
      owner_id: env::signer_account_id(),
      shop_id,
      metadata,
      products_by_shop: UnorderedMap::new(b"products_by_shop".to_vec()),
      products_by_owner: UnorderedMap::new(b"products_by_owner".to_vec()),
      products: Vector::new(b"products".to_vec()),
    }
  }

  pub fn add_product(&mut self, metadata: ProductMetadata, shop_id: Option<String>) -> ProductMetadata {
    // TODO: Check signer_account_id == shop owner
    let mut products_by_owner = self.products_by_owner.get(&env::signer_account_id()).unwrap_or_else(|| Vec::new());
    products_by_owner.push(metadata.clone());
    self.products_by_owner.insert(&env::signer_account_id(), &products_by_owner);

    self.products.push(metadata.clone());
    if let Some(value) = shop_id {
        let mut products_by_shop = self.products_by_shop.get(&value).unwrap_or_else(|| Vec::new());
        products_by_shop.push(metadata.clone());
        self.products_by_shop.insert(&value, &products_by_shop);
    }
    info
  }

  pub fn view_products_per_shop(&self, shop_id: String) -> Option<Vec<ProductMetadata>> {
    if let Some(prods) = self.products_by_shop.get(&shop_id) {
      Some(prods)
    } else {
      None
    }
  }

  pub fn view_products_per_owner(&self, account_id: AccountId) -> Option<Vec<ProductMetadata>> {
    if let Some(prods) = self.products_by_owner.get(&account_id) {
      Some(prods)
    } else {
      None
    }
  }

  pub fn view_product_by_id(&self, product_id: String) -> Option<ProductMetadata> {
    for item in self.products.iter() {
        if item.id == product_id {
            return Some(item);
        }
    }
    None
  }

  pub fn payment() {}
}