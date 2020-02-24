#![feature(proc_macro_hygiene)]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate holochain_json_derive;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use hdk::prelude::*;
use hdk_proc_macros::zome;
mod members;

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[zome_fn("hc_public")]
    fn is_member_valid(agent_address: u64) -> ZomeApiResult<bool> {
        members::is_member_valid(agent_address)
    }
}