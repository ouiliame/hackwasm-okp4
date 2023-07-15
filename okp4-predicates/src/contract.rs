#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, LogicCommand, QueryMsg, RunPredicateResponse, Term};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:okp4-predicates";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::PrologExtensionManifest {} => {
            unimplemented!()
        }
        QueryMsg::RunPredicate { name, args } => run_predicate(deps, env, name, args),
    }
}

fn run_predicate(deps: Deps, env: Env, name: String, args: Vec<Term>) -> StdResult<Binary> {
    match name.as_str() {
        "chain_id/1" => {
            let chain_id = env.block.chain_id;
            Ok(to_binary(&RunPredicateResponse {
                error: None,
                result: vec![LogicCommand::Unify(
                    args[0].clone(),
                    Term::Atom(chain_id.to_string()),
                )],
            })?)
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod tests {}
