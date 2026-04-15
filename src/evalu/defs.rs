/*
* Maybe we should explore more ways to evaluate a chess position that are not linear*/

/*Well define here the nececcary elements to make the evaluation work*/
// These are the costs of the pieces on board
//
//
// changed to float just because i need to use infinity during search algorithm
pub const PIECES_COST:[f64;12] = [      // These are ment to be indexed by 'Pieces::'
    100.0,
    500.0,
    300.0,
    350.0,
    1000.0,
    10000.0,
    -100.0,
    -500.0,
    -300.0,
    -350.0,
    -1000.0,
    -10000.0,
];

// These represent how much a pices cost depending on where it is
// Pawn positional score
// we index the piece then the square
pub const PIECES_LOCATION_COST: [[f64; 64]; 6] = [
    // Pawn positional score
    [
        90.0,  90.0,  90.0,  90.0,  90.0,  90.0,  90.0,  90.0,
        30.0,  30.0,  30.0,  40.0,  40.0,  30.0,  30.0,  30.0,
        20.0,  20.0,  20.0,  30.0,  30.0,  30.0,  20.0,  20.0,
        10.0,  10.0,  10.0,  20.0,  20.0,  10.0,  10.0,  10.0,
         5.0,   5.0,  10.0,  20.0,  20.0,   5.0,   5.0,   5.0,
         0.0,   0.0,   0.0,   5.0,   5.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,-10.0,-10.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
    ],
    // Rook positional score
    [
        50.0,  50.0,  50.0,  50.0,  50.0,  50.0,  50.0,  50.0,
        50.0,  50.0,  50.0,  50.0,  50.0,  50.0,  50.0,  50.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,   0.0,  20.0,  20.0,   0.0,   0.0,   0.0,
    ],
    // Knight positional score
    [
        -5.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,  -5.0,
        -5.0,   0.0,   0.0,  10.0,  10.0,   0.0,   0.0,  -5.0,
        -5.0,   5.0,  20.0,  20.0,  20.0,  20.0,   5.0,  -5.0,
        -5.0,  10.0,  20.0,  30.0,  30.0,  20.0,  10.0,  -5.0,
        -5.0,  10.0,  20.0,  30.0,  30.0,  20.0,  10.0,  -5.0,
        -5.0,   5.0,  20.0,  10.0,  10.0,  20.0,   5.0,  -5.0,
        -5.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,  -5.0,
        -5.0,-10.0,   0.0,   0.0,   0.0,   0.0,-10.0,  -5.0,
    ],
    // Bishop positional score
    [
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,  10.0,  10.0,   0.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,   0.0,  10.0,  20.0,  20.0,  10.0,   0.0,   0.0,
         0.0,  10.0,   0.0,   0.0,   0.0,   0.0,  10.0,   0.0,
         0.0,  30.0,   0.0,   0.0,   0.0,   0.0,  30.0,   0.0,
         0.0,   0.0,-10.0,   0.0,   0.0,-10.0,   0.0,   0.0,
    ],
    // Queen positional score
    [
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
    ],
    // King positional score
    [
         0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,   0.0,
         0.0,   0.0,   5.0,   5.0,   5.0,   5.0,   0.0,   0.0,
         0.0,   5.0,   5.0,  10.0,  10.0,   5.0,   5.0,   0.0,
         0.0,   5.0,  10.0,  20.0,  20.0,  10.0,   5.0,   0.0,
         0.0,   5.0,  10.0,  20.0,  20.0,  10.0,   5.0,   0.0,
         0.0,   0.0,   5.0,  10.0,  10.0,   5.0,   0.0,   0.0,
         0.0,   5.0,   5.0,  -5.0,  -5.0,   0.0,   5.0,   0.0,
         0.0,   0.0,   5.0,   0.0,-15.0,   0.0,  10.0,   0.0,
    ],
];

// This is the mirror image func for black pieces
// to evaluate the same costs
// 0 => 63 , 1 => 62 , 2 => 61 | f: x --> 63-x 
//                               f: [0..63] --> [63-0]
//  a map that achives : 0 -> 63-8 does work only because its the COST maps 
//  are semitrical , once more maps are used either this func below will work or we use 
//  color dependent array for each color 
/*I though of making a array but no you can make a func that transform already SQUARE*/
pub fn square_mirror(square:u8) -> usize {
    // What this does bassicly is it just mirroes the rows and 
    // keeps the cols as they are , doing 63-square switches both
    let row = square / 8;
    let col = square % 8;
    ((7-row) * 8 + col).into()
}
