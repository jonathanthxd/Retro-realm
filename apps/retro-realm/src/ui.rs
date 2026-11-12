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
// El HTML incrusta RetroPx en WOFF. Lo rasterizamos en PNG vía Canvas para
// mantener su aspecto sin introducir dependencia de formatos de fuente.
fn font_centered(assets:&Assets,s:&str,x:f32,baseline:f32,size:f32,color:Color) {
    let Some(atlas)=assets.font_atlas() else {centered(s,x,baseline,size,color);return;};
    let extra="ÁÉÍÓÚÜÑáéíóúüñ¡¿";
    let adv=size*10.2/16.0;
    let count=s.chars().count() as f32;
    let mut left=x-count*adv/2.0;
    let scale=size/16.0;
    for ch in s.chars(){
        let idx=if (32..127).contains(&(ch as u32)){
            (ch as usize)-32
        } else if let Some(j)=extra.chars().position(|c|c==ch) {
            95+j
        } else {
            31 // '?' = ASCII 63 - 32
        };
        let col=(idx%16) as f32;
        let row=(idx/16) as f32;
        draw_texture_ex(atlas,left,baseline-16.0*scale,color,DrawTextureParams{
            source:Some(Rect::new(col*18.0,row*20.0,18.0,20.0)),
            dest_size:Some(vec2(18.0*scale,20.0*scale)),
            ..Default::default()
        });
        left+=adv;
    }
}
pub fn draw_world_text(assets:&Assets,label:&str,x:f32,y:f32,size:f32,color:Color){
    font_centered(assets,label,x,y,size,color);
}
pub fn draw_player_tag(assets:&Assets,name:&str,x:f32,y:f32){
    // Nombre flotante amarillo del personaje, permanece en cámara del mundo.
    font_centered(assets,name,x+1.0,y+1.0,10.5,rgba(0,0,0,220));
    font_centered(assets,name,x,y,10.5,rgb(245,210,115));
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
fn button(x:f32,y:f32,w:f32,h:f32,title:&str,assets:&Assets) {
    framed(x,y,w,h,true);
    draw_rectangle(x+4.0,y+3.0,w-8.0,1.5,rgba(255,251,215,65));
    font_centered(assets,title,x+w/2.0,y+h*0.68,
        if title.len()>10{7.0}else{9.5},rgb(249,233,186));
}
#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum HomePage{Landing,Selection}
pub fn home_play_rect(vw:f32)->Rect{Rect::new(vw-59.0,155.0,54.0,20.0)}
pub fn home_hero_rect()->Rect{Rect::new(5.0,146.0,22.0,22.0)}
pub fn home_name_rect(vw:f32)->Rect{Rect::new(vw/2.0-40.0,20.0,80.0,15.0)}
pub fn home_back_rect(vw:f32)->Rect{Rect::new(vw/2.0-29.0,140.0,58.0,19.0)}
pub fn home_selection_rect(i:usize,vw:f32)->Rect{
    // #hs .cd: max-width 130px + padding y borde a 960px reales.
    Rect::new(vw/2.0-82.0+i as f32*57.0,68.0,50.0,54.0)
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
        font_centered(assets,"RETRO REALM",vw/2.0,15.3,13.0,rgb(250,215,111));
        // La captura HTML utiliza un panel de ~237x43 px en 960x540.
        framed(vw/2.0-40.0,20.0,80.0,14.0,true);
        font_centered(assets,name,vw/2.0,29.8,7.4,rgb(244,236,215));
        framed(vw/2.0-13.2,36.4,26.4,8.8,true);
        font_centered(assets,&format!("NIVEL {}",level),vw/2.0,43.2,6.7,rgb(247,217,133));
        // Cuadro de héroe abajo a la izquierda, como HTML #hb.
        let r=home_hero_rect();
        framed(r.x,r.y,r.w,r.h,true);
        portrait(assets,hero,r.x+2.0,r.y+1.0,18.0,18.0);
        font_centered(assets,"HÉROE",r.x+r.w/2.0,r.y+r.h+6.3,6.7,rgb(251,220,151));
        let r=home_play_rect(vw);
        button(r.x,r.y,r.w,r.h,"JUGAR",assets);
    }else{
        draw_rectangle(0.0,0.0,vw,180.0,rgba(0,0,0,175));
        font_centered(assets,"ELEGIR PERSONAJE",vw/2.0,42.0,10.0,rgb(246,213,110));
        // Respetar el ajuste multilinea de la tarjeta .cd original.
        let descriptions=[
            ("MISIL / RAYO","NOVA"),
            ("CUCHILLOS","ABANICO"),
            ("GARRAS","ENERGIA"),
        ];
        for i in 0..3{
            let cl=HeroClass::from_index(i);
            let r=home_selection_rect(i,vw);
            framed(r.x,r.y,r.w,r.h,cl==hero);
            portrait(assets,cl,r.x+13.0,r.y+3.0,24.0,26.0);
            font_centered(assets,cl.name(),r.x+r.w/2.0,r.y+38.0,5.4,
                if cl==hero{rgb(254,226,144)}else{rgb(220,218,226)});
            font_centered(assets,descriptions[i].0,r.x+r.w/2.0,r.y+46.5,4.9,
                rgb(179,189,209));
            font_centered(assets,descriptions[i].1,r.x+r.w/2.0,r.y+51.8,4.9,
                rgb(179,189,209));
        }
        let r=home_back_rect(vw);button(r.x,r.y,r.w,r.h,"VOLVER",assets);
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
fn boss_bar(w:&World,vw:f32,assets:&Assets){
    let Some(boss)=w.boss() else{return;};
    let current:f32=w.enemies.iter().filter(|e|e.boss).map(|e|e.hp.max(0.0)).sum();
    let total:f32=w.enemies.iter().filter(|e|e.boss).map(|e|e.max_hp.max(1.0)).sum();
    let hp=(current/total.max(1.0)).clamp(0.0,1.0);
    // ZBossBar original: 212px, nueve segmentos, remates plateados y calavera.
    let x=(vw-212.0)/2.0;
    let y=12.0;
    draw_rectangle(x-7.0,y-1.0,226.0,13.0,rgb(12,8,20));
    draw_rectangle(x-5.0,y,222.0,11.0,rgb(46,30,64));
    draw_rectangle(x,y+1.0,212.0,9.0,rgb(22,10,28));
    let width=212.0*hp;
    draw_rectangle(x,y+1.0,width,9.0,rgb(184,40,56));
    draw_rectangle(x,y+1.0,width,2.0,rgb(238,106,118));
    draw_rectangle(x,y+8.0,width,2.0,rgb(112,20,36));
    for i in 1..9{
        draw_rectangle(x+(212*i/9) as f32-1.0,y,2.0,11.0,rgb(26,14,38));
    }
    for side in [-1.0f32,1.0]{
        let ex=if side<0.0{x-9.0}else{x+221.0};
        draw_rectangle(ex-2.0,y-2.0,5.0,15.0,rgb(232,228,240));
        draw_rectangle(ex+side*3.0-1.0,y-4.0,3.0,4.0,rgb(232,228,240));
        draw_rectangle(ex+side*3.0-1.0,y+11.0,3.0,4.0,rgb(232,228,240));
        draw_rectangle(ex-1.0,y,3.0,11.0,rgb(46,30,64));
    }
    let cx=vw/2.0;
    draw_rectangle(cx-14.0,y-6.0,28.0,6.0,rgb(22,10,28));
    for i in -4i32..=4 {
        draw_rectangle(cx+i as f32*3.0-1.0,
            y-8.0+(i&1) as f32*2.0-if i.abs()>2{2.0}else{0.0},
            2.0,6.0,rgb(46,30,64));
    }
    draw_circle(cx,y-4.0,5.0,rgb(232,228,240));
    draw_rectangle(cx-3.0,y-1.0,6.0,3.0,rgb(232,228,240));
    draw_rectangle(cx-3.0,y-5.0,2.0,2.0,rgb(22,10,28));
    draw_rectangle(cx+1.0,y-5.0,2.0,2.0,rgb(22,10,28));
    draw_rectangle(cx-1.0,y-2.0,2.0,1.0,rgb(22,10,28));
    font_centered(assets,boss.name(),cx,y+22.0,7.0,rgb(240,208,112));
}
pub fn draw_hud(w:&World,vw:f32,assets:&Assets,name:&str){
    let p=&w.player;
    boss_bar(w,vw,assets);
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
    font_centered(assets,name,65.0,12.1,9.5,rgb(170,243,251));
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
    if w.boss().is_none() {
        font_centered(assets,w.map.name(),vw*0.58,10.0,9.0,rgb(243,212,158));
        font_centered(assets,&format!("{}/{}",w.killed.min(w.map.quota()),w.map.quota()),
            vw*0.58,19.0,8.0,rgb(203,228,225));
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
        draw_circle(cx+r*0.65,cy+r*0.6,4.3,rgb(14,12,22));
        draw_circle_lines(cx+r*0.65,cy+r*0.6,4.3,0.65,rgb(242,204,95));
        centered(&(1+cd as u32/60).to_string(),cx+r*0.65,cy+r*0.6+2.0,
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