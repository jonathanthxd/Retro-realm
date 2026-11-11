use crate::render::{self, ButtonId};
use macroquad::prelude::*;
use realm_core::{Input, World};

fn virtual_point(point: Vec2, virtual_width: f32) -> Vec2 {
    vec2(
        point.x / screen_width().max(1.0) * virtual_width,
        point.y / screen_height().max(1.0) * 180.0,
    )
}
pub fn pressed_points(vw: f32) -> Vec<Vec2> {
    let mut result = Vec::new();
    if is_mouse_button_pressed(MouseButton::Left) {
        let (x, y) = mouse_position();
        result.push(virtual_point(vec2(x, y), vw));
    }
    for t in touches() {
        if t.phase == TouchPhase::Started {
            result.push(virtual_point(t.position, vw));
        }
    }
    result
}
fn active_buttons(vw: f32) -> Vec<ButtonId> {
    let mut result = Vec::new();
    if is_mouse_button_down(MouseButton::Left) {
        let (x, y) = mouse_position();
        let p = virtual_point(vec2(x, y), vw);
        for b in render::buttons(vw) {
            if (p - b.center).length_squared() <= b.radius * b.radius {
                result.push(b.id);
            }
        }
    }
    for t in touches() {
        if t.phase == TouchPhase::Ended || t.phase == TouchPhase::Cancelled {
            continue;
        }
        let p = virtual_point(t.position, vw);
        for b in render::buttons(vw) {
            if (p - b.center).length_squared() <= b.radius * b.radius {
                result.push(b.id);
            }
        }
    }
    result
}
pub fn pause_hit(vw: f32) -> bool {
    pressed_points(vw)
        .iter()
        .any(|p| p.x > vw - 26.0 && p.y < 28.0)
}
pub fn menu_hit(vw: f32) -> bool {
    pressed_points(vw)
        .iter()
        .any(|p| p.x > vw / 2.0 - 65.0 && p.x < vw / 2.0 + 65.0 && p.y > 105.0 && p.y < 128.0)
}
pub fn restart_hit(vw: f32) -> bool {
    pressed_points(vw)
        .iter()
        .any(|p| p.x > vw / 2.0 - 70.0 && p.x < vw / 2.0 + 70.0 && p.y > 95.0 && p.y < 145.0)
}
pub fn game_input(vw: f32, w: &World) -> Input {
    let buttons = active_buttons(vw);
    let just = pressed_points(vw);
    let pressed = |id| buttons.contains(&id);
    let started = |id: ButtonId| {
        just.iter().any(|p| {
            render::buttons(vw)
                .iter()
                .any(|b| b.id == id && (b.center - *p).length_squared() <= b.radius * b.radius)
        })
    };
    let movement =
        (if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) || pressed(ButtonId::Right) {
            1.0
        } else {
            0.0
        }) - (if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) || pressed(ButtonId::Left) {
            1.0
        } else {
            0.0
        });
    Input {
        movement,
        jump: is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::W)
            || is_key_pressed(KeyCode::Up)
            || started(ButtonId::Jump),
        melee: is_key_pressed(KeyCode::F) || is_key_pressed(KeyCode::X) || started(ButtonId::Melee),
        skills: [
            is_key_pressed(KeyCode::Key1)
                || is_key_pressed(KeyCode::J)
                || started(ButtonId::Skill0),
            is_key_pressed(KeyCode::Key2)
                || is_key_pressed(KeyCode::K)
                || started(ButtonId::Skill1),
            is_key_pressed(KeyCode::Key3)
                || is_key_pressed(KeyCode::L)
                || started(ButtonId::Skill2),
        ],
        dash: is_key_pressed(KeyCode::LeftShift)
            || is_key_pressed(KeyCode::C)
            || is_key_pressed(KeyCode::Key4)
            || started(ButtonId::Dash),
        shield: is_key_pressed(KeyCode::Q)
            || is_key_pressed(KeyCode::E)
            || started(ButtonId::Shield),
        interact: is_key_pressed(KeyCode::Enter)
            || started(ButtonId::Interact)
            || (w.player_near_portal() && is_key_pressed(KeyCode::Up)),
    }
}
