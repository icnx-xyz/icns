#![cfg(test)]

use crate::{
    msg::{QueryMsg, GetAddressesResponse},
    msg::{AdminResponse, ExecuteMsg},
    ContractError, contract::is_admin,
};

use cosmwasm_std::{Addr, Empty, StdResult};
use cw_multi_test::{BasicApp, Executor};
use icns_name_nft::{msg::ExecuteMsg as NameExecuteMsg, msg::QueryMsg as NameQueryMsg};
use cw721_base::{ExecuteMsg as CW721BaseExecuteMsg, Extension, MintMsg};


use super::helpers::{
    instantiate_name_nft, instantiate_resolver_with_name_nft, TestEnv,
    TestEnvBuilder,
};

#[test]
fn set_get_single_record() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    let addresses = |app: &BasicApp, name: String| -> StdResult<_> {
        let GetAddressesResponse { addresses, .. } = app.wrap().query_wasm_smart(
            resolver_contract_addr.clone(),
            &QueryMsg::GetAddresses {
                user_name: name,
            },
        )?;

        Ok(addresses)
    };

    // mint name nft to bob
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: "bob".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    ).is_err();
    assert_eq!(mint, false);

    // now set record
    app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                    ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
                ],
            }, 
        &[],
    ).unwrap();

    // now get record
    let addresses = addresses(&app, "bob".to_string()).unwrap();
    assert_eq!(addresses.len(), 2);
    assert_eq!(addresses[0].0, "cosmos");
    assert_eq!(addresses[0].1, "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe");
    assert_eq!(addresses[1].0, "juno");
    assert_eq!(addresses[1].1, "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts");
}

#[test]
fn set_duplicate_username() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // mint name nft to bob
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: "bob".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    ).is_err();
    assert_eq!(mint, false);

    // now set record
    app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                    ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
                ],
            }, 
        &[],
    ).unwrap();

    // now set record again, this should error
    let err = app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                    ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
                ],
            },
            &[],
        ).is_err();

    assert_eq!(err, true);
}

#[test]
fn bech32_verification() {
    let admin1 = String::from("admin1");
    let admin2 = String::from("admin2");
    let admins = vec![admin1.clone(), admin2.clone()];
    let registrar = String::from("default-registrar");

    // first instantiate name nft
    let (name_nft_contract, mut app) = instantiate_name_nft(admins, registrar.clone());

    // now instantiate resolver using name nft contract
    let resolver_contract_addr =
        instantiate_resolver_with_name_nft(&mut app, name_nft_contract.clone());

    // mint name nft to bob
    let mint = app.execute_contract(
        Addr::unchecked(registrar.clone()),
        name_nft_contract.clone(),
        &NameExecuteMsg::CW721Base(CW721BaseExecuteMsg::<Extension, Empty>::Mint(MintMsg {
            token_id: "bob".to_string(),
            owner: "bob".to_string(),
            token_uri: None,
            extension: None,
        })),
        &[],
    ).is_err();
    assert_eq!(mint, false);

    // now set record, first try setting invalid bech32 address
    let err = app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("cosmos".to_string(), "cosmos1dsfsfasdfknsfkndfknskdfns".to_string()),
                ],
            },
            &[],
        ).is_err();
    assert_eq!(err, true);

    // now try setting record with unmatching bech32 prefix and address
    let err = app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("cosmos".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                ],
            },
            &[],
        ).is_err();
    assert_eq!(err, true);

    // now set record with valid bech32 prefix and addresses, this should succeed
    let err = app.execute_contract(
        Addr::unchecked(admin1.clone()),
        resolver_contract_addr.clone(),
        &ExecuteMsg::SetRecord {
                user_name: "bob".to_string(),
                addresses: vec![
                    ("juno".to_string(), "juno1kn27c8fu9qjmcn9hqytdzlml55mcs7dl2wu2ts".to_string()),
                    ("cosmos".to_string(), "cosmos1gf3dm2mvqhymts6ksrstlyuu2m8pw6dhv43wpe".to_string()),
                ],
            }, 
        &[],
    ).is_err();
    assert_eq!(err, false);

}