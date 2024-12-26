/*
* The impimentation for the UCI protocol will be done here
* For more commands check : https://official-stockfish.github.io/docs/stockfish-wiki/UCI-&-Commands.html
*/
use crate::Chessboard;
use crate::chessboard::attacks::AttackMasks;
use crate::chessboard::defs::{Pieces,FenPositions,algb_to_square,SQUARE};

use crate::movegen::MoveGenerator;
use crate::movegen::movecode::{Move};
use crate::{
    get_move_dst,
    get_move_src,
    get_move_promotion,
};
use crate::MoveMask;
use crate::move_type::ALL_MOVES;

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
//here is the function that parses the move gotten from the UCI and also be made
// This also ignores invalid moves
pub fn parse_move(generator:&mut MoveGenerator,move_str:&str) -> Move{
    // Moves from the UCI look like this : e2e4 , e7e8q
    if move_str.len() == 4 || move_str.len() == 5 {
        // defining these in case we have a promotion
        let mut promoted:&str = "0";
        if move_str.len() == 5 {
            promoted = move_str.get(4..5).unwrap();
        }
        let mut src:u8 = SQUARE::NO_SQUARE;
        let mut dst:u8 = SQUARE::NO_SQUARE;
        let result = algb_to_square(move_str.get(0..2).unwrap());
        match result {
            Some(x) => {
                src = x;
            }
            _ => (),
        }
        let result = algb_to_square(move_str.get(2..4).unwrap());
        match result {
            Some(x) => {
                dst = x;
            }
            _ => (),
        }
        // Check for if the src and the dst are valid or not
        if src == SQUARE::NO_SQUARE || dst == SQUARE::NO_SQUARE {
            // again we ignore
            return 0;
        }
        // Now we generate the moves for this pos and make the move
        for i in 0..generator.moves.count {
            let mov = generator.moves.list[i];
            // We found our move
            if src == get_move_src!(mov) as u8 && dst == get_move_dst!(mov) as u8 {
                // check if the promotion also matches
                let p = get_move_promotion!(mov) as usize;
                if p != Pieces::NONE as usize {
                    if (p == Pieces::Q || p == Pieces::q) && promoted == "q" {
                        return mov;
                    }
                    else if (p == Pieces::B || p == Pieces::b) && promoted == "b" {
                        return mov;
                    }
                    else if (p == Pieces::N || p == Pieces::n) && promoted == "n" {
                        return mov;
                    }
                    else if (p == Pieces::R || p == Pieces::r) && promoted == "r" {
                        return mov;
                    }
                    // We go again
                    continue;
                }
                return mov;
            }
        }
    }
    0
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
        board.print_chessboard();
        return ;
    }
    // else we get the moves in a vector and make them if they are : availble and legal 
    let moves_set = &parts[(moves_index+1)..];
    // I create it here because that is faster then making a new one each move
    let mut generator = MoveGenerator::new(board,&atk);
    for mov in moves_set {
        // mov.parse().decode().seach_in_generator() if found => make it : ignore it
        // println!("move is {mov}");
        generator.generate_moves();
        let mv = parse_move(&mut generator,mov);
        // Getting the move failed for some reason , we dont care
        if mv != 0 {
            generator.make_move(mv,ALL_MOVES);
        }
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
