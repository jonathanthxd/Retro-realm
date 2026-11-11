//! Renderizador pixel-art provisional. Se portarán sprites y shaders originales
//! sin modificar la API de simulación ni sus parámetros.
use macroquad::prelude::*;
use crate::assets::Assets;
use realm_core::{World,HeroClass,MapId,ProjectileKind,PickupKind,BossStage,TORCHES,WORLD_WIDTH};

fn rgb(r:u8,g:u8,b:u8)->Color {Color::from_rgba(r,g,b,255)}
fn rgba(r:u8,g:u8,b:u8,a:u8)->Color {Color::from_rgba(r,g,b,a)}
fn label(s:&str,x:f32,y:f32,size:f32,color:Color) {
    // Capa de texto temporal (la fuente pixel original se exportará desde HTML).
    draw_text(s,x+1.0,y+1.0,size,BLACK);draw_text(s,x,y,size,color);
}
fn centered(s:&str,x:f32,y:f32,size:f32,color:Color) {
    let width=measure_text(s,None,size as u16,1.0).width; label(s,x-width/2.0,y,size,color);
}
fn panel(x:f32,y:f32,w:f32,h:f32){
    draw_rectangle(x+1.0,y+2.0,w,h,rgba(0,0,0,180));
    draw_rectangle(x,y,w,h,rgb(16,23,41));
    draw_rectangle_lines(x,y,w,h,1.0,rgb(87,111,161));
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ButtonId {Left,Right,Jump,Melee,Shield,Skill0,Skill1,Skill2,Dash,Interact}
#[derive(Clone,Copy)]
pub struct Button {pub id:ButtonId,pub center:Vec2,pub radius:f32,pub caption:&'static str}
pub fn buttons(vw:f32)->[Button;10]{
    [
        Button{id:ButtonId::Left,center:vec2(27.0,151.0),radius:16.0,caption:"<"},
        Button{id:ButtonId::Right,center:vec2(67.0,151.0),radius:16.0,caption:">"},
        Button{id:ButtonId::Jump,center:vec2(vw-30.0,153.0),radius:17.0,caption:"SAL"},
        Button{id:ButtonId::Melee,center:vec2(vw-70.0,153.0),radius:17.0,caption:"ATK"},
        Button{id:ButtonId::Shield,center:vec2(vw-110.0,153.0),radius:17.0,caption:"ESC"},
        Button{id:ButtonId::Skill0,center:vec2(vw-28.0,105.0),radius:17.0,caption:"1"},
        Button{id:ButtonId::Skill1,center:vec2(vw-67.0,105.0),radius:17.0,caption:"2"},
        Button{id:ButtonId::Skill2,center:vec2(vw-106.0,105.0),radius:17.0,caption:"3"},
        Button{id:ButtonId::Dash,center:vec2(vw-145.0,105.0),radius:17.0,caption:"DASH"},
        Button{id:ButtonId::Interact,center:vec2(vw/2.0,153.0),radius:17.0,caption:"USE"},
    ]
}
fn hero_color(c:HeroClass)->Color {
    let [r,g,b]=c.color();rgb(r,g,b)
}
fn hero(x:f32,y:f32,c:HeroClass,dir:f32,time:u64,shield:bool,dash:bool,moving:bool,attack:bool,assets:&Assets) {
    let accent=hero_color(c);
    let anim=(time/7%2) as f32;
    let foot=y-1.0;
    draw_ellipse(x,foot+0.5,11.0,2.0,0.0,rgba(0,0,0,130));
    if dash {
        for i in 1..4 {draw_rectangle(x-dir*i as f32*7.0-6.0,y-24.0,12.0,20.0,
            rgba(39,224,232,(70/i) as u8));}
    }
    let sprite=if attack {assets.hero_attack(c, (time/3 % 3) as usize)}
        else {assets.hero(c,if moving {1+(time/7 % 8) as usize}else{0})};
    if let Some(texture)=sprite {
        // El Canvas original tiene punto de apoyo (18, 50) y tamaño 42x52.
        let flipped=dir<0.0;
        let anchor_x=if flipped {texture.width()-18.0}else{18.0};
        draw_texture_ex(texture, x-anchor_x,y-50.0,WHITE,DrawTextureParams {
            flip_x:flipped,
            ..Default::default()
        });
    } else {
        // Recurso ausente: silueta de depuración, no un sustituto del arte.
        draw_rectangle(x-6.0,y-19.0,12.0,14.0,accent);
        draw_rectangle(x-5.0,y-27.0,10.0,9.0,rgb(232,210,196));
        draw_rectangle(x-6.0,y-20.0,2.0,11.0,rgba(0,0,0,95));
        draw_rectangle(x-4.0,y-5.0+(anim*1.0),4.0,5.0,rgb(35,37,65));
        draw_rectangle(x+1.0,y-5.0-(anim*1.0),4.0,5.0,rgb(35,37,65));
        draw_rectangle(x+3.0*dir,y-25.0,2.0,2.0,rgb(230,255,255));
        match c {
            HeroClass::LunarMage => {
                draw_triangle(vec2(x-9.0,y-26.0),vec2(x+9.0,y-26.0),
                    vec2(x,y-37.0),rgb(69,75,164));
                draw_rectangle(x+dir*11.0,y-32.0,2.0,28.0,rgb(122,86,54));
                draw_circle(x+dir*12.0,y-32.0,3.0,rgb(74,240,208));
            }
            HeroClass::Huntress => {
                draw_rectangle(x-6.0,y-30.0,13.0,4.0,rgb(217,112,46));
                draw_line(x+dir*9.0,y-21.0,x+dir*20.0,y-33.0,2.0,rgb(232,240,248));
            }
            HeroClass::CyberRaccoon => {
                draw_triangle(vec2(x-5.0,y-25.0),vec2(x-10.0,y-36.0),
                    vec2(x,y-30.0),rgb(70,80,100));
                draw_triangle(vec2(x+5.0,y-25.0),vec2(x+10.0,y-36.0),
                    vec2(x,y-30.0),rgb(70,80,100));
                draw_rectangle(x-6.0,y-25.0,12.0,2.0,rgb(39,224,232));
                draw_line(x+dir*7.0,y-17.0,x+dir*18.0,y-21.0,2.0,rgb(145,250,255));
            }
        }
    }
    if shield {
        draw_circle(x,y-17.0,22.0,rgba(90,200,255,35));
        draw_circle_lines(x,y-17.0,22.0,2.0,rgba(145,230,255,210));
    }
}

fn terrain(w:&World,vw:f32,cam:f32,assets:&Assets){
    let min=(cam/16.0).floor() as usize;
    let max=((cam+vw)/16.0).ceil() as usize;
    let dark=w.map==MapId::DarkForest;
    for i in min..max.min(320) {
        let x=i as f32*16.0-cam;let y=w.terrain[i];
        draw_rectangle(x,y,16.0,180.0-y,if dark {rgb(34,29,59)}else{rgb(46,28,65)});
        for yy in ((y/16.0) as i32+1)..12 {
            let cy=yy as f32*16.0;
            draw_rectangle(x+2.0,cy+2.0,12.0,1.0,if dark {rgb(51,47,79)}else{rgb(70,43,87)});
        }
        if let Some(tile)=assets.soil(i%4,dark) {
            for yy in ((y/16.0) as i32+1)..12 {
                draw_texture_ex(tile,x,yy as f32*16.0,WHITE,DrawTextureParams::default());
            }
        }
        if let Some(tile)=assets.grass(i%4,dark) {
            draw_texture_ex(tile,x,y,WHITE,DrawTextureParams::default());
        } else {
            draw_rectangle(x,y,16.0,3.0,
                if dark {rgb(103,171,154)}else{rgb(137,164,48)});
        }
    }
}
fn background(w:&World,vw:f32,cam:f32){
    let dark=w.map==MapId::DarkForest;
    let top=if dark {[7.0,11.0,24.0]}else{[8.0,17.0,48.0]};
    let low=if dark {[18.0,30.0,60.0]}else{[41.0,69.0,113.0]};
    for i in 0..24 {
        let a=i as f32/24.0;
        draw_rectangle(0.0,i as f32*8.0,vw,8.0,rgb(
            (top[0]*(1.0-a)+low[0]*a) as u8,
            (top[1]*(1.0-a)+low[1]*a) as u8,
            (top[2]*(1.0-a)+low[2]*a) as u8
        ));
    }
    // Estrellas y capas parallax sin generar texturas por fotograma.
    for i in 0..80 {
        let x=(i as f32*71.0-cam*0.04).rem_euclid(vw+45.0);
        let y=(i*37%90) as f32;
        if dark || i%3==0 {draw_rectangle(x,y,1.0,1.0,rgba(226,238,255,(65+i%6*26) as u8));}
    }
    let moon_x=(if dark {90.0}else{250.0})-cam*0.015;
    draw_circle(moon_x,35.0,if dark {17.0}else{15.0},if dark {rgb(198,208,228)}else{rgb(230,240,138)});
    for layer in 0..3 {
        let parallax=[0.12,0.24,0.40][layer];
        let span=[76.0,54.0,41.0][layer];
        let color=if dark {[rgb(27,36,70),rgb(17,26,52),rgb(10,17,36)][layer]}
                  else {[rgb(34,67,106),rgb(21,52,83),rgb(12,38,62)][layer]};
        for i in -2..(vw/span).ceil() as i32+3 {
            let x=i as f32*span-(cam*parallax).rem_euclid(span);
            let h=50.0+((i*19+layer as i32*17).rem_euclid(29)) as f32;
            draw_triangle(vec2(x-14.0,133.0),vec2(x+15.0,133.0),vec2(x,133.0-h),color);
            draw_rectangle(x-3.0,105.0,6.0,28.0,color);
        }
    }
}
fn props(w:&World,vw:f32,cam:f32,assets:&Assets) {
    let mi=w.map.index();
    for &x in TORCHES[mi] {
        let sx=x-cam;if sx < -20.0 || sx>vw+20.0 {continue;}
        let y=w.floor(x);
        draw_rectangle(sx-2.0,y-23.0,4.0,22.0,rgb(76,48,32));
        draw_circle(sx,y-27.0,8.0,rgba(255,130,40,40));
        draw_triangle(vec2(sx-4.0,y-27.0),vec2(sx+4.0,y-27.0),vec2(sx,y-36.0),
            rgb(255,182,47));
    }
    for chest in &w.chests[mi] {
        let sx=chest.x-cam;if sx< -20.0||sx>vw+20.0 {continue;}
        let y=w.floor(chest.x);
        if let Some(sprite)=assets.chest(chest.opened) {
            draw_texture_ex(sprite,sx-sprite.width()/2.0,y-sprite.height()+1.0,
                WHITE,DrawTextureParams::default());
        } else {
            draw_rectangle(sx-9.0,y-14.0,18.0,13.0,rgb(130,82,36));
            draw_rectangle(sx-9.0,y-14.0,18.0,3.0,rgb(180,132,65));
            draw_rectangle(sx-2.0,y-10.0,4.0,5.0,rgb(232,192,78));
            if chest.opened {draw_rectangle(sx-9.0,y-17.0,18.0,3.0,rgb(90,64,35));}
        }
    }
    if w.cleared[mi] || w.boss_stages[mi]==BossStage::Defeated || w.spawned>0 {
        portal(w,5040.0,cam,w.portal_available());
        if w.map==MapId::DarkForest {portal(w,4920.0,cam,w.portal_available());}
    }
}
fn portal(w:&World,x:f32,cam:f32,open:bool){
    let sx=x-cam;let y=w.floor(x);
    let stone=rgb(148,145,126);
    draw_rectangle(sx-17.0,y-57.0,7.0,57.0,stone);
    draw_rectangle(sx+10.0,y-57.0,7.0,57.0,stone);
    draw_circle_lines(sx,y-52.0,14.0,7.0,stone);
    draw_rectangle(sx-9.0,y-50.0,18.0,50.0,if open {rgb(35,132,146)} else {rgb(8,11,19)});
    if open {draw_circle(sx,y-25.0,6.0,rgb(74,240,216));}
}
fn enemy(e:&realm_core::Enemy,cam:f32,t:u64,assets:&Assets){
    let x=e.x-cam;let y=e.y;let scale=e.scale;
    let color=match e.kind {
        0|20..=39=>rgb(83,190,105),
        3=>rgb(75,83,139),4=>rgb(230,229,207),
        40..=49=>rgb(142,111,118),60|61=>rgb(83,120,77),
        62=>rgb(171,158,144),_=>rgb(118,108,156)
    };
    let width=if e.boss {28.0*scale}else{13.0*scale};
    let height=if e.boss {35.0*scale}else{20.0*scale};
    let anim_frame=if (40..=49).contains(&e.kind) {
        if e.attack>0 {0}else{1+(t/9%8) as usize}
    } else if e.kind==60 || e.kind==61 {(t/8%4) as usize}
    else {(t/12%2) as usize};
    let sprite=if e.kind==62 {
        if e.attack>0 {assets.king("s1",(t/5%2) as usize)}
        else {assets.king("w",(t/8%4) as usize)}
    } else if (40..=49).contains(&e.kind) && e.attack>0 {
        assets.humanoid_attack(e.kind,(t/5%3) as usize)
    } else {assets.enemy(e.kind,anim_frame)};
    if let Some(sprite)=sprite {
        let sw=sprite.width()*scale;
        let sh=sprite.height()*scale;
        draw_texture_ex(sprite,x-sw/2.0,y-sh+scale,WHITE,DrawTextureParams {
            dest_size:Some(vec2(sw,sh)),flip_x:e.facing<0.0,
            ..Default::default()
        });
    } else {
    draw_ellipse(x,y+1.0,width*0.4,3.0,0.0,rgba(0,0,0,140));
    if e.kind==0 || (20..=39).contains(&e.kind) {
        draw_circle(x,y-height*0.5,width*0.55,color);
        draw_rectangle(x-width*0.3,y-height*0.52,3.0*scale,3.0*scale,WHITE);
        draw_rectangle(x+width*0.1,y-height*0.52,3.0*scale,3.0*scale,WHITE);
    } else {
        draw_rectangle(x-width*0.4,y-height*0.75,width*0.8,height*0.58,color);
        draw_rectangle(x-width*0.32,y-height,width*0.64,height*0.3,if e.flash>0 {WHITE}else{color});
        draw_rectangle(x-width*0.3,y-height*0.22,width*0.24,height*0.2,rgb(29,25,38));
        draw_rectangle(x+width*0.08,y-height*0.22,width*0.24,height*0.2,rgb(29,25,38));
        draw_rectangle(x+width*0.11,y-height*0.85,3.0,2.0,rgb(252,108,81));
        if e.kind==62 {draw_rectangle(x-width*0.5,y-height*1.1,width,5.0,rgb(216,176,80));}
    }
    }
    let bar_y=y-height-6.0;
    draw_rectangle(x-width*0.5,bar_y,width,3.0,rgb(14,13,20));
    draw_rectangle(x-width*0.5,bar_y,width*(e.hp/e.max_hp).clamp(0.0,1.0),2.0,rgb(220,68,74));
    if e.boss {centered(e.name(),x,bar_y-3.0,9.0,rgb(246,206,157));}
    let _=t;
}
fn game_hud(w:&World,vw:f32){
    panel(5.0,4.0,137.0,34.0);
    label("LV",10.0,13.0,9.0,rgb(250,203,112));
    label(&w.player.level.to_string(),27.0,13.0,9.0,WHITE);
    let hp=(w.player.hp/w.player.max_hp).clamp(0.0,1.0);
    let mp=(w.player.mp/w.player.max_mp).clamp(0.0,1.0);
    draw_rectangle(9.0,17.0,92.0,5.0,rgb(48,30,42));
    draw_rectangle(9.0,17.0,92.0*hp,5.0,rgb(228,60,86));
    draw_rectangle(9.0,25.0,92.0,4.0,rgb(35,46,80));
    draw_rectangle(9.0,25.0,92.0*mp,4.0,rgb(77,186,255));
    label(&format!("{}G {}D",w.player.gold,w.player.gems),102.0,20.0,8.0,rgb(255,213,110));
    centered(w.map.name(),vw/2.0,13.0,12.0,rgb(243,214,167));
    centered(&format!("{} / {}",w.killed.min(w.map.quota()),w.map.quota()),
        vw/2.0,24.0,9.0,rgb(190,219,226));
    if let Some(b)=w.boss(){
        draw_rectangle(vw*0.24,38.0,vw*0.52,6.0,rgb(13,9,26));
        draw_rectangle(vw*0.24,38.0,vw*0.52*(b.hp/b.max_hp).clamp(0.0,1.0),6.0,rgb(206,53,70));
        centered(b.name(),vw/2.0,36.0,11.0,rgb(252,204,175));
    }
    draw_rectangle(vw-26.0,4.0,21.0,21.0,rgba(20,35,60,220));
    centered("II",vw-15.5,20.0,13.0,WHITE);
    for b in buttons(vw) {
        if b.id==ButtonId::Interact && !w.player_near_portal(){continue;}
        let color=if matches!(b.id,ButtonId::Skill0|ButtonId::Skill1|ButtonId::Skill2) {
            rgb(66,64,113)
        } else if b.id==ButtonId::Dash {rgb(94,74,28)}
          else if b.id==ButtonId::Shield {rgb(25,90,134)}
          else {rgb(44,53,67)};
        draw_circle(b.center.x,b.center.y,b.radius,rgba(0,0,0,130));
        draw_circle(b.center.x,b.center.y,b.radius-2.0,color);
        draw_circle_lines(b.center.x,b.center.y,b.radius-1.0,1.0,rgb(151,158,177));
        centered(b.caption,b.center.x,b.center.y+3.0,
            if b.caption.len()>3 {8.0}else{11.0},WHITE);
        let cd=match b.id {
            ButtonId::Skill0=>w.player.cooldowns[0],
            ButtonId::Skill1=>w.player.cooldowns[1],
            ButtonId::Skill2=>w.player.cooldowns[2],
            ButtonId::Dash=>w.player.cooldowns[3],
            ButtonId::Shield=>w.player.shield_cd,
            _=>0
        };
        if cd>0 {centered(&cd.to_string(),b.center.x,b.center.y-12.0,8.0,rgb(255,193,90));}
    }
    if w.message_ticks>0 {centered(&w.message,vw/2.0,57.0,15.0,rgb(250,202,99));}
}
pub fn draw_game(w:&World,vw:f32,assets:&Assets){
    let cam=(w.player.x-vw/2.0+w.player.facing*24.0).clamp(0.0,WORLD_WIDTH-vw);
    background(w,vw,cam);
    terrain(w,vw,cam,assets);
    props(w,vw,cam,assets);
    for pickup in &w.pickups {
        let x=pickup.x-cam;
        let color=match pickup.kind {
            PickupKind::Gold=>rgb(245,202,78),PickupKind::Gem=>rgb(57,219,225),
            PickupKind::Heart=>rgb(237,71,85),PickupKind::Mana=>rgb(83,155,249)
        };
        draw_circle(x,pickup.y,3.0,color);
    }
    for e in &w.enemies {if (e.x-cam).abs()<vw+50.0 {enemy(e,cam,w.tick,assets);}}
    hero(w.player.x-cam,w.player.y,w.player.class,w.player.facing,w.tick,
        w.player.shield_ticks>0,w.player.dash_ticks>0,w.player.moving,
        w.player.melee_ticks>0,assets);
    for q in &w.projectiles {
        let color=match q.kind {
            ProjectileKind::Orb=>rgb(176,107,255),ProjectileKind::Knife=>rgb(235,240,255),
            ProjectileKind::Explosive=>rgb(255,131,57),ProjectileKind::EnemyArrow=>rgb(242,192,92),
        };
        draw_circle(q.x-cam,q.y,q.radius,color);
        draw_circle(q.x-cam,q.y,(q.radius/2.0).max(1.0),WHITE);
    }
    game_hud(w,vw);
    if w.game_over {
        draw_rectangle(0.0,0.0,vw,180.0,rgba(0,0,0,195));
        centered("HAS CAIDO",vw/2.0,80.0,25.0,rgb(245,89,91));
        centered("R o tocar para reiniciar",vw/2.0,117.0,13.0,WHITE);
    }
}
fn home_card_rect(i:usize,vw:f32)->Rect {
    Rect::new(vw/2.0-125.0+i as f32*86.0,63.0,79.0,57.0)
}
pub fn home_character_hit(p:Vec2,vw:f32)->Option<usize> {
    (0..3).find(|i|home_card_rect(*i,vw).contains(p))
}
pub fn home_play_hit(p:Vec2,vw:f32)->bool {
    Rect::new(vw/2.0-62.0,132.0,124.0,30.0).contains(p)
}
pub fn draw_home(vw:f32,class:HeroClass,_level:u16,assets:&Assets){
    for i in 0..22 {
        draw_rectangle(0.0,i as f32*9.0,vw,9.0,rgb((8+i/3) as u8,(14+i/2) as u8,(30+i) as u8));
    }
    for i in 0..40 {draw_rectangle((i*61) as f32%vw,(i*47%65) as f32,1.0,1.0,rgba(185,237,245,170));}
    centered("RETRO REALM",vw/2.0,27.0,29.0,rgb(245,212,111));
    centered("ELIGE TU PERSONAJE",vw/2.0,47.0,11.0,rgb(214,225,240));
    for i in 0..3 {
        let chosen=HeroClass::from_index(i);
        let r=home_card_rect(i,vw);
        panel(r.x,r.y,r.w,r.h);
        if class==chosen {draw_rectangle_lines(r.x,r.y,r.w,r.h,2.0,rgb(240,201,87));}
        let name=match i {0=>"MAGA LUNAR",1=>"CAZADORA",_=>"MAPACHE CYBER"};
        hero(r.x+r.w/2.0,r.y+41.0,chosen,1.0,0,false,false,false,false,assets);
        centered(name,r.x+r.w/2.0,r.y+53.0,10.0,rgb(231,213,166));
    }
    draw_rectangle(vw/2.0-62.0,132.0,124.0,30.0,rgb(18,63,102));
    draw_rectangle_lines(vw/2.0-62.0,132.0,124.0,30.0,2.0,rgb(79,205,255));
    centered("JUGAR",vw/2.0,153.0,20.0,WHITE);
    centered("1 / 2 / 3: personaje     ENTER: jugar",vw/2.0,175.0,10.0,rgb(155,184,204));
}
pub fn draw_pause(vw:f32){
    draw_rectangle(0.0,0.0,vw,180.0,rgba(0,0,0,187));
    centered("PAUSA",vw/2.0,65.0,28.0,rgb(248,206,113));
    centered("ESC - CONTINUAR",vw/2.0,89.0,15.0,WHITE);
    draw_rectangle(vw/2.0-65.0,105.0,130.0,24.0,rgb(26,49,84));
    centered("H - MENU PRINCIPAL",vw/2.0,121.0,12.0,rgb(215,231,255));
}
