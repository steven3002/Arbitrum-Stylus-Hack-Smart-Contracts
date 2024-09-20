#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

use alloy_primitives::{ Address, U8, I8, U256, U32, I32 };
use stylus_sdk::{ prelude::*, block };

sol_storage! {
    #[entrypoint]
    pub struct VotesState {
        mapping(uint8 => Votes[]) voters;
        mapping(uint8 => MetaData) content_vote;
    }

    pub struct Votes{
        address user_id;
        uint256 stake;
        int8 vote;
        uint32 time_stamp;
    }

    pub struct MetaData{
        int32 total_votes;
        bool good_outcome;
        bool rewarded;

    }
}

#[public]
impl VotesState {
    pub fn vote_content(&mut self, content_id: u8, vote: i8, voter: Address, stake: U256) {
        // votes is int and not uint because we are to represent upvote as 1 and down vote as -1
        let mut content = self.voters.setter(U8::from(content_id));

        for index in 0..content.len() {
            // this is to makr sure you can only vote once
            if content.get(index).unwrap().user_id.get() == voter {
                return;
            }
        }

        let mut vote_x = content.grow();
        vote_x.user_id.set(voter);
        vote_x.stake.set(stake);
        vote_x.vote.set(I8::unchecked_from(vote));
        vote_x.time_stamp.set(U32::from(block::timestamp()));

        let mut metatdata_x = self.content_vote.setter(U8::from(content_id));
        let total_vote = metatdata_x.total_votes.get();

        metatdata_x.total_votes.set(total_vote + I32::unchecked_from(vote));
    }

    pub fn get_voters(&self, content_id: u8) -> Vec<String> {
        let content = self.voters.get(U8::from(content_id));
        let mut votes = vec![];
        for index in 0..content.len() {
            let vote_x = content.get(index).unwrap();
            // here you will notice that i am not passing the vote made by the user but only their stake
            // this is to make sure that the vote is not manipulated against the purpose of the platform
            votes.push(
                format!(
                    r#"{{"voters_id":"{}","stake":"{}","time_stamp":{}}}"#,
                    vote_x.user_id.get(),
                    vote_x.stake.get(),
                    vote_x.time_stamp.get()
                )
            );
        }
        votes
    }

    pub fn get_total_votes(&self, content_id: u8) -> String {
        let content = self.content_vote.get(U8::from(content_id));
        format!("{}", content.total_votes.get())
    }
}
