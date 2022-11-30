use cosmwasm_std::{Addr, Deps};

use crate::{error::ContractError, state::CONFIG};

pub fn check_transferrable(deps: Deps) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if !config.transferrable {
        return Err(cw721_base::ContractError::Unauthorized {}.into());
    }

    Ok(())
}

pub fn check_admin(deps: Deps, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(deps.storage)?;

    for admin in config.admins {
        if admin == *sender {
            return Ok(());
        }
    }

    Err(cw721_base::ContractError::Unauthorized {}.into())
}

pub fn validate_name(name: &str) -> Result<(), ContractError> {
    if name.contains('.') {
        return Err(ContractError::InvalidName {});
    }

    Ok(())
}