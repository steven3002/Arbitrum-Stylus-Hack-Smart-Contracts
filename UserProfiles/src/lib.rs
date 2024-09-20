#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U32, I8, U8 };
use stylus_sdk::{ prelude::*, block };

sol_storage! {
    #[entrypoint]
    pub struct Users {
        mapping(address => User) users;
        mapping(address => uint8[] ) user_community;
        mapping(address => uint8[] ) author_content;
    }

    pub struct User{
        address user_id;
        bool has_registered;
        Profile profile;
    }

    pub struct Profile{
        // string bio;
        int8 reputation;
        uint32 joined_at;
    }

}

#[public]
impl Users {
    pub fn register_user(&mut self, address: Address) {
        if self.has_registered(address) {
            return;
        }

        // if self.check_name_taken(name.clone()) {
        //     return;
        // }

        let mut new_user = self.users.setter(address);
        let current_time = U32::from(block::timestamp());

        new_user.user_id.set(address);
        // new_user.username.set_str(user_name);
        // new_user.profile.bio.set_str(bio);
        new_user.profile.reputation.set(I8::unchecked_from(0));
        new_user.profile.joined_at.set(current_time);
    }

    pub fn has_registered(&self, user_address: Address) -> bool {
        self.users.get(user_address).has_registered.get()
    }
    // pub fn check_name_taken(&self, name: String) -> bool {
    //     for index in 0..self.users.len() {
    //         if self.users.get(index).unwrap().username.get_string() == name.to_lowercase() {
    //             return true;
    //         }
    //     }
    //     return false;
    // }

    // will continue this later
    // pub fn get_profile(&self, name: String) -> bool {}

    pub fn change_reputation_state(&mut self, user_id: Address, points: i64) {
        let mut user = self.users.setter(user_id);

        let current_point = user.profile.reputation.get();
        let new_point = current_point + I8::unchecked_from(points);
        user.profile.reputation.set(new_point);
    }

    pub fn get_profile(&self, user_id: Address) -> String {
        let user = self.users.get(user_id);

        format!(
            r#"{{"user_id":"{}","reputation":"{}","joined_at":"{}"}}"#,
            user.user_id.get(),
            user.profile.reputation.get(),
            user.profile.joined_at.get()
        )
    }

    pub fn set_my_stakes(&mut self, user: Address, content_id: u8) {
        let mut author_content_x = self.author_content.setter(user);
        author_content_x.push(U8::from(content_id));
    }

    pub fn set_community(&mut self, user: Address, community_id: u8) {
        let mut author_content_x = self.author_content.setter(user);
        author_content_x.push(U8::from(community_id));
    }

    pub fn get_community(&mut self, user: Address) -> Vec<u8> {
        let user_community_x = self.user_community.get(user);
        let mut list = vec![];
        for inx in 0..user_community_x.len() {
            let test = user_community_x.get(inx).unwrap();
            let tests: [u8; 1] = test.to_le_bytes();
            list.push(tests[0]);
        }
        return list;
    }
    pub fn get_my_stakes(&mut self, user: Address) -> Vec<u8> {
        let user_community_x = self.author_content.get(user);
        let mut list = vec![];
        for inx in 0..user_community_x.len() {
            let test = user_community_x.get(inx).unwrap();
            let tests: [u8; 1] = test.to_le_bytes();
            list.push(tests[0]);
        }
        return list;
    }
}
