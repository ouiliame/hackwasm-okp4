#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{AskResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use logru::{solver::query_dfs, textual::TextualUniverse};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cwprolog";
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
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Ask { q } => {
            let mut u = TextualUniverse::new();
            let mut solns: Vec<String> = vec![];

            u.load_str(
                r#"
    is_natural(z).
    is_natural(s($0)) :- is_natural($0).

    add($0, z, $0) :- is_natural($0).
    add($0, s($1), s($2)) :- add($0, $1, $2).

    mul($0, z, z) :- is_natural($0).
    mul($0, s($1), $2) :- mul($0,$1,$3), add($0,$3,$2).

    "#,
            )
                .unwrap();

            let query = u.prepare_query("mul($0,$0,$1).").unwrap();
            let solutions = query_dfs(u.inner(), &query);

            for solution in solutions.take(10) {
                for (index, var) in solution.into_iter().enumerate() {
                    if let Some(term) = var {
                        solns.push(format!("${} = {}", index, u.pretty().term_to_string(&term)));
                    } else {
                        solns.push(format!("${} = <no solution>", index));
                    }
                }
            }

            let answer = solns.iter().fold(String::new(), |acc, x| acc + x + "\n");

            let r = AskResponse { answer };
            Ok(to_binary(&r)?)
        }
    }
}

#[cfg(test)]
mod tests {}
