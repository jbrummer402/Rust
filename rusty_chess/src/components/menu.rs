use raylib::prelude::*;
use raylib::consts::MouseButton::*;
use raylib::ffi::CheckCollisionPointRec;

const MENU_HEIGHT: i32 = 450;
const MENU_WIDTH: i32 = 684;

const BUTTON_WIDTH: f32 = 200.0;
const BUTTON_HEIGHT: f32 = 60.0;


pub fn create_menu(d: &mut RaylibDrawHandle, game_state: &mut u8) {
    let _menu_rect = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 512.0,
        height: 256.0
    };

    let new_game_rect = Rectangle {
        x: 240.0, 
        y: 400.0, 
        width: BUTTON_WIDTH, 
        height: BUTTON_HEIGHT
    };

    if d.is_mouse_button_released(MOUSE_LEFT_BUTTON) {
        unsafe {
        if CheckCollisionPointRec(d.get_mouse_position().into(), new_game_rect.into()) {
            *game_state = 1;
        }
    }
    }

    d.draw_rectangle((960 / 2) - (MENU_WIDTH/2), 540/2 - (MENU_HEIGHT/2), MENU_WIDTH, MENU_HEIGHT, Color::SKYBLUE);
    d.draw_text("Rusty Chess", MENU_WIDTH/2 - 90, (960 / 2) - (MENU_WIDTH/2), 75, Color::BLACK);

    d.draw_rectangle(240, 400, 200, 60, Color::GRAY);
    d.draw_rectangle(500, 400, 200, 60, Color::GRAY);
}

pub struct MainMenu {
    

}
