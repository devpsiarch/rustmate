use crate::{chessboard::{self, Chessboard, zobrist::{self, Zobrist}}, movegen::movecode::Move};

pub enum TTflag {
    HashExact,HashAlpha,HashBeta
}

struct TranspositionEntry {
    pub key: u64,
    pub depth: u32,
    pub flag: TTflag,
    pub eval: f64,
    pub bestmove: Move          // for now i wont use it since it is not compatible with my API
}

const TRANSPOSITION_TABLE_SIZE: u64 = 100;

pub struct TranspositionTable {
    hasher: Zobrist,
    entries: [Option<TranspositionEntry>; TRANSPOSITION_TABLE_SIZE as usize]
}

impl TranspositionTable {
    pub fn new() -> Self{
        Self {
            hasher: zobrist::Zobrist::new(),
            entries: [const { None }; TRANSPOSITION_TABLE_SIZE as usize],
        }
    }
    pub fn probeHash(&self,depth: u32,alpha: f64,beta: f64,board: &Chessboard) -> Option<f64> {
        let key = self.hasher.hash(board);
        if let Some(cache_hit) = &self.entries[(key % TRANSPOSITION_TABLE_SIZE) as usize] {
            if cache_hit.key == key && cache_hit.depth >= depth {
                return match cache_hit.flag {
                    TTflag::HashBeta => Some(cache_hit.eval),
                    TTflag::HashAlpha if cache_hit.eval <= alpha => Some(alpha),
                    TTflag::HashExact if cache_hit.eval >= beta=> Some(beta),
                    _ => None
                }
            }else{
                None
            }
        }else{
            None
        }
    }
    pub fn recordHash(&mut self,_depth: u32,_eval: f64,_flag: TTflag,_board: &Chessboard) -> () {
        let _key = self.hasher.hash(_board);
        let index:usize = (_key % TRANSPOSITION_TABLE_SIZE) as usize;
        self.entries[index] = Some(TranspositionEntry {
            depth: _depth,
            key: _key,
            flag: _flag,
            eval: _eval,
            bestmove: 0,
        });
    }
}
