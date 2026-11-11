//! Aplicación nativa, sin WebView. El núcleo \`realm-core\` sigue siendo puro Rust.
mod assets;
mod controls;
mod render;

use macroquad::audio::{load_sound, play_sound, stop_sound, PlaySoundParams, Sound};
use macroquad::prelude::*;
use realm_core::{HeroClass, Input, MapId, World, TICK_RATE};

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
    boss: Option<Sound>,
    active: i32,
}
impl Music {
    async fn load() -> Self {
        // Los .ogg se generan a partir de las pistas MP3 originales con
        // scripts/prepare-audio.sh o scripts/prepare-audio.ps1.
        let menu = load_sound("menu.ogg").await.ok();
        let forest = load_sound("bosque_oscuro.ogg").await.ok();
        let boss = load_sound("jefe.ogg").await.ok();
        Self {
            menu,
            forest,
            boss,
            active: -1,
        }
    }
    fn track(&self, i: i32) -> Option<&Sound> {
        match i {
            0 => self.menu.as_ref(),
            1 => self.forest.as_ref(),
            2 => self.boss.as_ref(),
            _ => None,
        }
    }
    fn sync(&mut self, screen: Screen, w: &World) {
        let next = if screen == Screen::Home {
            0
        } else if w.map == MapId::DarkForest && w.boss().is_some() {
            2
        } else if w.map == MapId::DarkForest {
            1
        } else {
            -1
        };
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
    let assets = assets::Assets::load().await;
    let mut music = Music::load().await;
    let mut chosen = HeroClass::LunarMage;
    let mut world = World::new(chosen);
    let mut screen = Screen::Home;
    let mut viewport_width = 320u32;
    let mut target = render_target(viewport_width, 180);
    target.texture.set_filter(FilterMode::Nearest);
    let mut accumulator = 0.0f32;
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

        // Los clics/toques del menú son independientes del movimiento del juego.
        if screen == Screen::Home {
            if is_key_pressed(KeyCode::Key1) {
                chosen = HeroClass::LunarMage;
            }
            if is_key_pressed(KeyCode::Key2) {
                chosen = HeroClass::Huntress;
            }
            if is_key_pressed(KeyCode::Key3) {
                chosen = HeroClass::CyberRaccoon;
            }
            for point in controls::pressed_points(virtual_width) {
                if let Some(i) = render::home_character_hit(point, virtual_width) {
                    chosen = HeroClass::from_index(i);
                }
                if render::home_play_hit(point, virtual_width) {
                    world = World::new(chosen);
                    screen = Screen::Playing;
                    accumulator = 0.0;
                }
            }
            if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                world = World::new(chosen);
                screen = Screen::Playing;
                accumulator = 0.0;
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
                if is_key_pressed(KeyCode::H) || controls::menu_hit(virtual_width) {
                    screen = Screen::Home;
                    accumulator = 0.0;
                }
            } else if screen == Screen::Playing {
                if world.game_over
                    && (is_key_pressed(KeyCode::R) || controls::restart_hit(virtual_width))
                {
                    world.reset();
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
                    world.tick(pending);
                    pending = Input::default();
                    accumulator -= step;
                    cycles += 1;
                }
            }
        }
        music.sync(screen, &world);

        let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, virtual_width, 180.0));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(BLACK);
        match screen {
            Screen::Home => render::draw_home(virtual_width, chosen, world.player.level, &assets),
            Screen::Playing => render::draw_game(&world, virtual_width, &assets),
            Screen::Paused => {
                render::draw_game(&world, virtual_width, &assets);
                render::draw_pause(virtual_width);
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
        next_frame().await;
    }
}
