use std::ops::Neg;

use raylib::prelude::*;
use crate::game::Game;
use crate::board::space::{Space};

enum Movement {
    Diagonal {max_steps: u8},
    Straight {max_steps: u8},
}

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn, 
    Rook,
    Knight,
    Bishop,
    Queen,
    King 
}

impl PieceType {

    fn gen_movement_vec(self, g: &Game, piece_struct: &Piece) -> Vec<Movement> {

        let movement_vec = match piece_struct.piece_type {
            // Pawn can move 1 or 2 spaces, forward or diagonal
            PieceType::Pawn => vec![Movement::Straight{max_steps:2}, Movement::Diagonal{max_steps:1}],

            // Rook can move any number of spaces in a straight line
            PieceType::Rook => vec![Movement::Straight{max_steps:8}],

            // Knight can move 2 spaces and then 3 spaces in straight lines
            PieceType::Knight => vec![Movement::Straight{max_steps:2}, Movement::Straight{max_steps:3}],

            // Bishop can move any number of spaces diagonally
            PieceType::Bishop => vec![Movement::Diagonal{max_steps:8}],

            // Queen can move any number of spaces in any direction
            PieceType::Queen => vec![Movement::Straight{max_steps:8}, Movement::Diagonal{max_steps:8}],

            // King can move one space in any direction
            PieceType::King => vec![Movement::Straight{max_steps:1}, Movement::Diagonal{max_steps:1}],
        };

        movement_vec
    }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub rect: Rectangle,
    pub owner: u8,
    pub piece_type: PieceType,
    pub origin: (u8, u8),
}


impl Piece {
    fn within_bounds(self, g: &Game, coords: (i8, i8)) -> bool {
        let rank = coords.0;
        let file = coords.1;

        if rank < 8 && rank >= 0 {
            if file < 8 && rank >= 0 {
                return true;
            }
        }
        false
    }

    pub fn draw_straight(self, g: &Game, d: &mut RaylibDrawHandle, coords: (i8, i8)) {
        // North and south
        for i in 0..8 {
            d.draw_rectangle_rec(
            g.layout[coords.0 as usize][i as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
            d.draw_rectangle_rec(
            g.layout[i as usize][coords.1 as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
        }
    }

    pub fn draw_diagonal(self, g: &Game, d: &mut RaylibDrawHandle, coords: (i8, i8)) {
        // Max amount of spaces will always be 8
        let file = coords.0;
        let rank = coords.1;

        let dx = (file - 7).abs();
        let dy = (rank - 7).abs();
        
        for i in 0..8 {
            if i + rank < 8 && i + file < 8 {
                println!("North east");
                d.draw_rectangle_rec(
                g.layout[(file + i) as usize][(rank + i) as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
            }
            if rank - i >= 0 && file - i >= 0 {
                d.draw_rectangle_rec(
                g.layout[(file - i) as usize][(rank - i) as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
            }
            if i + rank < 8 && file - i >= 0 {
                d.draw_rectangle_rec(
                g.layout[(rank + i) as usize][(file - i) as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
            }
            if rank - i >= 0 && i + file < 8 {
                d.draw_rectangle_rec(
                g.layout[(rank - i) as usize][(file + i) as usize].rect,
                color::Color {r: 255, g: 0, b: 0, a: 150 });
            }
        }

    }

    pub fn draw_valid_moves(self, g: &Game, d: &mut RaylibDrawHandle) {
        for i in self.origin.0..8 {
            match self.piece_type {
                PieceType::Bishop => {
                    d.draw_rectangle_rec(
                        g.layout[self.origin.0 as usize][self.origin.1 as usize].rect,
                        color::Color {r: 255, g: 0, b: 0, a: 255 });
                },
                _ => todo!(),
            }
        }
    }
    pub fn piece_to_name(index: u8) -> PieceType {
        match index {
            1 => PieceType::Pawn,
            2 => PieceType::Rook,
            3 => PieceType::Knight,
            4 => PieceType::Bishop,
            5 => PieceType::Queen,
            6 => PieceType::King,
            _ => PieceType::Pawn,
        }
    }

    pub fn new(space: Space, owner: u8, piece_type: u8) -> Piece {
        Piece {
            rect: space.rect,
            owner,
            piece_type: Self::piece_to_name(piece_type),
            origin: (space.rect.x as u8, space.rect.y as u8),
        }
    }
}
