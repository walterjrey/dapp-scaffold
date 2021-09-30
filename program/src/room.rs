extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate solana_sdk;

//mod dashboard;
//mod error;
//mod game;
//mod program_command;
//mod program_state;
//mod simple_serde;

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


impl Room {

    pub fn create(creator: &Pubkey, u64 big_blind, u8 max_players) -> Game {
        let mut room = Room::default();
        room.creator = *creator;
        room.big_blind = *big_blind;
        room.small_blind = room.big_blind / 2;
        room.max_players = *max_players;
        room.big_blind_position = 0;
        room.players = Vec::new();
        //assert_eq!(room.game_state, GameState::WaitingBlinds);
        room
    }

    #[cfg(test)]
    pub fn new(creator: Pubkey, u64 big_blind, u8 max_players) -> Game {
        let mut room = Room::create(&creator, big_blind, max_players);
        //room.join_game(player_o, 1).unwrap();
        room
    }

    pub fn join(self: &mut Room, player: Pubkey) -> ProgramResult {
        if self.players.len() < self.max_players {
            let mut pokerPlayer = PokerPlayer::default();
            pokerPlayer.address = player;
            self.players.push(pokerPlayer);
        } else {
            Err(SolanaPokerError::RoomFull.into())
        }
    }

    pub fn leave(self: &mut Room, player: Pubkey) -> ProgramResult {
        let mut i = 0;
        while i < self.players.len() {
            let PubKey currentPlayer = &mut self.players.get(i);
            if currentPlayer.address == player {
                self.players[i].remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn available(self: &mut Room, player: Pubkey) -> ProgramResult {
        let mut i = 0;
        while i < self.players.len() {
            let PubKey currentPlayer = &mut self.players.get(i);
            if currentPlayer.address == player {
                self.players[i].playerStatus = PlayerStatus::Playing;
            } else {
                i += 1;
            }
        }
    }

    pub fn disabled(self: &mut Room, player: Pubkey) -> ProgramResult {
        let mut i = 0;
        while i < self.players.len() {
            let PubKey currentPlayer = &mut self.players.get(i);
            if currentPlayer.address == player {
                self.players[i].playerStatus = PlayerStatus::Out;
            } else {
                i += 1;
            }
        }
    }

}