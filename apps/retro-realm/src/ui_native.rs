//! Interfaz nativa de alta resolución basada en capturas PNG del DOM HTML.
 //! La escena continúa a 180px; estos elementos NO pasan por el render target
 //! pixelado. Las dimensiones son CSS-px a 960×540 y están ancladas como el DOM.
use crate::assets::Assets;
use macroquad::prelude::*;
use realm_core::{World,HeroClass};

fn rgb(r:u8,g:u8,b:u8)->Color {Color::from_rgba(r,g,b,255)}
fn rgba(r:u8,g:u8,b:u8,a:u8)->Color {Color::from_rgba(r,g,b,a)}

/// Resolución CSS de referencia para un viewport landscape. En una pantalla
/// más ancha se mantienen tamaños físicos y las anclas al borde.
pub fn css_width()->f32 {
    (540.0*screen_width()/screen_height().max(1.0)).max(960.0)
}
fn class_index(c:HeroClass)->usize{
    match c { HeroClass::LunarMage=>0,HeroClass::Huntress=>1,HeroClass::CyberRaccoon=>2 }
}
fn source_glyph(ch:char)->usize {
    let cp=ch as u32;
    if (32..127).contains(&cp){(cp-32) as usize}
    else {
        let chars="ÁÉÍÓÚÜÑáéíóúüñ¡¿";
        chars.chars().position(|c|c==ch).map(|x|95+x).unwrap_or(31)
    }
}
fn glyph_advance(assets:&Assets,ch:char,size:f32)->f32{
    let idx=source_glyph(ch);
    // Browser-exported measureText at 48px: exact spacing for narrow
    // letters (i/l), accents and wide letters (M/W), no fixed steps.
    assets.glyph_advance(idx).unwrap_or(30.6)*(size/48.0)
}
fn glyph_width(assets:&Assets,text:&str,size:f32)->f32 {
    text.chars().map(|ch|glyph_advance(assets,ch,size)).sum()
}
fn native_text(assets:&Assets,txt:&str,x:f32,center_y:f32,size:f32,col:Color,centered:bool){
    if let Some(atlas)=assets.font_atlas_hi(){
        let scale=size/48.0;
        let width=glyph_width(assets,txt,size);
        let mut left=if centered{x-width/2.0}else{x};
        // Keep each glyph snapped to final CSS pixels. Rasterization happens
        // at 48px in Chromium; the atlas only receives a final downsample.
        let top=(center_y-34.0*scale).round();
        for ch in txt.chars(){
            let idx=source_glyph(ch);
            let shift=3.0*scale;
            draw_texture_ex(atlas,(left-shift).round(),top,col,DrawTextureParams{
                source:Some(Rect::new((idx%16) as f32*64.0,(idx/16) as f32*72.0,
                    64.0,72.0)),
                dest_size:Some(vec2(64.0*scale,72.0*scale)),
                ..Default::default()
            });
            left+=glyph_advance(assets,ch,size);
        }
    }else if let Some(atlas)=assets.font_atlas(){
        // Compatibility for older asset bundles.
        let scale=size/16.0;
        let step=size*10.2/16.0;
        let mut left=if centered{x-step*txt.chars().count() as f32/2.0}else{x};
        for ch in txt.chars(){
            let idx=source_glyph(ch);
            draw_texture_ex(atlas,left.round(),(center_y-10.0*scale).round(),col,
                DrawTextureParams{
                    source:Some(Rect::new((idx%16) as f32*18.0,(idx/16) as f32*20.0,
                        18.0,20.0)),
                    dest_size:Some(vec2(18.0*scale,20.0*scale)),
                    ..Default::default()
                });
            left+=step;
        }
    }else{
        let w=measure_text(txt,None,size as u16,1.0).width;
        draw_text(txt,if centered{x-w/2.0}else{x},center_y+size*0.4,size,col);
    }
}
fn native_outlined(assets:&Assets,text:&str,x:f32,y:f32,size:f32,col:Color){
    // A crisp, restrained original-style black border for names on scenery.
    for (dx,dy) in [(-1.0,0.0),(1.0,0.0),(0.0,-1.0),(0.0,1.0)] {
        native_text(assets,text,x+dx,y+dy,size,rgba(6,9,15,228),true);
    }
    native_text(assets,text,x,y,size,col,true);
}
fn bar_fill(rect:Rect,fraction:f32,red:bool){
    let amount=(rect.w*fraction.clamp(0.0,1.0)-1.0).max(0.0);
    if amount<=0.0{return;}
    let x=rect.x+0.7;let y=rect.y+1.0;let h=(rect.h-2.0).max(1.0);
    draw_rectangle(x,y,amount,h,
        if red{rgb(140,16,24)}else{rgb(16,56,168)});
    draw_rectangle(x,y,amount,h*0.56,
        if red{rgb(212,32,46)}else{rgb(42,106,232)});
    draw_rectangle(x,y,amount,h*0.20,
        if red{rgb(255,122,128)}else{rgb(120,192,255)});
    draw_rectangle(x+3.0,y+1.0,(amount-6.0).max(0.0),1.0,rgba(255,255,255,67));
}
fn draw_scene_lights(w:&World,assets:&Assets,cam_x:f32,cam_y:f32,world_width:f32,vw:f32){
    let Some(glow)=assets.torch_bloom() else{return;};
    for &tx in realm_core::TORCHES[w.map.index()] {
        let sx=(tx-cam_x)/world_width*vw;
        if sx< -112.0 || sx>vw+112.0 {continue;}
        // Torch bloom center is y-30 in original zProps.
        let sy=(w.floor(tx)-30.0-cam_y.round())*3.0;
        if sy< -112.0 || sy>660.0 {continue;}
        let pulse=(w.tick as f32*0.30+tx).sin();
        let flicker=(0.94+0.06*pulse).clamp(0.75,1.0);
        draw_texture_ex(glow,(sx-102.0).round(),(sy-102.0).round(),
            Color::new(1.0,1.0,1.0,flicker),
            DrawTextureParams{dest_size:Some(vec2(204.0,204.0)),..Default::default()});
    }
}
fn draw_world_name(w:&World,assets:&Assets,name:&str,cam_x:f32,cam_y:f32,
    world_width:f32,vw:f32){
    let sx=(w.player.x-cam_x)/world_width*vw;
    let sy=(w.player.y-49.0-cam_y.round())*3.0;
    if sx>0.0 && sx<vw && sy>7.0 && sy<540.0 {
        native_outlined(assets,name,sx.round(),sy.round(),21.0,rgb(245,215,125));
    }
}
fn draw_hud(world:&World,assets:&Assets,name:&str){
    let Some(chrome)=assets.dom_hud(world.player.class) else{return;};
    let pos=assets.dom_hud_layout("frame").unwrap_or(Rect::new(6.0,6.0,chrome.width(),chrome.height()));
    draw_texture_ex(chrome,pos.x,pos.y,WHITE,DrawTextureParams{
        dest_size:Some(vec2(pos.w,pos.h)),..Default::default()
    });
    let p=&world.player;
    if let Some(hp)=assets.dom_hud_layout("hp") {
        bar_fill(hp,p.hp/p.max_hp.max(1.0),true);
        if let Some(text_rect)=assets.dom_hud_layout("hp_text"){
            native_text(assets,&format!("{}/{}",p.hp as i32,p.max_hp as i32),
                text_rect.x+text_rect.w/2.0,text_rect.y+text_rect.h/2.0,
                12.0,WHITE,true);
        }
        if p.hp/p.max_hp.max(1.0)<0.3 && world.tick%30<15 {
            draw_rectangle_lines(hp.x-2.0,hp.y-2.0,hp.w+4.0,hp.h+4.0,
                1.8,rgba(255,70,70,180));
        }
    }
    if let Some(mp)=assets.dom_hud_layout("mp") {
        bar_fill(mp,p.mp/p.max_mp.max(1.0),false);
        if let Some(text_rect)=assets.dom_hud_layout("mp_text"){
            native_text(assets,&format!("{}/{}",p.mp as i32,p.max_mp as i32),
                text_rect.x+text_rect.w/2.0,text_rect.y+text_rect.h/2.0,
                12.0,WHITE,true);
        }
    }
    if let Some(r)=assets.dom_hud_layout("name"){
        native_outlined(assets,name,
            (r.x+r.w/2.0).round(),(r.y+r.h/2.0).round(),
            19.0,rgb(160,244,255));
    }
    if let Some(r)=assets.dom_hud_layout("level"){
        native_text(assets,&p.level.to_string(),
            r.x+r.w/2.0,r.y+r.h/2.0,11.0,rgb(255,232,144),true);
    }
    if let Some(r)=assets.dom_hud_layout("gold"){
        native_text(assets,&p.gold.to_string(),r.x,r.y+r.h/2.0,
            12.0,rgb(255,239,184),false);
    }
    if let Some(r)=assets.dom_hud_layout("gems"){
        native_text(assets,&p.gems.to_string(),r.x,r.y+r.h/2.0,
            12.0,rgb(138,226,255),false);
    }
}
fn place(rect:Rect,id:&str,vw:f32)->Rect{
    // HTML #ctl tiene botones anclados a left / right / center.
    let x=match id {
        "bl"|"br"=>rect.x,
        "ap"=>rect.x+(vw-960.0)*0.5,
        _=>rect.x+vw-960.0,
    };
    Rect::new(x,rect.y,rect.w,rect.h)
}
fn dom_button(assets:&Assets,id:&str,class:HeroClass,vw:f32) {
    let Some(rect)=assets.dom_layout(id) else{return;};
    let Some(sprite)=assets.dom_button(id,class) else{return;};
    let rect=place(rect,id,vw);
    draw_texture_ex(sprite,rect.x,rect.y,WHITE,DrawTextureParams{
        dest_size:Some(vec2(rect.w,rect.h)),..Default::default()
    });
}
fn controls(w:&World,assets:&Assets,vw:f32){
    for id in ["bl","br","s0","s1","s2","s3","bs","bj","bm"] {
        dom_button(assets,id,w.player.class,vw);
    }
    // Original: el botón ap muestra la clase en la parte superior central.
    dom_button(assets,"ap",w.player.class,vw);
    if let (Some(sprite),Some(layout))=(assets.dom_pause(),assets.dom_layout("pause")) {
        let r=place(layout,"pause",vw);
        draw_texture_ex(sprite,r.x,r.y,WHITE,DrawTextureParams{
            dest_size:Some(vec2(r.w,r.h)),..Default::default()
        });
    }
    // Los DOM .sk usan --cd para oscurecer la parte inferior del botón.
    // Hacemos solo una película ligera para mantener visibles los iconos.
    let cds=[
        ("s0",w.player.cooldowns[0],60u16),
        ("s1",w.player.cooldowns[1],120),
        ("s2",w.player.cooldowns[2],240),
        ("s3",w.player.cooldowns[3],60),
        ("bs",w.player.shield_cd,480),
    ];
    for (id,remain,total) in cds {
        if remain==0{continue;}
        if let Some(r)=assets.dom_layout(id) {
            let r=place(r,id,vw);
            let fraction=remain as f32/total as f32;
            let overlay_height=r.h*fraction.clamp(0.0,1.0);
            draw_rectangle(r.x+7.0,r.y+r.h-overlay_height-6.0,
                (r.w-14.0).max(0.0),overlay_height.min(r.h-14.0).max(0.0),
                rgba(4,7,17,145));
        }
    }
}
/// Invocar una sola vez al final del frame, tras componer el mundo de 180px.
pub fn draw_game(w:&World,assets:&Assets,name:&str,
    camera_x:f32,camera_y:f32,world_width:f32){
    if !assets.has_native_hud(){return;}
    push_camera_state();
    let vw=css_width();
    // En el framebuffer principal no se aplica el flip_y del render target.
    // Invertir la cámara nativa mantiene origen (0,0) arriba a la izquierda.
    let mut ui_camera=Camera2D::from_display_rect(Rect::new(0.0,0.0,vw,540.0));
    ui_camera.zoom.y=-ui_camera.zoom.y;
    set_camera(&ui_camera);
    // Radial light and player tag are in this full-resolution layer, then
    // the HUD stays above both (as the original CSS stack does).
    draw_scene_lights(w,assets,camera_x,camera_y,world_width,vw);
    draw_world_name(w,assets,name,camera_x,camera_y,world_width,vw);
    draw_hud(w,assets,name);
    controls(w,assets,vw);
    pop_camera_state();
}
/// La UI de Inicio y Selección procede del DOM real (#hm/#hs), no de
/// figuras aproximadas dibujadas dentro del render target de 180px.
pub fn draw_home(
    assets:&Assets,hero:HeroClass,page:crate::ui::HomePage,
    name:&str,level:u16
){
    if !assets.has_native_menu(){return;}
    push_camera_state();
    let vw=css_width();
    // En el framebuffer principal no se aplica el flip_y del render target.
    // Invertir la cámara nativa mantiene origen (0,0) arriba a la izquierda.
    let mut ui_camera=Camera2D::from_display_rect(Rect::new(0.0,0.0,vw,540.0));
    ui_camera.zoom.y=-ui_camera.zoom.y;
    set_camera(&ui_camera);
    let x=(vw-960.0)*0.5;
    let selected=match page{
        crate::ui::HomePage::Landing=>assets.dom_home(hero),
        crate::ui::HomePage::Selection=>assets.dom_selection(hero),
        crate::ui::HomePage::Credits=>assets.dom_credits().or_else(||assets.dom_home(hero)),
    };
    if let Some(image)=selected {
        draw_texture_ex(image,x,0.0,WHITE,DrawTextureParams{
            dest_size:Some(vec2(960.0,540.0)),..Default::default()
        });
    }
    if page==crate::ui::HomePage::Landing {
        if let Some(r)=assets.dom_menu_rect("name"){
            native_text(assets,name,x+r.x+r.w*0.5,r.y+r.h*0.49,
                20.0,rgb(246,232,184),true);
        }
        if let Some(r)=assets.dom_menu_rect("level"){
            native_text(assets,&format!("NIVEL {}",level),
                x+r.x+r.w*0.5,r.y+r.h*0.52,13.0,rgb(240,208,112),true);
        }
    }
    pop_camera_state();
}
pub fn pause_hit(assets:&Assets,key:&str,point:Vec2,ui_width:f32)->Option<bool>{
    let r=assets.dom_pause_rect(key)?;
    let x=r.x+(ui_width-960.0)*0.5;
    Some(Rect::new(x,r.y,r.w,r.h).contains(point))
}
pub fn menu_hit(assets:&Assets,key:&str,point:Vec2,ui_width:f32)->Option<bool>{
    let r=assets.dom_menu_rect(key)?;
    let x=r.x+(ui_width-960.0)*0.5;
    Some(Rect::new(x,r.y,r.w,r.h).contains(point))
}
/// Pantalla de pausa a resolución CSS. El modal del HTML original mide
/// máximo 420px, tiene doble borde dorado y fondo acero violeta.
pub fn draw_pause(assets:&Assets,audio_enabled:bool){
    push_camera_state();
    let vw=css_width();
    // En el framebuffer principal no se aplica el flip_y del render target.
    // Invertir la cámara nativa mantiene origen (0,0) arriba a la izquierda.
    let mut ui_camera=Camera2D::from_display_rect(Rect::new(0.0,0.0,vw,540.0));
    ui_camera.zoom.y=-ui_camera.zoom.y;
    set_camera(&ui_camera);
    draw_rectangle(0.0,0.0,vw,540.0,rgba(0,0,0,204));
    // Panel exacto del HTML (Gradientes, serif, bisel y controles CSS).
    if let (Some(panel),Some(frame))=(assets.dom_pause_panel(),assets.dom_pause_rect("frame")) {
        draw_texture_ex(panel,frame.x+(vw-960.0)*0.5,frame.y,WHITE,
            DrawTextureParams{
                dest_size:Some(vec2(frame.w,frame.h)),
                ..Default::default()
            });
        pop_camera_state();
        return;
    }
    let x=vw*0.5-210.0;
    let y=60.0;
    let w=420.0;
    let h=425.0;
    draw_rectangle(x-2.0,y-2.0,w+4.0,h+4.0,rgb(12,10,16));
    draw_rectangle(x,y,w,h,rgb(216,170,60));
    draw_rectangle(x+4.0,y+4.0,w-8.0,h-8.0,rgb(38,36,44));
    draw_rectangle(x+6.0,y+6.0,w-12.0,87.0,rgb(74,70,80));
    draw_rectangle(x+6.0,y+90.0,w-12.0,h-96.0,rgb(39,37,46));
    native_text(assets,"PAUSA",vw*0.5,y+35.0,22.0,rgb(245,208,112),true);
    native_text(assets,"Música",x+27.0,y+82.0,13.0,rgb(246,231,191),false);
    native_text(assets,"Sonidos",x+27.0,y+132.0,13.0,rgb(246,231,191),false);
    for (row,val) in [(0,if audio_enabled{0.7}else{0.0}),
        (1,if audio_enabled{0.8}else{0.0})] {
        let ry=y+77.0+row as f32*50.0;
        draw_rectangle(x+182.0,ry-4.0,198.0,16.0,rgb(13,16,29));
        draw_rectangle(x+187.0,ry+1.0,188.0,6.0,rgb(102,102,120));
        draw_rectangle(x+187.0,ry+1.0,188.0*val,6.0,rgb(238,203,102));
        draw_rectangle(x+183.0+188.0*val,ry-3.0,12.0,14.0,rgb(233,213,166));
    }
    native_text(assets,"Estilo de música",x+27.0,y+180.0,12.0,rgb(238,227,196),false);
    native_text(assets,"Épica    Folk    Nocturna",
        vw*0.5+62.0,y+180.0,10.4,rgb(246,216,140),true);
    native_text(assets,"Controles",x+27.0,y+228.0,12.0,rgb(238,227,196),false);
    native_text(assets,"Botones    Joystick",
        vw*0.5+75.0,y+228.0,11.0,rgb(246,216,140),true);
    for (i,label) in ["Mover controles","Restablecer"].iter().enumerate() {
        let bx=x+12.0+i as f32*204.0;
        draw_rectangle(bx,y+259.0,194.0,35.0,rgb(52,48,60));
        draw_rectangle_lines(bx,y+259.0,194.0,35.0,2.0,rgb(139,133,152));
        native_text(assets,label,bx+97.0,y+275.0,11.0,rgb(244,225,184),true);
    }
    draw_rectangle(x+13.0,y+309.0,w-26.0,31.0,rgb(52,48,60));
    draw_rectangle_lines(x+13.0,y+309.0,w-26.0,31.0,2.0,rgb(139,133,152));
    native_text(assets,"Cambiar personaje",vw*0.5,y+325.0,12.0,rgb(244,225,184),true);
    for (i,label) in ["Menú principal","Continuar"].iter().enumerate(){
        let bx=x+13.0+i as f32*199.0;
        draw_rectangle(bx,y+355.0,193.0,39.0,
            if i==1 {rgb(87,70,40)}else{rgb(52,48,60)});
        draw_rectangle_lines(bx,y+355.0,193.0,39.0,2.0,
            if i==1 {rgb(240,205,91)}else{rgb(139,133,152)});
        native_text(assets,label,bx+96.5,y+376.0,12.0,rgb(246,230,192),true);
    }
    pop_camera_state();
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn control_anchor_not_stretched(){
        let r=Rect::new(860.0,440.0,90.0,90.0);
        assert_eq!(place(r,"bm",960.0).x,860.0);
        assert_eq!(place(r,"bm",1200.0).x,1100.0);
        assert_eq!(place(Rect::new(10.0,454.0,74.0,74.0),"bl",1200.0).x,10.0);
    }
    #[test]
    fn glyph_map_has_spanish_chars(){
        assert_eq!(source_glyph('A'),33);
        assert!(source_glyph('ñ')>=95);
    }
}