use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {}

#[cw_serde]
pub enum QueryMsg {
    PrologExtensionManifest {},
    RunPredicate { name: String, args: Vec<Term> },
}

#[cw_serde]
pub struct RunPredicateResponse {
    pub error: Option<String>,
    pub result: Vec<LogicCommand>,
}
#[cw_serde]
pub enum Term {
    Var(String),
    Atom(String),
}

#[cw_serde]
pub enum LogicCommand {
    Unify(Term, Term),
}

// ChainId {},
// BlockHeight {},
// BlockTime {},
// BankBalances {},
// BankSpendableBalances {},
// BankLockedBalances {},
// DidComponents {},
// ShaHash {},
// HexBytes {},
// Bech32Address {},
// SourceFile {},
// JsonProlog {},
// UriEncoded {},
// ReadString {},
