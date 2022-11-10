use std::collections::BinaryHeap;

use cosmwasm_std::{
    coin, entry_point, to_binary, Addr, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdError, StdResult, Uint128,
};

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, ResolveRecordResponse,
};
use crate::state::{
    config, config_read, resolver, resolver_read,
     Config, Record, OWNER, ADDRESSES
};

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, StdError> {
    let mut admin_addrs = Vec::new();
    for admin in msg.admins {
        admin_addrs.push(deps.api.addr_validate(&admin)?);
    }

    let mut registrar_addrs = Vec::new();
    for registrar in msg.registrar_addresses {
        registrar_addrs.push(deps.api.addr_validate(&registrar)?);
    }

    let config_state = Config {
        admins: admin_addrs,
        registrar_addresses: registrar_addrs,
    };
    config(deps.storage).save(&config_state)?;

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetRecord { user_name, owner, addresses } => execute_set_record(deps, env, info, user_name, owner, addresses),
    }
}


pub fn execute_set_record(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    user_name: String,
    owner: Addr,
    addresses: Vec<(i32, String)>,
) -> Result<Response, ContractError> {
    // Add admin check here
    let config_state = config(deps.storage).load()?;

    // check if the user_name is already registered
    let existing = OWNER.may_load(deps.storage, user_name.clone())?;
    match existing {
        Some(_) => Err(ContractError::UserAlreadyRegistered { name: user_name }),
        None => {
            // save the new record
          
            OWNER.save(deps.storage, user_name.clone(), &owner)?;
            Ok(Response::default())
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetResolver { name } => query_resolver(deps, env, name),
        QueryMsg::Config {} => to_binary(&config_read(deps.storage).load()?),
    }
}

fn query_resolver(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    let key = name.as_bytes();

    let address = match resolver_read(deps.storage).may_load(key)? {
        Some(record) => {
            Some(String::from(&record.resolver))
        }
        None => None,
    };
    let resp = ResolveRecordResponse { address };

    to_binary(&resp)
}


#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, DepsMut, Addr, coins, from_binary};

    use crate::msg::InstantiateMsg;

    use super::*;

    fn mock_init(
        deps: DepsMut,
        admins: Vec<String>,
        registrar_addrs: Vec<String>,
    ) {
        let msg = InstantiateMsg {
            admins: admins,
            registrar_addresses: registrar_addrs,
        };

        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps, mock_env(), info, msg)
        .expect("contract successfully handles InstantiateMsg");
    }

    fn assert_config_state(deps: Deps, expected: Config) {
        let res = query(deps, mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value, expected);
    }

    fn get_name_owner(deps: Deps, name: &str) -> String {
        let res = query(
            deps,
            mock_env(),
            QueryMsg::GetResolver  {
                name: name.to_string(),
            },
        )
        .unwrap();

        let value: ResolveRecordResponse = from_binary(&res).unwrap();
        value.address.unwrap()
    }

    fn change_admin_string_to_vec(deps: DepsMut, admins: Vec<String>) -> Vec<Addr>{
        let mut admin_addr = Vec::new();
        for admin in admins {
            admin_addr.push(deps.api.addr_validate(&admin).unwrap());
        }
        admin_addr
    }

    fn mock_register_resolver_for_alice(deps: DepsMut, sent: &[Coin], resolver: String) {
        // alice can register an available name
        let info = mock_info("alice_key", sent);
        let msg = ExecuteMsg::SetResolver {
            name: "alice".to_string(),
            resolver_addr: resolver.to_string(),
        };
        let _res = execute(deps, mock_env(), info, msg)
            .expect("contract successfully handles Register message");
    }

    #[test]
    fn proper_init_with_fees() {
        let mut deps = mock_dependencies();

        let admins = vec![String::from("test_admin")];
        let registrar_addrs = vec![];
        mock_init(deps.as_mut(), admins, registrar_addrs);

        let registrar_addrs = vec![];

        let admins = vec![String::from("test_admin")];
        let exp = change_admin_string_to_vec(deps.as_mut(), admins);

        assert_config_state(
            deps.as_ref(),
            Config { admins: exp, registrar_addresses: registrar_addrs },
        );

        mock_register_resolver_for_alice(deps.as_mut(), &coins(2, "token"), String::from("test_resolver"));

        let registered_resolver = get_name_owner(deps.as_ref(), "alice");

        assert_ne!(registered_resolver, String::from("invalid_resolvera"));
        assert_eq!(registered_resolver, String::from("test_resolver"));
    }
}
