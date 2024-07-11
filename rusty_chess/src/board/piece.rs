use raylib::prelude::*;
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
pub struct MovementVector {
    pub x_range: [i8; 2],
    pub y_range: [i8; 2],
    //indicate if the number of spaces it can move is fixed
    pub fixed: bool,
    //indicate if the piece can move diagonally
    pub diagonal: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub rect: Rectangle,
    pub owner: u8,
    pub piece_type: PieceType,
    pub origin: (u8, u8),
    pub mv_vec: Option<MovementVector>
}


impl Piece {
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
            mv_vec: None
        }
    }
}
