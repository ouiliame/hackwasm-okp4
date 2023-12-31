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
pub struct PrologExtensionManifestResponse {
    pub predicates: Vec<PredicateManifest>,
}

#[cw_serde]
pub struct PredicateManifest {
    pub address: String,
    pub name: String,
    pub cost: u64,
}

#[cw_serde]
pub struct RunPredicateResponse {
    pub commands: Vec<LogicCommand>,
}

#[cw_serde]
pub enum Term {
    Var(i64),
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
