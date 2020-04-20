#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
use hdk::error::ZomeApiError;
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

use hdk::holochain_persistence_api::{
    cas::content::Address,
};

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

/* 	trait from here: https://github.com/juntofoundation/Holochain-Trait-Definitions
	this will go to a different file at some point */ 
type Identity = hdk::holochain_core_types::agent::AgentId;

/// Trait that provides an interface for creating and maintaining a social graph 
/// between agents. 
///
/// Follows & Connections between agents can take an optional
/// metadata parameter; "by".
/// This parameter is used to associate some semantic between relationships.
/// In Junto's case this field could be leveraged to create
/// user definable perspectives. Example; follow graph for my:
/// holochain connections, personal connections and drone connections
///
/// The other possibility is to create a new DNA implementing this trait 
/// for each social graph context the user wants to define.

trait SocialGraph {
    // Follow Related Operations
    fn my_followers(by: Option<String>) -> Vec<Identity>;
    fn followers(followed_agent: Identity, by: Option<String>) -> Vec<Identity>;
    fn nth_level_followers(n: u32, followed_agent: Identity, by: Option<String>) -> Vec<Identity>;

    fn my_followings(by: Option<String>) -> Vec<Identity>;
    fn following(following_agent: Identity, by: Option<String>) -> Vec<Identity>;
    fn nth_level_following(n: u32, following_agent: Identity, by: Option<String>) -> Vec<Identity>;

    fn follow(other_agent: Identity, by: Option<String>) -> Result<(), ZomeApiError>;
    fn unfollow(other_agent: Identity, by: Option<String>) -> Result<(), ZomeApiError>;
    
    // Connection Related Operations (i.e. bidirectional friendship)
    fn my_friends() -> Vec<Identity>;
    fn friends_of(agent: Identity) -> Vec<Identity>;
    
    fn request_friendship(other_agent: Identity);
    fn decline_friendship(other_agent: Identity);
    
    fn incoming_friendship_requests() -> Vec<Identity>;
    fn outgoing_friendship_requests() -> Vec<Identity>;
    
    fn drop_friendship(other_agent: Identity) -> Result<(), ZomeApiError>;
}

// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct MyEntry {
    content: String,
}

pub fn handle_create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
    let entry = Entry::App("my_entry".into(), entry.into());
    let address = hdk::commit_entry(&entry)?;
    Ok(address)
}

pub fn handle_get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
    hdk::get_entry(&address)
}

fn definition() -> ValidatingEntryType {
    entry!(
        name: "my_entry",
        description: "this is a same entry defintion",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },

        validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
            Ok(())
        }
    )
}

define_zome! {
    entries: [
       definition()
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
        create_my_entry: {
            inputs: |entry: MyEntry|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: handle_create_my_entry
        }
        get_my_entry: {
            inputs: |address: Address|,
            outputs: |result: ZomeApiResult<Option<Entry>>|,
            handler: handle_get_my_entry
        }
    ]

    traits: {
        hc_public [create_my_entry,get_my_entry]
    }
}
