#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_json_derive;

use hdk::error::ZomeApiResult;
// use hdk::error::ZomeApiError;
use hdk::holochain_core_types::{dna::entry_types::Sharing, entry::Entry};

// use hdk::holochain_persistence_api::cas::content::Address;

use hdk::holochain_json_api::{error::JsonError, json::JsonString};
use hdk::prelude::{LinkMatch};
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

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct FriendshipRequest {}

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Friendship {}
pub fn handle_get_my_agent_address() -> ZomeApiResult<HashString> {
	Ok(hdk::AGENT_ADDRESS.clone().into())
}

pub fn handle_get_my_followers() -> ZomeApiResult<Vec<HashString>> {
	let my_followers: Vec<HashString> = Vec::new(); // TODO: implement
	Ok(my_followers)
}

pub fn handle_get_my_followings() -> ZomeApiResult<Vec<HashString>> {
	let my_agent_address = hdk::AGENT_ADDRESS.clone().into();
	match hdk::api::get_links(
		&my_agent_address, 
		LinkMatch::Exactly("follows"),
		LinkMatch::Any
	) {
		Ok(result) => Ok(result.addresses()),
		Err(_e) => {
			Err(hdk::error::ZomeApiError::ValidationFailed("yes it was here".into()))
		}
	}
}

pub fn handle_request_friendship(
	receiver_address: HashString,
) -> ZomeApiResult<()> {
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
	Ok(())
}

pub fn handle_decline_friendship_request() {}

pub fn handle_get_incoming_friendship_requests() -> ZomeApiResult<Vec<FriendshipRequest>> {
	let incoming_friendship_requests: Vec<FriendshipRequest> = Vec::new();
	Ok(incoming_friendship_requests)
}

pub fn handle_get_outgoing_friendship_requests() -> ZomeApiResult<Vec<HashString>> {
	let my_agent_address = hdk::AGENT_ADDRESS.clone().into(); 
	match hdk::api::get_links(
		&my_agent_address, 
		LinkMatch::Exactly("friendship_request_send"), 
		LinkMatch::Any
	) {
		Ok(result) => Ok(result.addresses()), 
		Err(e) => Err(e)
	}
}

pub fn handle_follow(target_agent_address: HashString) -> ZomeApiResult<()> {
	let sender_address = hdk::AGENT_ADDRESS.clone().into();
	hdk::link_entries(&sender_address, &target_agent_address, "follows", "")?;
	Ok(())
}

pub fn handle_unfollow(target_agent_address: HashString) -> ZomeApiResult<()> {
	let sender_address = hdk::AGENT_ADDRESS.clone().into();
	hdk::remove_link(&sender_address, &target_agent_address, "follows", "")?;
	Ok(())
}


define_zome! {
	entries: [
		entry!(
			name: "friendship_request",
			description: "expresses the willingness of one agent to be in a friendship relation with another one",
			sharing: Sharing::Public,
			validation_package: || {
				hdk::ValidationPackageDefinition::Entry
			},
			validation: | _validation_data: hdk::EntryValidationData<FriendshipRequest> | {
				Ok(())
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
		my_agent_address: {
			inputs: | |,
			outputs: |result: ZomeApiResult<HashString>|,
			handler: handle_get_my_agent_address
		}
		request_friendship: {
			inputs: |other_agent: hdk::holochain_persistence_api::hash::HashString|,
			outputs: |result: ZomeApiResult<()>|,
			handler: handle_request_friendship
		}
		outgoing_friendship_requests: {
			inputs: | |,
			outputs: | result: ZomeApiResult<Vec<HashString>> |,
			handler: handle_get_outgoing_friendship_requests
		}
		follow: {
			inputs: |target_agent_address: HashString|,
			outputs: |result: ZomeApiResult<()>|,
			handler: handle_follow
		}
		unfollow: {
			inputs: |target_agent_address: HashString|,
			outputs: |result: ZomeApiResult<()>|,
			handler: handle_unfollow
		}
		my_followings: {
			inputs: | |,
			outputs: |result: ZomeApiResult<Vec<HashString>>|, 
			handler: handle_get_my_followings
		}
	]

	traits: {
		hc_public [my_agent_address, request_friendship, outgoing_friendship_requests, follow, my_followings]
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
