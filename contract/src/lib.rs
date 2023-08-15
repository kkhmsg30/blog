use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::{self, sha256_array};
use near_sdk::serde::Serialize;
use near_sdk::store::{TreeMap, UnorderedMap, Vector};
use near_sdk::{near_bindgen, CryptoHash, PanicOnDefault};

pub use crate::category::*;
pub use crate::post::*;

mod category;
mod post;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    blog_name: String,
    owner: String,
    posts: Vector<Post>,
    category: UnorderedMap<String, TreeMap<u32, ()>>,
}

#[derive(BorshSerialize)]
enum StorageKeys {
    Posts,
    Category,
    CategoryInner { hash: CryptoHash },
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn default() -> Self {
        let mut category = UnorderedMap::new(StorageKeys::Category.try_to_vec().unwrap());
        category.insert(
            "trash".to_string(),
            TreeMap::new(
                StorageKeys::CategoryInner {
                    hash: sha256_array(&"trash".to_string().try_to_vec().unwrap()),
                }
                .try_to_vec()
                .unwrap(),
            ),
        );
        category.insert(
            "etc".to_string(),
            TreeMap::new(
                StorageKeys::CategoryInner {
                    hash: sha256_array(&"etc".to_string().try_to_vec().unwrap()),
                }
                .try_to_vec()
                .unwrap(),
            ),
        );
        Self {
            blog_name: "blog_name_test".to_string(),
            owner: "kkhmsg30".to_string(),
            posts: Vector::new(StorageKeys::Posts.try_to_vec().unwrap()),
            category: category,
        }
    }
    
    #[init]
    #[private]
    pub fn init(blog_name: String, owner: String) -> Self {
        let mut category = UnorderedMap::new(StorageKeys::Category.try_to_vec().unwrap());
        category.insert(
            "trash".to_string(),
            TreeMap::new(
                StorageKeys::CategoryInner {
                    hash: sha256_array(&"trash".to_string().try_to_vec().unwrap()),
                }
                .try_to_vec()
                .unwrap(),
            ),
        );
        category.insert(
            "etc".to_string(),
            TreeMap::new(
                StorageKeys::CategoryInner {
                    hash: sha256_array(&"etc".to_string().try_to_vec().unwrap()),
                }
                .try_to_vec()
                .unwrap(),
            ),
        );
        Self {
            blog_name: blog_name,
            owner: owner,
            posts: Vector::new(StorageKeys::Posts.try_to_vec().unwrap()),
            category: category,
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut contract = Contract::default();
        contract.write_post(
            "none".to_string(),
            "This is title".to_string(),
            "This is content".to_string(),
        );
        debug_assert_eq!(contract.get_number_of_posts(), 1);
        debug_assert_eq!(contract.get_number_of_posts(), 1);
        let a = contract.get_posts_by_page(1, None);
        let b = a[0].clone();
        println!("{}", b.post.title);
    }

    #[test]
    fn test2() {
        let mut contract = Contract::default();
        contract.write_post("CP".to_string(), "the first post".to_string(), "cp is too easy.".to_string());
        let post=contract.get_posts(None, None);
        println!("{} {} {}", post[0].post.title,post[0].post.category,post[0].post.content);
    }
}
