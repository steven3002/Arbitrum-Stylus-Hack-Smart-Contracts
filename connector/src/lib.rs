#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
// written by Steven Hert, omo this code needs to be formated after the competition

extern crate alloc;
use stylus_sdk::{ prelude::*, stylus_proc::entrypoint };
use stylus_sdk::{ console, msg, evm, alloy_primitives::{ Address, U8, U256 } };

use stylus_sdk::call::Call;

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

sol_storage! {
 #[entrypoint]
 pub struct Test{
    uint8 content_index;
    address content_contract;
    address vote_contract
 } 
}

sol_interface! {

    interface IContentState {
        function submitContent(address author_id, string calldata title, uint8 category, string calldata content, string calldata stake, uint8 community_id, bool is_global, uint8 content_counter_x) external;

        function getContent(uint8 content_id) external view returns (string memory);

        function getContentBySpecific(uint8 type_x, uint8 type_id) external view returns (string[] memory);

        function setVerified(uint8 content_id) external;
    }

    interface IVotesState {
        function voteContent(uint8 content_id, int8 vote, address voter, uint256 stake) external;

        function getVoters(uint8 content_id) external view returns (string[] memory);

        function getTotalVotes(uint8 content_id) external view returns (string memory);
    }
}

#[public]
impl Test {
    pub fn set_content_address(&mut self, address: Address) {
        self.content_contract.set(address);
    }

    pub fn set_vote_address(&mut self, address: Address) {
        self.vote_contract.set(address);
    }

    // pub fn get_content_list(&self, type_x: u8, type_id: u8) -> Vec<String> {
    //     let address = self.content_contract.get();
    //     let meta_date_contract = IContentState::new(address);
    //     let config = Call::new();
    //     meta_date_contract.get_content_by_specific(config, type_x, type_id).expect("drat")
    // }

    // pub fn get_content(&self, content_id: u8) -> String {
    //     let address = self.content_contract.get();
    //     let meta_date_contract = IContentState::new(address);
    //     let config = Call::new();
    //     meta_date_contract.get_content(config, content_id).expect("drat")
    // }

    pub fn add_content(
        &mut self,
        title: String,
        category: u8,
        content: String,
        stake: U256,
        community_id: u8,
        is_global: bool
    ) {
        let author_id = msg::sender();
        let content_counter_x = self.content_index.get();
        let content_index_ue: [u8; 1] = content_counter_x.to_le_bytes();
        let format_stake = format!("{}", stake);
        let vote: i8 = 1;

        let meta_date_contract = IContentState::new(*self.content_contract);
        let config = Call::new_in(self);
        let _ = meta_date_contract
            .submit_content(
                config,
                author_id,
                title,
                category,
                content,
                format_stake,
                community_id,
                is_global,
                content_index_ue[0]
            )
            .expect("Failed to call on MetaDate");
        self.vote_state(stake, vote, content_index_ue[0]);
        let new_index = content_counter_x + U8::from(1);
        self.content_index.set(new_index);
    }

    pub fn get_voters(&self, content_id: u8) -> Vec<String> {
        let address = self.vote_contract.get();
        let meta_date_contract = IVotesState::new(address);
        let config = Call::new();
        meta_date_contract.get_voters(config, content_id).expect("drat")
    }

    pub fn get_total_votes(&self, content_id: u8) -> String {
        let address = self.vote_contract.get();
        let meta_date_contract = IVotesState::new(address);
        let config = Call::new();
        meta_date_contract.get_total_votes(config, content_id).expect("drat")
    }

    pub fn vote(&mut self, stake: U256, vote: i8, content_id: u8) {
        self.vote_state(stake, vote, content_id)
    }
}

impl Test {
    pub fn vote_state(&mut self, stake: U256, vote: i8, content_id: u8) {
        let author_id = msg::sender();

        let meta_date_contract = IVotesState::new(*self.vote_contract);
        let config = Call::new_in(self);
        let _ = meta_date_contract
            .vote_content(config, content_id, vote, author_id, stake)
            .expect("Failed to call on MetaDate_contract");
    }
}
