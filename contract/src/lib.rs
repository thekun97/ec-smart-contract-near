use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{UnorderedMap, Vector};
use near_sdk::{env, near_bindgen, AccountId, PanicOnDefault, Balance};

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize)]
pub struct ProductMetadata {
  pub id_product: String,
  pub name: String,
  pub category: String,
  pub price: Balance,
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
  pub fn new(id: String, name: String, category: String, price: Balance, shop_id: String,
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

  pub fn add_product(&mut self, metadata: ProductMetadata, shop_id: Option<String>, owner: Option<AccountId>) -> ProductMetadata {
    let _owner = owner.unwrap_or(env::signer_account_id());
    // TODO: Check signer_account_id == shop owner
    let mut products_by_owner = self.products_by_owner.get(&_owner).unwrap_or_else(|| Vec::new());
    products_by_owner.push(metadata.clone());
    self.products_by_owner.insert(&_owner, &products_by_owner);

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

  pub fn view_product_by_id(&self, product_id: &String) -> Option<ProductMetadata> {
    for item in self.products.iter() {
        if item.id == *product_id {
            return Some(item);
        }
    }
    None
  }

  pub fn update_product_amount(&mut self, product_id: &String, decreased_amount: u64) -> Option<String> {
      for product in &mut self.products {
          if product.id == *product_id {
              current_amount = product.total_supply;
              product.total_supply = current_amount - decreased_amount;
              self.products = self.products.clone();
              Some("Success");
          }
      }
      None
  }

  // pub fn internal_transfer(&mut self, sender_id: AccountId, receiver_id: AccountId, amount: u128) {
  //     assert!(self.balances.contains_key(&sender_id), "Sender account does not exist");
  //     let sender_balance = self.balances.get(&sender_id).unwrap();
  //     assert!(sender_balance >= amount, "Insufficient balance for transfer");
  //
  //     let new_sender_balance = sender_balance - amount;
  //     self.balances.insert(&sender_id, &new_sender_balance);
  //
  //     let receiver_balance = self.balances.get(&receiver_id).unwrap_or(0);
  //     let new_receiver_balance = receiver_balance + amount;
  //     self.balances.insert(&receiver_id, &new_receiver_balance);
  //
  //     Promise::new(receiver_id).transfer(amount);
  // }

  pub fn buy_products(&mut self, product_id: String, amount: u64, seller_id: AccountId) {
    let mut product = self.view_product_by_id(&product_id);

    assert!(product.total_supply >= amount, "Not enough items you need to buy");
    let total_price = product.price * amount;
    assert!(env::attached_deposit() >= total_price as u128, "Not enough NEAR to buy this product");

    // Chuyển tiền từ người mua cho người bán
    let transfer_amount = product.price;
    // self.internal_transfer(&env::predecessor_account_id(), &seller_id, transfer_amount);

    // Trừ số lượng của sản phẩm
    self.update_product_amount(&product_id, amount);

    // Chuyển product cho người mua
    let mut new_product = product.clone();
    new_product.total_supply = amount;
    self.add_product(metadata: new_product,
                     shop_id: None,
                     owner: env::predecessor_account_id());
  }
}