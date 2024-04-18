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
    PawnCapture,   // When a pawn captures a piece
    PawnEnPassant, // When a pawn captures a piece en passant
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

impl Piece {
    pub fn get_color(&self) -> PieceColor {
        self.piece_color
    }

    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
pub struct BitBoard(u64);

impl BitBoard {
    pub fn set_bit(&mut self, idx: usize) {
        self.0 |= 1 << idx;
    }

    pub fn clear_bit(&mut self, idx: usize) {
        self.0 &= !(1 << idx);
    }

    pub fn get_bit(&self, idx: usize) -> bool {
        (self.0 & (1 << idx)) != 0
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

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
            white_pawn_bitboard: BitBoard(0),
            white_rook_bitboard: BitBoard(0),
            white_knight_bitboard: BitBoard(0),
            white_bishop_bitboard: BitBoard(0),
            white_queen_bitboard: BitBoard(0),
            white_king_bitboard: BitBoard(0),

            black_pawn_bitboard: BitBoard(0),
            black_rook_bitboard: BitBoard(0),
            black_knight_bitboard: BitBoard(0),
            black_bishop_bitboard: BitBoard(0),
            black_queen_bitboard: BitBoard(0),
            black_king_bitboard: BitBoard(0),
        }
    }

    fn is_move_avaliable(&self, from: usize, to: usize) -> Option<Move> {
        for m in self.current_moves.iter() {
            if m.from == from && m.to == to {
                tracing::info!("Move is available");
                return Some(m.clone());
            }
        }
        None
    }

    pub fn make_move(&mut self, from: usize, to: usize) -> bool {
        tracing::info!("Making move from {} to {}", from, to);
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
                tracing::info!("Pawn push from {} to {}", from, to);
                let mut bitboard = self.get_bitboard_from_piece(piece);
                bitboard.clear_bit(63 - from);
                bitboard.set_bit(63 - to);
                self.board[to] = self.board[from];
                self.board[from] = PieceType::None as u16;
            }
            _ => todo!(),
        }
        return true;
    }

    fn remove_piece_at_index(&mut self, idx: usize) {
        let piece = self.get_piece_at_index(idx);
        let mut bitboard = self.get_bitboard_from_piece(piece);
        bitboard.clear_bit(63 - idx);
    }

    /// Clears the moves list and generates all possible moves for the current position
    /// This function should be called after each move
    pub fn generate_moves_current_position(&mut self) {
        self.current_moves.clear();
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
                tracing::info!("Generating moves for pawn at {}", i);
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

        let co = self.get_coordinates_from_index(current_piece_idx);
        // calculates the front of the pawn if it's white or black
        let front = self.get_square(
            co.x,
            if piece.piece_color == PieceColor::White {
                co.y - 1
            } else {
                co.y + 1
            },
        );
        tracing::info!("Front: {}", front);
        let front_piece = self.get_piece_at_index(front);
        tracing::info!("Front piece: {:?}", front_piece);
        if front_piece.get_type() == PieceType::None {
            // Add front move to the list
            self.current_moves.push(Move {
                from: current_piece_idx,
                to: front,
                move_type: MoveType::PawnPush,
            });
            // checking for double push
            tracing::info!("Co y: {}", co.y);
            if co.y == 6 && piece.piece_color == PieceColor::White {
                let double_front = self.get_square(co.x, co.y - 2);
                let double_front_piece = self.get_piece_at_index(double_front);
                if double_front_piece.get_type() == PieceType::None {
                    tracing::info!("Double push from {} to {}", current_piece_idx, double_front);
                    self.current_moves.push(Move {
                        from: current_piece_idx,
                        to: double_front,
                        move_type: MoveType::PawnDoublePush,
                    });
                }
            } else if co.y == 1 && piece.piece_color == PieceColor::Black {
                let double_front = self.get_square(co.x, co.y + 2);
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

        // BUG: thread 'main' panicked at src/board.rs:304:13: attempt to subtract with overflow note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
        // check for captures
        let left = self.get_square(
            co.x - 1,
            if piece.piece_color == PieceColor::White {
                co.y - 1
            } else {
                co.y + 1
            },
        );

        let right = self.get_square(
            co.x - 1,
            if piece.piece_color == PieceColor::White {
                co.y - 1
            } else {
                co.y + 1
            },
        );

        let left_piece = self.get_piece_at_index(left);
        let right_piece = self.get_piece_at_index(right);

        if left_piece.get_color() != piece.get_color() && left_piece.get_type() != PieceType::None {
            self.current_moves.push(Move {
                from: current_piece_idx,
                to: left,
                move_type: MoveType::PawnCapture,
            });
        }

        if right_piece.get_color() != piece.get_color() && right_piece.get_type() != PieceType::None
        {
            self.current_moves.push(Move {
                from: current_piece_idx,
                to: right,
                move_type: MoveType::PawnCapture,
            });
        }
    }

    fn get_index_from_coordinates(&self, co: Coordinate) -> usize {
        self.get_square(co.x, co.y)
    }

    fn get_coordinates_from_index(&self, idx: usize) -> Coordinate {
        let x = idx % 8;
        let y = idx / 8;
        Coordinate { x, y }
    }

    pub fn print_debug(&self) {
        println!("White Pawn: {:?}", self.white_pawn_bitboard.0);
        println!("White Rook: {:?}", self.white_rook_bitboard.0);
        println!("White Knight: {:?}", self.white_knight_bitboard.0);
        println!("White Bishop: {:?}", self.white_bishop_bitboard.0);
        println!("White Queen: {:?}", self.white_queen_bitboard.0);
        println!("White King: {:?}", self.white_king_bitboard.0);

        println!("Black Pawn: {:?}", self.black_pawn_bitboard.0);
        println!("Black Rook: {:?}", self.black_rook_bitboard.0);
        println!("Black Knight: {:?}", self.black_knight_bitboard.0);
        println!("Black Bishop: {:?}", self.black_bishop_bitboard.0);
        println!("Black Queen: {:?}", self.black_queen_bitboard.0);
        println!("Black King: {:?}", self.black_king_bitboard.0);
    }

    pub fn get_turn(&self) -> PieceColor {
        if self.is_white_turn {
            PieceColor::White
        } else {
            PieceColor::Black
        }
    }

    fn get_bitboard_from_piece(&self, piece: Piece) -> BitBoard {
        match piece.get_color() {
            PieceColor::White => match piece.piece_type {
                PieceType::Pawn => self.white_pawn_bitboard,
                PieceType::Rook => self.white_rook_bitboard,
                PieceType::Knight => self.white_knight_bitboard,
                PieceType::Bishop => self.white_bishop_bitboard,
                PieceType::Queen => self.white_queen_bitboard,
                PieceType::King => self.white_king_bitboard,
                PieceType::None => panic!("Invalid Piece Type"),
            },
            PieceColor::Black => match piece.piece_type {
                PieceType::Pawn => self.black_pawn_bitboard,
                PieceType::Rook => self.black_rook_bitboard,
                PieceType::Knight => self.black_knight_bitboard,
                PieceType::Bishop => self.black_bishop_bitboard,
                PieceType::Queen => self.black_queen_bitboard,
                PieceType::King => self.black_king_bitboard,
                PieceType::None => panic!("Invalid Piece Type"),
            },
        }
    }

    pub fn toggle_turn(&mut self) {
        self.is_white_turn = !self.is_white_turn;
    }

    pub fn get_square(&self, x: usize, y: usize) -> usize {
        (y * 8) + x
    }

    pub fn get_piece_at_index(&self, idx: usize) -> Piece {
        self.board[idx].into()
    }

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
                    self.black_rook_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Rook as u16;
                    x += 1;
                }
                'n' => {
                    let idx = self.get_square(x, y);
                    self.black_knight_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Knight as u16;
                    x += 1;
                }
                'b' => {
                    let idx = self.get_square(x, y);
                    self.black_bishop_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Bishop as u16;
                    x += 1;
                }
                'q' => {
                    let idx = self.get_square(x, y);
                    self.black_queen_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Queen as u16;
                    x += 1;
                }
                'k' => {
                    let idx = self.get_square(x, y);
                    self.black_king_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::King as u16;
                    x += 1;
                }
                'p' => {
                    let idx = self.get_square(x, y);
                    self.black_pawn_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Pawn as u16;
                    x += 1;
                }
                'R' => {
                    let idx = self.get_square(x, y);
                    self.white_rook_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Rook as u16;
                    x += 1;
                }
                'N' => {
                    let idx = self.get_square(x, y);
                    self.white_knight_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Knight as u16;
                    x += 1;
                }
                'B' => {
                    let idx = self.get_square(x, y);
                    self.white_bishop_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Bishop as u16;
                    x += 1;
                }
                'Q' => {
                    let idx = self.get_square(x, y);
                    self.white_queen_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Queen as u16;
                    x += 1;
                }
                'K' => {
                    let idx = self.get_square(x, y);
                    self.white_king_bitboard.set_bit(63 - idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::King as u16;
                    x += 1;
                }
                'P' => {
                    let idx = self.get_square(x, y);
                    self.white_pawn_bitboard.set_bit(63 - idx);
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
