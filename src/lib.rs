use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MinistryState {
    pub ministry_id: u32,
    pub ministry_name: String,
    pub project_name: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditRecord {
    pub record_id: String,
    pub timestamp: u64,
    pub state_hash: String,
    pub verified: bool,
}

pub fn get_metadata() -> MinistryState {
    MinistryState {
        ministry_id: 7,
        ministry_name: "Ministry of Commerce and Industry".to_string(),
        project_name: "TradeBridge AI".to_string(),
        strategy: "Programmatic Utility ($TRADE) -- B2B Liquidity and Factoring Settlement".to_string(),
        target_chain: "Internet Computer (ICP) + Base L2".to_string(),
        is_active: true,
    }
}
