#![deny(missing_docs)]

use std::ops::Not;

use iter_tools::Itertools;

/// Represents the color of a given piece
#[derive(Eq, Hash, PartialEq, Debug, Clone, Copy)]
pub enum PieceColor {
    White = 0,
    Black = 8,
}

/// Represents the type of a given piece
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

/// Represents a move on a board from idex to idex with a movetype
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub move_type: MoveType,
}

/// Represents different types of moves
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum MoveType {
    #[default]
    None,
    PawnPush {
        promotion_piece: Option<PieceType>,
    },
    PawnDoublePush,
    PawnCapture {
        promotion_piece: Option<PieceType>,
    }, // When a pawn captures a piece
    PawnEnPassant(Coordinate), // When a pawn captures a piece en passant

    QueenMove,
    RookMove,
    BishopMove,
    KnightMove,
    KingMove,

    CastelKingSide,
    CastelQueenSide,
}

/// Represents a single piece.
///
/// a 4bit integer is used to represent the piece
///
/// 0000
///
/// first bit is used as colour bit and last 3 bit is used as piece type bit
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

    pub fn is_none(&self) -> bool {
        self.piece_type == PieceType::None
    }
}

/// The direction a pawn is able to capture
enum PawnCaptureDirection {
    Left,
    Right,
}

impl Piece {
    /// Returns colors
    pub fn get_color(&self) -> PieceColor {
        self.piece_color
    }

    /// Returns type
    pub fn get_type(&self) -> PieceType {
        self.piece_type
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
/// Coordinates of the board as x and y
/// where (0, 0) -> h1 (aka idx 0)
/// where (7, 7) -> a8 (aka idx 63)
pub struct Coordinate {
    x: usize,
    y: usize,
}

/// Uses isize instea of usize to safely determine can an index be out of bounds or not
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct SafeCoordinate {
    x: isize,
    y: isize,
}

impl SafeCoordinate {
    // TODO: pawn promote
    fn new(x: isize, y: isize) -> Self {
        SafeCoordinate { x, y }
    }

    fn is_out_of_bounds(&self) -> bool {
        self.x < 0 || self.x > 7 || self.y < 0 || self.y > 7
    }

    /// Convertes [SafeCoordinate] to [Coordinate]
    /// will assert the inner `x` and `y` are in bounds see [SafeCoordinate::is_out_of_bounds]
    fn to_coordinate(&self) -> Coordinate {
        assert!(!self.is_out_of_bounds());
        Coordinate {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}

#[derive(Debug, Default, Clone, Hash, Eq, PartialEq)]
/// a 64 bit integer matrix to represent the board
pub struct BitBoard {
    inner: u64,
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        BitBoard { inner: value }
    }
}

impl BitBoard {
    /// Sets a bit to 1 at idx
    pub fn set_bit(&mut self, idx: usize) {
        self.inner |= 1u64 << (idx);
    }

    /// Sets a bit to 0 at idx
    pub fn clear_bit(&mut self, idx: usize) {
        self.inner &= !(1u64 << (idx));
    }

    /// Gets the bit at idx
    pub fn get_bit(&self, idx: usize) -> bool {
        (self.inner & (1u64 << (idx))) != 0
    }

    /// Replaces the `inner` value to new `value`
    pub fn set(&mut self, value: u64) {
        self.inner = value;
    }

    pub fn zero(&mut self) {
        self.inner = 0;
    }
}

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

#[derive(Debug, Hash, Eq, PartialEq)]
/// Board Representation
///
/// TODO: make moves for both colour
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

    black_control_bitboard: BitBoard,
    white_control_bitboard: BitBoard,

    white_castling_right: BitBoard,
    black_castling_right: BitBoard,

    /// Each cell holds a value which represents a piece
    board: [u16; 64],

    /// Current avaliable moves
    current_moves: Vec<Move>,

    move_history: Vec<Move>,

    white_current_moves: Vec<Move>,
    black_current_moves: Vec<Move>,

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

            white_current_moves: Vec::new(),
            black_current_moves: Vec::new(),

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

            white_control_bitboard: BitBoard { inner: 0 },
            black_control_bitboard: BitBoard { inner: 0 },

            // specified values for right and left rooks on each colour complex
            white_castling_right: BitBoard { inner: 129 },
            black_castling_right: BitBoard {
                inner: 9295429630892703744,
            },
        }
    }

    pub fn get_moves(&self) -> Vec<Move> {
        self.all_moves()
    }

    fn get_moves_for_turn(&self) -> &[Move] {
        if self.is_white_turn {
            &self.white_current_moves
        } else {
            &self.black_current_moves
        }
    }

    //TODO: add king checks
    fn is_move_avaliable(&self, from: usize, to: usize) -> Option<Move> {
        for m in self.get_moves_for_turn().iter() {
            if m.from == from && m.to == to {
                return Some(m.clone());
            }
        }
        None
    }

    /// Adds moves to `self.current_moves` whilest updating the white/black board control bitboard
    fn update_color_control_square_for_move(&mut self, mov: Move, color: &PieceColor) {
        let bitboard = match color {
            PieceColor::White => &mut self.white_control_bitboard,
            PieceColor::Black => &mut self.black_control_bitboard,
        };
        // updating the control bit board, as king and pawn pushes cannot be used to check the
        // opponent, we can simply ignore them
        match mov.move_type {
            MoveType::PawnPush { .. } | MoveType::PawnDoublePush | MoveType::KingMove => { /* Do nothing */
            }
            _ => bitboard.set_bit(mov.to),
        }
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
            tracing::warn!("Move Not avaliable");
            return false;
        };

        assert!(mo.from == from);
        assert!(mo.to == to);

        match mo.move_type {
            MoveType::PawnDoublePush => {
                self.move_piece(&mo);
            }
            MoveType::PawnPush { promotion_piece } => {
                if let Some(promoting_to) = promotion_piece {
                    self.promote_pawn(&mo, promoting_to);
                }
                self.move_piece(&mo);
            }
            MoveType::PawnEnPassant(capture_piece) => {
                let pawn_to_capture_idx = self.get_square(capture_piece.x, capture_piece.y);
                let pawn_to_capture = self.get_piece_at_index(pawn_to_capture_idx);
                assert!(pawn_to_capture.get_type() == PieceType::Pawn);
                assert!(pawn_to_capture.get_color() != piece.get_color());
                assert!(target.get_type() == PieceType::None);
                let bitboard = self.get_bitboard_from_piece(piece);
                bitboard.clear_bit(pawn_to_capture_idx);
                self.board[pawn_to_capture_idx] = 0;
                self.move_piece(&mo);
            }
            MoveType::None => todo!(),
            MoveType::PawnCapture { promotion_piece } => {
                if let Some(promoting_to) = promotion_piece {
                    self.promote_pawn(&mo, promoting_to);
                }
                self.capture_piece(&mo);
                self.move_piece(&mo);
            }
            MoveType::KingMove => {
                // if the target square is not empty we need to capture the piece
                if target.get_type() != PieceType::None {
                    self.capture_piece(&mo);
                }
                self.move_piece(&mo);
                // Setting casteling right for both side to none
                if piece.get_color() == PieceColor::White {
                    self.white_castling_right.set(0);
                } else {
                    self.black_castling_right.set(0);
                }
            }
            MoveType::RookMove => {
                // if the target square is not empty we need to capture the piece
                if target.get_type() != PieceType::None {
                    self.capture_piece(&mo);
                }
                self.move_piece(&mo);
                // Setting casteling right for both side to none
                if piece.get_color() == PieceColor::White {
                    self.white_castling_right.clear_bit(from);
                } else {
                    self.black_castling_right.clear_bit(from);
                }
            }

            MoveType::QueenMove | MoveType::BishopMove | MoveType::KnightMove => {
                // if the target square is not empty we need to capture the piece
                if target.get_type() != PieceType::None {
                    self.capture_piece(&mo);
                }
                self.move_piece(&mo);
            }
            MoveType::CastelKingSide => {
                assert!(target.get_type() == PieceType::None);
                assert!(piece.get_type() == PieceType::King);
                self.move_piece(&mo);
                let (rook_pos, new_rook_pos) = if piece.get_color() == PieceColor::White {
                    (0, to + 1)
                } else {
                    (56, to + 1)
                };

                let rook_mov = Move {
                    from: rook_pos,
                    to: new_rook_pos,
                    move_type: MoveType::None,
                };
                self.move_piece(&rook_mov);

                // Setting casteling right for both side to none
                if piece.get_color() == PieceColor::White {
                    self.white_castling_right.set(0);
                } else {
                    self.black_castling_right.set(0);
                }
            }
            MoveType::CastelQueenSide => {
                assert!(target.get_type() == PieceType::None);
                assert!(piece.get_type() == PieceType::King);
                self.move_piece(&mo);
                let (rook_pos, new_rook_pos) = if piece.get_color() == PieceColor::White {
                    (7, to - 1)
                } else {
                    (63, to - 1)
                };
                let rook_mov = Move {
                    from: rook_pos,
                    to: new_rook_pos,
                    move_type: MoveType::None,
                };
                self.move_piece(&rook_mov);

                // Setting casteling right for both side to none
                if piece.get_color() == PieceColor::White {
                    self.white_castling_right.set(0);
                } else {
                    self.black_castling_right.set(0);
                }
            }
            MoveType::None => todo!(),
        }
        self.move_history.push(mo);

        true
    }

    fn promote_pawn(&mut self, mo: &Move, promoting_to: PieceType) {
        if !matches!(
            promoting_to,
            PieceType::Queen | PieceType::Bishop | PieceType::Knight | PieceType::Rook
        ) {
            tracing::error!("Invalid Piece to promote");
            tracing::error!("PieceType: {:?}", promoting_to);
            tracing::error!("Move: {:?}", mo);
            panic!("Invalid Piece Type for promotion");
        }
    }

    /// Only moves the piece on the board
    /// does not perform a capture and will fail the assert otherwise
    fn move_piece(&mut self, current_move: &Move) {
        let target = self.get_piece_at_index(current_move.to);
        assert!(target.get_type() == PieceType::None);
        let piece = self.get_piece_at_index(current_move.from);
        let bitboard = self.get_bitboard_from_piece(piece);
        bitboard.clear_bit(current_move.from);
        bitboard.set_bit(current_move.to);
        self.board[current_move.to] = self.board[current_move.from];
        self.board[current_move.from] = 0;
    }

    /// Captures the piece from the move
    /// does not move the piece in question
    fn capture_piece(&mut self, current_move: &Move) {
        let target = self.get_piece_at_index(current_move.to);
        assert!(target.get_type() != PieceType::None);
        let bitboard = self.get_bitboard_from_piece(target);
        bitboard.clear_bit(current_move.to);
        self.board[current_move.to] = 0;
    }

    fn clear_moves(&mut self) {
        self.white_current_moves.clear();
        self.black_current_moves.clear();
    }

    /// Clears the moves list and generates all possible moves for the current position
    /// This function should be called after each move
    pub fn generate_moves_current_position(&mut self) {
        self.clear_moves();
        assert!(self.white_current_moves.is_empty());
        assert!(self.black_current_moves.is_empty());

        let turn = self.get_turn();

        let board = self
            .board
            .iter()
            .map(|p| Piece::from(*p))
            .collect::<Vec<_>>();

        // Filters over the current turn pieces and generates all possible moves
        for (i, piece) in board.iter().enumerate() {
            // if piece.get_color() != turn {
            //     continue;
            // }
            let moves = match piece.piece_type {
                PieceType::Pawn => self.generate_pawn_moves(i, *piece),
                PieceType::Rook => self.generate_rook_moves(i, *piece),
                PieceType::Bishop => self.generate_bishop_moves(i, *piece),
                PieceType::Queen => self.generate_queen_moves(i, *piece),
                PieceType::Knight => self.generate_knight_moves(i, *piece),
                PieceType::King => self.generate_king_moves(i, *piece),
                PieceType::None => {
                    continue;
                }
            };

            match piece.get_color() {
                PieceColor::White => self.white_current_moves.extend(moves),
                PieceColor::Black => self.black_current_moves.extend(moves),
            }
        }
        self.white_control_bitboard.zero();
        self.black_control_bitboard.zero();

        for x in self.all_moves().iter() {
            self.update_color_control_square_for_move(x.clone(), &turn);
        }
    }

    fn all_moves(&self) -> Vec<Move> {
        [
            self.white_current_moves.clone(),
            self.black_current_moves.clone(),
        ]
        .concat()
        .to_vec()
    }

    fn generate_queen_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::Queen);
        let directions = [
            SafeCoordinate::new(1, 1),
            SafeCoordinate::new(-1, 1),
            SafeCoordinate::new(1, -1),
            SafeCoordinate::new(-1, -1),
            SafeCoordinate::new(0, 1),
            SafeCoordinate::new(0, -1),
            SafeCoordinate::new(1, 0),
            SafeCoordinate::new(-1, 0),
        ];
        self.generate_moves_for_direction(
            current_piece_idx,
            piece,
            &directions,
            MoveType::QueenMove,
        )
    }

    // TODO: checks
    fn generate_king_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::King);
        let directions = [
            SafeCoordinate::new(1, 1),
            SafeCoordinate::new(-1, 1),
            SafeCoordinate::new(1, -1),
            SafeCoordinate::new(-1, -1),
            SafeCoordinate::new(0, 1),
            SafeCoordinate::new(0, -1),
            SafeCoordinate::new(1, 0),
            SafeCoordinate::new(-1, 0),
        ];
        let mut res = vec![];
        for dir in directions.iter() {
            let current = self.get_safe_coordinates_from_index(current_piece_idx);
            let target = SafeCoordinate {
                x: current.x + dir.x,
                y: current.y + dir.y,
            };
            if target.is_out_of_bounds() {
                continue;
            }
            let idx = self.get_square_isize(target.x, target.y);
            let target_piece = self.get_piece_at_index(idx);
            if target_piece.get_type() != PieceType::None
                && target_piece.get_color() == piece.get_color()
            {
                continue;
            }
            res.push(Move {
                from: current_piece_idx,
                to: idx,
                move_type: MoveType::KingMove,
            });
        }

        res.extend(self.generate_king_castle_moves(current_piece_idx, piece));

        res
    }

    /// Generates king moves for both sides,
    /// NOTE: without regards to checks
    fn generate_king_castle_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        let mut res = vec![];
        let expected_king_pos = if piece.get_color() == PieceColor::White {
            3
        } else {
            59
        };

        // Return early if king is not at the correct position to castle
        if current_piece_idx != expected_king_pos {
            return res;
        }
        let king_side_path_idx: [usize; 2] = if piece.get_color() == PieceColor::White {
            [1, 2]
        } else {
            [56, 57]
        };
        let queen_side_path_idx: [usize; 3] = if piece.get_color() == PieceColor::White {
            [4, 5, 6]
        } else {
            [60, 61, 62]
        };

        /// checks if the path to the finish line has no piece
        fn all_clear(
            path: &[usize],
            board: &[u16; 64],
            opponent_control_bitboard: &BitBoard,
        ) -> bool {
            for x in path.iter() {
                let piece = Piece::from(board[*x]);
                if !piece.is_none() || opponent_control_bitboard.get_bit(*x) {
                    tracing::debug!("Not Clear{:?}", path);
                    return false;
                }
            }
            tracing::debug!("All clear for this side {:?}", path);
            true
        }

        let (rook, h_file_idx, a_file_idx, opp_control_bitboard) = if self.is_white_turn {
            (
                Piece {
                    piece_color: PieceColor::White,
                    piece_type: PieceType::Rook,
                },
                0,
                7,
                &self.black_control_bitboard,
            )
        } else {
            (
                Piece {
                    piece_color: PieceColor::Black,
                    piece_type: PieceType::Rook,
                },
                56,
                63,
                &self.white_control_bitboard,
            )
        };

        if self.get_piece_at_index(h_file_idx) == rook
            && all_clear(&king_side_path_idx, &self.board, opp_control_bitboard)
        {
            let mov = Move {
                from: expected_king_pos,
                to: expected_king_pos - 2,
                move_type: MoveType::CastelKingSide,
            };
            res.push(mov);
        }

        if self.get_piece_at_index(a_file_idx) == rook
            && all_clear(&queen_side_path_idx, &self.board, opp_control_bitboard)
        {
            let mov = Move {
                from: expected_king_pos,
                to: expected_king_pos + 2,
                move_type: MoveType::CastelQueenSide,
            };
            res.push(mov);
        }

        res
    }

    fn generate_knight_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::Knight);
        let directions = [
            SafeCoordinate::new(1, 2),
            SafeCoordinate::new(-1, 2),
            SafeCoordinate::new(1, -2),
            SafeCoordinate::new(-1, -2),
            SafeCoordinate::new(2, 1),
            SafeCoordinate::new(-2, 1),
            SafeCoordinate::new(2, -1),
            SafeCoordinate::new(-2, -1),
        ];
        let current_cord = self.get_safe_coordinates_from_index(current_piece_idx);
        let mut res = vec![];
        for dir in directions.iter() {
            let target_cord = SafeCoordinate {
                x: current_cord.x + dir.x,
                y: current_cord.y + dir.y,
            };

            if target_cord.is_out_of_bounds() {
                continue;
            }
            let target_cord = target_cord.to_coordinate();

            let target_piece =
                self.get_piece_at_index(self.get_square(target_cord.x, target_cord.y));

            if target_piece.get_type() != PieceType::None
                && target_piece.get_color() == piece.get_color()
            {
                continue;
            }

            res.push(Move {
                from: current_piece_idx,
                to: self.get_square(target_cord.x, target_cord.y),
                move_type: MoveType::KnightMove,
            });
        }
        res
    }

    fn generate_bishop_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::Bishop);
        let direction = [
            SafeCoordinate::new(1, 1),
            SafeCoordinate::new(-1, 1),
            SafeCoordinate::new(1, -1),
            SafeCoordinate::new(-1, -1),
        ];
        self.generate_moves_for_direction(
            current_piece_idx,
            piece,
            &direction,
            MoveType::BishopMove,
        )
    }

    fn generate_moves_for_direction(
        &mut self,
        current_piece_idx: usize,
        piece: Piece,
        directions: &[SafeCoordinate],
        move_type: MoveType,
    ) -> Vec<Move> {
        let mut res = vec![];
        let piece_cord = self.get_safe_coordinates_from_index(current_piece_idx);

        for dir in directions.iter() {
            let mut current_look_up_cord = SafeCoordinate {
                x: piece_cord.x + dir.x,
                y: piece_cord.y + dir.y,
            };
            'beyond: loop {
                if current_look_up_cord.is_out_of_bounds() {
                    break 'beyond;
                }
                let cluc = current_look_up_cord.to_coordinate();

                let current_look_up_piece = self.get_piece_at_index_from_cord(&cluc);

                if !current_look_up_piece.is_none() {
                    // fiendly piece cannot attack nor move, abandon the search for this
                    // direction
                    if current_look_up_piece.get_color() == piece.get_color() {
                        break 'beyond;
                    } else {
                        // enemy piece add it and move to the next direction - we exit afterwards
                        let mov = Move {
                            from: current_piece_idx,
                            to: self.get_index_from_coordinates(cluc),
                            move_type,
                        };
                        res.push(mov);
                        break 'beyond;
                    }
                }

                let mov = Move {
                    from: current_piece_idx,
                    to: self.get_index_from_coordinates(cluc),
                    move_type,
                };
                res.push(mov);
                current_look_up_cord = SafeCoordinate {
                    x: current_look_up_cord.x + dir.x,
                    y: current_look_up_cord.y + dir.y,
                };
            }
        }
        res
    }

    fn get_piece_at_index_from_cord(&self, cord: &Coordinate) -> Piece {
        self.get_piece_at_index(self.get_square(cord.x, cord.y))
    }

    fn generate_rook_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen);
        let directions = [
            SafeCoordinate::new(0, 1),
            SafeCoordinate::new(0, -1),
            SafeCoordinate::new(1, 0),
            SafeCoordinate::new(-1, 0),
        ];
        self.generate_moves_for_direction(current_piece_idx, piece, &directions, MoveType::RookMove)
    }

    fn generate_pawn_moves(&mut self, current_piece_idx: usize, piece: Piece) -> Vec<Move> {
        assert!(piece.piece_type == PieceType::Pawn);

        // Pawns can move forward one square if the square is empty
        // Pawns can move forward two squares if the square is empty and the pawn is on the starting rank
        // Pawns can capture diagonally
        // Pawns can capture en passant
        // Pawns can promote

        // if there a piece in front of the pawn it shall not move

        let co = self.get_safe_coordinates_from_index(current_piece_idx);
        let mut res = vec![];

        let front_co = SafeCoordinate {
            x: co.x,
            y: if piece.piece_color == PieceColor::White {
                co.y + 1
            } else {
                co.y - 1
            },
        };

        if !front_co.is_out_of_bounds() {
            // calculates the front of the pawn if it's white or black
            let front = self.get_square(front_co.x as usize, front_co.y as usize);
            let front_piece = self.get_piece_at_index(front);
            if front_piece.get_type() == PieceType::None {
                // Add front move to the list
                res.push(Move {
                    from: current_piece_idx,
                    to: front,
                    move_type: MoveType::PawnPush {
                        promotion_piece: None,
                    },
                });
                // checking for double push
                if co.y == 1 && piece.piece_color == PieceColor::White {
                    let double_front = self.get_square_isize(front_co.x, front_co.y + 1);
                    let double_front_piece = self.get_piece_at_index(double_front);
                    if double_front_piece.get_type() == PieceType::None {
                        res.push(Move {
                            from: current_piece_idx,
                            to: double_front,
                            move_type: MoveType::PawnDoublePush,
                        });
                    }
                } else if co.y == 6 && piece.piece_color == PieceColor::Black {
                    let double_front = self.get_square_isize(front_co.x, front_co.y - 1);
                    let double_front_piece = self.get_piece_at_index(double_front);
                    if double_front_piece.get_type() == PieceType::None {
                        res.push(Move {
                            from: current_piece_idx,
                            to: double_front,
                            move_type: MoveType::PawnDoublePush,
                        });
                    }
                }
            }
        }
        if let Some(m) = self.pawn_capture(piece, &co, PawnCaptureDirection::Right) {
            res.push(m);
        }

        if let Some(m) = self.pawn_capture(piece, &co, PawnCaptureDirection::Left) {
            res.push(m);
        }

        if let Some(m) = self.enpassant_capture(piece, &co) {
            res.push(m);
        }
        res
    }

    fn enpassant_capture(&self, piece: Piece, current_cord: &SafeCoordinate) -> Option<Move> {
        // if the last move by the opponent was a double pawn push on either side of the current
        // pawn we can capture it en passant

        if self.move_history.is_empty() {
            tracing::debug!("No Move history");
            return None;
        }

        let last_move = self.move_history.last()?;

        if last_move.move_type != MoveType::PawnDoublePush {
            return None;
        }

        let dir = [1, -1];

        for d in dir.iter() {
            let adj = SafeCoordinate {
                x: current_cord.x + *d,
                y: current_cord.y,
            };

            if adj.is_out_of_bounds() {
                continue;
            }
            let adj = adj.to_coordinate();

            let adj_piece = self.get_piece_at_index(self.get_square(adj.x, adj.y));
            if adj_piece.get_type() != PieceType::Pawn {
                continue;
            }
            if adj_piece.get_color() == piece.get_color() {
                continue;
            }
            let last_move_cord = self.get_coordinates_from_index(last_move.to);

            if last_move_cord.y == adj.y && last_move_cord.x == adj.x {
                let end_pos = Coordinate {
                    x: adj.x,
                    y: if piece.get_color() == PieceColor::White {
                        adj.y + 1
                    } else {
                        adj.y - 1
                    },
                };

                let mov = Move {
                    from: self.get_index_from_coordinates(current_cord.to_coordinate()),
                    to: self.get_index_from_coordinates(end_pos),
                    move_type: MoveType::PawnEnPassant(last_move_cord),
                };
                return Some(mov);
            }
        }
        None
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
                current_cord.y + 1
            } else {
                current_cord.y - 1
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
            return None;
        }

        let right = self.get_square_isize(right_co.x, right_co.y);
        let mov = Move {
            from: self.get_index_from_coordinates(current_cord.to_coordinate()),
            to: right,
            move_type: MoveType::PawnCapture {
                promotion_piece: None,
            },
        };
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
        println!();

        self.board
            .iter()
            .chunks(8)
            .into_iter()
            .enumerate()
            .for_each(|(idx, c)| {
                for (i, m) in c.enumerate() {
                    let idx = (idx * 8) + i;
                    print!("[{:0>2} {:0>2}] ", idx, m);
                }
                println!();
            });

        println!();

        println!("White control: {:?}", self.white_control_bitboard.inner);
        println!("Black control: {:?}", self.black_control_bitboard.inner);
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
                PieceType::None => panic!("Invalid Piece Type, {:?}", piece.piece_type),
            },
            PieceColor::Black => match piece.piece_type {
                PieceType::Pawn => &mut self.black_pawn_bitboard,
                PieceType::Rook => &mut self.black_rook_bitboard,
                PieceType::Knight => &mut self.black_knight_bitboard,
                PieceType::Bishop => &mut self.black_bishop_bitboard,
                PieceType::Queen => &mut self.black_queen_bitboard,
                PieceType::King => &mut self.black_king_bitboard,
                PieceType::None => panic!("Invalid Piece Type, {:?}", piece.piece_type),
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
        self.board[idx].into()
    }

    /// Loads a position from a FEN string
    /// ```no_run
    /// let mut board = Board::new();
    /// board.load_position("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    /// ```
    pub fn load_position(&mut self, fen: String) {
        let mut idx: usize = 63;

        for c in fen.chars() {
            match c {
                '1'..='8' => {
                    let offset = c.to_digit(10).unwrap() as usize;
                    idx = idx.saturating_sub(offset);
                }
                'r' => {
                    self.black_rook_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Rook as u16;
                    idx = idx.saturating_sub(1);
                }
                'n' => {
                    self.black_knight_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Knight as u16;
                    idx = idx.saturating_sub(1);
                }
                'b' => {
                    self.black_bishop_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Bishop as u16;
                    idx = idx.saturating_sub(1);
                }
                'q' => {
                    self.black_queen_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Queen as u16;
                    idx = idx.saturating_sub(1);
                }
                'k' => {
                    self.black_king_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::King as u16;
                    idx = idx.saturating_sub(1);
                }
                'p' => {
                    self.black_pawn_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::Black as u16 | PieceType::Pawn as u16;
                    idx = idx.saturating_sub(1);
                }
                'R' => {
                    self.white_rook_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Rook as u16;
                    idx = idx.saturating_sub(1);
                }
                'N' => {
                    self.white_knight_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Knight as u16;
                    idx = idx.saturating_sub(1);
                }
                'B' => {
                    self.white_bishop_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Bishop as u16;
                    idx = idx.saturating_sub(1);
                }
                'Q' => {
                    self.white_queen_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Queen as u16;
                    idx = idx.saturating_sub(1);
                }
                'K' => {
                    self.white_king_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::King as u16;
                    idx = idx.saturating_sub(1);
                }
                'P' => {
                    self.white_pawn_bitboard.set_bit(idx);
                    self.board[idx] = PieceColor::White as u16 | PieceType::Pawn as u16;
                    idx = idx.saturating_sub(1);
                }
                '/' => {}
                _ => {
                    tracing::error!("Invalid FEN character: {}", c);
                }
            }
        }

        println!("{:?}", self.board);

        self.is_white_turn = self.is_white_turn.not();
        self.generate_moves_current_position();

        self.is_white_turn = self.is_white_turn.not();
        self.generate_moves_current_position();
    }
}
