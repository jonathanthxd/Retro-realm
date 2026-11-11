//! UI nativa siguiendo la última composición del HTML original (no la UI
//! provisional de la primera APK). Dimensiones: Canvas lógico de 180px alto.
use crate::{assets::Assets,render};
use macroquad::prelude::*;
use realm_core::{HeroClass,World};
use render::{Button,ButtonId};

fn rgb(r:u8,g:u8,b:u8)->Color{Color::from_rgba(r,g,b,255)}
fn rgba(r:u8,g:u8,b:u8,a:u8)->Color{Color::from_rgba(r,g,b,a)}
fn txt(s:&str,x:f32,y:f32,size:f32,color:Color){
    draw_text(s,x+0.7,y+0.8,size,rgba(0,0,0,215));
    draw_text(s,x,y,size,color);
}
fn centered(s:&str,x:f32,y:f32,size:f32,color:Color){
    let width=measure_text(s,None,size as u16,1.0).width;
    txt(s,x-width/2.0,y,size,color);
}
fn framed(x:f32,y:f32,w:f32,h:f32,selected:bool){
    let border=if selected{rgb(245,201,85)}else{rgb(139,134,152)};
    draw_rectangle(x+1.3,y+2.0,w,h,rgba(0,0,0,140));
    draw_rectangle(x-1.2,y-1.2,w+2.4,h+2.4,rgb(12,10,17));
    draw_rectangle(x,y,w,h,border);
    draw_rectangle(x+1.5,y+1.5,w-3.0,h-3.0,rgb(38,36,46));
    draw_rectangle(x+2.5,y+2.5,w-5.0,2.2,rgb(76,72,83));
    draw_rectangle(x+2.5,y+h-4.0,w-5.0,1.5,rgb(28,26,36));
    if selected{
        draw_rectangle_lines(x-2.0,y-2.0,w+4.0,h+4.0,0.7,rgba(255,224,117,155));
    }
}
fn button(x:f32,y:f32,w:f32,h:f32,title:&str) {
    framed(x,y,w,h,true);
    draw_rectangle(x+4.0,y+3.0,w-8.0,1.5,rgba(255,251,215,65));
    centered(title,x+w/2.0,y+h*0.65,
        if title.len()>10{11.0}else{14.0},rgb(249,233,186));
}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum HomePage{Landing,Selection}
pub fn home_play_rect(vw:f32)->Rect{Rect::new(vw-88.0,132.0,75.0,33.0)}
pub fn home_hero_rect()->Rect{Rect::new(10.0,128.0,51.0,42.0)}
pub fn home_name_rect(vw:f32)->Rect{Rect::new(vw/2.0-57.0,27.0,114.0,17.0)}
pub fn home_back_rect(vw:f32)->Rect{Rect::new(vw/2.0-43.0,145.0,86.0,23.0)}
pub fn home_selection_rect(i:usize,vw:f32)->Rect{
    Rect::new(vw/2.0-122.0+i as f32*83.0,55.0,78.0,78.0)
}
fn portrait(assets:&Assets,cl:HeroClass,x:f32,y:f32,w:f32,h:f32){
    if let Some(sprite)=assets.portrait(cl){
        draw_texture_ex(sprite,x,y,WHITE,DrawTextureParams{
            dest_size:Some(vec2(w,h)),..Default::default()
        });
    }else if let Some(sprite)=assets.hero(cl,0){
        draw_texture_ex(sprite,x,y,WHITE,DrawTextureParams{
            dest_size:Some(vec2(w,h)),..Default::default()
        });
    }
}
pub fn draw_home(
    w:&World,vw:f32,hero:HeroClass,page:HomePage,name:&str,level:u16,assets:&Assets
){
    render::draw_menu_scene(w,vw,hero,assets);
    // Doble borde dorado del #home y tipografía principal.
    draw_rectangle_lines(3.2,3.2,vw-6.4,173.6,0.7,rgba(237,197,97,150));
    draw_rectangle_lines(5.0,5.0,vw-10.0,170.0,0.3,rgba(255,233,139,80));
    if page==HomePage::Landing{
        centered("RETRO REALM",vw/2.0,20.0,22.0,rgb(250,215,111));
        draw_rectangle(vw/2.0-61.0,25.0,122.0,20.0,rgba(0,0,0,150));
        framed(vw/2.0-58.0,27.0,116.0,17.0,false);
        centered(name,vw/2.0,39.0,13.0,rgb(244,236,215));
        framed(vw/2.0-29.0,48.0,58.0,13.0,true);
        centered(&format!("NIVEL {}",level),vw/2.0,57.5,9.0,rgb(247,217,133));
        // Cuadro de héroe abajo a la izquierda, como HTML #hb.
        let r=home_hero_rect();
        framed(r.x,r.y,r.w-2.0,r.h-3.0,true);
        portrait(assets,hero,r.x+10.0,r.y+1.0,28.0,31.0);
        centered("HÉROE",r.x+24.5,r.y+r.h+6.0,10.0,rgb(251,220,151));
        let r=home_play_rect(vw);
        button(r.x,r.y,r.w,r.h,"JUGAR");
    }else{
        draw_rectangle(0.0,0.0,vw,180.0,rgba(0,0,0,175));
        centered("ELEGIR PERSONAJE",vw/2.0,38.0,21.0,rgb(246,213,110));
        let descriptions=[
            "MISIL / RAYO / NOVA",
            "CUCHILLOS / ABANICO",
            "GARRAS / ENERGIA",
        ];
        for i in 0..3{
            let cl=HeroClass::from_index(i);
            let r=home_selection_rect(i,vw);
            framed(r.x,r.y,r.w,r.h,cl==hero);
            portrait(assets,cl,r.x+20.0,r.y+8.0,38.0,41.0);
            centered(cl.name(),r.x+r.w/2.0,r.y+59.0,9.0,
                if cl==hero{rgb(254,226,144)}else{rgb(220,218,226)});
            centered(descriptions[i],r.x+r.w/2.0,r.y+69.0,6.0,
                rgb(179,189,209));
        }
        let r=home_back_rect(vw);button(r.x,r.y,r.w,r.h,"VOLVER");
    }
}
fn oct(x:f32,y:f32,w:f32,h:f32,cut:f32,fill:Color,stroke:Color){
    let p=[vec2(x+cut,y),vec2(x+w-cut,y),vec2(x+w,y+cut),vec2(x+w,y+h-cut),
        vec2(x+w-cut,y+h),vec2(x+cut,y+h),vec2(x,y+h-cut),vec2(x,y+cut)];
    for i in 1..7 {draw_triangle(p[0],p[i],p[i+1],fill);}
    for i in 0..8 {
        let a=p[i];let b=p[(i+1)%8];
        draw_line(a.x,a.y,b.x,b.y,0.75,stroke);
    }
}
fn bar(x:f32,y:f32,w:f32,val:f32,red:bool){
    draw_rectangle(x-1.1,y-1.3,w+2.2,7.1,rgb(7,14,28));
    draw_rectangle_lines(x-1.1,y-1.3,w+2.2,7.1,0.6,rgb(103,120,157));
    let fill=w*val.clamp(0.0,1.0);
    draw_rectangle(x,y,w,4.5,rgb(16,22,38));
    draw_rectangle(x,y,fill,4.5,if red{rgb(145,23,39)}else{rgb(31,71,189)});
    draw_rectangle(x,y,fill,1.5,if red{rgb(254,136,145)}else{rgb(143,209,255)});
}
pub fn draw_hud(w:&World,vw:f32,assets:&Assets){
    let p=&w.player;
    let hp=(p.hp/p.max_hp).clamp(0.0,1.0);
    let mp=(p.mp/p.max_mp).clamp(0.0,1.0);
    // HUD DOM final del HTML: #hd2, con retrato, nombre y barras.
    draw_circle(19.0,19.0,17.7,rgba(1,6,17,185));
    draw_circle(19.0,18.4,16.6,rgb(98,113,142));
    draw_circle(19.0,18.4,15.6,rgb(175,197,218));
    draw_circle(19.0,18.4,13.6,rgb(23,50,67));
    portrait(assets,p.class,6.4,5.8,25.2,25.2);
    draw_circle_lines(19.0,18.4,13.6,1.4,rgb(37,54,80));
    for &(a,b) in &[(8.0,7.0),(30.0,7.0),(8.0,29.0),(30.0,29.0)] {
        draw_rectangle(a-1.6,b-1.6,3.2,3.2,rgb(206,217,234));
    }
    oct(27.0,29.3,15.0,9.0,2.5,rgb(17,28,52),rgb(239,200,83));
    centered(&p.level.to_string(),34.5,36.0,8.5,rgb(252,225,142));
    oct(31.2,4.2,68.0,11.0,1.7,rgb(53,77,102),rgb(137,174,210));
    centered("JUGADOR",65.0,12.1,9.5,rgb(170,243,251));
    bar(39.0,18.9,54.0,mp,false);
    draw_circle(36.0,21.4,5.0,rgb(18,32,55));
    draw_triangle(vec2(36.0,17.5),vec2(32.8,23.0),vec2(39.2,23.0),rgb(105,231,255));
    centered(&format!("{}/{}",p.mp as i32,p.max_mp as i32),66.0,23.0,7.2,WHITE);
    bar(39.0,28.9,54.0,hp,true);
    draw_circle(36.0,31.3,5.0,rgb(18,32,55));
    draw_circle(34.5,30.6,2.0,rgb(252,82,99));
    draw_circle(37.5,30.6,2.0,rgb(252,82,99));
    draw_triangle(vec2(32.5,31.5),vec2(39.5,31.5),vec2(36.0,35.3),rgb(246,61,82));
    centered(&format!("{}/{}",p.hp as i32,p.max_hp as i32),66.0,33.0,7.2,WHITE);
    if hp<0.3 && (w.tick/15)%2==0{
        draw_rectangle_lines(37.6,27.7,56.8,8.0,0.9,rgb(255,67,76));
    }
    oct(33.0,37.7,30.5,10.0,3.0,rgb(14,23,41),rgb(59,75,105));
    draw_circle(39.5,42.7,3.5,rgb(245,192,50));
    draw_circle(38.6,41.6,1.0,rgb(255,243,170));
    txt(&p.gold.to_string(),45.0,45.7,8.3,rgb(252,231,155));
    oct(65.0,37.7,31.5,10.0,3.0,rgb(14,23,41),rgb(59,75,105));
    draw_triangle(vec2(72.0,38.9),vec2(68.5,42.8),vec2(72.0,46.0),rgb(54,197,228));
    draw_triangle(vec2(72.0,38.9),vec2(75.5,42.8),vec2(72.0,46.0),rgb(161,240,255));
    txt(&p.gems.to_string(),80.0,45.7,8.3,rgb(143,233,251));
    let xp=(p.xp as f32/(p.level as f32*30.0).max(1.0)).clamp(0.0,1.0);
    draw_rectangle(36.8,48.8,57.0,1.4,rgb(20,20,30));
    draw_rectangle(36.8,48.8,57.0*xp,1.4,rgb(240,202,77));
    centered(w.map.name(),vw*0.58,10.0,9.5,rgb(243,212,158));
    centered(&format!("{}/{}",w.killed.min(w.map.quota()),w.map.quota()),
        vw*0.58,19.0,8.0,rgb(203,228,225));
    if let Some(b)=w.boss(){
        let bw=vw*0.46;
        oct(vw/2.0-bw/2.0,26.0,bw,10.0,2.5,rgb(19,13,30),rgb(232,185,83));
        let fill=(b.hp/b.max_hp).clamp(0.0,1.0)*(bw-6.0);
        draw_rectangle(vw/2.0-bw/2.0+3.0,29.0,fill,4.0,rgb(204,48,61));
        centered(b.name(),vw/2.0,25.0,9.0,rgb(246,212,171));
    }
    // Botón de pausa estilo engranaje/metálico, reemplaza la caja azul vieja.
    framed(vw-23.0,3.0,18.0,18.0,true);
    draw_rectangle(vw-17.7,8.0,2.4,8.0,rgb(248,218,148));
    draw_rectangle(vw-12.5,8.0,2.4,8.0,rgb(248,218,148));

    for b in render::buttons(vw){
        if b.id==ButtonId::Interact && !w.player_near_portal(){continue;}
        draw_control(b,p.class,p.cooldowns,p.shield_cd,assets);
    }
    if w.message_ticks>0{
        framed(vw/2.0-64.0,54.0,128.0,17.0,true);
        centered(&w.message,vw/2.0,65.3,10.5,rgb(255,228,156));
    }
}
fn draw_control(b:Button,hero:HeroClass,cds:[u16;4],shield_cd:u16,assets:&Assets){
    let (accent,dark)=match b.id{
        ButtonId::Left|ButtonId::Right=>(rgb(226,193,97),rgb(83,82,98)),
        ButtonId::Jump=>(rgb(235,197,89),rgb(48,46,57)),
        ButtonId::Melee=>(rgb(255,92,86),rgb(60,32,42)),
        ButtonId::Skill0=>(rgb(164,108,255),rgb(35,30,67)),
        ButtonId::Skill1=>(rgb(90,200,255),rgb(31,39,70)),
        ButtonId::Skill2=>(rgb(150,185,255),rgb(30,40,70)),
        ButtonId::Dash=>(rgb(254,128,68),rgb(56,40,36)),
        ButtonId::Shield=>(rgb(106,218,232),rgb(31,52,72)),
        ButtonId::Interact=>(rgb(107,240,205),rgb(36,60,58)),
    };
    let cx=b.center.x;let cy=b.center.y;let r=b.radius;
    // Botones originales con disco, doble bisel y halo de color.
    draw_circle(cx,cy+1.4,r+2.2,rgba(0,0,0,150));
    draw_circle(cx,cy,r+1.0,rgb(11,11,19));
    draw_circle(cx,cy,r,accent);
    draw_circle(cx,cy,r-1.2,rgb(113,110,124));
    draw_circle(cx,cy,r-2.2,dark);
    draw_circle(cx-1.5,cy-2.0,(r-4.0).max(2.0),rgba(200,211,244,28));
    let key=match b.id{
        ButtonId::Left=>"bl",ButtonId::Right=>"br",ButtonId::Jump=>"bj",
        ButtonId::Melee=>"bm",ButtonId::Skill0=>"s0",ButtonId::Skill1=>"s1",
        ButtonId::Skill2=>"s2",ButtonId::Dash=>"s3",_=>"",
    };
    if let Some(texture)=assets.ui_icon(key,hero){
        let d=if matches!(b.id,ButtonId::Left|ButtonId::Right){r*2.0}else{r*1.40};
        draw_texture_ex(texture,cx-d/2.0,cy-d/2.0,WHITE,
            DrawTextureParams{dest_size:Some(vec2(d,d)),..Default::default()});
    }else{
        centered(b.caption,cx,cy+2.8,if b.caption.len()>2{7.0}else{9.0},WHITE);
    }
    let cd=match b.id{
        ButtonId::Skill0=>cds[0],ButtonId::Skill1=>cds[1],
        ButtonId::Skill2=>cds[2],ButtonId::Dash=>cds[3],
        ButtonId::Shield=>shield_cd,_=>0
    };
    if cd>0{
        draw_circle(cx+r*.65,cy+r*.6,4.3,rgb(14,12,22));
        draw_circle_lines(cx+r*.65,cy+r*.6,4.3,0.65,rgb(242,204,95));
        centered(&(1+cd as u32/60).to_string(),cx+r*.65,cy+r*.6+2.0,
            6.4,rgb(255,230,145));
    }
}
pub fn draw_pause(vw:f32,audio_enabled:bool){
    draw_rectangle(0.0,0.0,vw,180.0,rgba(0,0,0,190));
    let x=vw/2.0-75.0;
    framed(x,24.0,150.0,137.0,true);
    centered("PAUSA",vw/2.0,50.0,24.0,rgb(251,215,115));
    centered("RETRO REALM",vw/2.0,63.0,10.0,rgb(189,204,229));
    centered("ESC / II  CONTINUAR",vw/2.0,82.0,10.5,WHITE);
    framed(vw/2.0-65.0,103.0,130.0,27.0,false);
    centered("MENU PRINCIPAL",vw/2.0,120.0,12.0,rgb(246,228,183));
    framed(vw/2.0-65.0,133.0,130.0,27.0,false);
    centered(if audio_enabled{"AUDIO: ACTIVADO"}else{"AUDIO: DESACTIVADO"},
        vw/2.0,150.0,11.0,rgb(231,222,200));
}