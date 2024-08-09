use bevy::prelude::{Res, ButtonInput, MouseButton};

pub fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        println!("Wääää");
    }
}