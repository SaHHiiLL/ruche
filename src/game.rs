use crate::board::{self, Piece, PieceColor, PieceType};
use iter_tools::Itertools;
use raylib::prelude::*;
use std::{collections::HashMap, path::Path};

#[derive(Debug, Clone, Default)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct Game {
    size: u32,
    x: u32,
    y: u32,
    cell_size: u32,
    pub board: board::Board,

    cursor: Vector2,
    pub selected: Option<Vector2>,
    image_map: HashMap<Piece, raylib::core::texture::Texture2D>,
}

impl Game {
    pub fn new(size: u32, x: u32, y: u32) -> Self {
        Self {
            size,
            x,
            y,
            cell_size: size / 8,
            board: board::Board::new(),

            cursor: Vector2 { x: 0.0, y: 0.0 },
            selected: None,
            image_map: HashMap::new(),
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
            (x as u32 * self.cell_size + self.x) as i32,
            (y as u32 * self.cell_size + self.y) as i32,
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

        tracing::info!("Making");
        let selected = self.selected.clone().unwrap();
        if self.board.make_move(
            self.board
                .get_square(selected.x as usize, selected.y as usize),
            self.board
                .get_square(self.cursor.x as usize, self.cursor.y as usize),
        ) {
            self.board.toggle_turn();
            self.board.generate_moves_current_position();
            self.unset_selected();
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

        // this is important as it loops over the board in reverse just to display it correctly
        let copy_arr = self.board.clone_board();

        for (idx, p) in copy_arr.iter().enumerate() {
            let x = idx % 8;
            let y = idx / 8;
            let color = if (x + y) % 2 == 0 { white } else { black };

            let pos = Vector2 {
                x: x as f32,
                y: y as f32,
            };

            if self.cursor.x as usize == x as usize && self.cursor.y as usize == y as usize {
                d.draw_rectangle(
                    (self.x + x as u32 * self.cell_size) as i32,
                    (self.y + y as u32 * self.cell_size) as i32,
                    self.cell_size as i32,
                    self.cell_size as i32,
                    cursor_color,
                );
            } else if self.selected.is_some() {
                let selected = self.selected.clone().unwrap();
                if selected.x as usize == x as usize && selected.y as usize == y as usize {
                    d.draw_rectangle(
                        (self.x + x as u32 * self.cell_size) as i32,
                        (self.y + y as u32 * self.cell_size) as i32,
                        self.cell_size as i32,
                        self.cell_size as i32,
                        selected_color,
                    );
                } else {
                    d.draw_rectangle(
                        (self.x + x as u32 * self.cell_size) as i32,
                        (self.y + y as u32 * self.cell_size) as i32,
                        self.cell_size as i32,
                        self.cell_size as i32,
                        color,
                    );
                }
            } else {
                d.draw_rectangle(
                    (self.x + x as u32 * self.cell_size) as i32,
                    (self.y + y as u32 * self.cell_size) as i32,
                    self.cell_size as i32,
                    self.cell_size as i32,
                    color,
                );
            }
            self.draw_piece(d, idx, (*p).into());
        }
    }

    pub fn follow_mouse(&mut self, d: &raylib::core::RaylibHandle) {
        let mouse = d.get_mouse_position();
        self.cursor.x = (mouse.x - self.x as f32) / self.cell_size as f32;
        self.cursor.y = (mouse.y - self.y as f32) / self.cell_size as f32;
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

    pub fn load_images(&mut self) {
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
                .insert(piece.clone(), self.get_texture(piece));
        }
    }

    fn get_texture(&self, piece: &Piece) -> raylib::core::texture::Texture2D {
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

        image.resize(self.cell_size as i32, self.cell_size as i32);
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
