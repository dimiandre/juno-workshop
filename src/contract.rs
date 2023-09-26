#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, CosmosMsg, coin};
use cw2::set_contract_version;
use anybuf::Anybuf;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg};
use crate::state::{Config, CONFIG};

const CONTRACT_NAME: &str = "crates.io:cw-ibc-example";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    CONFIG.save(deps.storage, &Config { val: 0 })?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => {
   
            let mut config = CONFIG.load(deps.storage)?;
            config.val += 1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new())
        }
    }
}

// sudo msg
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ClockEndBlock {} => {
            if env.block.height % 50 != 0 {
                return Ok(Response::new());
            }

            // Test with 1 ucosm
            let one_cosm: Coin = coin(10_000_000, "unice");

            // MsgDistributeTokens
            let proto = Anybuf::new()
                .append_string(1, &env.contract.address)
                .append_message(2, &Anybuf::new()
                    .append_string(1, &one_cosm.denom)
                    .append_string(2, &one_cosm.amount.to_string())
                )
                .into_vec();

            let msg = CosmosMsg::Stargate { 
                type_url: "/juno.drip.v1.MsgDistributeTokens".to_string(), 
                value: proto.into() 
            };
            
            let mut config = CONFIG.load(deps.storage)?;
            config.val += 1;
            CONFIG.save(deps.storage, &config)?;

            Ok(Response::new()
            .add_message(msg))
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<Config> {
    let count = CONFIG.load(deps.storage)?.val;
    Ok(Config { val: count })
}
