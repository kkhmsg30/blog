use crate::*;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Post {
    pub title: String,
    pub category: String,
    pub created_at: u64,
    pub content: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct JsonPost {
    pub post_id: u32,
    pub post: Post,
}

const PAGE_SIZE: usize = 10;

#[near_bindgen]
impl Contract {
    pub fn write_post(&mut self, category: String, title: String, content: String) {
        let post_id = self.posts.len();
        self.add_post_to_category(category.clone(), post_id);
        self.posts.push(Post {
            title: title,
            category: category,
            created_at: env::block_timestamp_ms(),
            content: content,
        });
    }

    pub fn get_number_of_posts(&self) -> u32 {
        self.posts.len()
    }

    pub fn get_posts(&self, from: Option<usize>, limit: Option<usize>) -> Vec<JsonPost> {
        let from = from.unwrap_or(0);
        let limit = limit.unwrap_or(10);
        self.posts
            .iter()
            .enumerate()
            .rev()
            .skip(from)
            .take(limit)
            .map(|(i, post)| JsonPost {
                post_id: i.try_into().unwrap(),
                post: post.clone(),
            })
            .collect()
    }

    // no page == 1 page
    pub fn get_number_of_pages(&self, size: Option<usize>) -> u32 {
        let size: u32 = size.unwrap_or(PAGE_SIZE).try_into().unwrap();
        if self.posts.len() == 0 {
            1
        } else {
            (self.posts.len() - 1) / size + 1
        }
    }

    // start from 1
    pub fn get_posts_by_page(&self, page: usize, size: Option<usize>) -> Vec<JsonPost> {
        let size = size.unwrap_or(PAGE_SIZE);
        let from = (page - 1) * size;
        vec![]
    }
}
