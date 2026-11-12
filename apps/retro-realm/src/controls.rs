//! Single source of touch geometry: matches the responsive renderer.
use crate::assets::Assets;
use crate::layout::{ControlPrefs,UiLayout};
use crate::render::ButtonId;
use crate::ui_native;
use macroquad::prelude::*;
use realm_core::{Input,World};

const IDS:[(ButtonId,&str);10]=[
    (ButtonId::Left,"bl"),(ButtonId::Right,"br"),
    (ButtonId::Jump,"bj"),(ButtonId::Melee,"bm"),
    (ButtonId::Shield,"bs"),(ButtonId::Skill0,"s0"),
    (ButtonId::Skill1,"s1"),(ButtonId::Skill2,"s2"),
    (ButtonId::Dash,"s3"),(ButtonId::Interact,"ap"),
];

/// All incoming points use the same 540-high layout as UI painting.
pub fn ui_pressed_points()->Vec<Vec2>{
    let layout=UiLayout::current();
    let mut points=Vec::new();
    if is_mouse_button_pressed(MouseButton::Left){
        let (x,y)=mouse_position();
        points.push(layout.from_screen(vec2(x,y)));
    }
    for touch in touches() {
        if touch.phase==TouchPhase::Started {
            points.push(layout.from_screen(touch.position));
        }
    }
    points
}
pub fn pressed_points(_virtual_width:f32)->Vec<Vec2>{
    ui_pressed_points().into_iter().map(|p|p/3.0).collect()
}
pub fn ui_active_points()->Vec<Vec2>{
    let layout=UiLayout::current();
    let mut points=Vec::new();
    if is_mouse_button_down(MouseButton::Left){
        let (x,y)=mouse_position();
        points.push(layout.from_screen(vec2(x,y)));
    }
    for touch in touches(){
        if touch.phase!=TouchPhase::Ended && touch.phase!=TouchPhase::Cancelled {
            points.push(layout.from_screen(touch.position));
        }
    }
    points
}
fn active_buttons(assets:&Assets,prefs:&ControlPrefs)->Vec<ButtonId>{
    ui_active_points().into_iter().flat_map(|point|{
        IDS.iter().filter_map(move |(id,key)|{
            if prefs.joystick&&(*key=="bl"||*key=="br"){return None;}
            ui_native::control_hit(assets,key,point,prefs).then_some(*id)
        })
    }).collect()
}
pub fn pause_hit(assets:&Assets,prefs:&ControlPrefs)->bool{
    ui_pressed_points().into_iter().any(|point|
        ui_native::control_hit(assets,"pause",point,prefs))
}
pub fn sound_hit(vw:f32)->bool {
    pressed_points(vw).iter().any(|p|p.x>vw/2.0-65.0&&p.x<vw/2.0+65.0
        &&p.y>134.0&&p.y<158.0)
}
pub fn menu_hit(vw:f32)->bool {
    pressed_points(vw).iter().any(|p|p.x>vw/2.0-65.0&&p.x<vw/2.0+65.0
        &&p.y>105.0&&p.y<128.0)
}
pub fn restart_hit(_vw:f32)->bool {
    ui_pressed_points().iter().any(|p|ui_native::defeat_hit(*p))
}
pub fn joystick_vector(prefs:&ControlPrefs)->Vec2 {
    if !prefs.joystick{return Vec2::ZERO;}
    let layout=UiLayout::current();
    let base=Rect::new(18.0,414.0,112.0,112.0);
    let rect=layout.control_rect("joy",base,prefs);
    let center=vec2(rect.x+rect.w*0.5,rect.y+rect.h*0.5);
    let radius=(rect.w.min(rect.h)*0.5).max(1.0);
    for point in ui_active_points() {
        let delta=point-center;
        if delta.length()<=radius*1.40 {
            let fraction=(delta.length()/(radius*0.70)).min(1.0);
            return if delta.length_squared()>0.001 {
                delta.normalize()*fraction
            }else{Vec2::ZERO};
        }
    }
    Vec2::ZERO
}
pub fn game_input(assets:&Assets,prefs:&ControlPrefs,w:&World)->Input{
    let buttons=active_buttons(assets,prefs);
    let just=ui_pressed_points();
    let pressed=|id|buttons.contains(&id);
    let started=|id:ButtonId|->bool{
        let Some((_,key))=IDS.iter().find(|(candidate,_)|*candidate==id) else{return false;};
        just.iter().any(|p|ui_native::control_hit(assets,key,*p,prefs))
    };
    let joy=joystick_vector(prefs);
    let buttons_x=(if is_key_down(KeyCode::D)||is_key_down(KeyCode::Right)||pressed(ButtonId::Right){1.0}else{0.0})
        -(if is_key_down(KeyCode::A)||is_key_down(KeyCode::Left)||pressed(ButtonId::Left){1.0}else{0.0});
    let movement=if prefs.joystick&&joy.x.abs()>0.35{joy.x.signum()}else{buttons_x};
    Input {
        movement,
        jump:is_key_pressed(KeyCode::Space)||is_key_pressed(KeyCode::W)
            ||is_key_pressed(KeyCode::Up)||started(ButtonId::Jump)
            ||(prefs.joystick&&joy.y< -0.55),
        melee:is_key_pressed(KeyCode::F)||is_key_pressed(KeyCode::X)
            ||started(ButtonId::Melee),
        skills:[
            is_key_pressed(KeyCode::Key1)||is_key_pressed(KeyCode::J)||started(ButtonId::Skill0),
            is_key_pressed(KeyCode::Key2)||is_key_pressed(KeyCode::K)||started(ButtonId::Skill1),
            is_key_pressed(KeyCode::Key3)||is_key_pressed(KeyCode::L)||started(ButtonId::Skill2),
        ],
        dash:is_key_pressed(KeyCode::LeftShift)||is_key_pressed(KeyCode::C)
            ||is_key_pressed(KeyCode::Key4)||started(ButtonId::Dash),
        shield:is_key_pressed(KeyCode::Q)||is_key_pressed(KeyCode::E)
            ||started(ButtonId::Shield),
        interact:is_key_pressed(KeyCode::Enter)||started(ButtonId::Interact)
            ||(w.player_near_portal()&&is_key_pressed(KeyCode::Up)),
    }
}
