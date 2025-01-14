use {
    crate::rpc_client_utils::MultiClient,
    serde::{Deserialize, Serialize},
    solana_sdk::pubkey::Pubkey,
    std::{
        collections::{HashMap, HashSet},
        error,
    },
};

#[derive(Debug, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum ValidatorStakeState {
    None,     // Validator should receive no stake
    Baseline, // Validator has earned the baseline stake level
    Bonus,    // Validator has earned the bonus stake level
}

impl Default for ValidatorStakeState {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ValidatorStake {
    pub identity: Pubkey,
    pub vote_address: Pubkey,
    pub stake_state: ValidatorStakeState,
    pub priority: bool,
}

pub type EpochStakeNotes = Vec<String>;
pub type ValidatorStakeActions = HashMap<Pubkey, String>;
pub type UnfundedValidators = HashSet<Pubkey>;

pub trait GenericStakePool {
    /// Fourth value in returned tuple is the calculated bonus stake amount
    fn apply(
        &mut self,
        client: &MultiClient,
        dry_run: bool,
        desired_validator_stake: &[ValidatorStake],
        bonus_multiplier: Option<f64>,
    ) -> Result<
        (
            EpochStakeNotes,
            ValidatorStakeActions,
            UnfundedValidators,
            u64, // bonus stake amount
        ),
        Box<dyn error::Error>,
    >;
}
