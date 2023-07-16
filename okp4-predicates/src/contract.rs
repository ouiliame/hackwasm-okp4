#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, LogicCommand, PredicateManifest, PrologExtensionManifestResponse,
    QueryMsg, RunPredicateResponse, Term,
};

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
        QueryMsg::PrologExtensionManifest {} => run_prolog_extension_manifest(deps, env),
        QueryMsg::RunPredicate { name, args } => run_predicate(deps, env, name, args),
    }
}

fn run_prolog_extension_manifest(deps: Deps, env: Env) -> StdResult<Binary> {
    let mut predicates = Vec::new();
    predicates.push(PredicateManifest {
        address: env.contract.address.to_string(),
        name: "chain_id/1".to_string(),
        cost: 0,
    });
    Ok(to_binary(&PrologExtensionManifestResponse { predicates })?)
}

fn run_predicate(deps: Deps, env: Env, name: String, args: Vec<Term>) -> StdResult<Binary> {
    match name.as_str() {
        "chain_id/1" => {
            let chain_id = env.block.chain_id;
            Ok(to_binary(&RunPredicateResponse {
                commands: vec![LogicCommand::Unify(
                    args[0].clone(),
                    Term::Atom(chain_id.to_string()),
                )],
            })?)
        }
        _ => Err(StdError::generic_err("unknown predicate")),
    }
}

#[cfg(test)]
mod tests {}
