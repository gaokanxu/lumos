#![cfg(feature = "test-sbf")]

use {lumos_program::pubkey::Pubkey, lumos_program_test::*, lumos_sdk::signer::Signer};

mod program_test;

use {
    crate::program_test::args::RealmSetupArgs,
    program_test::*,
    lpl_governance::{
        error::GovernanceError,
        state::{realm::GoverningTokenConfigAccountArgs, realm_config::GoverningTokenType},
    },
};

#[tokio::test]
async fn test_set_realm_config() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert
    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert_eq!(realm_cookie.account, realm_account);
}

#[tokio::test]
async fn test_set_realm_config_with_authority_must_sign_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    // Act

    let err = governance_test
        .set_realm_config_using_instruction(
            &mut realm_cookie,
            &realm_setup_args,
            |i| i.accounts[1].is_signer = false,
            Some(&[]),
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceError::RealmAuthorityMustSign.into());
}

#[tokio::test]
async fn test_set_realm_config_with_no_authority_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    governance_test
        .set_realm_authority(&realm_cookie, None)
        .await
        .unwrap();

    // Act

    let err = governance_test
        .set_realm_config_using_instruction(
            &mut realm_cookie,
            &realm_setup_args,
            |i| i.accounts[1].is_signer = false,
            Some(&[]),
        )
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceError::RealmHasNoAuthority.into());
}

#[tokio::test]
async fn test_set_realm_config_with_invalid_authority_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    let realm_cookie2 = governance_test.with_realm().await;

    // Try to use authority from other realm
    realm_cookie.realm_authority = realm_cookie2.realm_authority;

    // Act

    let err = governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(err, GovernanceError::InvalidAuthorityForRealm.into());
}

#[tokio::test]
async fn test_set_realm_config_with_remove_council() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs {
        use_council_mint: false,
        ..Default::default()
    };

    // Act
    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert
    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert_eq!(realm_cookie.account, realm_account);
    assert_eq!(None, realm_account.config.council_mint);
}

#[tokio::test]
async fn test_set_realm_config_with_council_change_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    // Try to replace council mint
    realm_cookie.account.config.council_mint = serde::__private::Some(Pubkey::new_unique());

    // Act
    let err = governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        err,
        GovernanceError::RealmCouncilMintChangeIsNotSupported.into()
    );
}

#[tokio::test]
async fn test_set_realm_config_with_council_restore_error() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let mut realm_setup_args = RealmSetupArgs {
        use_council_mint: false,
        ..Default::default()
    };

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Try to restore council mint after removing it
    realm_setup_args.use_council_mint = true;
    realm_cookie.account.config.council_mint = serde::__private::Some(Pubkey::new_unique());

    // Act
    let err = governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        err,
        GovernanceError::RealmCouncilMintChangeIsNotSupported.into()
    );
}

#[tokio::test]
async fn test_set_realm_config_with_liquid_community_token_cannot_be_changed_to_memebership_error()
{
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let mut realm_setup_args = RealmSetupArgs::default();

    // Try to change Community token type to Membership
    realm_setup_args.community_token_config_args.token_type = GoverningTokenType::Membership;

    // Act
    let err = governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .err()
        .unwrap();

    // Assert
    assert_eq!(
        err,
        GovernanceError::CannotChangeCommunityTokenTypeToMembership.into()
    );
}

#[tokio::test]
async fn test_set_realm_config_for_community_token_config() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    // Change Community token type to Dormant and set plugins
    let realm_setup_args = RealmSetupArgs {
        community_token_config_args: GoverningTokenConfigAccountArgs {
            voter_weight_addin: Some(Pubkey::new_unique()),
            max_voter_weight_addin: Some(Pubkey::new_unique()),
            token_type: GoverningTokenType::Dormant,
        },
        ..Default::default()
    };

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_account = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(
        realm_config_account.community_token_config.token_type,
        GoverningTokenType::Dormant
    );

    assert_eq!(
        realm_config_account
            .community_token_config
            .voter_weight_addin,
        realm_setup_args
            .community_token_config_args
            .voter_weight_addin
    );

    assert_eq!(
        realm_config_account
            .community_token_config
            .max_voter_weight_addin,
        realm_setup_args
            .community_token_config_args
            .max_voter_weight_addin
    );
}

#[tokio::test]
async fn test_set_realm_config_for_council_token_config() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    // Change Council token type to Membership and set plugins
    let realm_setup_args = RealmSetupArgs {
        council_token_config_args: GoverningTokenConfigAccountArgs {
            voter_weight_addin: Some(Pubkey::new_unique()),
            max_voter_weight_addin: Some(Pubkey::new_unique()),
            token_type: GoverningTokenType::Membership,
        },
        ..Default::default()
    };

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert

    let realm_config_account = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(
        realm_config_account.council_token_config.token_type,
        GoverningTokenType::Membership
    );

    assert_eq!(
        realm_config_account.council_token_config.voter_weight_addin,
        realm_setup_args
            .council_token_config_args
            .voter_weight_addin
    );

    assert_eq!(
        realm_config_account
            .council_token_config
            .max_voter_weight_addin,
        realm_setup_args
            .council_token_config_args
            .max_voter_weight_addin
    );
}

#[tokio::test]
async fn test_set_realm_config_without_existing_realm_config() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let realm_setup_args = RealmSetupArgs::default();

    governance_test.remove_realm_config_account(&realm_cookie.realm_config.address);

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert
    let realm_account = governance_test
        .get_realm_account(&realm_cookie.address)
        .await;

    assert_eq!(realm_cookie.account, realm_account);
}

#[tokio::test]
async fn test_set_realm_config_with_token_owner_record_lock_authorities() {
    // Arrange
    let mut governance_test = GovernanceProgramTest::start_new().await;

    let mut realm_cookie = governance_test.with_realm().await;

    let community_token_owner_record_lock_authority_cookie = governance_test
        .with_community_token_owner_record_lock_authority(&realm_cookie)
        .await
        .unwrap();

    let council_token_owner_record_lock_authority_cookie = governance_test
        .with_council_token_owner_record_lock_authority(&realm_cookie)
        .await
        .unwrap();

    let realm_setup_args = RealmSetupArgs::default();

    // Act

    governance_test
        .set_realm_config(&mut realm_cookie, &realm_setup_args)
        .await
        .unwrap();

    // Assert
    let realm_config_account = governance_test
        .get_realm_config_account(&realm_cookie.realm_config.address)
        .await;

    assert_eq!(
        vec![community_token_owner_record_lock_authority_cookie
            .authority
            .pubkey()],
        realm_config_account.community_token_config.lock_authorities
    );

    assert_eq!(
        vec![council_token_owner_record_lock_authority_cookie
            .authority
            .pubkey()],
        realm_config_account.council_token_config.lock_authorities
    );
}
