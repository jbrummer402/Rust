use raylib::prelude::*;
use crate::board::piece::*;

#[derive(Copy, Clone)]
pub struct Space {
    pub rect: Rectangle,
    pub piece: Option<Piece>,
    pub is_occupied: bool
}

impl Space {
    pub fn default() -> Space {
        Space {
            rect: Rectangle {
                    x: 0.0 * 60.0,
                    y: 0.0 * 60.0,
                    width: 60.0,
                    height: 60.0,
                },
            piece: None,
            is_occupied: false,
        }
    }
    
    pub fn new(coords: (u8,u8), piece: Option<Piece>, is_occupied: bool) -> Space {
        let space_rect: Rectangle = Rectangle {
                    x: coords.0 as f32 * 60.0,
                    y: coords.1 as f32 * 60.0,
                    width: 60.0,
                    height: 60.0,
                };
        Space {
            rect: space_rect,
            piece,
            is_occupied,
        }
    }
}
