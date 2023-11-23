//! # A Concordium V1 smart contract
use concordium_std::{*, collections::BTreeMap};
use core::fmt::Debug;

type VotingOption = String;
type VotingIndex = u32;

/// Your smart contract state.
#[derive(Serialize, SchemaType)]
pub struct State {
    pub description: String,
    pub options : Vec<VotingOption>,
    pub end_time: Timestamp,
    pub ballots: BTreeMap<AccountAddress, VotingIndex>,
}

#[derive(Serialize, SchemaType)]
pub struct InitParameter {
    pub description: String,
    pub options: Vec<VotingOption>,
    pub end_time: Timestamp,
}

/// Init function that creates a new smart contract.
#[init(contract = "voting", parameter = "InitParameter")]
fn init(ctx: &InitContext, _state_builder: &mut StateBuilder) -> InitResult<State> {
    
    let parameter: InitParameter = ctx.parameter_cursor().get()?;

    let state: State = State {
        description: parameter.description,
        options:parameter.options,
        end_time: parameter.end_time,
        ballots: BTreeMap::new(),
    };

    Ok(state)
}

/// Your smart contract errors.
#[derive(Debug, PartialEq, Eq, Reject, Serialize, SchemaType)]
pub enum Error {
    /// Failed parsing the parameter.
    #[from(ParseError)]
    ParseParams,
    /// Your error
    VotingFinished,
    ContractVoter,
    InvalidVotingOption,
}

#[receive(
    contract = "voting",
    name = "vote",
    parameter = "VotingOption",
    error = "Error",
    mutable
)]
fn vote(ctx: &ReceiveContext, host: &mut Host<State>) -> Result<(), Error> {
    
    if ctx.metadata().slot_time() > host.state().end_time {
        return Err(Error::VotingFinished);
    }

    let acc: AccountAddress = match ctx.sender() {
        Address::Account(acc: AccountAddress) => acc,
        Address::Contract(_) => return Err(Error::ContractVoter),
    };

    let new_vote: VotingOption = ctx.parameter_cursor().get()?;

    let new_vote_index = match host.state().options.iter().position(|o| *o == new_vote) {
        Some(position) => position as u32,
        None => return Err(Error::InvalidVotingOption),
    };

    host.state_mut().ballots.entry(acc)
        .and_modify(|old_vote| *old_vote = new_vote_index)
        .or_insert(new_vote_index);

    Ok(())
}

/// View function that returns the content of the state.
#[receive(contract = "voting", name = "view", return_value = "State")]
fn view<'b>(_ctx: &ReceiveContext, host: &'b Host<State>) -> ReceiveResult<&'b State> {
    Ok(host.state())
}
