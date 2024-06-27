use raylib::consts::MouseButton::*;
use raylib::prelude::*;
use std::fmt::Error;
use std::fs::File;
use std::io::{self, Read};
use crate::board::space::{Space};

#[derive(Debug, Eq, Hash, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub rect: Rectangle,
    pub identity: u8,
    pub piece_type: PieceType,
    pub is_dragging: bool,
}


impl Piece {
    pub fn is_valid_move(&self, target: Space) -> bool {
        let dx = (target.rect.x - self.rect.x).abs() / 60.0;
        let dy = (target.rect.y - self.rect.y).abs() / 60.0;

        match self.piece_type {
            PieceType::King => dx <= 1.0 && dy <= 1.0,
            PieceType::Queen => dx == dy || dx == 0.0 || dy == 0.0,
            PieceType::Bishop => dx == dy,
            PieceType::Rook  => dx == 0.0 || dy == 0.0,
            PieceType::Knight => (dx == 2.0 && dy == 1.0) || (dx == 1.0 && dy == 2.0),
            PieceType::Pawn => true, 
        };

        false
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

    pub fn new(space: Space, identity: u8, piece_type: u8) -> Piece {
        
        Piece {
            rect: space.rect,
            identity,
            piece_type: Self::piece_to_name(piece_type),
            is_dragging: false,
        }
    }
}
