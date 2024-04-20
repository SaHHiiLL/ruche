use iter_tools::Itertools;

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 8,
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum PieceType {
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,

    None = -1,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Move {
    from: usize,
    to: usize,
    move_type: MoveType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
enum MoveType {
    #[default]
    None,
    PawnPush,
    PawnDoublePush,
    PawnCapture,               // When a pawn captures a piece
    PawnEnPassant(Coordinate), // When a pawn captures a piece en passant
}

#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub struct Piece {
    pub piece_color: PieceColor,
    pub piece_type: PieceType,
}

impl Piece {
    pub fn new(piece_color: PieceColor, piece_type: PieceType) -> Self {
        Piece {
            piece_color,
            piece_type,
        }
    }
}

enum PawnCaptureDirection {
    Left,
    Right,
}

impl Piece {
    pub fn get_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
/// Seess the board from whites perspective
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct SafeCoordinate {
    x: isize,
    y: isize,
}

impl SafeCoordinate {
    fn new(x: isize, y: isize) -> Self {
        SafeCoordinate { x, y }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.x < 0 || self.x > 7 || self.y < 0 || self.y > 7
    }

    fn can_move_right(&self) -> bool {
        self.x < 7
    }

    fn can_move_left(&self) -> bool {
        self.x > 0
    }

    fn can_move_up(&self) -> bool {
        self.y < 7
    }

    fn can_move_down(&self) -> bool {
        self.y > 0
    }

    fn to_coordinate(&self) -> Coordinate {
        assert!(!self.is_out_of_bounds());
        Coordinate {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

impl Coordinate {
    fn new(x: usize, y: usize) -> Self {
        Coordinate { x, y }
    }

    fn is_right_out_of_bounds(&self) -> bool {
        self.x > 0
    }

    fn is_left_out_of_bounds(&self) -> bool {
        self.x < 7
    }

    fn is_top_out_of_bounds(&self) -> bool {
        self.y > 7
    }

    fn is_bottom_out_of_bounds(&self) -> bool {
        self.y < 0
    }

    fn can_move_right(&self) -> bool {
        !self.is_right_out_of_bounds()
    }

    fn can_move_left(&self) -> bool {
        !self.is_left_out_of_bounds()
    }
}

#[derive(Debug, Default, Hash, Eq, PartialEq)]
pub struct BitBoard {
    inner: u64,
}

impl BitBoard {
    pub fn set_bit(&mut self, idx: usize) {
        self.inner |= 1u64 << (idx);
    }

    pub fn clear_bit(&mut self, idx: usize) {
        self.inner &= !(1u64 << (idx));
    }

    pub fn get_bit(&self, idx: usize) -> bool {
        (self.inner & (1u64 << idx)) != 0
    }
}

// size_t colormask = 0b1000;
// size_t typemask =  0b0111;
impl From<u16> for Piece {
    fn from(value: u16) -> Self {
        let color_mask: u16 = 0b1000;
        let type_mask: u16 = 0b0111;

        let piece_color = match value & color_mask {
            0 => PieceColor::White,
            _ => PieceColor::Black,
        };

        let piece_type = match value & type_mask {
            1 => PieceType::Pawn,
            2 => PieceType::Knight,
            3 => PieceType::Bishop,
            4 => PieceType::Rook,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => PieceType::None,
        };

        Piece {
            piece_color,
            piece_type,
        }
    }
}

// 63, 61, 62, 60, 59, 58, 57, 56
// 55, 54, 53, 52, 51, 50, 49, 48
// 47, 46, 45, 44, 43, 42, 41, 40
// 39, 38, 37, 36, 35, 34, 33, 32
// 31, 30, 29, 28, 27, 26, 25, 24
// 23, 22, 21, 20, 19, 18, 17, 16
// 15, 14, 13, 12, 11, 10, 9,  8
// 7,  6,  5,  4,  3,  2,  1,  0
//

// 12, 10, 11, 13, 14, 11, 10, 12,
// 9,  9,  9,  9,  9,  9,  9,  9,
// 0,  0,  0,  0,  0,  0,  0,  0,
// 0,  0,  0,  0,  0,  0,  0,  0,
// 0,  0,  0,  1,  0,  0,  0,  0,
// 0,  0,  0,  0,  0,  0,  0,  0,
// 1,  1,  1,  0,  1,  1,  1,  1,
// 4,  2,  3,  5,  6,  3,  2,  4
//
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Board {
    white_pawn_bitboard: BitBoard,
    white_rook_bitboard: BitBoard,
    white_knight_bitboard: BitBoard,
    white_bishop_bitboard: BitBoard,
    white_queen_bitboard: BitBoard,
    white_king_bitboard: BitBoard,

    black_pawn_bitboard: BitBoard,
    black_rook_bitboard: BitBoard,
    black_knight_bitboard: BitBoard,
    black_bishop_bitboard: BitBoard,
    black_queen_bitboard: BitBoard,
    black_king_bitboard: BitBoard,

    /// Each cell holds a value which represents a piece
    board: [u16; 64],

    current_moves: Vec<Move>,
    move_history: Vec<Move>,

    /// The current turn
    is_white_turn: bool,
}

impl Board {
    pub fn clone_board(&self) -> Vec<u16> {
        self.board.to_vec().clone()
    }
    pub fn new() -> Self {
        Board {
            board: [0; 64],
            is_white_turn: true,
            current_moves: Vec::new(),
            move_history: Vec::new(),
            white_pawn_bitboard: BitBoard { inner: 0 },
            white_rook_bitboard: BitBoard { inner: 0 },
            white_knight_bitboard: BitBoard { inner: 0 },
            white_bishop_bitboard: BitBoard { inner: 0 },
            white_queen_bitboard: BitBoard { inner: 0 },
            white_king_bitboard: BitBoard { inner: 0 },

            black_pawn_bitboard: BitBoard { inner: 0 },
            black_rook_bitboard: BitBoard { inner: 0 },
            black_knight_bitboard: BitBoard { inner: 0 },
            black_bishop_bitboard: BitBoard { inner: 0 },
            black_queen_bitboard: BitBoard { inner: 0 },
            black_king_bitboard: BitBoard { inner: 0 },
        }
    }

    fn is_move_avaliable(&self, from: usize, to: usize) -> Option<Move> {
        for m in self.current_moves.iter() {
            if m.from == from && m.to == to {
                return Some(m.clone());
            }
        }
        None
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> bool {
        let piece = self.get_piece_at_index(from);
        let target = self.get_piece_at_index(to);

        if piece.get_type() == PieceType::None {
            tracing::error!("Invalid piece type");
            return false;
        }

        if piece.get_color() != self.get_turn() {
            tracing::error!("Invalid turn");
            return false;
        }

        // if no move is available return
        let Some(mo) = self.is_move_avaliable(from, to) else {
            tracing::error!("Invalid move");
            return false;
        };

        assert!(mo.from == from);
        assert!(mo.to == to);

        match mo.move_type {
            MoveType::PawnPush | MoveType::PawnDoublePush => {
                // if the move is a pawn push or double push
                // we need to update the bitboard
                assert!(target.get_type() == PieceType::None);
                let bitboard = self.get_bitboard_from_piece(piece);
                bitboard.clear_bit(from);
                bitboard.set_bit(to);
                self.board[to] = self.board[from];
                self.board[from] = 0; // 0 indicates Piece::None
            }
            MoveType::PawnEnPassant(_) => {
                todo!();
            }
            MoveType::PawnCapture => {
                tracing::debug!("Capturing piece {:?}", mo);
                self.capture_piece(&mo);
            }
            MoveType::None => todo!(),
        }
        self.move_history.push(mo);
        true
    }

    fn capture_piece(&mut self, current_move: &Move) {
        let target = self.get_piece_at_index(current_move.to);
        assert!(target.get_type() != PieceType::None);
        let bitboard = self.get_bitboard_from_piece(target);
        bitboard.clear_bit(current_move.to);
        self.board[current_move.to] = 0;

        // move the piece to the target square
        let piece = self.get_piece_at_index(current_move.from);
        let bitboard = self.get_bitboard_from_piece(piece);
        bitboard.clear_bit(current_move.from);
        bitboard.set_bit(current_move.to);
        self.board[current_move.to] = self.board[current_move.from];
        self.board[current_move.from] = 0;
    }

    /// Clears the moves list and generates all possible moves for the current position
    /// This function should be called after each move
    pub fn generate_moves_current_position(&mut self) {
        self.current_moves.clear();
        assert!(self.current_moves.is_empty());
        let turn = self.get_turn();

        let board = self
            .clone_board()
            .iter()
            .map(|p| Piece::from(*p))
            .collect::<Vec<_>>();

        // Filters over the current turn pieces and generates all possible moves
        for (i, piece) in board.iter().enumerate() {
            if piece.get_color() != turn {
                continue;
            }
            if let PieceType::Pawn = piece.piece_type {
                self.generate_pawn_moves(i, *piece);
            }
        }
    }

    fn generate_pawn_moves(&mut self, current_piece_idx: usize, piece: Piece) {
        assert!(piece.piece_type == PieceType::Pawn);

        // Pawns can move forward one square if the square is empty
        // Pawns can move forward two squares if the square is empty and the pawn is on the starting rank
        // Pawns can capture diagonally
        // Pawns can capture en passant
        // Pawns can promote

        // if there a piece in front of the pawn it shall not move

        let co = self.get_safe_coordinates_from_index(current_piece_idx);

        let front_co = SafeCoordinate {
            x: co.x,
            y: if piece.piece_color == PieceColor::White {
                co.y - 1
            } else {
                co.y + 1
            },
        };

        if !front_co.is_out_of_bounds() {
            // calculates the front of the pawn if it's white or black
            let front = self.get_square(front_co.x as usize, front_co.y as usize);
            let front_piece = self.get_piece_at_index(front);
            if front_piece.get_type() == PieceType::None {
                // Add front move to the list
                self.current_moves.push(Move {
                    from: current_piece_idx,
                    to: front,
                    move_type: MoveType::PawnPush,
                });
                // checking for double push
                if co.y == 6 && piece.piece_color == PieceColor::White {
                    let double_front = self.get_square_isize(front_co.x, front_co.y - 1);
                    let double_front_piece = self.get_piece_at_index(double_front);
                    if double_front_piece.get_type() == PieceType::None {
                        self.current_moves.push(Move {
                            from: current_piece_idx,
                            to: double_front,
                            move_type: MoveType::PawnDoublePush,
                        });
                    }
                } else if co.y == 1 && piece.piece_color == PieceColor::Black {
                    let double_front = self.get_square_isize(front_co.x, front_co.y + 1);
                    let double_front_piece = self.get_piece_at_index(double_front);
                    if double_front_piece.get_type() == PieceType::None {
                        self.current_moves.push(Move {
                            from: current_piece_idx,
                            to: double_front,
                            move_type: MoveType::PawnDoublePush,
                        });
                    }
                }
            }
        }
        if let Some(m) = self.pawn_capture(piece, &co, PawnCaptureDirection::Right) {
            self.current_moves.push(m);
        }

        if let Some(m) = self.pawn_capture(piece, &co, PawnCaptureDirection::Left) {
            self.current_moves.push(m);
        }

        if let Some(m) = self.enpassant_capture(piece, &co) {
            self.current_moves.push(m);
        }
    }

    // TODO::
    fn enpassant_capture(&self, piece: Piece, current_cord: &SafeCoordinate) -> Option<Move> {
        // if the last move by the opponent was a double pawn push on either side of the current
        // pawn we can capture it en passant

        if self.move_history.is_empty() {
            return None;
        }

        let last_move = self.move_history.last()?;

        if last_move.move_type != MoveType::PawnDoublePush {
            return None;
        }

        if current_cord.y != 1 || current_cord.y != 6 {
            return None;
        }

        let last_move_cord = self.get_safe_coordinates_from_index(last_move.to);
        let end_pos = Coordinate {
            x: last_move_cord.x as usize,
            y: if piece.get_color() == PieceColor::White {
                (last_move_cord.y as usize) + 1
            } else {
                last_move_cord.y as usize - 1
            },
        };

        let mov = Move {
            from: self.get_index_from_coordinates(current_cord.to_coordinate()),
            to: self.get_index_from_coordinates(end_pos),
            move_type: MoveType::PawnEnPassant(last_move_cord.to_coordinate()),
        };
        Some(mov)
    }

    fn pawn_capture(
        &self,
        piece: Piece,
        current_cord: &SafeCoordinate,
        side: PawnCaptureDirection,
    ) -> Option<Move> {
        if current_cord.is_out_of_bounds() {
            return None;
        }
        let right_co = SafeCoordinate {
            x: match side {
                PawnCaptureDirection::Right => current_cord.x + 1,
                PawnCaptureDirection::Left => current_cord.x - 1,
            },
            y: if piece.get_color() == PieceColor::White {
                current_cord.y - 1
            } else {
                current_cord.y + 1
            },
        };
        if right_co.is_out_of_bounds() {
            return None;
        }

        let right_piece = self.get_piece_at_index(
            self.get_square(right_co.to_coordinate().x, right_co.to_coordinate().y),
        );

        if right_piece.get_color() == piece.get_color() {
            return None;
        }

        if right_piece.get_type() == PieceType::None {
            tracing::debug!("No piece to capture");
            return None;
        }
        tracing::debug!("Piece to capture: {:?}", right_piece);

        let right = self.get_square_isize(right_co.x, right_co.y);
        let mov = Move {
            from: self.get_index_from_coordinates(current_cord.to_coordinate()),
            to: right,
            move_type: MoveType::PawnCapture,
        };
        tracing::debug!("Pawn capture: {:?}", mov);
        Some(mov)
    }

    /// Returns the index of the square given the x and y coordinates
    fn get_index_from_coordinates(&self, co: Coordinate) -> usize {
        self.get_square(co.x, co.y)
    }

    /// Returns the coordinate of the given index
    fn get_coordinates_from_index(&self, idx: usize) -> Coordinate {
        let x = idx % 8;
        let y = idx / 8;
        Coordinate { x, y }
    }

    fn get_safe_coordinates_from_index(&self, idx: usize) -> SafeCoordinate {
        let x = idx % 8;
        let y = idx / 8;
        SafeCoordinate::new(x as isize, y as isize)
    }

    //TODO: remove this function
    pub fn print_debug(&self) {
        println!("White Pawn: {:?}", self.white_pawn_bitboard.inner);
        println!("White Rook: {:?}", self.white_rook_bitboard.inner);
        println!("White Knight: {:?}", self.white_knight_bitboard.inner);
        println!("White Bishop: {:?}", self.white_bishop_bitboard.inner);
        println!("White Queen: {:?}", self.white_queen_bitboard.inner);
        println!("White King: {:?}", self.white_king_bitboard.inner);

        println!("Black Pawn: {:?}", self.black_pawn_bitboard.inner);
        println!("Black Rook: {:?}", self.black_rook_bitboard.inner);
        println!("Black Knight: {:?}", self.black_knight_bitboard.inner);
        println!("Black Bishop: {:?}", self.black_bishop_bitboard.inner);
        println!("Black Queen: {:?}", self.black_queen_bitboard.inner);
        println!("Black King: {:?}", self.black_king_bitboard.inner);

        self.board.iter().chunks(8).into_iter().for_each(|c| {
            for m in c {
                print!("{:?},", m);
            }
            println!();
        });

        self.current_moves.iter().for_each(|m| {
            println!("{:?}", m);
        });
    }

    /// Returns the current turn
    pub fn get_turn(&self) -> PieceColor {
        if self.is_white_turn {
            PieceColor::White
        } else {
            PieceColor::Black
        }
    }

    /// Returns a mutable reference to the bitboard of the given piece
    fn get_bitboard_from_piece(&mut self, piece: Piece) -> &mut BitBoard {
        match piece.get_color() {
            PieceColor::White => match piece.piece_type {
                PieceType::Pawn => &mut self.white_pawn_bitboard,
                PieceType::Rook => &mut self.white_rook_bitboard,
                PieceType::Knight => &mut self.white_knight_bitboard,
                PieceType::Bishop => &mut self.white_bishop_bitboard,
                PieceType::Queen => &mut self.white_queen_bitboard,
                PieceType::King => &mut self.white_king_bitboard,
                PieceType::None => panic!("Invalid Piece Type"),
            },
            PieceColor::Black => match piece.piece_type {
                PieceType::Pawn => &mut self.black_pawn_bitboard,
                PieceType::Rook => &mut self.black_rook_bitboard,
                PieceType::Knight => &mut self.black_knight_bitboard,
                PieceType::Bishop => &mut self.black_bishop_bitboard,
                PieceType::Queen => &mut self.black_queen_bitboard,
                PieceType::King => &mut self.black_king_bitboard,
                PieceType::None => panic!("Invalid Piece Type"),
            },
        }
    }

    /// Changes the turn of the board
    pub fn toggle_turn(&mut self) {
        self.is_white_turn = !self.is_white_turn;
    }

    /// Returns the index of the square given the x and y coordinates
    /// asserts that the index is within the board 0 > idx < 64
    pub fn get_square(&self, x: usize, y: usize) -> usize {
        let res = (y * 8) + x;
        assert!((0..64).contains(&res));
        res
    }

    pub fn get_square_isize(&self, x: isize, y: isize) -> usize {
        let res = (y * 8) + x;
        assert!((0..64).contains(&res));
        res as usize
    }

    /// Gets the piece at the given index as a Piece struct
    pub fn get_piece_at_index(&self, idx: usize) -> Piece {
        let piece = self.board[idx].into();
        tracing::trace!("Piece at index: {:?} is {:?}", idx, piece);
        piece
    }

    /// Loads a position from a FEN string
    /// ```no_run
    /// let mut board = Board::new();
    /// board.load_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn load_position(&mut self, fen: String) {
        let mut y = 0;
        let mut x = 0;

        for c in fen.chars() {
            match c {
                '1'..='8' => {
                    let offset = c.to_digit(10).unwrap() as usize;
                    x += offset;
                }
                '/' => {
                    y += 1;
                    x = 0;
                }
                'r' => {
                    println!("r");
                    let idx = self.get_square(x, y);
                    self.black_rook_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Rook as u16;
                    x += 1;
                }
                'n' => {
                    let idx = self.get_square(x, y);
                    self.black_knight_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Knight as u16;
                    x += 1;
                }
                'b' => {
                    let idx = self.get_square(x, y);
                    self.black_bishop_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Bishop as u16;
                    x += 1;
                }
                'q' => {
                    let idx = self.get_square(x, y);
                    self.black_queen_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Queen as u16;
                    x += 1;
                }
                'k' => {
                    let idx = self.get_square(x, y);
                    self.black_king_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::King as u16;
                    x += 1;
                }
                'p' => {
                    let idx = self.get_square(x, y);
                    self.black_pawn_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Pawn as u16;
                    x += 1;
                }
                'R' => {
                    let idx = self.get_square(x, y);
                    self.white_rook_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Rook as u16;
                    x += 1;
                }
                'N' => {
                    let idx = self.get_square(x, y);
                    self.white_knight_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Knight as u16;
                    x += 1;
                }
                'B' => {
                    let idx = self.get_square(x, y);
                    self.white_bishop_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Bishop as u16;
                    x += 1;
                }
                'Q' => {
                    let idx = self.get_square(x, y);
                    self.white_queen_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Queen as u16;
                    x += 1;
                }
                'K' => {
                    let idx = self.get_square(x, y);
                    self.white_king_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::King as u16;
                    x += 1;
                }
                'P' => {
                    let idx = self.get_square(x, y);
                    self.white_pawn_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Pawn as u16;
                    x += 1;
                }
                _ => {
                    tracing::error!("Invalid FEN character: {}", c);
                }
            }
        }

        println!("{:?}", self.board);
        self.generate_moves_current_position();
    }
}
