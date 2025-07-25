#![cfg(test)]
extern crate std;

use crate::{ContributorReputation, ContributorReputationClient};
use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env, Map, String, Vec};
use crate::storage::*;
use crate::reputation::*;
use crate::types::*;

#[test]
fn test_initialize_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));

    assert_eq!(user_id, 1, "User ID should be 1");
    let areas = contract_client.get_expertise_areas(&user_id);
    assert_eq!(areas.len(), 0);
}

#[test]
#[should_panic(expected = "Error(Auth, InvalidAction)")]
fn test_initialize_user_unauthorized() {
    let env = Env::default();

    let contract_id = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_id);

    let caller = Address::generate(&env);
    let name = String::from_str(&env, "Bob");

    // Try to initialize without authentication
    contract_client.initialize_user(&caller, &name);
}

#[test]
fn test_mint_credential_token_for_verified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user = String::from_str(&env, "Alice");
    let user_id = contract_client.initialize_user(&caller, &user);

    // Verify user first
    let verification_details = String::from_str(&env, "valid credentials");
    contract_client.verify_user(&caller, &user_id, &verification_details);

    // Now mint the credential token
    let token_id = contract_client.mint_credential_token(&caller, &user_id);

    assert_eq!(token_id, 1, "Token ID should be 1");
    let user = contract_client.get_user(&user_id);
    assert_eq!(user.verified, true);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_mint_credential_token_non_existent_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.mint_credential_token(&caller, &999);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_mint_credential_token_for_unverified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.mint_credential_token(&caller, &user_id);
}

#[test]
fn test_update_reputation_for_verified() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);
    contract_client.update_reputation(
        &caller,
        &user_id,
        &String::from_str(&env, "Mathematics"),
        &100,
    );

    // Check if the reputation was updated correctly
    let score = contract_client.get_reputation(&user_id, &String::from_str(&env, "Mathematics"));
    assert_eq!(score, 100);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_update_reputation_unverified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.update_reputation(
        &caller,
        &user_id,
        &String::from_str(&env, "Mathematics"),
        &100,
    );
}

#[test]
fn test_update_expertise_areas() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    let mut expertise_areas = Map::new(&env);

    expertise_areas.set(String::from_str(&env, "Mathematics"), 5);
    expertise_areas.set(String::from_str(&env, "Physics"), 3);
    contract_client.update_expertise_areas(&caller, &user_id, &expertise_areas);

    let retrieved_areas = contract_client.get_expertise_areas(&user_id);
    assert_eq!(retrieved_areas.len(), 2);
    assert_eq!(
        retrieved_areas
            .get(String::from_str(&env, "Mathematics"))
            .unwrap(),
        5
    );
    assert_eq!(
        retrieved_areas
            .get(String::from_str(&env, "Physics"))
            .unwrap(),
        3
    );
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_update_expertise_areas_non_existent_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let expertise_areas = Map::new(&env);
    contract_client.update_expertise_areas(&caller, &999, &expertise_areas);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #3)")]
fn test_reverify_verified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
}

#[test]
fn test_verify_content() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "Mathematics"), 5);
    contract_client.update_expertise_areas(&caller, &user_id, &expertise_areas);

    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));

    assert!(true, "Content verification completed without errors");
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_verify_content_unverified_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #4)")]
fn test_verify_content_no_expertise() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.mint_credential_token(&caller, &user_id);
    contract_client.verify_content(&caller, &user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")]
fn test_get_reputation_non_existent() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.get_reputation(&user_id, &String::from_str(&env, "Mathematics"));
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_get_expertise_areas_non_existent_user() {
    let env = Env::default();
    let _caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    contract_client.get_expertise_areas(&999);
}

#[test]
fn test_verify_user_success() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();

    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    let user = contract_client.get_user(&user_id);
    assert!(user.verified, "User should be verified");
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #5)")]
fn test_verify_user_rejection() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();

    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));

    // Attempt to verify without valid or empty details
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, ""));
}

#[test]
fn test_multiple_tokens_mint_same_user() {
    let env = Env::default();
    let caller = Address::generate(&env);

    let contract_address = env.register(ContributorReputation, ());
    let contract_client = ContributorReputationClient::new(&env, &contract_address);

    env.mock_all_auths();
    let user_id = contract_client.initialize_user(&caller, &String::from_str(&env, "Alice"));
    contract_client.verify_user(&caller, &user_id, &String::from_str(&env, "Valid details"));
    contract_client.mint_credential_token(&caller, &user_id);

    // Attempt to remint the token for the same user
    contract_client.mint_credential_token(&caller, &user_id);
}

// Tests from test_recovery_analytics.rs
#[test]
#[allow(deprecated)]
fn test_dispute_resolution_flow() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);
    let reviewer = Address::generate(&env);

    env.mock_all_auths();

    // Initialize users first
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "test_user"));
    let reviewer_id =
        client.initialize_user(&reviewer, &String::from_str(&env, "test_reviewer"));

    // Then verify users
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));
    client.verify_user(&admin, &reviewer_id, &String::from_str(&env, "verified"));

    // First update reputation to create data
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &75u32);

    // Submit a dispute
    let dispute_id = client.submit_dispute(
        &user,
        &user_id,
        &String::from_str(&env, "math"),
        &75u32,
        &String::from_str(&env, "Unfair reputation reduction"),
    );

    // Verify dispute was created
    let dispute = client.get_dispute(&dispute_id);
    assert_eq!(dispute.user_id, user_id);
    assert_eq!(dispute.subject, String::from_str(&env, "math"));

    // Resolve the dispute
    client.resolve_dispute(
        &admin,
        &dispute_id,
        &true,
        &String::from_str(&env, "admin_resolver"),
    );

    // Verify dispute resolution
    let resolved_dispute = client.get_dispute(&dispute_id);
    assert!(matches!(
        resolved_dispute.status,
        crate::types::DisputeStatus::Approved
    ));
}

#[test]
#[allow(deprecated)]
fn test_recovery_plan_creation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify user
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "user"));
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));

    // Set up expertise areas first with low reputation to be eligible for recovery
    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "math"), 40u32);
    client.update_expertise_areas(&admin, &user_id, &expertise_areas);

    // First update reputation to create data (low score to be eligible for recovery)
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &40u32);

    // Create recovery plan with milestones
    let mut milestones = Map::new(&env);
    milestones.set(String::from_str(&env, "math"), 85u32);
    client.create_recovery_plan(&admin, &user_id, &90u32, &milestones, &30u32);

    // Get recovery plan
    let plan = client.get_recovery_plan(&user_id);
    assert_eq!(plan.user_id, user_id);
    assert_eq!(plan.target_score, 90u32);
    assert_eq!(plan.completed, false);
}

#[test]
#[allow(deprecated)]
fn test_probation_system() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify user
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "user"));
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));

    // User should not be on probation initially
    assert_eq!(client.is_on_probation(&user_id), false);

    // Set user on probation with restrictions
    let mut restrictions = Map::new(&env);
    restrictions.set(String::from_str(&env, "posting"), false);
    client.set_probation(
        &admin,
        &user_id,
        &30u32,
        &String::from_str(&env, "violation"),
        &restrictions,
    );

    // User should now be on probation
    assert_eq!(client.is_on_probation(&user_id), true);
}

#[test]
#[allow(deprecated)]
fn test_analytics_generation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify user
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "user"));
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));

    // Set up expertise areas first
    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "math"), 85u32);
    client.update_expertise_areas(&admin, &user_id, &expertise_areas);

    // Update user reputation to create some data
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &85u32);

    // Generate user analytics
    let analytics = client.generate_user_analytics(&user_id, &30u32);
    assert!(analytics.data.len() > 0);

    // Generate platform analytics
    let platform_analytics = client.calculate_platform_analytics();
    assert!(platform_analytics.data.len() > 0);
}

#[test]
#[allow(deprecated)]
fn test_domain_expertise_mapping() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify user
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "user"));
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));

    // Set up expertise areas first
    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "math"), 85u32);
    expertise_areas.set(String::from_str(&env, "science"), 90u32);
    client.update_expertise_areas(&admin, &user_id, &expertise_areas);

    // Update reputation in multiple domains
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &85u32);
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "science"), &90u32);

    // Generate domain expertise
    let domain_expertise = client.generate_domain_expertise(&String::from_str(&env, "math"));
    assert!(domain_expertise.total_contributors >= 1);
    assert!(domain_expertise.average_score > 0);
}

#[test]
#[allow(deprecated)]
fn test_reputation_trends() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify user
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user_id = client.initialize_user(&user, &String::from_str(&env, "user"));
    client.verify_user(&admin, &user_id, &String::from_str(&env, "verified"));

    // Set up expertise areas first
    let mut expertise_areas = Map::new(&env);
    expertise_areas.set(String::from_str(&env, "math"), 75u32);
    client.update_expertise_areas(&admin, &user_id, &expertise_areas);

    // Update reputation multiple times to create trend data
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &75u32);
    
    // Advance time to ensure different timestamps
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 86400; // Add 1 day
    });
    
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &80u32);
    
    // Advance time again
    env.ledger().with_mut(|li| {
        li.timestamp = li.timestamp + 86400; // Add another day
    });
    
    client.update_reputation(&admin, &user_id, &String::from_str(&env, "math"), &85u32);

    // Get reputation trends - now that we have multiple reputation updates, this should work
    let trends = client.get_reputation_trends(&user_id, &String::from_str(&env, "math"), &7u32);
    assert!(trends.len() > 0);

    // Predict reputation development - this also needs history data
    let prediction = client.predict_reputation_development(&user_id, &String::from_str(&env, "math"), &30u32);
    assert!(prediction > 0);

    // Test that the user has the expected reputation
    let expertise = client.get_expertise_areas(&user_id);
    assert!(expertise.len() > 0);
    
    // Verify the final reputation score
    let final_reputation = client.get_reputation(&user_id, &String::from_str(&env, "math"));
    assert_eq!(final_reputation, 85u32);
}

#[test]
#[allow(deprecated)]
fn test_platform_analytics() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ContributorReputation);
    let client = ContributorReputationClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    env.mock_all_auths();

    // Initialize and verify multiple users
    let _admin_id = client.initialize_user(&admin, &String::from_str(&env, "admin"));
    let user1_id = client.initialize_user(&user1, &String::from_str(&env, "user1"));
    let user2_id = client.initialize_user(&user2, &String::from_str(&env, "user2"));
    client.verify_user(&admin, &user1_id, &String::from_str(&env, "verified"));
    client.verify_user(&admin, &user2_id, &String::from_str(&env, "verified"));

    // Update reputations to create platform data
    client.update_reputation(&admin, &user1_id, &String::from_str(&env, "math"), &85u32);
    client.update_reputation(
        &admin,
        &user2_id,
        &String::from_str(&env, "science"),
        &90u32,
    );
    client.update_reputation(&admin, &user2_id, &String::from_str(&env, "math"), &80u32);

    // Calculate platform analytics
    let platform_analytics = client.calculate_platform_analytics();
    assert!(platform_analytics.data.len() > 0);

    // Set up expertise areas for both users
    let mut user1_expertise = Map::new(&env);
    user1_expertise.set(String::from_str(&env, "math"), 85u32);
    client.update_expertise_areas(&admin, &user1_id, &user1_expertise);

    let mut user2_expertise = Map::new(&env);
    user2_expertise.set(String::from_str(&env, "science"), 90u32);
    user2_expertise.set(String::from_str(&env, "math"), 80u32);
    client.update_expertise_areas(&admin, &user2_id, &user2_expertise);

    // Generate peer benchmark (need multiple users in same domain)
    let benchmark = client.generate_peer_benchmark(&user1_id, &String::from_str(&env, "math"));
    assert!(benchmark.rank > 0);
}

// Tests from test_remaining_functions.rs
fn create_test_env() -> Env {
    Env::default()
}

fn create_test_user(env: &Env, id: u64, name: &str) -> User {
    User {
        id,
        name: String::from_str(env, name),
        verified: false,
        expertise_areas: Map::new(env),
    }
}

fn create_test_reputation(env: &Env, user_id: u64, subject: &str, score: u32) -> Reputation {
    Reputation {
        user_id,
        subject: String::from_str(env, subject),
        score,
    }
}

fn create_test_credential(env: &Env, token_id: u64, user_id: u64) -> CredentialToken {
    CredentialToken {
        token_id,
        user_id,
        issued_at: env.ledger().timestamp(),
    }
}

fn create_test_dispute(env: &Env, dispute_id: u64, user_id: u64) -> Dispute {
    Dispute {
        id: dispute_id,
        user_id,
        subject: String::from_str(env, "test_subject"),
        original_score: 80,
        disputed_score: 60,
        evidence: String::from_str(env, "test_evidence"),
        status: DisputeStatus::Pending,
        created_at: env.ledger().timestamp(),
        resolved_at: None,
        resolver: None,
    }
}

fn create_test_recovery_plan(env: &Env, user_id: u64) -> RecoveryPlan {
    RecoveryPlan {
        user_id,
        target_score: 75,
        milestones: Map::new(env),
        created_at: env.ledger().timestamp(),
        deadline: env.ledger().timestamp() + 30 * 86400,
        progress: Map::new(env),
        completed: false,
    }
}

#[test]
fn test_get_reputation_with_history() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create and store a user
        let user = create_test_user(&env, 1, "Alice");
        store_user(&env, &user);
        
        // Create and store reputation entries
        let rep1 = create_test_reputation(&env, 1, "math", 80);
        let rep2 = create_test_reputation(&env, 1, "science", 75);
        store_reputation(&env, &rep1);
        store_reputation(&env, &rep2);
        
        // Test getting reputation with history
        let result = get_reputation_with_history(env.clone(), 1, String::from_str(&env, "math"));
        assert!(result.is_ok());
        let (score, history) = result.unwrap();
        assert_eq!(score, 80);
        assert_eq!(history.user_id, 1);
    });
}

#[test]
fn test_calculate_reputation_change() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Test reputation change calculation
        let user_id = 1u64;
        let subject = String::from_str(&env, "math");
        let change = calculate_reputation_change(env.clone(), user_id, subject, 1).unwrap();
        assert_eq!(change, 0); // No history means no change
    });
}

#[test]
fn test_increment_user_id() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Test user ID increment
        let id1 = increment_user_id(&env);
        let id2 = increment_user_id(&env);
        assert_eq!(id2, id1 + 1);
    });
}

#[test]
fn test_get_next_token_id() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Test getting next token ID
        let token_id = get_next_token_id(&env);
        assert!(token_id > 0);
    });
}

#[test]
fn test_increment_token_id() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Test token ID increment
        let id1 = increment_token_id(&env);
        let id2 = increment_token_id(&env);
        assert_eq!(id2, id1 + 1);
    });
}

#[test]
fn test_increment_dispute_id() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Test dispute ID increment
        let id1 = increment_dispute_id(&env);
        let id2 = increment_dispute_id(&env);
        assert_eq!(id2, id1 + 1);
    });
}

#[test]
fn test_store_and_get_credential() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create and store credential
        let credential = create_test_credential(&env, 1, 1);
        store_credential(&env, &credential);
        
        // Retrieve and verify credential
        let retrieved = get_credential(&env, 1).unwrap();
        assert_eq!(retrieved.token_id, credential.token_id);
        assert_eq!(retrieved.user_id, credential.user_id);
    });
}

#[test]
fn test_get_credential_not_found() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Try to get non-existent credential
        let result = get_credential(&env, 999);
        assert!(result.is_err());
    });
}

#[test]
fn test_get_dispute() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create and store dispute
        let dispute = create_test_dispute(&env, 1, 1);
        store_dispute(&env, &dispute);
        
        // Retrieve and verify dispute
        let retrieved = get_dispute(&env, 1).unwrap();
        assert_eq!(retrieved.id, dispute.id);
        assert_eq!(retrieved.user_id, dispute.user_id);
    });
}

#[test]
fn test_get_dispute_not_found() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Try to get non-existent dispute
        let result = get_dispute(&env, 999);
        assert!(result.is_err());
    });
}

#[test]
fn test_store_and_get_user_disputes() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create dispute IDs vector
        let mut dispute_ids = Vec::new(&env);
        dispute_ids.push_back(1);
        dispute_ids.push_back(2);
        dispute_ids.push_back(3);
        
        // Store user disputes
        store_user_disputes(&env, 1, &dispute_ids);
        
        // Retrieve and verify
        let retrieved = get_user_disputes(&env, 1);
        assert_eq!(retrieved.len(), 3);
        assert!(retrieved.contains(&1));
        assert!(retrieved.contains(&2));
        assert!(retrieved.contains(&3));
    });
}

#[test]
fn test_get_user_disputes_empty() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Get disputes for user with no disputes
        let disputes = get_user_disputes(&env, 999);
        assert_eq!(disputes.len(), 0);
    });
}

#[test]
fn test_store_and_get_recovery_plan() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create and store recovery plan
        let plan = create_test_recovery_plan(&env, 1);
        store_recovery_plan(&env, &plan);
        
        // Retrieve and verify
        let retrieved = get_recovery_plan(&env, 1).unwrap();
        assert_eq!(retrieved.user_id, plan.user_id);
        assert_eq!(retrieved.target_score, plan.target_score);
        assert_eq!(retrieved.created_at, plan.created_at);
    });
}

#[test]
fn test_get_recovery_plan_not_found() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Try to get non-existent recovery plan
        let result = get_recovery_plan(&env, 999);
        assert!(result.is_err());
    });
}

// Tests from test_utility_functions.rs
fn create_test_analytics(env: &Env, key: &str) -> Analytics {
    Analytics {
        key: String::from_str(env, key),
        data: Map::new(env),
        trends: Map::new(env),
        last_updated: env.ledger().timestamp(),
    }
}

#[test]
fn test_get_analytics() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        let analytics = create_test_analytics(&env, "test_key");
        
        // Store analytics
        store_analytics(&env, &analytics);
        
        // Test get_analytics
        let retrieved = get_analytics(&env, String::from_str(&env, "test_key"));
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().key, analytics.key);
        
        // Test non-existent key
        let non_existent = get_analytics(&env, String::from_str(&env, "non_existent"));
        assert!(non_existent.is_none());
    });
}

#[test]
fn test_user_exists() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        let user = create_test_user(&env, 1, "Alice");
        
        // User should not exist initially
        assert!(!user_exists(&env, 1));
        
        // Store user
        store_user(&env, &user);
        
        // User should exist now
        assert!(user_exists(&env, 1));
        
        // Non-existent user should not exist
        assert!(!user_exists(&env, 999));
    });
}

#[test]
fn test_reputation_exists() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        let reputation = create_test_reputation(&env, 1, "math", 85);
        
        // Reputation should not exist initially
        assert!(!reputation_exists(&env, 1, String::from_str(&env, "math")));
        
        // Store reputation
        store_reputation(&env, &reputation);
        
        // Reputation should exist now
        assert!(reputation_exists(&env, 1, String::from_str(&env, "math")));
        
        // Non-existent reputation should not exist
        assert!(!reputation_exists(&env, 1, String::from_str(&env, "science")));
        assert!(!reputation_exists(&env, 999, String::from_str(&env, "math")));
    });
}

#[test]
fn test_dispute_exists() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        let dispute = create_test_dispute(&env, 1, 1);
        
        // Dispute should not exist initially
        assert!(!dispute_exists(&env, 1));
        
        // Store dispute
        store_dispute(&env, &dispute);
        
        // Dispute should exist now
        assert!(dispute_exists(&env, 1));
        
        // Non-existent dispute should not exist
        assert!(!dispute_exists(&env, 999));
    });
}

#[test]
fn test_get_all_user_ids() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Initially should be empty
        let user_ids = get_all_user_ids(&env);
        assert_eq!(user_ids.len(), 0);
        
        // Create and store multiple users
        let user1 = create_test_user(&env, 1, "Alice");
        let user2 = create_test_user(&env, 2, "Bob");
        let user3 = create_test_user(&env, 3, "Charlie");
        
        store_user(&env, &user1);
        store_user(&env, &user2);
        store_user(&env, &user3);
        
        // Update next user ID to simulate proper ID generation
        env.storage().instance().set(&DataKey::NextUserId, &4u64);
        
        // Get all user IDs
        let user_ids = get_all_user_ids(&env);
        assert_eq!(user_ids.len(), 3);
        assert!(user_ids.contains(&1));
        assert!(user_ids.contains(&2));
        assert!(user_ids.contains(&3));
    });
}

#[test]
fn test_get_all_dispute_ids() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Initially should be empty
        let dispute_ids = get_all_dispute_ids(&env);
        assert_eq!(dispute_ids.len(), 0);
        
        // Create and store multiple disputes
        let dispute1 = create_test_dispute(&env, 1, 1);
        let dispute2 = create_test_dispute(&env, 2, 2);
        let dispute3 = create_test_dispute(&env, 3, 1);
        
        store_dispute(&env, &dispute1);
        store_dispute(&env, &dispute2);
        store_dispute(&env, &dispute3);
        
        // Update next dispute ID to simulate proper ID generation
        env.storage().instance().set(&DataKey::NextDisputeId, &4u64);
        
        // Get all dispute IDs
        let dispute_ids = get_all_dispute_ids(&env);
        assert_eq!(dispute_ids.len(), 3);
        assert!(dispute_ids.contains(&1));
        assert!(dispute_ids.contains(&2));
        assert!(dispute_ids.contains(&3));
    });
}

#[test]
fn test_cleanup_expired_probations() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Set a specific timestamp to ensure predictable behavior
        env.ledger().set_timestamp(10000);
        let current_time = env.ledger().timestamp();
        
        // Create users
        let user1 = create_test_user(&env, 1, "Alice");
        let user2 = create_test_user(&env, 2, "Bob");
        store_user(&env, &user1);
        store_user(&env, &user2);
        env.storage().instance().set(&DataKey::NextUserId, &3u64);
        
        // Create probation statuses - one expired, one active
        let expired_probation = ProbationStatus {
            user_id: 1,
            active: true,
            start_date: 1000,
            end_date: 5000, // Expired (current_time is 10000)
            reason: String::from_str(&env, "Test violation"),
            restrictions: Map::new(&env),
        };
        
        let active_probation = ProbationStatus {
            user_id: 2,
            active: true,
            start_date: current_time,
            end_date: current_time + 1000, // Still active
            reason: String::from_str(&env, "Another violation"),
            restrictions: Map::new(&env),
        };
        
        store_probation_status(&env, &expired_probation);
        store_probation_status(&env, &active_probation);
        
        // Run cleanup
        cleanup_expired_probations(&env);
        
        // Check results
        let user1_probation = get_probation_status(&env, 1);
        let user2_probation = get_probation_status(&env, 2);
        
        // User 1's probation should be deactivated
        assert!(!user1_probation.active);
        
        // User 2's probation should still be active
        assert!(user2_probation.active);
    });
}

#[test]
fn test_get_all_user_ids_with_gaps() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create users with gaps in IDs
        let user1 = create_test_user(&env, 1, "Alice");
        let user3 = create_test_user(&env, 3, "Charlie");
        let user5 = create_test_user(&env, 5, "Eve");
        
        store_user(&env, &user1);
        store_user(&env, &user3);
        store_user(&env, &user5);
        
        // Set next user ID to 6
        env.storage().instance().set(&DataKey::NextUserId, &6u64);
        
        // Get all user IDs
        let user_ids = get_all_user_ids(&env);
        assert_eq!(user_ids.len(), 3);
        assert!(user_ids.contains(&1));
        assert!(user_ids.contains(&3));
        assert!(user_ids.contains(&5));
        assert!(!user_ids.contains(&2));
        assert!(!user_ids.contains(&4));
    });
}

#[test]
fn test_get_all_dispute_ids_with_gaps() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create disputes with gaps in IDs
        let dispute1 = create_test_dispute(&env, 1, 1);
        let dispute3 = create_test_dispute(&env, 3, 2);
        let dispute5 = create_test_dispute(&env, 5, 1);
        
        store_dispute(&env, &dispute1);
        store_dispute(&env, &dispute3);
        store_dispute(&env, &dispute5);
        
        // Set next dispute ID to 6
        env.storage().instance().set(&DataKey::NextDisputeId, &6u64);
        
        // Get all dispute IDs
        let dispute_ids = get_all_dispute_ids(&env);
        assert_eq!(dispute_ids.len(), 3);
        assert!(dispute_ids.contains(&1));
        assert!(dispute_ids.contains(&3));
        assert!(dispute_ids.contains(&5));
        assert!(!dispute_ids.contains(&2));
        assert!(!dispute_ids.contains(&4));
    });
}

#[test]
fn test_cleanup_expired_probations_no_users() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Run cleanup with no users - should not panic
        cleanup_expired_probations(&env);
        
        // Should complete without issues
        assert!(true);
    });
}

#[test]
fn test_cleanup_expired_probations_no_active_probations() {
    let env = create_test_env();
    let contract_address = env.register(ContributorReputation, ());
    
    env.as_contract(&contract_address, || {
        // Create a user with no active probation
        let user = create_test_user(&env, 1, "Alice");
        store_user(&env, &user);
        env.storage().instance().set(&DataKey::NextUserId, &2u64);
        
        // Run cleanup - should not panic
        cleanup_expired_probations(&env);
        
        // Probation should remain inactive
        let probation = get_probation_status(&env, 1);
        assert!(!probation.active);
    });
}
