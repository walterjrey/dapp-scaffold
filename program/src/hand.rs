extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate solana_sdk;

//mod dashboard;
//mod error;
//mod game;
//mod program_command;
//mod program_state;
mod simple_serde;

use crate::utils::{spl_token_transfer};
use crate::error::SolanaPokerError;

use crate::PREFIX;

use program_command::Command;
use program_state::State;
use simple_serde::SimpleSerde;
use solana_sdk::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    info,
    program_error::{PrintProgramError, ProgramError},
    program_utils::next_account_info,
    pubkey::Pubkey
};
use spl_token::state::Account;

///TokenTransferParams
pub struct TokenTransferParams<'a: 'b, 'b> {
    /// source
    pub source: AccountInfo<'a>,
    /// destination
    pub destination: AccountInfo<'a>,
    /// amount
    pub amount: u64,
    /// authority
    pub authority: AccountInfo<'a>,
    /// token_program
    pub token_program: AccountInfo<'a>,
}

struct Accounts<'a, 'b: 'a> {
    auction: &'a AccountInfo<'b>,
    auction_extended: &'a AccountInfo<'b>,
    bidder_meta: &'a AccountInfo<'b>,
    bidder_pot: &'a AccountInfo<'b>,
    bidder_pot_token: &'a AccountInfo<'b>,
    bidder: &'a AccountInfo<'b>,
    bidder_token: &'a AccountInfo<'b>,
    clock_sysvar: &'a AccountInfo<'b>,
    mint: &'a AccountInfo<'b>,
    payer: &'a AccountInfo<'b>,
    rent: &'a AccountInfo<'b>,
    system: &'a AccountInfo<'b>,
    token_program: &'a AccountInfo<'b>,
    transfer_authority: &'a AccountInfo<'b>,
}

fn parse_accounts<'a, 'b: 'a>(
    program_id: &Pubkey,
    accounts: &'a [AccountInfo<'b>],
) -> Result<Accounts<'a, 'b>, ProgramError> {
    let account_iter = &mut accounts.iter();
    let accounts = Accounts {
        player: next_account_info(account_iter)?,
        transfer_authority: next_account_info(account_iter)?,
        payer: next_account_info(account_iter)?,
        rent: next_account_info(account_iter)?,
        system: next_account_info(account_iter)?,
        token_program: next_account_info(account_iter)?,
    };

    Ok(accounts)
}

#[inline(always)]
pub fn spl_token_transfer(params: TokenTransferParams<'_, '_>) -> ProgramResult {
    let TokenTransferParams {
        source,
        destination,
        authority,
        token_program,
        amount,
    } = params;

    let result = invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            source.key,
            destination.key,
            authority.key,
            &[authority.key],
            amount,
        )
        .unwrap(),
        &[
            token_program_id,
            program_authority_account,
            source_account,
            destination_account,
        ],
        signers,
    )

    result.map_err(|_| SolanaPokerError::TokenTransferFailed.into())
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum HandState {
    WaitingBigBlind,
    WaitingSmallBlind,
    WaitingPlayer,
    Finished
}

impl Default for HandState {
    fn default() -> HandState {
        HandState::WaitingBigBlind
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Hand {
    players: Vec<PokerPlayer>,//ready????
    turn_index: u8,
    big_blind_position: u8,
    big_blind_address: Pubkey,
    small_blind_position: u8,
    small_blind_address: Pubkey,
    pub hand_state: HandState,
    pub pot: u64
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameState {
    WaitingSmallBlind,
    WaitingBigBlind,
    WaitingPlayerBet,
    Finished,
    Ready,
    Step,//turn, river, flop, preflop
    Started
}

impl Default for GameState {
    fn default() -> GameState {
        GameState::Ready
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlayerStatus {
    Playing,
    Out
}

impl Default for PlayerStatus {
    fn default() -> PlayerStatus {
        PlayerStatus::Out
    }
}

pub struct PokerPlayer {
    address: Pubkey,
    status: PlayerStatus,
    last_activity: u64
}

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    small_blind: u64,
    big_blind: u64,
    max_players: u8,
    players: Vec<PokerPlayer>,//ready????
    creator: Pubkey,
    big_blind_position: u8,
    big_blind_address: Pubkey,
    small_blind_position: u8,
    small_blind_address: Pubkey,
    pub game_state: GameState,
    current_hand: Hand
}

impl Hand {
    pub fn call_big_blind(self: &mut Room, program_id: &Pubkey, accounts: &'r [AccountInfo<'b>]) -> ProgramResult {

        let accounts = parse_accounts(program_id, accounts)?;
        
        spl_token_transfer(TokenTransferParams {
            source: accounts.player.clone(),
            destination: accounts.program_id.clone(),
            authority: accounts.transfer_authority.clone(),
            token_program: accounts.token_program.clone(),
            amount: self.big_blind,
        })?;

        self.game_state = GameState::WaitingSmallBlind;
    }

    pub fn startHand(self: &mut Room) -> ProgramResult {
        let mut count_available = 0;
        let mut big_blind_position = 0;
        let mut hand = Hand::default();
        hand.players = Vec::new();
        while i < self.players.len() {
            let PubKey current_player = &mut self.players.get(i);
            if current_player.playerStatus == PlayerStatus::Playing {
                count_available += 1;
                players.push(current_player);
            }
            i += 1;
            if(current_player.address == self.big_blind_address) {
                big_blind_position = i;
            }
        }
        if(count_available >= 2) {
            let mut new_big_blind_position = (big_blind_position > count_available ? 0 : big_blind_position);
            let mut new_small_blind_position = (new_big_blind_position < count_available ? new_big_blind_position + 1 : 0);
            hand.big_blind_position = new_big_blind_position;
            hand.big_blind_address = players.get(new_big_blind_position);
            hand.small_blind_position = new_small_blind_position;
            hand.small_blind_address = players.get(new_small_blind_position);
            hand.turn_index = 0;
            hand.pot = 0;
            hand
        } else {
            Err(SolanaPokerError::RoomFull.into())//minimo players
        }
    }
}