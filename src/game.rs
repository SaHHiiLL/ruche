use crate::board::{self, Move, MoveError, Piece, PieceColor, PieceType};
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Default)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x.floor() == other.x.floor() && self.y.floor() == other.y.floor()
    }
}

trait ToVector2 {
    fn to_vec2(&self) -> Vector2;
}

impl ToVector2 for usize {
    fn to_vec2(&self) -> Vector2 {
        let x: f32 = (*self as f32 % 8.0).floor();
        let y: f32 = (*self as f32 / 8.0).floor();

        Vector2 { x, y }
    }
}

pub struct Game {
    _size: u32,
    x_offset: u32,
    y_offset: u32,
    cell_size: u32,
    pub board: board::Board,

    cursor: Vector2,
    pub selected: Option<Vector2>,
    image_map: HashMap<Piece, raylib::core::texture::Texture2D>,

    pub pawn_promotion: bool,
    can_promote_to: Vec<Move>,
    pawn_promotion_img_map: HashMap<Piece, raylib::core::texture::Texture2D>,

    pawn_promotion_from_to: (usize, usize),
}

impl Game {
    pub fn new(_size: u32, x: u32, y: u32) -> Self {
        Self {
            _size,
            x_offset: x,
            y_offset: y,
            cell_size: _size / 8,
            board: board::Board::new(),

            cursor: Vector2 { x: 0.0, y: 0.0 },
            selected: None,
            image_map: HashMap::new(),

            pawn_promotion: false,
            can_promote_to: vec![],
            pawn_promotion_img_map: HashMap::new(),
            pawn_promotion_from_to: (0, 0),
        }
    }

    pub fn debug(&self) {
        self.board.print_debug();
    }

    pub fn draw_piece<T>(&self, d: &mut T, i: usize, piece: Piece)
    where
        T: raylib::core::drawing::RaylibDraw,
    {
        if piece.get_type() == PieceType::None {
            return;
        }

        let texture = self.image_map.get(&piece).unwrap();
        let x = i % 8;
        let y = i / 8;

        d.draw_texture(
            texture,
            (x as u32 * self.cell_size + self.x_offset) as i32,
            (y as u32 * self.cell_size + self.y_offset) as i32,
            raylib::core::color::Color::WHITE,
        );
    }

    fn draw_piece_for_promotion<T>(&self, d: &mut T, x: i32, piece: Piece)
    where
        T: raylib::core::drawing::RaylibDraw,
    {
        let texture = self.pawn_promotion_img_map.get(&piece).unwrap();
        d.draw_texture(
            texture,
            x,
            self.y_offset as i32,
            raylib::core::color::Color::WHITE,
        );
    }

    pub fn unset_selected(&mut self) {
        self.selected = None;
    }

    pub fn make_move(&mut self) {
        if self.selected.is_none() {
            return;
        }

        let selected = self.selected.clone().unwrap();
        let from = self
            .board
            .get_square(selected.x as usize, selected.y as usize);
        let to = self
            .board
            .get_square(self.cursor.x as usize, self.cursor.y as usize);

        //TODO: chanege None to pawn promotion
        match self.board.make_move(from, to, None) {
            Ok(_) => {
                self.board.toggle_turn();
                self.board.generate_moves_current_position();
                self.unset_selected();
                self.pawn_promotion = false;
            }
            Err(e) => {
                if let MoveError::MultipleLeagalMove(moves) = e {
                    self.pawn_promotion = true;
                    self.can_promote_to.clear();
                    self.can_promote_to.extend(moves);
                    self.pawn_promotion_from_to = (from, to);
                } else {
                    tracing::debug!("Invalid Move");
                }
            }
        }
    }

    pub fn draw_board<T>(&self, d: &mut T)
    where
        T: raylib::core::drawing::RaylibDraw,
    {
        let white = raylib::core::color::Color::from_hex("EBECD0").expect("Error parsing hex");
        let black = raylib::core::color::Color::from_hex("739552").expect("Error parsing hex");

        let cursor_color =
            raylib::core::color::Color::from_hex("ffee80").expect("Error parsing hex");
        let selected_color =
            raylib::core::color::Color::from_hex("8ab7ff").expect("Error parsing hex");

        let legal_color =
            raylib::core::color::Color::from_hex("ff11ff").expect("Error parsing hex");

        let copy_arr = self.board.clone_board();

        for (idx, p) in copy_arr.iter().enumerate() {
            let x = idx % 8;
            let y = idx / 8;
            let color = if (x + y) % 2 == 0 { white } else { black };

            if self.cursor.x as usize == x as usize && self.cursor.y as usize == y as usize {
                d.draw_rectangle(
                    (self.x_offset + x as u32 * self.cell_size) as i32,
                    (self.y_offset + y as u32 * self.cell_size) as i32,
                    self.cell_size as i32,
                    self.cell_size as i32,
                    cursor_color,
                );
            } else if self.selected.is_some() {
                let selected = self.selected.clone().unwrap();
                if selected.x as usize == x as usize && selected.y as usize == y as usize {
                    d.draw_rectangle(
                        (self.x_offset + x as u32 * self.cell_size) as i32,
                        (self.y_offset + y as u32 * self.cell_size) as i32,
                        self.cell_size as i32,
                        self.cell_size as i32,
                        selected_color,
                    );
                } else {
                    d.draw_rectangle(
                        (self.x_offset + x as u32 * self.cell_size) as i32,
                        (self.y_offset + y as u32 * self.cell_size) as i32,
                        self.cell_size as i32,
                        self.cell_size as i32,
                        color,
                    );
                }

                if let Some(selected) = &self.selected {
                    let moves = self.board.get_moves();
                    let moves = moves
                        .iter()
                        .filter(|x| {
                            let init = x.from.to_vec2();
                            return init.eq(&selected);
                        })
                        .collect::<Vec<_>>();

                    let moves = moves
                        .iter()
                        .map(|f| f.to.to_vec2())
                        .find(|v| v.x.floor() as usize == x && v.y.floor() as usize == y);

                    if let Some(found) = moves {
                        d.draw_rectangle(
                            (self.x_offset + found.x as u32 * self.cell_size) as i32,
                            (self.y_offset + found.y as u32 * self.cell_size) as i32,
                            self.cell_size as i32,
                            self.cell_size as i32,
                            legal_color,
                        );
                    }
                }
            } else {
                d.draw_rectangle(
                    (self.x_offset + x as u32 * self.cell_size) as i32,
                    (self.y_offset + y as u32 * self.cell_size) as i32,
                    self.cell_size as i32,
                    self.cell_size as i32,
                    color,
                );
            }

            self.draw_piece(d, idx, (*p).into());
        }

        if self.pawn_promotion {
            let y = self.y_offset;
            let pr = raylib::core::color::Color::from_hex("11fff0").expect("Error parsing hex");

            let piece_color = self.board.get_turn();
            let promotion_piece = [
                Piece {
                    piece_type: PieceType::Bishop,
                    piece_color,
                },
                Piece {
                    piece_type: PieceType::Knight,
                    piece_color,
                },
                Piece {
                    piece_type: PieceType::Rook,
                    piece_color,
                },
                Piece {
                    piece_type: PieceType::Queen,
                    piece_color,
                },
            ];

            for (x, p) in promotion_piece.iter().enumerate() {
                let x = self.x_offset + x as u32 * (self.cell_size * 2);

                d.draw_rectangle(
                    x as i32,
                    y as i32,
                    self.cell_size as i32 * 2,
                    self.cell_size as i32 * 2,
                    pr,
                );
                self.draw_piece_for_promotion(d, x as i32, *p);
            }
        }
    }

    pub fn selected_pawn_promotion(&mut self, idx: usize) {
        let promotion_piece = [
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
            PieceType::Queen,
        ];
        match self.board.make_move(
            self.pawn_promotion_from_to.0,
            self.pawn_promotion_from_to.1,
            Some(promotion_piece[idx]),
        ) {
            Ok(_) => {
                self.board.toggle_turn();
                self.board.generate_moves_current_position();
                self.unset_selected();
                self.pawn_promotion = false;
            }
            Err(e) => {
                if let MoveError::MultipleLeagalMove(moves) = e {
                    unreachable!("Should not happen");
                } else {
                    tracing::debug!("Invalid Move");
                }
            }
        };
    }

    pub fn follow_mouse(&mut self, d: &raylib::core::RaylibHandle) {
        let mouse = d.get_mouse_position();
        self.cursor.x = (mouse.x - self.x_offset as f32) / self.cell_size as f32;
        self.cursor.y = (mouse.y - self.y_offset as f32) / self.cell_size as f32;
    }

    pub fn select_piece(&mut self, d: &raylib::core::RaylibHandle) {
        let x = self.cursor.x as usize;
        let y = self.cursor.y as usize;

        let piece = self.board.get_piece_at_index(self.board.get_square(x, y));

        if piece.get_type() == PieceType::None {
            return;
        }

        if self.selected.is_some() {
            self.selected = None;
        }

        if piece.get_color() == self.board.get_turn() {
            self.selected = Some(Vector2 {
                x: self.cursor.x,
                y: self.cursor.y,
            });
        } else {
            tracing::info!("Wrong turn: {:?} ", self.board.get_turn());
        }
    }

    fn load_images_for_pawn_promotion(&mut self) {
        let pieces = [
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::Black,
            },
        ];

        for piece in pieces.iter() {
            self.pawn_promotion_img_map
                .insert(*piece, self.get_texture(piece, self.cell_size as i32 * 2));
        }
    }

    pub fn load_images(&mut self) {
        self.load_images_for_pawn_promotion();
        let pieces = [
            Piece {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::King,
                piece_color: PieceColor::White,
            },
            Piece {
                piece_type: PieceType::Pawn,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Rook,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Knight,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Bishop,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::Queen,
                piece_color: PieceColor::Black,
            },
            Piece {
                piece_type: PieceType::King,
                piece_color: PieceColor::Black,
            },
        ];

        for piece in pieces.iter() {
            self.image_map
                .insert(*piece, self.get_texture(piece, self.cell_size as i32));
        }
    }

    fn get_texture(&self, piece: &Piece, size: i32) -> raylib::core::texture::Texture2D {
        let mut buffer = String::from("./resource/output/");
        match piece.get_color() {
            PieceColor::White => buffer.push('w'),
            PieceColor::Black => buffer.push('b'),
        }

        match piece.get_type() {
            PieceType::Pawn => buffer.push('P'),
            PieceType::Rook => buffer.push('R'),
            PieceType::Knight => buffer.push('N'),
            PieceType::Bishop => buffer.push('B'),
            PieceType::Queen => buffer.push('Q'),
            PieceType::King => buffer.push('K'),
            PieceType::None => panic!("Invalid piece type"),
        }
        buffer.push_str(".svg.png");
        // check if the file exists

        if !Path::new(&buffer).exists() {
            tracing::error!("File does not exist: {:?}", buffer);
            panic!("File does not exist: {:?}", buffer);
        }

        let mut image = raylib::core::texture::Image::load_image(&buffer)
            .map_err(|err| {
                tracing::error!("Error loading image: {:?}", err);
            })
            .expect("Error loading image");

        image.resize(size, size);
        //
        // SAFETY: LoadTextureFromImage is a safe function
        unsafe {
            let texture = raylib::core::texture::Texture2D::from_raw(
                raylib::ffi::LoadTextureFromImage(*image),
            );
            return texture;
        }
    }
}
