/*
* The impimentation for the UCI protocol will be done here
* For more commands check : https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html
*/
pub mod parse;

use crate::Chessboard;
use crate::chessboard::attacks::AttackMasks;
use crate::chessboard::bitboard::print_bitboard;

use crate::movegen::MoveGenerator;

use std::io;
use std::io::{Write,stdout,BufRead};


use crate::comm::parse::{show_engine_info,position_handler,go_handler};
type UciExitStatus = Result<(),u8>;


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
                else if buffer == "uci" {
                    show_engine_info();
                }
                else if buffer == "isready" {
                    println!("readyok");
                }
                else if buffer == "go" {
                    todo!();
                }
                else if buffer == "ucinewgame" {
                    let temp = vec!["position","startpos"];
                    position_handler(board,atk,&temp);
                }
                // for debuging this shit
                else if buffer == "show_ocp" {
                    print_bitboard(&board.occupencies[0]);
                    print_bitboard(&board.occupencies[1]);
                    print_bitboard(&board.occupencies[2]);
                }
                else if buffer == "show_moves" {
                    let mut generator = MoveGenerator::new(board,atk);
                    generator.generate_moves();
                    generator.moves.print_all_moves();
                }
                else if buffer == "show_board" {
                    board.print_chessboard();
                }
                // Handling the commands happends here 
                // Getting the parts of the command 
                let parts: Vec<&str> = buffer.split(" ").collect();
                match parts[0] {
                    "position" => position_handler(board,atk,&parts),
                    "go" => go_handler(board,atk,&parts),
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
