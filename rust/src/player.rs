use godot::classes::{ InputEvent, InputEventMouse };
use godot::prelude::*;

use godot::{ classes::{ Sprite2D, ISprite2D }, obj::Base, prelude::GodotClass };

#[derive(GodotClass)]
#[class(base = Sprite2D)]
pub struct Player {
    base: Base<Sprite2D>,
}

#[godot_api]
impl ISprite2D for Player {
    fn init(base: Base<Sprite2D>) -> Self {
        Self {
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let radians = 3.4 * (delta as f32);
        self.base_mut().rotate(radians)
    }
    fn input(&mut self, event: Gd<InputEvent>) {
        // godot_print!("{:?}", event);

        match event.try_cast::<InputEventMouse>() {
            Ok(e) => {
                // godot_print!("{:?}", e.get_position());
                let mouse_position = e.get_position();
                self.base_mut().set_position(mouse_position);
                return;
            }
            Err(_) => {}
        }
        // if let Some(key_event) = event.as_event::<InputEventKey>() {
        //     if key_event.pressed() && key_event.scancode() == Key::Escape.scancode() {
        //         godot::godot_print!("Escape pressed");
        //     }
        // }

      
    }
}
