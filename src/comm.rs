/*
* The impimentation for the UCI protocol will be done here
*/
use std::io;
use std::io::{Write,stdout,BufRead};
type UciExitStatus = Result<(),u8>;

// Every line should end in a '\n'
pub fn Show_engine_info() {
    println!("id name rustmate");
    println!("id name devpsiarch");
    println!("uciok");
}
// Hey future me , i think its best when a command is not reconised for the UCI to just ignore it
pub fn uci() -> UciExitStatus {
    match stdout().flush() {
        Ok(()) => (),
        Err(e) => eprintln!("Failed to flush stdout: {}", e),
    }
    Show_engine_info();
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
                // Handling the commands happends here 
                println!("commmand is {}",buffer);
            }
            Err(e) => {
                println!("Error {e} reading the line !!! exisiting UCI mainloop");
                return Err(1);
            }
        }
    }
    Ok(())
}
