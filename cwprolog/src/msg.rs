use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub enum QueryMsg {
    Ask {
        q: String
    }
}

#[cw_serde]
pub struct AskResponse {
    pub answer: String
}
