use crate::*;

#[near_bindgen]
impl Contract {
    pub fn get_list_categories(&self) -> Vec<String> {
        self.category.keys().map(|s| s.clone()).collect()
    }

    pub fn add_post_to_category(&mut self, category: String, post_id: u32) {
        if !self.category.contains_key(&category) {
            self.category
                .insert(category.clone(), TreeMap::new(env::block_timestamp().try_to_vec().unwrap()));
        }
        self.category
            .get_mut(&category)
            .unwrap()
            .insert(post_id, ());
    }

    // pub fn get_posts_by_category()
}
