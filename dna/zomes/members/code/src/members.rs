use hdk::prelude::*;

pub fn is_member_valid(agent_address: u64) -> ZomeApiResult<bool> {
    if agent_address == 2 {
        Ok(true)
    } else {
        Ok(false)
    }
}