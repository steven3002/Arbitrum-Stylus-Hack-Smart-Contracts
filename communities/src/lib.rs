#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U8, U32, Uint };
use stylus_sdk::{ prelude::*, block };
//

sol_storage! {
    #[entrypoint]
    pub struct CommunityState {
        
        mapping( uint8 => Community) communities;
        mapping(string => bool) taken_names;

        // mapping(address => uint8[]) users_community;
        

    }
    pub struct Community {
    address creator;
    string name;
    string discription;
    uint32 created_at;
     address[] members;
    }

   
}
#[public]
impl CommunityState {
    pub fn create_community(
        &mut self,
        name: String,
        description: String,
        creator: Address,
        available_index: u8
    ) {
        if self.name_taken(name.clone()) {
            return;
        }
        let mut taken_name = self.taken_names.setter(name.clone());
        taken_name.set(true);
        let available_index = U8::from(available_index);

        // let test = U8::from(0);
        // let tests: [u8; 1] = test.to_le_bytes();

        // let mut my_community = self.users_community.setter(creator);
        // my_community.push(available_index);

        let mut new_community = self.communities.setter(available_index);
        new_community.creator.set(creator);
        new_community.name.set_str(name.clone());
        new_community.discription.set_str(description);
        new_community.created_at.set(U32::from(block::timestamp()));
    }

    pub fn get_community(&self, index: u8) -> String {
        let community = self.communities.get(U8::from(index));
        return format!(
            r#"{{"creator":"{}","name":"{}","discription":"{}","numbers of members":"{}","created_at":{}}}"#,
            community.creator.get(),
            community.name.get_string(),
            community.discription.get_string(),
            community.members.len(),
            community.created_at.get()
        );
    }

    pub fn add_user_to_community(&mut self, community_id: u8, user_id: Address) {
        if self.is_a_member(community_id, user_id) {
            return;
        }
        let mut community = self.communities.setter(U8::from(community_id));
        community.members.push(user_id);

        // let mut my_community = self.users_community.setter(user_id);
        // my_community.push(U8::from(community_id));
    }
    // pub fn get_last_index(&self) -> String {
    //     return format!("{}", self.available_index.get());
    // }
    pub fn is_a_member(&self, index: u8, user: Address) -> bool {
        let community = self.communities.get(U8::from(index));
        if community.creator.get() == user {
            return true;
        }
        for ix in 0..community.members.len() {
            if community.members.get(ix).unwrap() == user {
                return true;
            }
        }
        return false;
    }

    pub fn name_taken(&self, name: String) -> bool {
        self.taken_names.get(name)
    }

    // pub fn get_my_communities(&self, user: Address) -> Vec<String> {
    //     let mut data = vec![];
    //     let my_community = self.users_community.get(user);
    //     for ix in 0..my_community.len() {
    //         let state = format!("{}", my_community.get(ix).unwrap());
    //         data.push(state);
    //     }
    //     return data;
    // }
}
