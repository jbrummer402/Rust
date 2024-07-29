use crate::components::menu;
use crate::board::piece::{Piece, PieceType};

use std::fmt::Error;
use glob::glob;
use std::collections::HashMap;
use raylib::prelude::*;

const WIDTH: i32 = 960;
const HEIGHT: i32 = 540;

mod game;
mod board;
mod components;

fn piece_index(str: &String) -> (Result<usize, Error>, u8) {
    if str.contains("white") {
        (Ok(str.find("white").unwrap()), 1)
    } else {
        (Ok(str.find("black").unwrap()), 2)
    }
}
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
fn load_pieces_textures(
        rl: &mut RaylibHandle,
        thread: RaylibThread,
    ) -> Result<HashMap<PieceType,Vec<Texture2D>>, Error> {

    let mut piece_textures = HashMap::<PieceType, Vec<Texture2D>>::new();
        for file in glob("./imgs/pieces-basic-png/*").expect("D.piece_rect.ry not found") {
            let f = file.unwrap();
            let texture = rl.load_texture(
                &thread,
                f.clone().into_os_string().to_str().expect("failed"),
            );
            let t = texture.unwrap();

            let path_substring = &(f.clone().into_os_string().into_string().unwrap());

            let (index, _owner) = piece_index(&String::from(path_substring));

            let name = string_to_piece_name(&path_substring[index? + 6..path_substring.len() - 4]);

            let p_name = &name.unwrap();

            match piece_textures.entry(*p_name) {
                std::collections::hash_map::Entry::Occupied(mut entry) => {
                    entry.get_mut().push(t);
                }

                std::collections::hash_map::Entry::Vacant(_entry) => {
                    let mut texture_vec = Vec::<Texture2D>::new();
                    texture_vec.push(t);
                    piece_textures.insert(*p_name, texture_vec);
                }
            }
        }
        Ok(piece_textures)
}

fn main() -> Result<(), Error> {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("Rusty Chess")
        .build();

    let piece_textures = load_pieces_textures(&mut rl, thread.clone()).unwrap();
    let mut g = game::Game::default();

    while !(rl.window_should_close()) {
        let d: &mut RaylibDrawHandle<'_> = &mut rl.begin_drawing(&thread);
        match g.state {
            1 => menu::create_menu(d, &mut g.state),
            0 => g.run(d, thread.clone(), &piece_textures)?,
            _ => print!("asd"), 
        }
        d.clear_background(Color::WHITE);
    }

    Ok(())
}
