/************************ Import Required Libraries */
use hdk::{
    entry_definition::ValidatingEntryType,
    error::{ZomeApiError, ZomeApiResult},
    AGENT_ADDRESS, DNA_ADDRESS, PUBLIC_TOKEN,
};

use hdk::holochain_core_types::dna::entry_types::Sharing;
use hdk::holochain_core_types::{entry::Entry, validation::EntryValidationData};
use holochain_wasm_utils::api_serialization::{
    get_entry::{GetEntryOptions, GetEntryResult},
    get_links::GetLinksOptions,
};

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::holochain_persistence_api::cas::content::Address;
use hdk::prelude::AddressableContent;
use hdk::prelude::LinkMatch;
use hdk::ValidationData;
use std::convert::TryFrom;
use serde_json::json;
/******************************************* */


#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Course {
    title: String,
    pub teacher_address: Address,
    pub modules: Vec<Address>, // Implicit link, as relationship with module
    pub timestamp: u64,
}

impl Course {
    pub fn new(title: String, owner: Address, timestamp: u64) -> Self {
        Course {
            title,
            teacher_address: owner,
            modules: Vec::default(),
            timestamp
        }
    }
    pub fn from(title: String, owner: Address, timestamp: u64, modules: Vec<Address>) -> Self {
        Course {
            title,
            teacher_address: owner,
            modules,
            timestamp
        }
    }
    pub fn entry(&self) -> Entry {
        Entry::App("course".into(), self.into())
    }
}

////////////////////Course Entry Definition
pub fn course_entry_def() -> ValidatingEntryType {
    entry!(
        name: "course",
        description: "this is a course definition",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Course>| {
            match validation_data {
                EntryValidationData::Create { entry, validation_data } => {
                    //only the sender can create
                    //agent address is implicit into the validdation_data by the signatture of the entry
                    if !validation_data.sources().contains(&entry.teacher_address) {
                        return Err(String::from("Only the teacher can create their courses"));
                    }
                    validate_teacher_is_member(&entry.teacher_address)?;
                    validate_course_title(&entry.title)
                },

                EntryValidationData::Modify {new_entry, old_entry, validation_data, ..} => {
                    if new_entry.teacher_address != old_entry.teacher_address {
                        return Err(String::from("Cannot change the teacher of the course"));
                    }

                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can modify their courses"));
                    }

                    validate_course_title(&new_entry.title)
                },

                EntryValidationData::Delete {old_entry, validation_data, ..} => {
                    if !validation_data.sources().contains(&old_entry.teacher_address) {
                        return Err(String::from("Only the teacher can delete their"));
                    }
                    Ok(())
                }
                
            }
        },
        links: [
            from!( // to query all the courses of a user(all courses that a user is the teacher or owner of)
                "%agent_id",
                link_type: "teacher->courses",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!( // to query all courses that one user enrolled
                "%agent_id",
                link_type: "student->courses",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    // TODO: we need validation, use should just enrolle himself to a course, not others.

                    // if let hdk::LinkValidationData::LinkAdd{link, ..} = validation_data {
                    //      link.link.base().Address== _validation_data.sources().con
                    //     if link.link.tag() == "muffins" {
                    //         Err("This is the one tag that is not allowed!".into())
                    //     } else {
                    //         Ok(())
                    //     }
                    // } else {
                    //     Ok(())
                    // }
                    Ok(())
                }
            ),
            to!( // to query all enrolled users for a course)
                "%agent_id",
                link_type: "course->students",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )   
        ]
    )
}

//// Anchor Definition : This Anchor will be used to query all courses
pub fn anchor_entry_def() -> ValidatingEntryType {
    entry!(
        name: "anchor",
        description: "Anchor to all Courses",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: |_validation_data: hdk::EntryValidationData<String>| {
            Ok(())
        },
        links: [
            to!(
                "course",
                link_type: "course_list",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: |_validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

fn anchor_entry() -> Entry {
    Entry::App("anchor".into(), "course".into())
}

fn anchor_address() -> ZomeApiResult<Address> {
    let anchor_entry = anchor_entry();
    hdk::entry_address(&anchor_entry)
}

/*********************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

fn validate_teacher_is_member(teacher_address: &Address) -> ZomeApiResult<()> {
    hdk::debug(format!("qwertyuio {}", hdk::PUBLIC_TOKEN.to_string()))?;
    let is_valid_json: JsonString = hdk::call(
        hdk::THIS_INSTANCE, 
        "members", 
        Address::from(hdk::PUBLIC_TOKEN.to_string()), 
        "is_member_valid", 
        json!({"agent_address": teacher_address}).into(),    
    )?;

    let is_valid: Result<ZomeApiResult<bool>, _> = serde_json::from_str(&is_valid_json.to_string());

    match is_valid {
        Ok(Ok(true)) => Ok(()),
        _ => Err(ZomeApiError::from(String::from(
            "Teacher address is not valid",
        ))),
    }
}

pub fn create(title: String, timestamp: u64) -> ZomeApiResult<Address> {
    let anchor_entry = anchor_entry();
    let anchor_address = hdk::commit_entry(&anchor_entry)?;

    let new_course = Course::new(title, AGENT_ADDRESS.to_string().into(), timestamp);
    let new_course_entry = new_course.entry();
    let new_course_address = hdk::commit_entry(&new_course_entry)?;

    hdk::link_entries(&AGENT_ADDRESS, &new_course_address, "teacher->courses", "")?;

    hdk::link_entries(&anchor_address, &new_course_address, "course_list", "")?;

    Ok(new_course_address)
}

pub fn delete(course_address: Address) -> ZomeApiResult<Address> {
    let anchor_address = anchor_address()?;
    hdk::remove_link(&anchor_address, &course_address, "course_list", "")?;
    let students = get_students(course_address.to_string().into())?;
    for student in students {
        hdk::remove_link(&student, &course_address, "student->course", "")?;
    }
    hdk::remove_entry(&course_address)
}

pub fn update(title: String, modules_addresses: Vec<Address>, course_address: Address) -> ZomeApiResult<Address> {
    let course: Course = hdk::utils::get_as_type(course_address.to_string().into())?;
    
    let new_version_course = Course::from(
        title,
        course.teacher_address,
        course.timestamp,
        modules_addresses,
    );
    let new_version_course_entry = new_version_course.entry();

    hdk::update_entry(new_version_course_entry, &course_address)
}

pub fn list() -> ZomeApiResult<Vec<Address>> {
    //course_list anchor
    let addresses = hdk::get_links(
        &anchor_address()?, 
        LinkMatch::Exactly("course_list"), 
        LinkMatch::Any,
    )?;

    Ok(addresses.addresses())
}

pub fn get_my_courses() -> ZomeApiResult<Vec<Address>> {
    //teacher -> courses
    let links = hdk::get_links(
        &AGENT_ADDRESS,
        LinkMatch::Exactly("teacher->courses"),
        LinkMatch::Any,
    )?;

    Ok(links.addresses())
}

pub fn get_my_enrolled_courses() -> ZomeApiResult<Vec<Address>> {
    // student -> courses
    let links = hdk::get_links(
        &AGENT_ADDRESS, 
        LinkMatch::Exactly("student->courses"), 
        LinkMatch::Any
    )?;
    Ok(links.addresses())
}

pub fn enrol_in_course(course_address: Address) -> ZomeApiResult<Address> {
    hdk::link_entries(&AGENT_ADDRESS, &course_address, "student->courses", "")?;
    hdk::link_entries(&course_address, &AGENT_ADDRESS, "course->students", "")
}

pub fn get_students(course_address: Address) -> ZomeApiResult<Vec<Address>> {
    //course -> students
    let links = hdk::get_links(
        &course_address, 
        LinkMatch::Exactly("course->students"), 
        LinkMatch::Any
    )?;
    Ok(links.addresses())
}