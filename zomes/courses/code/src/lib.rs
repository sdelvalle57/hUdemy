/***************** Required Library */
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate hdk;
extern crate hdk_proc_macros;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::prelude::*;

//use hdk::holochain_json_api::json::JsonString;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::AGENT_ADDRESS;
use hdk_proc_macros::zome;

//use std::convert::TryInto;

/******************************** */

mod content;
mod course;
mod module;
use course::Course;


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
    fn get_my_address() -> ZomeApiResult<Address> {
        Ok(AGENT_ADDRESS.to_string().into())
    }

    /************ Course entry definition and function */
    #[entry_def]
    fn anchor_entry_definition() -> ValidatingEntryType {
        course::anchor_entry_def()
    }

    #[entry_def]
    fn course_entry_definition() -> ValidatingEntryType {
        course::course_entry_def()
    }

    #[zome_fn("hc_public")]
    fn create_course(title: String, timestamp: u64) -> ZomeApiResult<Address> {
        course::create(title, timestamp)
    }

    #[zome_fn("hc_public")]
    fn delete_course(course_address: Address) -> ZomeApiResult<Address> {
        course::delete(course_address)
    }

    #[zome_fn("hc_public")]
    fn update_course(title: String, module_address: Vec<Address>, course_address: Address) -> ZomeApiResult<Address> {
        course::update(title, module_address, course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_courses() -> ZomeApiResult<Vec<Address>> {
        course::list()
    }

    #[zome_fn("hc_public")]
    fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
        course::get_my_courses()
    }

    #[zome_fn("hc_public")]
    fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
        course::get_my_enrolled_courses()
    }

    #[zome_fn("hc_public")]
    fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
        course::enrol_in_course(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_all_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
        course::get_all_students(course_address)
    }

    #[zome_fn("hc_public")]
    fn get_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }


}