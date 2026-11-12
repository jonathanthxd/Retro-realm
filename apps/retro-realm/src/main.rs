//! Aplicación nativa, sin WebView. El núcleo \`realm-core\` sigue siendo puro Rust.
mod assets;
mod controls;
mod effects;
mod render;
mod scenery;
mod ui;
mod ui_native;
mod sfx;

use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use realm_core::{HeroClass, Input, MapId, World, TICK_RATE, WORLD_WIDTH};
use sfx::{Cue,Sfx};
use effects::{FrameSnapshot,VisualFx};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Screen {
    Home,
    Playing,
    Paused,
}

fn config() -> Conf {
    Conf {
        window_title: "Retro Realm".to_owned(),
        window_width: 960,
        window_height: 540,
        high_dpi: true,
        window_resizable: true,
        ..Default::default()
    }
}

struct Music {
    menu: Option<Sound>,
    forest: Option<Sound>,
    night: Option<Sound>,
    boss: Option<Sound>,
    active: i32,
}
impl Music {
    async fn load() -> Self {
        // Los .ogg se generan a partir de las pistas MP3 originales con
        // scripts/prepare-audio.sh o scripts/prepare-audio.ps1.
        let menu = load_sound("menu.ogg").await.ok();
        let forest = load_sound("bosque_oscuro.ogg").await.ok();
        let night = load_sound("bosque_nocturno.ogg").await.ok();
        let boss = load_sound("jefe.ogg").await.ok();
        Self {
            menu,
            forest,
            night,
            boss,
            active: -1,
        }
    }
    fn track(&self, i: i32) -> Option<&Sound> {
        match i {
            0 => self.menu.as_ref(),
            1 => self.forest.as_ref(),
            2 => self.boss.as_ref(),
            3 => self.night.as_ref(),
            _ => None,
        }
    }
    fn sync(&mut self, screen: Screen, w: &World,enabled:bool) {
        let next = if !enabled {-1}
            else if screen == Screen::Home {0}
            else if w.boss().is_some() {2}
            else if w.map == MapId::DarkForest {1} else {3};
        if next == self.active {
            return;
        }
        if let Some(old) = self.track(self.active) {
            stop_sound(old);
        }
        self.active = next;
        if let Some(sound) = self.track(next) {
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume: 0.56,
                },
            );
        }
    }
}

#[macroquad::main(config)]
async fn main() {
    macroquad::file::set_pc_assets_folder("apps/retro-realm/assets");
    // Solo para QA local / GitHub Actions; no se ejecuta en un APK normal.
    let snapshot=std::env::var("RETRO_REALM_SNAPSHOT").ok();
    let mut snapshot_frames=0u32;
    let assets = assets::Assets::load().await;
    let native_ui=assets.has_native_hud();
    let native_home=assets.has_native_menu();
    let mut music = if snapshot.is_some() {
        Music{menu:None,forest:None,night:None,boss:None,active:-1}
    } else { Music::load().await };
    let sounds = if snapshot.is_some() {Sfx::silent()}else{Sfx::load().await};
    let mut audio_enabled = true;
    let mut chosen = HeroClass::LunarMage;
    let mut home_page = ui::HomePage::Landing;
    let mut player_name = String::from("Jugador");
    let mut editing_name = false;
    let mut world = World::new(chosen);
    let mut effects=VisualFx::default();
    let mut screen = Screen::Home;
    if let Some(kind)=snapshot.as_deref(){
        match kind {
            "selection"=>home_page=ui::HomePage::Selection,
            "credits"=>home_page=ui::HomePage::Credits,
            "pause"=>screen=Screen::Paused,
            "night"=>screen=Screen::Playing,
            "dark"=>{screen=Screen::Playing;world.map=MapId::DarkForest;},
            _=>{}
        }
    }
    let mut viewport_width = 320u32;
    let mut target = render_target(viewport_width, 180);
    target.texture.set_filter(FilterMode::Nearest);
    let mut accumulator = 0.0f32;
    let mut camera_y = 0.0f32;
    let mut camera_x = 0.0f32;
    let mut pending = Input::default();

    loop {
        let virtual_width = (180.0 * screen_width() / screen_height().max(1.0))
            .round()
            .clamp(320.0, 480.0);
        if virtual_width as u32 != viewport_width {
            viewport_width = virtual_width as u32;
            target = render_target(viewport_width, 180);
            target.texture.set_filter(FilterMode::Nearest);
        }

        if is_key_pressed(KeyCode::M) {audio_enabled=!audio_enabled;}
        // Los clics/toques del menú son independientes del movimiento del juego.
        if screen == Screen::Home {
            if home_page == ui::HomePage::Selection {
                if is_key_pressed(KeyCode::Key1){chosen=HeroClass::LunarMage;}
                if is_key_pressed(KeyCode::Key2){chosen=HeroClass::Huntress;}
                if is_key_pressed(KeyCode::Key3){chosen=HeroClass::CyberRaccoon;}
            }
            for point in controls::pressed_points(virtual_width) {
                let hit=|key:&str,legacy:Rect| {
                    if native_home{
                        ui_native::menu_hit(&assets,key,point*3.0,virtual_width*3.0)
                            .unwrap_or_else(||legacy.contains(point))
                    }else{legacy.contains(point)}
                };
                if home_page==ui::HomePage::Landing {
                    if hit("credits",ui::home_credits_rect(virtual_width)){
                        home_page=ui::HomePage::Credits;
                        editing_name=false;
                    } else if hit("hero",ui::home_hero_rect()) {
                        home_page=ui::HomePage::Selection;
                        editing_name=false;
                    } else if hit("name",ui::home_name_rect(virtual_width)){
                        editing_name=true;
                    } else if hit("play",ui::home_play_rect(virtual_width)){
                        world=World::new(chosen);
                        effects.clear();
                        screen=Screen::Playing;
                        camera_y=0.0;
                        camera_x=0.0;
                        editing_name=false;
                        accumulator=0.0;
                    }
                }else if home_page==ui::HomePage::Selection{
                    for i in 0..3 {
                        if hit(&format!("card{}",i),ui::home_selection_rect(i,virtual_width)) {
                            chosen=HeroClass::from_index(i);
                        }
                    }
                    if hit("back",ui::home_back_rect(virtual_width)){
                        home_page=ui::HomePage::Landing;
                    }
                }else if home_page==ui::HomePage::Credits {
                    home_page=ui::HomePage::Landing;
                }
            }
            if is_key_pressed(KeyCode::Escape) && home_page!=ui::HomePage::Landing {
                home_page=ui::HomePage::Landing;
            }
            if editing_name {
                while let Some(ch)=get_char_pressed(){
                    if !ch.is_control() && player_name.chars().count()<14 {
                        if player_name=="Jugador" {player_name.clear();}
                        player_name.push(ch);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) {player_name.pop();}
                if is_key_pressed(KeyCode::Enter){
                    editing_name=false;
                    if player_name.trim().is_empty(){player_name="Jugador".to_string();}
                }
            } else if home_page==ui::HomePage::Landing
                && (is_key_pressed(KeyCode::Enter)||is_key_pressed(KeyCode::Space)) {
                world=World::new(chosen);
                effects.clear();
                screen=Screen::Playing;
                camera_y=0.0;
                camera_x=0.0;
                accumulator=0.0;
            }
        } else {
            if is_key_pressed(KeyCode::Escape) || controls::pause_hit(virtual_width) {
                screen = if screen == Screen::Playing {
                    Screen::Paused
                } else {
                    Screen::Playing
                };
                accumulator = 0.0;
            }
            if screen == Screen::Paused {
                let has_dom_pause=native_ui && assets.dom_pause_panel().is_some();
                let touches=if has_dom_pause {
                    controls::pressed_points(virtual_width)
                } else { Vec::new() };
                let clicked=|key:&str| {
                    touches.iter().any(|p|ui_native::pause_hit(&assets,key,
                        *p*3.0,virtual_width*3.0).unwrap_or(false))
                };
                if has_dom_pause {
                    if clicked("music")||clicked("sound") {audio_enabled=!audio_enabled;}
                    if clicked("resume") {
                        screen=Screen::Playing;
                        accumulator=0.0;
                    }else if clicked("character"){
                        screen=Screen::Home;
                        home_page=ui::HomePage::Selection;
                        effects.clear();
                        accumulator=0.0;
                    }else if is_key_pressed(KeyCode::H)||clicked("menu"){
                        screen=Screen::Home;
                        effects.clear();
                        home_page=ui::HomePage::Landing;
                        accumulator=0.0;
                    }
                }else{
                    if controls::sound_hit(virtual_width){audio_enabled=!audio_enabled;}
                    if is_key_pressed(KeyCode::H)||controls::menu_hit(virtual_width){
                        screen=Screen::Home;
                        effects.clear();
                        home_page=ui::HomePage::Landing;
                        accumulator=0.0;
                    }
                }
            } else if screen == Screen::Playing {
                if world.game_over
                    && (is_key_pressed(KeyCode::R) || controls::restart_hit(virtual_width))
                {
                    world.reset();
                    effects.clear();
                }
                let current = controls::game_input(virtual_width, &world);
                pending.movement = current.movement;
                pending.jump |= current.jump;
                pending.melee |= current.melee;
                pending.dash |= current.dash;
                pending.shield |= current.shield;
                pending.interact |= current.interact;
                for i in 0..3 {
                    pending.skills[i] |= current.skills[i];
                }

                // Simulación a 60Hz: la tasa de refresco nunca cambia las reglas.
                accumulator = (accumulator + get_frame_time().min(0.10)).min(0.15);
                let step = 1.0 / TICK_RATE;
                let mut cycles = 0;
                while accumulator >= step && cycles < 6 {
                    tick_with_audio(&mut world,pending,&sounds,audio_enabled,&mut effects);
                    pending = Input::default();
                    accumulator -= step;
                    cycles += 1;
                }
            }
        }
        // La cámara del HTML original se mueve hacia p.y-120 y amortigua
        // un 9% por tick lógico. Factor normalizado a la tasa de refresco.
        if screen != Screen::Home {
            let want=(world.player.y-120.0).clamp(-46.0,18.0);
            let smooth=1.0-0.91f32.powf((get_frame_time()*60.0).max(0.0));
            camera_y+=(want-camera_y)*smooth;
            let want_x=(world.player.x-virtual_width/2.0+world.player.facing*24.0)
                .clamp(0.0,WORLD_WIDTH-virtual_width);
            let smooth_x=1.0-0.9f32.powf((get_frame_time()*60.0).max(0.0));
            camera_x+=(want_x-camera_x)*smooth_x;
        }
        music.sync(screen, &world,audio_enabled);

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, virtual_width, 180.0));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(BLACK);
        match screen {
            Screen::Home => {
                if native_home {
                    render::draw_menu_scene(&world,virtual_width,chosen,&assets);
                }else{
                    ui::draw_home(&world,virtual_width,chosen,home_page,&player_name,
                        world.player.level,&assets);
                }
            },
            Screen::Playing => render::draw_game(&world,virtual_width,&assets,&player_name,&target,camera_y,camera_x,&effects,!native_ui),
            Screen::Paused => {
                render::draw_game(&world,virtual_width,&assets,&player_name,&target,camera_y,camera_x,&effects,!native_ui);
                if !native_ui {ui::draw_pause(virtual_width,audio_enabled);}
            }
        }

        set_default_camera();
        clear_background(BLACK);
        draw_texture_ex(
            &target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );
        if native_home && screen==Screen::Home {
            ui_native::draw_home(&assets,chosen,home_page,&player_name,world.player.level);
        }
        if native_ui && screen!=Screen::Home {
            ui_native::draw_game(&world,&assets,&player_name);
            if screen==Screen::Paused {
                ui_native::draw_pause(&assets,audio_enabled);
            }
        }
        if let Some(kind)=snapshot.as_deref(){
            snapshot_frames+=1;
            if snapshot_frames>=12 {
                let dir="native-captures";
                if std::fs::create_dir_all(dir).is_ok(){
                    get_screen_data().export_png(&format!("{}/{}.png",dir,kind));
                }
                break;
            }
        }
        next_frame().await;
    }
}


fn tick_with_audio(world: &mut World, input: Input, sounds: &Sfx, audio_enabled: bool,fx:&mut VisualFx) {
    let before=FrameSnapshot::capture(world);
    let hp=world.player.hp;
    let level=world.player.level;
    let y=world.player.y;
    let melee=world.player.melee_ticks;
    let cooldowns=world.player.cooldowns;
    let dash=world.player.dash_ticks;
    let shield=world.player.shield_ticks;
    let killed=world.killed;
    let current_map=world.map;
    let had_boss=world.boss().is_some();
    let opened=world.chests[current_map.index()].iter().filter(|c|c.opened).count();
    world.tick(input);
    fx.observe(&before,world,input);
    if !audio_enabled {return;}
    if input.jump && world.player.y<y {sounds.play(Cue::Jump);}
    if input.melee && melee==0 && world.player.melee_ticks>0 {sounds.play(Cue::Attack);}
    if (0..3).any(|i|input.skills[i] && cooldowns[i]==0 && world.player.cooldowns[i]>0) {
        sounds.play(Cue::Cast);
    }
    if input.dash && dash==0 && world.player.dash_ticks>0 {sounds.play(Cue::Dash);}
    if input.shield && shield==0 && world.player.shield_ticks>0 {sounds.play(Cue::Shield);}
    if world.player.hp<hp {sounds.play(Cue::Hurt);}
    if world.killed>killed {sounds.play(Cue::Kill);}
    if world.player.level>level {sounds.play(Cue::Level);}
    if world.map!=current_map {sounds.play(Cue::Portal);}
    if !had_boss && world.boss().is_some(){sounds.play(Cue::Boss);}
    if world.chests[current_map.index()].iter().filter(|c|c.opened).count()>opened {
        sounds.play(Cue::Chest);
    }
}
