#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::{
    // entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
};
// use hdk::error::ZomeApiError;
use hdk::holochain_core_types::{
    entry::Entry,
    dna::entry_types::Sharing,
};

// use hdk::holochain_persistence_api::cas::content::Address;

use hdk::holochain_json_api::{
    error::JsonError,
    json::JsonString,
};

use hdk::holochain_persistence_api::hash::HashString;

/* 	trait from here: https://github.com/juntofoundation/Holochain-Trait-Definitions
this will go to a different file at some point 
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
*/

// see https://developer.holochain.org/api/0.0.47-alpha1/hdk/ for info on using the hdk library

// This is a sample zome that defines an entry type "MyEntry" that can be committed to the
// agent's chain via the exposed function create_my_entry

/* Thoughts: 
	- Validation
		- A friend request acceptance requires an according friend request to be present
*/
#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)]
pub struct Followship{
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)] 
pub struct FriendshipRequest{
}

#[derive(Serialize, Deserialize, Debug, DefaultJson,Clone)] 
pub struct Friendship{
}

pub fn handle_get_my_agent_address() -> ZomeApiResult<HashString> {
	Ok(hdk::AGENT_ADDRESS.clone().into())
}

pub fn handle_get_my_followers() -> ZomeApiResult<Vec<HashString>> {
	let my_followers:Vec<HashString> = Vec::new();
	Ok(my_followers)
}

pub fn handle_get_my_followings() -> ZomeApiResult<Vec<HashString>> {
	let my_followings:Vec<HashString> = Vec::new();
	Ok(my_followings)
}

pub fn handle_request_friendship(
	receiver_address: HashString,
) -> ZomeApiResult<u32> {
	let sender_address = hdk::AGENT_ADDRESS.clone().into();
	let friendship_request = FriendshipRequest {};
	let entry = Entry::App("friendship_request".into(), friendship_request.into());
	let entry_address = hdk::commit_entry(&entry)?;
	hdk::link_entries(
		&sender_address,
		&entry_address,
		"friendship_request_send",
		"",
	)?;
	hdk::link_entries(
		&receiver_address,
		&entry_address,
		"friendship_request_receive",
		"",
	)?;
	Ok(4) // TODO '4' ist nonsens.
}

pub fn handle_accept_friendship_request() {
}

pub fn handle_decline_friendship_request() {
}

pub fn handle_unfollow() {
}

pub fn handle_follow() {
}

pub fn handle_get_incoming_friendship_requests() -> ZomeApiResult<Vec<FriendshipRequest>> {
	let incoming_friendship_requests:Vec<FriendshipRequest> = Vec::new();
	Ok(incoming_friendship_requests)
}

pub fn handle_get_outgoing_friendship_requests() -> ZomeApiResult<Vec<FriendshipRequest>> {
	let outgoing_friendship_requests:Vec<FriendshipRequest> = Vec::new();
	Ok(outgoing_friendship_requests)
}

define_zome! {
    entries: [
		entry!(
			name: "friendship_request",
			description: "an entry linked from an agent entry to this agent's outgoing followships",
			sharing: Sharing::Public, 
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<FriendshipRequest> | {
				Ok(())
				// maybe check here, that there is only one list per agent
			},
			links: [
				from!(
					"%agent_id",
					link_type: "friendship_request_send",
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					},
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				),
				from!(
					"%agent_id",
					link_type: "friendship_request_receive", 
					validation_package: || {
						hdk::ValidationPackageDefinition::Entry
					}, 
					validation: |_validation_data: hdk::LinkValidationData| {
						Ok(())
					}
				)
			]
		)
    ]

    init: || { Ok(()) }

    validate_agent: |validation_data : EntryValidationData::<AgentId>| {
        Ok(())
    }

    functions: [
		get_my_entry: {
			inputs: | |,
			outputs: |result: ZomeApiResult<HashString>|,
			handler: handle_get_my_agent_address
		}
        create_friend_request: {
            inputs: |receiver_address: hdk::holochain_persistence_api::hash::HashString|,
            outputs: |result: ZomeApiResult<u32>|,
            handler: handle_request_friendship
        }
    ]
    traits: {
        hc_public [create_my_entry, get_my_entry]
		/*SocialGraph [ 
			my_followers,
			followers, 
			nth_level_followers,
			follow, 
			unfollow, 
			my_friends, 
			friends_of, 
			request_friendship, 
			decline_friendship, 
			incoming_friendship_requests, 
			outgoing_friendship_requests, 
			drop_friendship
		]*/
    }
}
