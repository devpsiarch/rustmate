use crate::{chessboard, get_bit};

pub struct Zobrist {
    data: [[[u64; 64]; 2]; 12],
}

impl Zobrist {
    pub fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| {
                std::array::from_fn(|_| {
                    std::array::from_fn(|_| rand::random::<u64>())
                })
            }),
        }
    }
    pub fn get(&self,p: usize,s: chessboard::SIDES,sq: usize) -> u64 {
        return match s {
            chessboard::defs::SIDES::WHITE => self.data[p][0][sq],
            chessboard::defs::SIDES::BLACK => self.data[p][1][sq],
        }
    }

    pub fn hash(&self,board: &chessboard::Chessboard) -> u64 {
        let mut key: u64 = 0;
        
        for i in 0..64 {
            for j in 0..12 {
                if get_bit!(board.bitboards[j],i) == 1 {
                    key = key ^ self.get(j, board.side_to_move, i);
                }
            }
        }

        return key;
    }
}
