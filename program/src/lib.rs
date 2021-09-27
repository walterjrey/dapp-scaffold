extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate solana_sdk;

mod dashboard;
mod error;
mod game;
mod program_command;
mod program_state;
mod simple_serde;

use crate::error::SolanaPokerError;
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
    pubkey::Pubkey,
    sysvar::{clock::Clock, Sysvar},
};

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

#[repr(C)]
#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub struct Room {
    small_blind: u64,
    big_blind: u64,
    max_players: u8,
    players: Vec<Pubkey>,
    creator: Pubkey,
    big_blind_position: u8,
    keep_alive: [u64; 8],
    pub game_state: GameState,
    pub player_turn: u8,
    pub last_player_turn: u8
}

impl Room {
    pub fn create(creator: &Pubkey, u64 big_blind, u8 max_players) -> Game {
        let mut room = Room::default();
        room.creator = *creator;
        room.big_blind = *big_blind;
        room.small_blind = room.big_blind / 2;
        room.max_players = *max_players;
        room.big_blind_position = 1;
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
            self.players.push(player);
        } else {
            Err(SolanaPokerError::RoomFull.into())
        }
    }

    pub fn leave(self: &mut Room, player: Pubkey) -> ProgramResult {
        let mut i = 0;
        while i < vec.len() {
            let PubKey currentPlayer = &mut self.players.get(i);
            if currentPlayer == player {
                self.players[i].remove(i);
            } else {
                i += 1;
            }
        }
    }
}

entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
