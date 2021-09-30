extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate solana_sdk;

mod room;
mod hand;
mod error;
mod program_command;
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

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    info!("Solana Poker Rust program entrypoint");

    if !accounts[0].is_signer {
        info!("Account 0 did not sign the transaction");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let command = Command::deserialize(instruction_data)?;
    let account_info_iter = &mut accounts.iter();
    let first_account = next_account_info(account_info_iter)?;

    let room_account = first_account;
    let player_account = next_account_info(account_info_iter)?;

    match command {
        Command::CreateRoom(big_blind, max_players) => {
            info!("Create Room");
            let mut room_state = State::deserialize(&room_account.data.borrow())?;
            match room_state {
                State::Uninitialized => {
                    let room = room::Room::create(&player_account.key, big_blind, max_players);
                    room_state = State::Room(room);
                }
                _ => {
                    info!("Invalid room state");
                    return Err(ProgramError::InvalidArgument);
                }
            }
    
            room_state.serialize(&mut room_account.data.borrow_mut())?;
        }
        _ => {
            info!("invalid command for State::Room");
            return Err(ProgramError::InvalidArgument);
        }
    }

    /*let mut room_state = State::deserialize(&room_account.data.borrow())?;
    match room_state {
        State::Room(ref mut room) => {
            let player = player_account.key;
            let current_slot = Clock::from_account_info(sysvar_account)?.slot;

            match command {
                Command::Advertise => {
                    // Nothing to do here beyond the dashboard_update() below
                    info!("advertise game")
                }
                Command::Join => {
                    info!("join game");
                    game.join(*player, current_slot)?
                }
                Command::Move(x, y) => {
                    info!("move");
                    game.next_move(*player, x as usize, y as usize)?
                }
                Command::KeepAlive => {
                    info!("keep alive");
                    game.keep_alive(*player, current_slot)?
                }
                _ => {
                    info!("invalid command for State::Game");
                    return Err(ProgramError::InvalidArgument);
                }
            }

            match dashboard_state {
                State::Dashboard(ref mut dashboard) => {
                    dashboard.update(&game_account.key, &game)?
                }
                _ => {
                    info!("Invalid dashboard state");
                    return Err(ProgramError::InvalidArgument);
                }
            }
        }
        _ => {
            info!("Invalid game state}");
            return Err(ProgramError::InvalidArgument);
        }
    }*/

    room_state.serialize(&mut room_account.data.borrow_mut())?;

    
    //fund_to_cover_rent(dashboard_account, game_account)?;
    //fund_to_cover_rent(dashboard_account, player_account)
}

entrypoint!(_entrypoint);
fn _entrypoint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TicTacToeError>();
        return Err(error);
    }
    Ok(())
}
