use hdk::prelude::*;
use hdk::AGENT_ADDRESS;

#[derive(Serialize, Deserialize, Debug, self::DefaultJson, Clone)]
pub struct Course {
    title: String,
    teacher_address: Address,
    modules: Vec<Address>, // Implicit link, as relationship with module
    timestamp: u64,
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

/*********************** Course Validations */
fn validate_course_title(title: &str) -> Result<(), String> {
    if title.len() > 50 {
        Err("Course title is too long".into())
    } else {
        Ok(())
    }
}

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
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            from!( // to query all courses that one user enrolled
                "%agent_id",
                link_type: "student->courses",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            ),
            to!( // to query all enrolled users for a course)
                "%agent_id",
                link_type: "course->students",
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                }              ,
                validation: | _validation_data: hdk::LinkValidationData | {
                    Ok(())
                }
            )   
        ]
    )
}

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

pub fn anchor_entry() -> Entry {
    Entry::App("anchor".into(), "course".into())
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

fn anchor_address() -> ZomeApiResult<Address> {
    hdk::entry_address(&anchor_entry())
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
    Err(ZomeApiError::from(String::from("Do your homework please")))
}

pub fn enrol_in_course(_course_address: Address) -> ZomeApiResult<Address> {
    //student -> courses
    //course-> students
    Err(ZomeApiError::from(String::from("Do your homework please")))
}

pub fn get_all_students(_course_address: Address) -> ZomeApiResult<Vec<Address>> {
    //course -> students
    Err(ZomeApiError::from(String::from("Do your homework please")))
}

