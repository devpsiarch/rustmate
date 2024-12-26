/*
* The impimentation for the UCI protocol will be done here
* For more commands check : https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html
*/
use crate::Chessboard;
use crate::chessboard::attacks::AttackMasks;
use crate::chessboard::defs::{FenPositions};

use std::io;
use std::io::{Write,stdout,BufRead};
type UciExitStatus = Result<(),u8>;

// Every line should end in a '\n'
pub fn show_engine_info() {
    println!("id name rustmate");
    println!("id author devpsiarch");
    // We could have more above : option name [thingy]
    // We have to end with uciok
    println!("uciok");
}
// Here we define the "position" function handler
pub fn position_handler(board:&mut Chessboard,atk:&AttackMasks,parts:&Vec<&str>) {
    let mut moves_index :usize = 0;
    match parts[1] {
        // We just init the start position then read the move
        "startpos" => {
            board.init_board(FenPositions::STARTING_POSITION);
            // getting if there is moves after that
            if let Some(_state) = parts.get(2) {        // checking if parts[2] exists
                moves_index = 2; 
            }else{                  // else we are done here and return
                () 
            }
        } 
        "fen" => {
            // Checking if a fen is here that the engine can parse (idk if the uci can get diff fens)
            if let Some(_state) = parts.get(2..8) {
                // We collect the parts of them back in a fen 
                let fen_parts = &parts[2..8];
                let fen = fen_parts.join(" ");
                board.init_board(&fen);         // idk if its good idea that this panics
                if let Some(_state) = parts.get(8) {        // checking if parts[2] exists
                    moves_index = 8; 
                }else{                  // else we are done here and return
                    () 
                }
            }else{
                () 
            }
        }
        // ignore the invalid command
        _ => (),
    }
    
    // Now here we handle the moves and make them on the board
    // First we check if the "moves_index" is accualy pointing to a "moves" string
    if parts[moves_index] != "moves" {
        return;
    }
    // else we get the moves in a vector and make them if they are : availble and legal 
    let moves_set = &parts[(moves_index+1)..];
    for mov in moves_set {
        // mov.parse().decode().seach_in_generator() if found => make it : ignore it
        println!("move is {mov}");
    }
    board.print_chessboard();
} 
// Hey future me , i think its best when a command is not reconised for the UCI to just ignore it
pub fn uci(board:&mut Chessboard,atk:&AttackMasks) -> UciExitStatus {
    match stdout().flush() {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to flush stdout: {}", e),
    }
    show_engine_info();
    // This will store the incoming commands 
    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        match line {
            Ok(command) => {
                buffer.clear();
                buffer.push_str(&command);
                // Here we handle short and simple commands that are one word long ...
                if buffer == "quit" {
                    return Ok(())
                }
                if buffer == "uci" {
                    show_engine_info();
                }
                // Handling the commands happends here 
                // Getting the parts of the command 
                let parts: Vec<&str> = buffer.split(" ").collect();
                match parts[0] {
                    "position" => position_handler(board,atk,&parts),
                    _ => {
                        // We dont exit when encortering unreconised command
                        ()
                    }
                }
            }
            Err(e) => {
                println!("Error {e} reading the line !!! exisiting UCI mainloop");
                return Err(1);
            }
        }
    }
    Ok(())
}
