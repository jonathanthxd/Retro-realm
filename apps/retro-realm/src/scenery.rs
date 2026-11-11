//! Escenarios del HTML original: zSky, zRidge, zForest, zCloud,
//! capas FL/FL2, estrellas, rayos de luz, niebla y luciérnagas.
//! Los PNG son exportados por scripts/export-sprites.mjs desde Canvas.
use crate::assets::Assets;
use macroquad::prelude::*;
use realm_core::{MapId,World,WORLD_WIDTH};

fn rgb(r:u8,g:u8,b:u8)->Color{Color::from_rgba(r,g,b,255)}
fn rgba(r:u8,g:u8,b:u8,a:u8)->Color{Color::from_rgba(r,g,b,a)}
fn tinted(alpha:f32)->Color{Color::new(1.0,1.0,1.0,alpha.clamp(0.0,1.0))}
fn tiled(img:&Texture2D,vw:f32,offset:f32,alpha:f32) {
    if alpha<=0.001 || img.width()<1.0 {return;}
    let width=img.width();
    let mut x=-(offset.rem_euclid(width));
    while x<vw {
        draw_texture(img,x.floor(),0.0,tinted(alpha));
        x+=width;
    }
}
fn original_layer(assets:&Assets,vw:f32,cam:f32,key:&str,speed:f32,fade:f32){
    let base=format!("scene_{}_0",key);
    let alt=format!("scene_{}_1",key);
    if let Some(img)=assets.scene(&base){tiled(img,vw,cam*speed,1.0);}
    if let Some(img)=assets.scene(&alt){tiled(img,vw,cam*speed,fade);}
}
fn mist(vw:f32,dark:bool,alpha:f32){
    let (r,g,b)=if dark{(70,92,150)}else{(130,160,220)};
    for k in 0..25{
        let y=50.0+k as f32*4.0;
        let a=(alpha*(k as f32/25.0)*255.0).round() as u8;
        draw_rectangle(0.0,y,vw,4.0,rgba(r,g,b,a));
    }
}
fn layers_forest(assets:&Assets,vw:f32,cam:f32,map:usize,dark:bool,menu:bool,t:f32){
    if menu{return;}
    for (i,speed) in [0.22,0.5,0.85].iter().enumerate(){
        mist(vw,dark,if dark{[0.2,0.16,0.12][i]}else{[0.22,0.2,0.16][i]});
        if let Some(img)=assets.forest_layer(map,i){
            tiled(img,vw,cam*speed,1.0);
        }
        if i==1 && !dark{
            // Haces de luz azules del Bosque Nocturno.
            for j in 0..3 {
                let x=40.0+j as f32*120.0+(t*0.008+j as f32*2.0).sin()*12.0-(cam*0.1).rem_euclid(40.0);
                draw_triangle(vec2(x,0.0),vec2(x+22.0,0.0),
                    vec2(x-34.0,150.0),rgba(215,230,255,18));
                draw_triangle(vec2(x,0.0),vec2(x-34.0,150.0),
                    vec2(x-66.0,150.0),rgba(215,230,255,18));
            }
        }
    }
    // Bandas de bruma horizontales originales: 4 alturas, distinta intensidad.
    for i in 0..4{
        let y=118.0+i as f32*9.0;
        let span=vw+40.0;
        let mut x=-20.0;
        while x<vw+20.0 {
            let f=(0.05+(if dark{0.035}else{0.04})*(t*0.012+x*0.02+i as f32*2.0).sin()).max(0.0);
            let xx=(x-t*(0.18+i as f32*0.06)-cam*(0.3+i as f32*0.05))
                .rem_euclid(span)-20.0;
            draw_rectangle(xx,y,28.0,5.0,if dark{
                rgba(80,105,165,(f*255.0) as u8)
            }else{rgba(140,170,225,(f*255.0) as u8)});
            x+=18.0;
        }
    }
}
fn original_sky(assets:&Assets,vw:f32,fade:f32)->bool{
    let Some(base)=assets.scene("scene_sky_0") else {return false;};
    draw_texture_ex(base,0.0,0.0,WHITE,DrawTextureParams{
        dest_size:Some(vec2(vw,180.0)),..Default::default()
    });
    if let Some(alt)=assets.scene("scene_sky_1"){
        draw_texture_ex(alt,0.0,0.0,tinted(fade),DrawTextureParams{
            dest_size:Some(vec2(vw,180.0)),..Default::default()
        });
    }
    true
}
fn night(assets:&Assets,vw:f32,cam:f32,menu:bool,t:f32) {
    let progress=(cam/(WORLD_WIDTH-vw).max(1.0)).clamp(0.0,1.0);
    let fade=((progress-0.3)/0.35).clamp(0.0,1.0);
    if !original_sky(assets,vw,fade){
        for i in 0..45 {
            let u=i as f32/45.0;
            draw_rectangle(0.0,i as f32*4.0,vw,4.0,
                rgb((8.0+27.0*u) as u8,(17.0+36.0*u) as u8,(48.0+68.0*u) as u8));
        }
    }
    for i in 0..70 {
        let hash=(i*7919+127) as f32;
        let x=(hash*0.31).rem_euclid(vw);
        let y=(hash*0.12).rem_euclid(105.0);
        let a=((0.3+0.7*(t*0.03+i as f32).sin().abs())*(1.0-fade*0.55)*255.0) as u8;
        draw_rectangle(x,y,if i%7==0{2.0}else{1.0},1.0,rgba(255,255,235,a));
    }
    if (t as u64)%1100<28 {
        let u=(t%1100.0)/28.0;
        for i in 0..9 {
            draw_rectangle(40.0+u*120.0-i as f32*3.0,
                14.0+u*40.0-i as f32*1.2,2.0,1.0,rgba(255,255,255,((1.0-u)*255.0) as u8));
        }
    }
    let moon_x=vw-76.0;
    for i in 0..6{
        let r=(20.0+i as f32*9.0)*(1.0+0.07*(t*0.045).sin());
        draw_circle(moon_x,32.0,r,rgba(215,236,255,10));
    }
    if let Some(moon)=assets.scene("scene_moon"){
        draw_texture(moon,moon_x-18.0,14.0,WHITE);
    }else{
        draw_circle(moon_x,32.0,16.0,rgb(240,246,185));
    }
    // Orden de pintura original zBgx: nube, montes, bosques.
    for i in 0..3{draw_cloud(assets,i,vw,cam,t,fade);}
    original_layer(assets,vw,cam,"mf",0.03,fade);
    for i in 3..6{draw_cloud(assets,i,vw,cam,t,fade);}
    original_layer(assets,vw,cam,"mn",0.07,fade);
    original_layer(assets,vw,cam,"ff",0.16,fade);
    for i in 6..9{draw_cloud(assets,i,vw,cam,t,fade);}
    original_layer(assets,vw,cam,"fm",0.32,fade);
    for i in 0..4{
        let y=112.0+i as f32*9.0;
        for j in 0..((vw as i32/16)+3){
            let x=j as f32*16.0-16.0;
            let a=0.05+0.045*(t*0.012+x*0.02+i as f32*2.0).sin();
            draw_rectangle((x-t*(0.2+i as f32*0.07)-cam*(0.2+i as f32*0.05))
                .rem_euclid(vw+32.0)-16.0,y,26.0,5.0,
                rgba(140,210,230,(a.max(0.0)*255.0) as u8));
        }
    }
    for i in 0..14{
        let x=(i as f32*53.0+(t*0.02+i as f32).sin()*12.0-cam*0.5).rem_euclid(vw);
        let y=100.0+(i*37%50) as f32+(t*0.03+i as f32*2.0).sin()*6.0;
        let a=0.5+0.5*(t*0.06+i as f32*1.7).sin();
        draw_rectangle(x-2.0,y-2.0,5.0,5.0,rgba(216,255,128,(30.0*a) as u8));
        draw_rectangle(x,y,1.0,1.0,rgba(235,255,170,(a*255.0) as u8));
    }
    layers_forest(assets,vw,cam,0,false,menu,t);
}
fn draw_cloud(assets:&Assets,i:usize,vw:f32,cam:f32,t:f32,fade:f32){
    let cloud=i%4;
    let x=((i as f32*93.0+31.0-t*(0.08+i as f32*0.025)
        -cam*(0.03+(i%3) as f32*0.04)).rem_euclid(vw+240.0))-120.0;
    let y=8.0+(i*19%68) as f32;
    let a=0.55+(i%3) as f32*0.15;
    for scene in 0..2 {
        let opacity=if scene==0{a}else{a*fade};
        if opacity<0.01{continue;}
        if let Some(img)=assets.scene(&format!("scene_cloud_{}_{}",scene,cloud)){
            draw_texture(img,x,y,tinted(opacity));
        }
    }
}
fn dark(assets:&Assets,vw:f32,cam:f32,menu:bool,t:f32){
    for i in 0..45 {
        let u=i as f32/45.0;
        let (r,g,b)=if u<0.6 {
            (7.0+11.0*u/0.6,11.0+19.0*u/0.6,24.0+36.0*u/0.6)
        } else {
            let q=(u-0.6)/0.4;
            (18.0-8.0*q,30.0-12.0*q,60.0-22.0*q)
        };
        draw_rectangle(0.0,i as f32*4.0,vw,4.0,rgb(r as u8,g as u8,b as u8));
    }
    for i in 0..7 {
        let x=(i as f32*80.0-t*(0.1+i as f32*0.02)-cam*0.05).rem_euclid(vw+160.0)-80.0;
        let y=6.0+(i%3) as f32*14.0;
        for (dx,dy,r) in [(0.0,8.0,16.0),(16.0,4.0,20.0),(36.0,9.0,15.0)] {
            draw_circle(x+dx,y+dy,r,rgba(16,24,46,216));
        }
    }
    layers_forest(assets,vw,cam,1,true,menu,t);
}
pub fn draw(w:&World,vw:f32,cam:f32,assets:&Assets,menu:bool) {
    let t=if menu{get_time() as f32*60.0}else{w.tick as f32};
    if w.map==MapId::DarkForest&&!menu{dark(assets,vw,cam,menu,t);}
    else{night(assets,vw,cam,menu,t);}
}