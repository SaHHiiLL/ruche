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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum MoveType {
    #[default]
    None,
    PawnPush,
    PawnDoublePush,
    PawnCapture,               // When a pawn captures a piece
    PawnEnPassant(Coordinate), // When a pawn captures a piece en passant

    QueenMove,
    RookMove,
    BishopMove,
    KnightMove,
    KingMove,
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

    pub fn is_none(&self) -> bool {
        self.piece_type == PieceType::None
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

    fn to_coordinate(&self) -> Coordinate {
        assert!(!self.is_out_of_bounds());
        Coordinate {
            x: self.x as usize,
            y: self.y as usize,
        }
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

    pub fn _get_bit(&self, idx: usize) -> bool {
        (self.inner & (1u64 << (idx))) != 0
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
            return false;
        };

        assert!(mo.from == from);
        assert!(mo.to == to);

        match mo.move_type {
            MoveType::PawnPush | MoveType::PawnDoublePush => {
                // if the move is a pawn push or double push
                // we need to update the bitboard
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
            MoveType::PawnCapture => {
                self.capture_piece(&mo);
                self.move_piece(&mo);
            }
            MoveType::QueenMove
            | MoveType::KingMove
            | MoveType::RookMove
            | MoveType::BishopMove
            | MoveType::KnightMove => {
                // if the target square is not empty we need to capture the piece
                if target.get_type() != PieceType::None {
                    self.capture_piece(&mo);
                }
                self.move_piece(&mo);
            }
            MoveType::None => todo!(),
        }
        self.move_history.push(mo);
        true
    }

    fn promote_pawn(&mut self, piece_to: Piece, mo: &Move) {
        todo!("Not implemented");
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
            match piece.piece_type {
                PieceType::Pawn => self.generate_pawn_moves(i, *piece),
                PieceType::Rook => self.generate_rook_moves(i, *piece),
                PieceType::Bishop => self.generate_bishop_moves(i, *piece),
                PieceType::Queen => self.generate_queen_moves(i, *piece),
                PieceType::Knight => self.generate_knight_moves(i, *piece),
                PieceType::King => self.generate_king_moves(i, *piece),
                PieceType::None => {
                    continue;
                }
            }
        }
    }

    fn generate_queen_moves(&mut self, current_piece_idx: usize, piece: Piece) {
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
        );
    }

    // TODO: checks
    fn generate_king_moves(&mut self, current_piece_idx: usize, piece: Piece) {
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
            self.current_moves.push(Move {
                from: current_piece_idx,
                to: idx,
                move_type: MoveType::KingMove,
            });
        }
    }

    fn generate_knight_moves(&mut self, current_piece_idx: usize, piece: Piece) {
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

            self.current_moves.push(Move {
                from: current_piece_idx,
                to: self.get_square(target_cord.x, target_cord.y),
                move_type: MoveType::KnightMove,
            });
        }
    }

    fn generate_bishop_moves(&mut self, current_piece_idx: usize, piece: Piece) {
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
        );
    }

    fn generate_moves_for_direction(
        &mut self,
        current_piece_idx: usize,
        piece: Piece,
        directions: &[SafeCoordinate],
        move_type: MoveType,
    ) {
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

                if !current_look_up_piece.is_none()
                    && current_look_up_piece.get_color() == piece.get_color()
                {
                    break 'beyond;
                }
                let mov = Move {
                    from: current_piece_idx,
                    to: self.get_index_from_coordinates(cluc),
                    move_type,
                };
                self.current_moves.push(mov);
                current_look_up_cord = SafeCoordinate {
                    x: current_look_up_cord.x + dir.x,
                    y: current_look_up_cord.y + dir.y,
                };
            }
        }
    }

    fn get_piece_at_index_from_cord(&self, cord: &Coordinate) -> Piece {
        self.get_piece_at_index(self.get_square(cord.x, cord.y))
    }

    fn generate_rook_moves(&mut self, current_piece_idx: usize, piece: Piece) {
        assert!(piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen);
        let directions = [
            SafeCoordinate::new(0, 1),
            SafeCoordinate::new(0, -1),
            SafeCoordinate::new(1, 0),
            SafeCoordinate::new(-1, 0),
        ];
        self.generate_moves_for_direction(
            current_piece_idx,
            piece,
            &directions,
            MoveType::RookMove,
        );
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
                        adj.y - 1
                    } else {
                        adj.y + 1
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
            return None;
        }

        let right = self.get_square_isize(right_co.x, right_co.y);
        let mov = Move {
            from: self.get_index_from_coordinates(current_cord.to_coordinate()),
            to: right,
            move_type: MoveType::PawnCapture,
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
