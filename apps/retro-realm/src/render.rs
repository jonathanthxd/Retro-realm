//! Renderizador pixel-art provisional. Se portarán sprites y shaders originales
//! sin modificar la API de simulación ni sus parámetros.
use crate::assets::Assets;
use macroquad::prelude::*;
use realm_core::{
    BossStage, HeroClass, MapId, PickupKind, ProjectileKind, World, TORCHES, WORLD_WIDTH,
};

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}
fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}
fn label(s: &str, x: f32, y: f32, size: f32, color: Color) {
    // Capa de texto temporal (la fuente pixel original se exportará desde HTML).
    draw_text(s, x + 1.0, y + 1.0, size, BLACK);
    draw_text(s, x, y, size, color);
}
fn centered(s: &str, x: f32, y: f32, size: f32, color: Color) {
    let width = measure_text(s, None, size as u16, 1.0).width;
    label(s, x - width / 2.0, y, size, color);
}
fn panel(x: f32, y: f32, w: f32, h: f32) {
    draw_rectangle(x + 1.0, y + 2.0, w, h, rgba(0, 0, 0, 180));
    draw_rectangle(x, y, w, h, rgb(16, 23, 41));
    draw_rectangle_lines(x, y, w, h, 1.0, rgb(87, 111, 161));
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonId {
    Left,
    Right,
    Jump,
    Melee,
    Shield,
    Skill0,
    Skill1,
    Skill2,
    Dash,
    Interact,
}
#[derive(Clone, Copy)]
pub struct Button {
    pub id: ButtonId,
    pub center: Vec2,
    pub radius: f32,
    pub caption: &'static str,
}
pub fn buttons(vw: f32) -> [Button; 10] {
    // Centros reales de los controles DOM (CSS 960x540 / 3).
    // El radio es ligeramente menor que la imagen, como el área táctil CSS.
    [
        Button{id:ButtonId::Left,center:vec2(15.67,163.67),radius:11.0,caption:"<"},
        Button{id:ButtonId::Right,center:vec2(41.67,163.67),radius:11.0,caption:">"},
        Button{id:ButtonId::Jump,center:vec2(vw-45.0,165.0),radius:10.33,caption:"SAL"},
        Button{id:ButtonId::Melee,center:vec2(vw-18.33,161.67),radius:13.67,caption:"ATK"},
        Button{id:ButtonId::Shield,center:vec2(vw-78.33,139.67),radius:9.67,caption:"ESC"},
        Button{id:ButtonId::Skill0,center:vec2(vw-66.0,163.33),radius:9.33,caption:"1"},
        Button{id:ButtonId::Skill1,center:vec2(vw-54.67,142.67),radius:9.33,caption:"2"},
        Button{id:ButtonId::Skill2,center:vec2(vw-34.0,138.0),radius:9.33,caption:"3"},
        Button{id:ButtonId::Dash,center:vec2(vw-12.67,135.33),radius:10.0,caption:"DASH"},
        Button{id:ButtonId::Interact,center:vec2(vw/2.0,163.0),radius:10.0,caption:"USE"},
    ]
}
fn hero_color(c: HeroClass) -> Color {
    let [r, g, b] = c.color();
    rgb(r, g, b)
}
fn hero(
    x: f32,
    y: f32,
    c: HeroClass,
    dir: f32,
    time: u64,
    shield: bool,
    dash: bool,
    moving: bool,
    attack: bool,
    grounded: bool,
    vy: f32,
    inv: u16,
    attack_ticks: u16,
    cast_ticks: u8,
    assets: &Assets,
) {
    if inv>0 && time%4<2 {return;} // Parpadeo al recibir daño del HTML
    // drawP en HTML siempre redondea a píxeles del Canvas original.
    let x=x.round();
    let y=y.round();
    let accent = hero_color(c);
    let anim = (time / 7 % 2) as f32;
    let foot = y - 1.0;
    draw_ellipse(x, foot + 0.5, 11.0, 2.0, 0.0, rgba(0, 0, 0, 130));
    if dash {
        for i in 1..4 {
            draw_rectangle(
                x - dir * i as f32 * 7.0 - 6.0,
                y - 24.0,
                12.0,
                20.0,
                rgba(39, 224, 232, (70 / i) as u8),
            );
        }
    }
    // HF(cl, phase, idle, arm, slash) del drawP() HTML original.
    let sprite = if attack {
        assets.hero_attack(c,if attack_ticks>9 {0}else if attack_ticks>4{1}else{2})
    } else if cast_ticks>0 {
        assets.hero_cast(c)
    } else if !grounded {
        assets.hero(c,if vy<0.0 {9}else{11})
    } else if moving {
        assets.hero(c,1 + (time / 3 % 8) as usize)
    } else {
        assets.hero_idle(c,(time / 14 % 4) as usize)
    };
    if let Some(texture) = sprite {
        // Los sprites HF originales tienen ancla (18,50) y frame 42x52.
        let flipped = dir < 0.0;
        let anchor=18.0;
        // Estela de la pose original de Mapache Cyber.
        if c==HeroClass::CyberRaccoon && (moving || !grounded || dash) {
            for k in [2.0,1.0] {
                let back=x-dir*k*7.0;
                let ax=if flipped{texture.width()-anchor}else{anchor};
                draw_texture_ex(texture,back-ax,y-50.0,
                    Color::new(1.0,1.0,1.0,0.22/k),
                    DrawTextureParams{flip_x:flipped,..Default::default()});
            }
        }
        // Aura mágica y lanzamiento del HTML drawP.
        let glow=if cast_ticks>0 {9.0}else{5.0};
        draw_circle(x+dir*4.0,y-26.0,glow,if c==HeroClass::LunarMage{
            rgba(74,240,208,if cast_ticks>0{100}else{35})
        }else{rgba(255,240,200,if cast_ticks>0{95}else{30})});
        let anchor_x = if flipped {
            texture.width() - 18.0
        } else {
            18.0
        };
        draw_texture_ex(
            texture,
            x - anchor_x,
            y - 50.0,
            WHITE,
            DrawTextureParams {
                flip_x: flipped,
                ..Default::default()
            },
        );
    } else {
        // Recurso ausente: silueta de depuración, no un sustituto del arte.
        draw_rectangle(x - 6.0, y - 19.0, 12.0, 14.0, accent);
        draw_rectangle(x - 5.0, y - 27.0, 10.0, 9.0, rgb(232, 210, 196));
        draw_rectangle(x - 6.0, y - 20.0, 2.0, 11.0, rgba(0, 0, 0, 95));
        draw_rectangle(x - 4.0, y - 5.0 + (anim * 1.0), 4.0, 5.0, rgb(35, 37, 65));
        draw_rectangle(x + 1.0, y - 5.0 - (anim * 1.0), 4.0, 5.0, rgb(35, 37, 65));
        draw_rectangle(x + 3.0 * dir, y - 25.0, 2.0, 2.0, rgb(230, 255, 255));
        match c {
            HeroClass::LunarMage => {
                draw_triangle(
                    vec2(x - 9.0, y - 26.0),
                    vec2(x + 9.0, y - 26.0),
                    vec2(x, y - 37.0),
                    rgb(69, 75, 164),
                );
                draw_rectangle(x + dir * 11.0, y - 32.0, 2.0, 28.0, rgb(122, 86, 54));
                draw_circle(x + dir * 12.0, y - 32.0, 3.0, rgb(74, 240, 208));
            }
            HeroClass::Huntress => {
                draw_rectangle(x - 6.0, y - 30.0, 13.0, 4.0, rgb(217, 112, 46));
                draw_line(
                    x + dir * 9.0,
                    y - 21.0,
                    x + dir * 20.0,
                    y - 33.0,
                    2.0,
                    rgb(232, 240, 248),
                );
            }
            HeroClass::CyberRaccoon => {
                draw_triangle(
                    vec2(x - 5.0, y - 25.0),
                    vec2(x - 10.0, y - 36.0),
                    vec2(x, y - 30.0),
                    rgb(70, 80, 100),
                );
                draw_triangle(
                    vec2(x + 5.0, y - 25.0),
                    vec2(x + 10.0, y - 36.0),
                    vec2(x, y - 30.0),
                    rgb(70, 80, 100),
                );
                draw_rectangle(x - 6.0, y - 25.0, 12.0, 2.0, rgb(39, 224, 232));
                draw_line(
                    x + dir * 7.0,
                    y - 17.0,
                    x + dir * 18.0,
                    y - 21.0,
                    2.0,
                    rgb(145, 250, 255),
                );
            }
        }
    }
    if attack {
        crate::effects::hero_attack_fx(x,y,dir,attack_ticks,c);
    }
    if shield {
        draw_circle(x, y - 17.0, 22.0, rgba(90, 200, 255, 35));
        draw_circle_lines(x, y - 17.0, 22.0, 2.0, rgba(145, 230, 255, 210));
    }
}

fn terrain_hash(a:usize)->u32 {
    // Traducción bit-a-bit del hs(col) del Canvas original.
    let mut h=(a as i32).wrapping_mul(374_761_393) ^ 0x5bd1e995;
    h=(h^(h>>13)).wrapping_mul(1_274_126_177);
    (h^(h>>16)) as u32
}
fn terrain(w: &World, vw: f32, cam: f32, assets: &Assets) {
    let min = (cam / 16.0).floor() as usize;
    let max = ((cam + vw) / 16.0).ceil() as usize;
    let dark = w.map == MapId::DarkForest;
    for i in min..max.min(320) {
        let pattern=terrain_hash(i);
        let x = i as f32 * 16.0 - cam;
        let y = w.terrain[i];
        draw_rectangle(
            x,
            y,
            16.0,
            180.0 - y,
            if dark {
                rgb(34, 29, 59)
            } else {
                rgb(46, 28, 65)
            },
        );
        for yy in ((y / 16.0) as i32 + 1)..12 {
            let cy = yy as f32 * 16.0;
            draw_rectangle(
                x + 2.0,
                cy + 2.0,
                12.0,
                1.0,
                if dark {
                    rgb(51, 47, 79)
                } else {
                    rgb(70, 43, 87)
                },
            );
        }
        // Patrón de cada fila = (hs(col) >> 2) + profundidad.
        for yy in ((y / 16.0) as i32 + 1)..14 {
            let row=yy-((y/16.0) as i32);
            if let Some(tile)=assets.soil((((pattern >> 2) as usize)+row as usize)&3,dark){
                draw_texture_ex(
                    tile,
                    x,
                    yy as f32 * 16.0,
                    WHITE,
                    DrawTextureParams::default(),
                );
                // zTerrain original oscurece la tierra con la profundidad:
                // alpha = min(.6, .1 + r*.14), r empieza en 1.
                let shade=(0.1+row as f32*0.14).min(0.6);
                draw_rectangle(x,yy as f32*16.0,16.0,16.0,
                    rgba(2,6,12,(shade*255.0) as u8));
            }
        }
        if let Some(tile) = assets.grass((pattern & 3) as usize, dark) {
            draw_texture_ex(tile, x, y, WHITE, DrawTextureParams::default());
        } else {
            draw_rectangle(
                x,
                y,
                16.0,
                3.0,
                if dark {
                    rgb(103, 171, 154)
                } else {
                    rgb(137, 164, 48)
                },
            );
        }
    }
}
fn background(w: &World, vw: f32, cam: f32, assets: &Assets) {
    crate::scenery::draw(w,vw,cam,assets,false);
}
pub fn draw_menu_scene(w:&World,vw:f32,class:HeroClass,assets:&Assets){
    crate::scenery::draw(w,vw,0.0,assets,true);
    // menuDraw HTML: suelo a Y=138 y protagonista 1.4x.
    draw_rectangle(0.0,138.0,vw,42.0,rgb(23,32,42));
    draw_rectangle(0.0,138.0,vw,5.0,rgb(47,138,58));
    draw_rectangle(0.0,138.0,vw,2.0,rgb(106,208,80));
    if let Some(sprite)=assets.hero(class,0) {
        let scale=1.4;
        draw_texture_ex(sprite,vw/2.0-18.0*scale,138.0-50.0*scale,WHITE,
            DrawTextureParams{dest_size:Some(vec2(sprite.width()*scale,sprite.height()*scale)),
                ..Default::default()});
    }
}
fn decorations(w:&World,vw:f32,cam:f32,assets:&Assets){
    // En el original se dibujan primero cabañas, luego árboles, después objetos.
    // Dibujamos únicamente la parte visible de cada bosque.
    for layer in 0..3u8 {
        for deco in assets.decor(w.map.index()) {
            if deco.layer!=layer {continue;}
            let x=deco.x-cam;
            if x< -145.0 || x>vw+145.0 {continue;}
            if let Some(sprite)=assets.decor_texture(&deco.key) {
                let y=w.floor(deco.x)+2.0;
                let width=sprite.width()*deco.scale;
                let height=sprite.height()*deco.scale;
                let left=x-deco.ax*deco.scale;
                let top=y-deco.ay*deco.scale;
                if deco.glow {
                    draw_circle(x,y-8.0,11.0,rgba(120,170,255,37));
                }
                draw_texture_ex(sprite,left,top,WHITE,DrawTextureParams {
                    dest_size:Some(vec2(width,height)),..Default::default()
                });
            }
        }
    }
}
fn rain(w:&World,vw:f32,cam:f32,camera_y:f32) {
    if w.map!=MapId::DarkForest{return;}
    // zRain() original: 130 gotas, trazos en dos segmentos, salpicaduras
    // y relámpagos esporádicos. Variación determinista sin tocar realm-core.
    draw_rectangle(0.0,camera_y-8.0,vw,188.0,rgba(0,6,20,16));
    for i in 0..130 {
        let hash=(i*9973+37) as f32;
        let speed=5.0+(i*37%35) as f32/10.0;
        let length=3.0+(i%4) as f32;
        let x=(hash*1.13-w.tick as f32*1.1).rem_euclid(vw+30.0)-8.0;
        let y=(hash*0.72+w.tick as f32*speed).rem_euclid(240.0)-55.0;
        let alpha=if length>4.0{128}else{82};
        draw_rectangle(x.round(),y.round(),1.0,2.0,
            rgba(165,190,235,alpha));
        draw_rectangle(x.round()-1.0,y.round()+2.0,1.0,length-1.0,
            rgba(165,190,235,alpha));
        // zRain crea RS al tocar el suelo. No se guarda estado mutable:
        // la proximidad suelo/tiempo genera la misma lectura de salpicadura.
        let ground=w.floor(cam+x);
        if y>=ground-6.0 && y<=ground+2.0{
            let width=2.0+((i as f32+w.tick as f32)*0.24).sin().abs()*3.0;
            draw_rectangle(x-width/2.0,ground-1.0,width,1.0,
                rgba(180,205,245,137));
        }
    }
    // Un destello poco frecuente con la silueta ramificada de zRain/ZLT.
    let phase=(w.tick+233)%1667;
    if phase<16 {
        let a=((16-phase) as f32/16.0).max(0.0);
        draw_rectangle(0.0,camera_y-8.0,vw,188.0,
            rgba(210,225,255,(a*51.0) as u8));
        if phase<5 {
            let mut x=60.0+(cam*7.0).rem_euclid(200.0);
            let mut y=camera_y-8.0;
            for k in 0..12{
                let nx=x+(((k*43+7) as f32*1.13).sin()*8.0);
                let ny=y+12.0;
                draw_line(x,y,nx,ny,1.5,rgba(235,242,255,220));
                x=nx;y=ny;
            }
        }
    }
}
fn props(w: &World, vw: f32, cam: f32, assets: &Assets) {
    let mi = w.map.index();
    for &x in TORCHES[mi] {
        let sx = x - cam;
        if sx < -20.0 || sx > vw + 20.0 {
            continue;
        }
        let y = w.floor(x);
        let px = sx.round();
        // Exact pixel geometry from zProps() of Retro Realm.html:
        // dark wood handle, metallic clamp and three dancing flame tones.
        draw_rectangle(px-1.0,y-22.0,3.0,22.0,rgb(74,48,32));
        draw_rectangle(px-3.0,y-24.0,7.0,3.0,rgb(90,90,102));
        draw_rectangle(px-2.0,y-26.0,5.0,3.0,rgb(58,58,68));
        let flicker=(w.tick as f32*0.3+x).sin()*1.2;
        let f_y=flicker.round();
        draw_rectangle(px-2.0,y-31.0+f_y,5.0,5.0,rgb(255,122,32));
        draw_rectangle(px-1.0,y-34.0+f_y,3.0,4.0,rgb(255,176,48));
        draw_rectangle(px,y-31.0+f_y,1.0,3.0,rgb(255,244,176));
        // One ember per 14 ticks, capped and deterministic.
        for i in 0..3u64 {
            let age=(w.tick+i*5+(x as u64)%7)%25;
            if age>17 {continue;}
            let rise=age as f32*0.55;
            let drift=(w.tick as f32*0.10+i as f32*2.4+x*0.03).sin()*3.0;
            let opacity=((17-age) as f32/17.0*195.0) as u8;
            draw_rectangle((px+drift).round(),(y-35.0-rise).round(),
                1.0,1.0,rgba(255,181,60,opacity));
        }
    }
    for chest in &w.chests[mi] {
        let sx = chest.x - cam;
        if sx < -20.0 || sx > vw + 20.0 {
            continue;
        }
        let y = w.floor(chest.x);
        if let Some(sprite) = assets.chest(chest.opened) {
            draw_texture_ex(
                sprite,
                sx - sprite.width() / 2.0,
                y - sprite.height() + 1.0,
                WHITE,
                DrawTextureParams::default(),
            );
        } else {
            draw_rectangle(sx - 9.0, y - 14.0, 18.0, 13.0, rgb(130, 82, 36));
            draw_rectangle(sx - 9.0, y - 14.0, 18.0, 3.0, rgb(180, 132, 65));
            draw_rectangle(sx - 2.0, y - 10.0, 4.0, 5.0, rgb(232, 192, 78));
            if chest.opened {
                draw_rectangle(sx - 9.0, y - 17.0, 18.0, 3.0, rgb(90, 64, 35));
            }
        }
    }
    if w.cleared[mi] || w.boss_stages[mi] == BossStage::Defeated || w.spawned > 0 {
        portal(w, 5040.0, cam, w.portal_available());
        if w.map == MapId::DarkForest {
            portal(w, 4920.0, cam, w.portal_available());
        }
    }
}
fn portal(w: &World, x: f32, cam: f32, open: bool) {
    // Geometry taken from zPortal() in Retro Realm.html. The previous
    // rectangle-and-circle placeholder looked noticeably different.
    let sx=(x-cam).round();
    let y=w.floor(x);
    let t=w.tick as f32;
    // Planted stone base, grass cap and ground sigil.
    draw_rectangle(sx-24.0,y-3.0,48.0,3.0,rgb(95,125,28));
    draw_rectangle(sx-24.0,y-3.0,48.0,1.0,rgb(202,219,60));
    for i in 0..10 {
        draw_rectangle(sx-24.0+i as f32*5.0,y-5.0+(i&1) as f32,
            3.0,3.0,rgb(138,166,38));
    }
    draw_rectangle(sx-20.0,y,40.0,2.0,rgb(58,42,72));
    for (dx,dy,size,phase) in [(-30.0,6.0,3.0,0.0),
        (-26.0,13.0,2.0,1.0),(27.0,8.0,3.0,2.0),
        (31.0,15.0,2.0,3.0),(0.0,22.0,3.0,4.0)]
    {
        let bob=(t*0.05+phase).sin().round()*2.0;
        draw_rectangle(sx+dx,y+dy+bob,size+1.0,size,rgb(106,58,74));
        draw_rectangle(sx+dx,y+dy+bob,size,1.0,rgb(138,90,106));
    }
    draw_circle(sx,y+9.0,6.0,rgb(12,10,16));
    draw_circle(sx,y+9.0,5.0,rgb(74,106,112));
    draw_triangle(vec2(sx,y+4.0),vec2(sx+3.0,y+9.0),vec2(sx,y+14.0),
        if open{rgb(74,240,216)}else{rgb(42,90,88)});
    draw_triangle(vec2(sx,y+4.0),vec2(sx,y+14.0),vec2(sx-3.0,y+9.0),
        if open{rgb(74,240,216)}else{rgb(42,90,88)});
    // Portal cavity with clipped-looking stepped arch.
    draw_rectangle(sx-9.0,y-52.0,18.0,49.0,rgb(7,10,18));
    draw_circle(sx,y-52.0,9.0,rgb(7,10,18));
    if open {
        for i in (0..9).rev(){
            let inner=9.0-i as f32;
            let color=if i<3{rgb(226,255,247)}
                else if i<6{rgb(74,240,216)}else{rgb(15,72,87)};
            draw_rectangle(sx-inner,y-51.0,inner*2.0,48.0,color);
        }
        // Tiny spiral flecks like the original 46 glowing particles.
        for i in 0..46{
            let a=i as f32*0.42+t*0.07;
            let radius=i as f32*0.20;
            let alpha=((0.35+0.50*(1.0-i as f32/46.0))*255.0) as u8;
            draw_rectangle((sx+a.cos()*radius).round(),
                (y-30.0+a.sin()*radius*2.1).round(),2.0,2.0,
                rgba(232,255,250,alpha));
        }
    }else{
        for i in 0..24{
            let a=i as f32*0.45+t*0.01;
            let radius=i as f32*0.28;
            draw_rectangle((sx+a.cos()*radius).round(),
                (y-30.0+a.sin()*radius*2.0).round(),1.0,1.0,
                rgba(120,160,200,46));
        }
    }
    // Cut-stone pillars, joints and top arch, all in original palette.
    for (side,shadow,highlight) in [
        (-1.0,-12.0,-18.0),(1.0,16.0,10.0)
    ] {
        let main_x=if side<0.0{sx-18.0}else{sx+10.0};
        draw_rectangle(main_x,y-58.0,8.0,56.0,rgb(138,132,112));
        draw_rectangle(main_x,y-58.0,3.0,56.0,rgb(168,162,138));
        draw_rectangle(sx+shadow,y-58.0,2.0,56.0,rgb(90,86,70));
        draw_rectangle(sx+highlight-1.0,y-60.0,10.0,4.0,rgb(168,162,138));
        for i in 0..4 {
            draw_rectangle(if side<0.0{sx-16.0}else{sx+12.0},
                y-46.0+i as f32*10.0,4.0,2.0,rgb(74,70,56));
        }
    }
    // Three concentric strokes approximate the original semicircle exactly.
    for (radius,width,color,offset) in [
        (14.0,7.0,rgb(138,132,112),0.0),
        (14.0,2.0,rgb(168,162,138),-3.0),
        (14.0,1.5,rgb(90,86,70),3.0)
    ] {
        let cy=y-52.0+offset;
        let mut prev=vec2(sx-radius,cy);
        for i in 1..=22 {
            let a=std::f32::consts::PI+(i as f32/22.0)*std::f32::consts::PI;
            let next=vec2(sx+a.cos()*radius,cy+a.sin()*radius);
            draw_line(prev.x,prev.y,next.x,next.y,width,color);
            prev=next;
        }
    }
    // Floating cyan gem atop the arch.
    let gem=if open{rgb(74,240,216)}else{rgb(46,106,102)};
    draw_triangle(vec2(sx,y-75.0),vec2(sx+4.0,y-70.0),vec2(sx,y-64.0),
        rgb(12,10,16));
    draw_triangle(vec2(sx,y-75.0),vec2(sx,y-64.0),vec2(sx-4.0,y-70.0),
        rgb(12,10,16));
    draw_triangle(vec2(sx,y-74.0),vec2(sx+3.0,y-70.0),vec2(sx,y-65.0),gem);
    draw_triangle(vec2(sx,y-74.0),vec2(sx,y-65.0),vec2(sx-3.0,y-70.0),gem);
    draw_rectangle(sx-1.0,y-72.0,2.0,3.0,
        if open{rgb(232,255,250)}else{rgb(74,138,132)});
}
fn enemy(e: &realm_core::Enemy, cam: f32, t: u64, player_x:f32, assets: &Assets) {
    let x = (e.x - cam).round();
    let y = e.y.round();
    let scale = e.scale;
    let color = match e.kind {
        0 | 20..=39 => rgb(83, 190, 105),
        3 => rgb(75, 83, 139),
        4 => rgb(230, 229, 207),
        40..=49 => rgb(142, 111, 118),
        60 | 61 => rgb(83, 120, 77),
        62 => rgb(171, 158, 144),
        _ => rgb(118, 108, 156),
    };
    let width = if e.boss { 28.0 * scale } else { 13.0 * scale };
    let height = if e.boss { 35.0 * scale } else { 20.0 * scale };
    let anim_frame = if (40..=49).contains(&e.kind) {
        if e.attack > 0 {
            0
        } else {
            1 + ((t/4+e.x as u64) % 8) as usize
        }
    } else if e.kind == 60 || e.kind == 61 {
        ((t / if e.boss {9}else{7}+e.x as u64) % 4) as usize
    } else {
        ((t / if (20..40).contains(&e.kind){9}else{12}
            +e.x as u64) % 2) as usize
    };
    let (sprite,key): (Option<&Texture2D>,String)=if e.kind==62 {
        let frame=if e.king_pose=="w" {(t/8%4) as usize}
            else{(t/18%2) as usize};
        let key=format!("king_{}_{}",e.king_pose,frame);
        (assets.king(e.king_pose,frame).or_else(||assets.enemy(62,0)),key)
    } else if (e.kind==60||e.kind==61) && e.attack>0 {
        let phase=(1.0-e.attack as f32/36.0).clamp(0.0,1.0);
        let pose=if phase<0.35{"a"}else if phase<0.7{"s"}else{"r"};
        (assets.ogre_attack(e.kind,pose).or_else(||assets.enemy(e.kind,anim_frame)),
            format!("ogre_{}_{}",e.kind,pose))
    } else if (40..=49).contains(&e.kind) && e.attack>0 {
        let phase=(1.0-e.attack as f32/30.0).clamp(0.0,1.0);
        let frame=if phase<0.35{0}else if phase<0.7{1}else{2};
        (assets.humanoid_attack(e.kind,frame),format!("humanoid_attack_{}_{}",e.kind,frame))
    }else {
        (assets.enemy(e.kind,anim_frame),format!("enemy_{}_{}",e.kind,anim_frame))
    };
    let mut height_for_bar=height;
    if let Some(sprite) = sprite {
        let anchor=assets.anchor(&key,vec2(sprite.width()/2.0,sprite.height()-1.0));
        let mut sx=1.0f32;
        let mut sy=1.0f32;
        let mut bob=0.0;
        if (20..40).contains(&e.kind) {
            // Squash/stretch + salto de los slimes zEn2.
            let w=t as f32*0.2+e.x;
            let jump=(w*0.5).sin().abs();
            bob=-jump*4.0;
            sx=1.0+0.14*(1.0-jump)-0.1*jump;
            sy=1.0-0.14*(1.0-jump)+0.14*jump;
            if e.attack>0{
                let phase=(1.0-e.attack as f32/30.0).clamp(0.0,1.0);
                let compression=(phase*std::f32::consts::PI).sin();
                sx*=1.0+0.25*compression;
                sy*=1.0-0.3*compression;
                bob-=5.0*compression;
            }
        }else if e.kind==60||e.kind==61{
            sy*=1.0+0.012*(t as f32*0.06+e.x).sin();
        }
        if e.flash>0 {
            sx*=1.05;sy*=0.94;
        }
        let sw=sprite.width()*scale*sx;
        let sh=sprite.height()*scale*sy;
        height_for_bar=anchor.y*scale*sy+bob.abs();
        let flipped=e.facing<0.0;
        let anchor_x=if flipped {sprite.width()-anchor.x}else{anchor.x};
        draw_ellipse(x,y+1.0,12.0*scale,2.0,0.0,rgba(0,0,0,90));
        draw_texture_ex(
            sprite,x-anchor_x*scale*sx,y+bob*scale-anchor.y*scale*sy,
            if e.flash>0{rgba(255,217,223,255)}else{WHITE},
            DrawTextureParams{
                dest_size:Some(vec2(sw,sh)),flip_x:flipped,
                ..Default::default()
            }
        );
        if e.flash>0 {
            draw_circle(x,y-19.0*scale,15.0*scale,
                rgba(255,240,245,(e.flash.min(6)*10) as u8));
        }
    } else {
        draw_ellipse(x, y + 1.0, width * 0.4, 3.0, 0.0, rgba(0, 0, 0, 140));
        if e.kind == 0 || (20..=39).contains(&e.kind) {
            draw_circle(x, y - height * 0.5, width * 0.55, color);
            draw_rectangle(
                x - width * 0.3,
                y - height * 0.52,
                3.0 * scale,
                3.0 * scale,
                WHITE,
            );
            draw_rectangle(
                x + width * 0.1,
                y - height * 0.52,
                3.0 * scale,
                3.0 * scale,
                WHITE,
            );
        } else {
            draw_rectangle(
                x - width * 0.4,
                y - height * 0.75,
                width * 0.8,
                height * 0.58,
                color,
            );
            draw_rectangle(
                x - width * 0.32,
                y - height,
                width * 0.64,
                height * 0.3,
                if e.flash > 0 { WHITE } else { color },
            );
            draw_rectangle(
                x - width * 0.3,
                y - height * 0.22,
                width * 0.24,
                height * 0.2,
                rgb(29, 25, 38),
            );
            draw_rectangle(
                x + width * 0.08,
                y - height * 0.22,
                width * 0.24,
                height * 0.2,
                rgb(29, 25, 38),
            );
            draw_rectangle(
                x + width * 0.11,
                y - height * 0.85,
                3.0,
                2.0,
                rgb(252, 108, 81),
            );
            if e.kind == 62 {
                draw_rectangle(
                    x - width * 0.5,
                    y - height * 1.1,
                    width,
                    5.0,
                    rgb(216, 176, 80),
                );
            }
        }
    }
    crate::effects::enemy_attack_fx(e,x,t);
    if !e.boss {
        // Barra pixel-art definitiva del HTML: borde de dos tonos,
        // detalle de nivel en el lateral y nombre cercano.
        let w=22.0;
        let top=y-height_for_bar-6.0;
        draw_rectangle(x-w/2.0-1.0,top-1.0,w+2.0,5.0,rgb(12,10,16));
        draw_rectangle(x-w/2.0,top,w,3.0,rgb(66,70,79));
        draw_rectangle(x-w/2.0,top,w,1.0,rgb(83,89,103));
        let fill=(w-2.0)*(e.hp/e.max_hp).clamp(0.0,1.0);
        if fill>0.0{
            draw_rectangle(x-w/2.0+1.0,top,fill,3.0,
                if e.split_on_death&&e.generation==0 {
                    rgb(216,98,42)
                } else{rgb(196,48,72)});
            draw_rectangle(x-w/2.0+1.0,top,fill,1.0,
                if e.split_on_death&&e.generation==0{
                    rgb(255,160,94)
                }else{rgb(238,108,126)});
        }
        draw_rectangle(x+w/2.0,top-2.0,7.0,6.0,rgb(12,10,16));
        draw_rectangle(x+w/2.0+1.0,top-1.0,5.0,4.0,rgb(42,38,48));
        label(&e.level.to_string(),x+w/2.0+1.5,top+3.0,5.5,rgb(247,207,98));
        if (e.x-player_x).abs()<70.0{
            centered(e.name(),x,top-8.0,7.0,rgb(235,213,182));
        }
    }

}
fn vignette(vw:f32) {
    // Vignette radial aproximada en franjas translúcidas (HTML VG);
    // se dibuja ANTES del HUD, para mantener la UI legible.
    for i in 0..12 {
        let f=(12-i) as f32/12.0;
        let a=(11.0*f*f).round() as u8;
        draw_rectangle(i as f32,0.0,1.0,180.0,rgba(0,0,10,a));
        draw_rectangle(vw-1.0-i as f32,0.0,1.0,180.0,rgba(0,0,10,a));
    }
    for i in 0..18 {
        let f=(18-i) as f32/18.0;
        let a=(15.0*f*f).round() as u8;
        draw_rectangle(0.0,i as f32,vw,1.0,rgba(0,0,12,a));
        draw_rectangle(0.0,179.0-i as f32,vw,1.0,rgba(0,0,12,a));
    }
}
pub fn draw_game(w: &World, vw: f32, assets: &Assets, name:&str,
    target:&RenderTarget, camera_y:f32,camera_x:f32,
    fx:&crate::effects::VisualFx,legacy_ui:bool) {
    // El mundo se mueve verticalmente, pero el HUD y los controles permanecen
    // fijos en coordenadas de la pantalla; el Canvas HTML hace lo mismo.
    push_camera_state();
    let mut wc=Camera2D::from_display_rect(Rect::new(0.0,camera_y.round(),vw,180.0));
    wc.render_target=Some(target.clone());
    set_camera(&wc);
    draw_rectangle(0.0,-60.0,vw,300.0,
        if w.map==MapId::DarkForest{rgb(7,11,24)}else{rgb(5,11,30)});
    let cam=camera_x.clamp(0.0,WORLD_WIDTH-vw);
    background(w, vw, cam, assets);
    decorations(w, vw, cam, assets);
    terrain(w, vw, cam, assets);
    props(w, vw, cam, assets);
    for pickup in &w.pickups {
        let x = pickup.x - cam;
        let color = match pickup.kind {
            PickupKind::Gold => rgb(245, 202, 78),
            PickupKind::Gem => rgb(57, 219, 225),
            PickupKind::Heart => rgb(237, 71, 85),
            PickupKind::Mana => rgb(83, 155, 249),
        };
        draw_circle(x, pickup.y, 3.0, color);
    }
    for e in &w.enemies {
        if (e.x - cam).abs() < vw + 50.0 {
            enemy(e, cam, w.tick, w.player.x, assets);
        }
    }
    hero(
        w.player.x - cam,
        w.player.y,
        w.player.class,
        w.player.facing,
        w.tick,
        w.player.shield_ticks > 0,
        w.player.dash_ticks > 0,
        w.player.moving,
        w.player.melee_ticks > 0,
        w.player.ground,w.player.vy,w.player.inv,w.player.melee_ticks,fx.cast_ticks,
        assets,
    );
    // Etiqueta flotante sobre el protagonista como en el HTML (#name tag).
    if legacy_ui {
        crate::ui::draw_player_tag(assets,name,w.player.x-cam,w.player.y-46.0);
    }
    for q in &w.projectiles {
        let color = match q.kind {
            ProjectileKind::Orb => rgb(176, 107, 255),
            ProjectileKind::Knife => rgb(235, 240, 255),
            ProjectileKind::Explosive => rgb(255, 131, 57),
            ProjectileKind::EnemyArrow => rgb(242, 192, 92),
        };
        let x=q.x-cam;
        if matches!(q.kind,ProjectileKind::Knife|ProjectileKind::EnemyArrow) {
            // Cuchillos y flechas tienen forma y estela como drawKnife().
            let length=if q.kind==ProjectileKind::Knife{10.0}else{8.0};
            let dx=if q.vx<0.0{-length}else{length};
            draw_line(x-dx/2.0,q.y,x+dx/2.0,q.y,2.0,color);
            draw_line(x+dx/2.0,q.y,x+dx/2.0+q.vx.signum()*2.0,q.y,
                1.0,WHITE);
        }else{
            draw_circle(x,q.y,q.radius,color);
            draw_circle(x,q.y,(q.radius/2.0).max(1.0),WHITE);
            for i in 1..4{
                let fade=0.34/i as f32;
                draw_circle(x-q.vx*i as f32*1.6,q.y-q.vy*i as f32*1.6,
                    (q.radius-i as f32*0.6).max(0.3),
                    Color::new(color.r,color.g,color.b,fade));
            }
        }
    }
    fx.draw_world(cam,assets,vw);
    rain(w,vw,cam,camera_y);
    pop_camera_state();
    vignette(vw);
    if legacy_ui {crate::ui::draw_hud(w,vw,assets,name);}
    if w.game_over {
        draw_rectangle(0.0, 0.0, vw, 180.0, rgba(0, 0, 0, 195));
        centered("HAS CAIDO", vw / 2.0, 80.0, 25.0, rgb(245, 89, 91));
        centered("R o tocar para reiniciar", vw / 2.0, 117.0, 13.0, WHITE);
    }
}
