#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use stylus_sdk::{ prelude::*, block, alloy_primitives::{ Address, U8 } };
//

sol_storage! {
    #[entrypoint]
    pub struct ContentState {
        mapping( uint8 => Content) contents;
        mapping( uint8  =>  uint8[]) community_content; //so i am making thi map the community to their list of contents



    }
    pub struct Content {
         uint8 content_id;
        address author_id;
         uint8 time_stamp;
        string title; //size issues so title will be saved as a json with the title and discription
         uint8 category;//due to file size issue the category will be stored on another contract
        string content;
         string stake;
        bool verified;
         uint8 community_id; //communities start index from 1, so 0 index will be treated as null
        // string[] tags;
    }

   
}

#[public]
impl ContentState {
    pub fn submit_content(
        &mut self,
        author_id: Address,
        title: String,
        category: u8,
        content: String,
        stake: String,
        community_id: u8,
        is_global: bool,
        content_counter_x: u8
    ) {
        let content_counter = U8::from(content_counter_x);

        if !is_global {
            let community_x = U8::from(community_id);
            let mut content_community = self.community_content.setter(community_x);
            content_community.push(content_counter);
        } else {
            let global_x = U8::from(0);
            let mut global_v = self.community_content.setter(global_x);
            global_v.push(content_counter);
        }

        let mut new_content = self.contents.setter(content_counter);

        new_content.content_id.set(content_counter);
        new_content.author_id.set(author_id);
        new_content.time_stamp.set(U8::from(block::timestamp()));
        new_content.title.set_str(title);
        new_content.category.set(U8::from(category));
        new_content.content.set_str(content);
        new_content.stake.set_str(stake);
        new_content.verified.set(false);
    }

    pub fn get_content(&self, content_id: u8) -> String {
        let content_x = self.contents.get(U8::from(content_id));
        return content_x.content.get_string();
    }

    pub fn get_content_by_specific(&self, type_x: u8, type_id: u8) -> Vec<String> {
        // `raw_data` will be assigned based on the match condition
        let raw_data = match type_x {
            1 => self.community_content.get(U8::from(type_id)),
            _ => self.community_content.get(U8::from(0)), // Default to global content
        };

        // Prepare to collect content information
        let mut info = vec![];

        // Iterate over `raw_data` and collect each content item
        for index in 0..raw_data.len() {
            if let Some(content_id) = raw_data.get(index) {
                // Fetch content using content_id
                let content = self.contents.get(content_id);
                // Push formatted content information to `info`
                info.push(
                    format!(
                        r#"{{"content_id":"{}","author_id":"{}","title":"{}","category":"{}","verified":{},"stake":"{}","time_stamp":{}}}"#,
                        content_id,
                        content.author_id.get(),
                        content.title.get_string(),
                        content.category.get(),
                        content.verified.get(),
                        content.stake.get_string(),
                        content.time_stamp.get()
                    )
                );
            }
        }

        info
    }

    pub fn set_verified(&mut self, content_id: u8) {
        let mut content_x = self.contents.setter(U8::from(content_id));
        content_x.verified.set(true);
    }
}
