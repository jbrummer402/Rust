use crate::board::piece::{Piece, PieceType};
use crate::board::piece::MovementVector;
use crate::board::space::{Space};
use glob::glob;
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::collections::HashMap;
use std::fmt::{Error};
use std::{panic, u8};

fn string_to_piece_name(s: &str) -> Result<PieceType, Error> {
    match s {
        "pawn" | "Pawn" => Ok(PieceType::Pawn),
        "rook" | "Rook" => Ok(PieceType::Rook),
        "bishop" | "Bishop" => Ok(PieceType::Bishop),
        "knight" | "Knight" => Ok(PieceType::Knight),
        "king" | "King" => Ok(PieceType::King),
        "queen" | "Queen" => Ok(PieceType::Queen),
        _ => panic!("Piece not found! Zoinks!"),
    }
}

pub struct Game {
    pub turn: bool,
    pub layout: [[Space; 8]; 8],
    // Map the names of pieces to the number of them left
    piece_textures: HashMap<PieceType, Vec<Texture2D>>,
}

impl Game {
    fn draw_straight(arg: Type) { 

    }

    fn draw_diagonal(&self, space: &Space, d: &mut RaylibDrawHandle, delta: (i8,i8), coords: (usize, usize)) -> bool {
        
        d.draw_rectangle(
            (60 * (space.rect.x / 60.0 + delta.0 as f32) as i8).into(),
            (60 * (space.rect.y / 60.0 + delta.1 as f32) as i8).into(),
            60,
            60,
            color::Color::new(255,0,0,150)
        );

        let next_space = self.layout[coords.0][coords.1];

        match self.layout[(space.rect.x as i8 + delta.0) as usize][(space.rect.y as i8 + delta.1) as usize].piece {
            Some(p) => false,
            None => self.draw_diagonal(&next_space, d, delta, (coords.0 + delta.0 as usize, coords.1 + delta.1 as usize))
        };
        
        false 
    }

    fn draw_valid_moves(&self, p: &Piece, d: &mut RaylibDrawHandle) {
        // Represent movement of pieces as vectors
        // eg: Pawn -> (0, +-1)
        // Bishop -> (+-X, +-Y)
        // etc etc
        let move_vector: MovementVector = match p.piece_type {
            PieceType::King => MovementVector {x_range: [-1,1], y_range:[-1,1], fixed: true, diagonal: true}, 
            PieceType::Queen => MovementVector {x_range: [-1,1], y_range:[-1,1], fixed: false, diagonal: true},
            PieceType::Bishop => MovementVector {x_range: [-1,1], y_range:[-1,1], fixed: false, diagonal: true},
            PieceType::Knight => MovementVector {x_range: [-1,1], y_range:[-1,1], fixed: true, diagonal: false},
            PieceType::Rook => MovementVector {x_range: [-1,1], y_range:[-1,1], fixed: false, diagonal: false},
            PieceType::Pawn => MovementVector {x_range: [0,0], y_range:[-1,1], fixed: true, diagonal: true},
        };

        let mut free_to_move = true; 
        let mut dx = 0;
        let mut dy = 0;

        if move_vector.diagonal {
            while free_to_move {
                // North east
                self.draw_diagonal(&self.layout[][], d, (1,1), (usize, usize)));

                // Northwest
                self.draw_diagonal(&self.layout[][], d, (-1,1), (usize, usize)));

                // Southeast
                self.draw_diagonal(&self.layout[][], d, (1,-1), (usize, usize)));

                // Southwest
                free_to_move = self.draw_diagonal(&self.layout[][], d, (-1,-1), (usize, usize)));
            }
        } else {
                d.draw_rectangle(
                    (60).into(),
                    (60).into(),
                    60,
                    60,
                    color::Color::new(255,0,0,150)
                );
            }
        
    }
    fn get_piece_at(&self, mouse_position: Vector2) -> Option<Piece> {
        let row = (mouse_position.x / 60.0).floor();
        let col = (mouse_position.y / 60.0).floor();

        if row >= 8.0 || col >= 8.0 {
            return None;
        }

        self.layout[row as usize][col as usize].piece
    }
    fn to_board_cooridinates(mouse_position: Vector2) -> (usize, usize) {
        let row = (mouse_position.x / 60.0).floor();
        let col = (mouse_position.y / 60.0).floor();

        (row as usize, col as usize)
    }

    fn draw_board(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..8 {
            for col in 0..8 {
                d.draw_rectangle(
                    60 * row,
                    60 * col,
                    60,
                    60,
                    if (col + row) % 2 == 0 {
                        color::Color::LIGHTGRAY
                    } else {
                        color::Color::GRAY
                    },
                );
            }
        }
    }

    fn draw_pieces(&mut self, d: &mut RaylibDrawHandle) {
        for row in 0..self.layout.len() {
            for col in 0..self.layout[row].len() {
                let identity = match self.layout[row][col].piece {
                    Some(p) => p.owner,
                    None => continue,
                };

                let p_text = &self
                    .piece_textures
                    .get(&self.layout[row][col].piece.unwrap().piece_type)
                    .unwrap()[identity as usize];

                // Draw texture for respective piece type
                d.draw_texture_pro(
                    p_text,
                    Rectangle {
                        x: 0.0,
                        y: 0.0,
                        width: p_text.width() as f32,
                        height: p_text.height() as f32,
                    },
                    self.layout[row][col].rect,
                    Vector2 { x: 0.0, y: 0.0 },
                    0.0,
                    Color::WHITE,
                );
            }
        }
    }

    pub fn run(&mut self, rl: &mut RaylibHandle, thread: RaylibThread) -> Result<(), Error> {
        // Load all the textures for each piece first
        let _ = self.load_pieces_textures(rl, thread.clone());
        let mut dragging = false;
        let mut offset = Vector2::default();

        let mut dragging_piece: Option<Piece> = None;
        let mut mouse_coords = Game::to_board_cooridinates(rl.get_mouse_position());

        while !(rl.window_should_close()) {
            if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
                mouse_coords = Game::to_board_cooridinates(rl.get_mouse_position());

                dragging_piece = match self.get_piece_at(rl.get_mouse_position()) {
                    Some(p) => {
                        dragging = true;
                        offset.x = rl.get_mouse_x() as f32
                            - self.layout[mouse_coords.0][mouse_coords.1].rect.x;
                        offset.y = rl.get_mouse_y() as f32
                            - self.layout[mouse_coords.0][mouse_coords.1].rect.y;
                        Some(p)
                    },
                    None => { 
                        None
                    } 
                };
            } else if rl.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
                dragging = false;
                self.layout[mouse_coords.0][mouse_coords.1].rect.x = dragging_piece.unwrap().rect.x;
                self.layout[mouse_coords.0][mouse_coords.1].rect.y = dragging_piece.unwrap().rect.y;
            }
            if dragging {
                self.layout[mouse_coords.0][mouse_coords.1].rect.x =
                    rl.get_mouse_x() as f32 - offset.x;
                self.layout[mouse_coords.0][mouse_coords.1].rect.y =
                    rl.get_mouse_y() as f32 - offset.y;

            }             // Begin drawing the textures after loading
            let d: &mut RaylibDrawHandle<'_> = &mut rl.begin_drawing(&thread);
            // clear background each frame
            d.clear_background(Color::WHITE);
            if dragging_piece.is_some() {
                self.draw_valid_moves(&dragging_piece.unwrap(), d);
                
            }
            self.draw_board(d);
            self.draw_pieces(d);
        }
        Ok(())
    }
    fn piece_index(str: &String) -> (Result<usize, Error>, u8) {
        if str.contains("white") {
            (Ok(str.find("white").unwrap()), 1)
        } else {
            (Ok(str.find("black").unwrap()), 2)
        }
    }

    fn load_pieces_textures(
        &mut self,
        rl: &mut RaylibHandle,
        thread: RaylibThread,
    ) -> Result<(), Error> {
        for file in glob("./imgs/pieces-basic-png/*").expect("D.piece_rect.ry not found") {
            let f = file.unwrap();
            let texture = rl.load_texture(
                &thread,
                f.clone().into_os_string().to_str().expect("failed"),
            );
            let t = texture.unwrap();

            let path_substring = &(f.clone().into_os_string().into_string().unwrap());

            let (index, _owner) = Self::piece_index(&String::from(path_substring));

            let name = string_to_piece_name(&path_substring[index? + 6..path_substring.len() - 4]);

            let p_name = &name.unwrap();

            match self.piece_textures.entry(*p_name) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(t);
                }

                std::collections::hash_map::Entry::Vacant(_entry) => {
                    let mut texture_vec = Vec::<Texture2D>::new();
                    texture_vec.push(t);
                    self.piece_textures.insert(*p_name, texture_vec);
                }
            }
        }
        Ok(())
    }

    pub fn default() -> Game {
        let mut layout: [[Space; 8]; 8] = [[Space::default(); 8]; 8];
        let piece_order = [2, 3, 4, 5, 6, 4, 3, 2];

        for row in 0..layout.len() {
            for col in 0..layout[row].len() {
                let space_rect: Rectangle = Rectangle {
                                x: col as f32 * 60.0,
                                y: row as f32 * 60.0,
                                width: 60.0,
                                height: 60.0
                            };

                let owner = if row < 2 { 0 } else { 1 };
                let piece_type = if row < 1 || row == 7 { piece_order[col] } else { 1 }; 

                let mut space: Space = Space {
                    rect: space_rect,
                    piece: None,
                    is_occupied: false,
                };

                if !(2..6).contains(&row) {
                    let piece: Piece = Piece::new(space, owner, piece_type);
                    space.piece = Some(piece);
                }

                layout[col][row] = space;
            }
        }

        Game {
            turn: true,
            layout,
            piece_textures: HashMap::<PieceType, Vec<Texture2D>>::new(),
            // Map the names of pieces to the number of them left
            // per player
        }
    }
}
