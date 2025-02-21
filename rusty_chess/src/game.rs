use crate::board::space::Space;
use crate::board::piece::{Piece, PieceType};
use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::fmt::{Error};
use std::ops::Neg;
use std::collections::HashMap;
use std::{panic, u8};

pub struct Game {
    pub turn: bool,
    pub layout: [[Space; 8]; 8],
    // Map the names of pieces to the number of them left
    pub state: u8,
    offset: Vector2,
    dragging_piece: Option<Piece>,
    dragging: bool,
    mouse_coords: (usize, usize)
}

impl Game {

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

    fn draw_pieces(&mut self, d: &mut RaylibDrawHandle, textures: &HashMap<PieceType, Vec<Texture2D>>) {
        for row in 0..self.layout.len() {
            for col in 0..self.layout[row].len() {
                let identity = match self.layout[row][col].piece {
                    Some(p) => p.owner,
                    None => continue,
                };

                let p_text = 
                    &textures
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

    pub fn run(&mut self, d: &mut RaylibDrawHandle, thread: RaylibThread, textures: &HashMap<PieceType, Vec<Texture2D>>) -> Result<(), Error> {
        d.clear_background(Color::WHITE);
        self.draw_board(d);
        if d.is_mouse_button_pressed(MOUSE_BUTTON_LEFT) {
            self.mouse_coords = Game::to_board_cooridinates(d.get_mouse_position());

            self.dragging_piece = match self.get_piece_at(d.get_mouse_position()) {
                Some(p) => {
                    self.dragging = true;
                    self.offset.x = d.get_mouse_x() as f32 - self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.x;
                    self.offset.y = d.get_mouse_y() as f32 - self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.y;
                    Some(p)
                },
                None => { 
                    None
                } 
            };
        } else if d.is_mouse_button_released(MOUSE_BUTTON_LEFT) {
            self.dragging = false;
            match self.dragging_piece {
                Some(value) => {
                    self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.x = value.rect.x;
                    self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.y = value.rect.y;
                },
                None => {
                    println!("no piece there")
                } 
            }
            self.dragging_piece = None;
        }

        if self.dragging {
            self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.x = d.get_mouse_x() as f32 - self.offset.x;
            self.layout[self.mouse_coords.0][self.mouse_coords.1].rect.y = d.get_mouse_y() as f32 - self.offset.y;
        }             

        if self.dragging_piece.is_some() {
            match self.dragging_piece.unwrap().piece_type {
               PieceType::Bishop => {
                self.dragging_piece.expect("WHY ISN'T THERE A PIECE HERE").draw_diagonal(&self, d, (self.mouse_coords.0 as i8, self.mouse_coords.1 as i8));
               } 
               _ => {
                self.dragging_piece.expect("WHY ISN'T THERE A PIECE HERE").draw_straight(&self, d, (self.mouse_coords.0 as i8, self.mouse_coords.1 as i8));
               }
            }
        }

        self.draw_pieces(d, textures);
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
                println!("{:?}", layout[row][col].piece)
            }
        }

        Game {
            turn: true,
            layout,
            state: 0,
            offset: Vector2::default(),
            dragging_piece: None,
            dragging: false,
            mouse_coords: (0, 0)
        }
        }
    }
