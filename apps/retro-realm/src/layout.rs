//! Shared screen / UI geometry. Screen dimensions from Macroquad are already
//! DPI-normalized: never multiply touches by a magic 3x factor.
use macroquad::prelude::*;
use std::collections::HashMap;

pub const UI_HEIGHT:f32=540.0;
pub const SOURCE_WIDTH:f32=960.0;
pub const SOURCE_HEIGHT:f32=540.0;

#[derive(Clone,Debug)]
pub struct ControlPrefs {
    pub global_scale:f32,
    pub individual:HashMap<String,f32>,
    /// Offsets in UI units relative to the respective screen edge.
    pub offsets:HashMap<String,Vec2>,
    pub joystick:bool,
}
impl Default for ControlPrefs {
    fn default()->Self{
        Self{global_scale:1.35,individual:HashMap::new(),
            offsets:HashMap::new(),joystick:false}
    }
}
impl ControlPrefs {
    pub fn scale(&self,id:&str)->f32{
        let f=self.individual.get(id).copied().unwrap_or(1.0);
        (self.global_scale.clamp(0.75,1.9)*f.clamp(0.4,2.2)).clamp(0.55,3.0)
    }
}
#[derive(Clone,Copy,Debug)]
pub struct UiLayout {
    pub width:f32,
    pub height:f32,
    pub screen:Vec2,
}
impl UiLayout {
    pub fn new(screen_w:f32,screen_h:f32)->Self{
        let w=screen_w.max(1.0);
        let h=screen_h.max(1.0);
        Self{width:UI_HEIGHT*w/h,height:UI_HEIGHT,screen:vec2(w,h)}
    }
    pub fn current()->Self {Self::new(screen_width(),screen_height())}
    pub fn world_width(&self)->f32 {self.width/3.0}
    pub fn from_screen(&self,p:Vec2)->Vec2 {
        vec2(p.x*self.width/self.screen.x,p.y*self.height/self.screen.y)
    }
    pub fn camera(&self)->Camera2D{
        // Onscreen framebuffer uses upper-left origin, unlike a render target.
        let mut cam=Camera2D::from_display_rect(Rect::new(0.0,0.0,self.width,self.height));
        cam.zoom.y=-cam.zoom.y;
        cam
    }
    /// 960x540 source DOM full-screen screenshot: keep its aspect *and*
    /// center within narrow or wide screens without cropping.
    pub fn source_scale(&self)->f32 {
        (self.width/SOURCE_WIDTH).min(1.0).min(self.height/SOURCE_HEIGHT)
    }
    pub fn source_rect(&self,r:Rect)->Rect {
        let s=self.source_scale();
        let dx=(self.width-SOURCE_WIDTH*s)*0.5;
        let dy=(self.height-SOURCE_HEIGHT*s)*0.5;
        Rect::new(dx+r.x*s,dy+r.y*s,r.w*s,r.h*s)
    }
    pub fn control_rect(&self,id:&str,r:Rect,prefs:&ControlPrefs)->Rect {
        let factor=prefs.scale(id);
        let size=vec2(r.w*factor,r.h*factor);
        let x=match id {
            "bl"|"br"=>12.0+(r.x-10.0)*factor,
            "ap"=>(self.width-size.x)*0.5,
            "pause"=>self.width-12.0-size.x,
            _=>self.width-12.0-(SOURCE_WIDTH-r.x)*factor,
        };
        let y=if id=="pause"||id=="ap"{8.0}
            else {self.height-12.0-(SOURCE_HEIGHT-r.y)*factor};
        let offset=prefs.offsets.get(id).copied().unwrap_or(Vec2::ZERO);
        let mut out=Rect::new(x+offset.x,y+offset.y,size.x,size.y);
        // Drag editing must never strand a control outside the visible area.
        out.x=out.x.clamp(0.0,(self.width-out.w).max(0.0));
        out.y=out.y.clamp(0.0,(self.height-out.h).max(0.0));
        out
    }
    pub fn hit_rect(&self,r:Rect)->Rect {
        // 48 logical dp is the recommended minimum hit target, without
        // changing visual size; Macroquad's sizes are logical pixels.
        let min_units=48.0*UI_HEIGHT/self.screen.y;
        let w=r.w.max(min_units);
        let h=r.h.max(min_units);
        Rect::new(r.x+(r.w-w)*0.5,r.y+(r.h-h)*0.5,w,h)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn world_stays_undistorted_for_common_aspects() {
        for (w,h) in [(960.0,540.0),(1024.0,768.0),(2400.0,1080.0)]{
            let l=UiLayout::new(w,h);
            assert!((l.width/l.height-w/h).abs()<0.001);
            assert!((l.world_width()/180.0-w/h).abs()<0.001);
        }
    }
    #[test] fn menu_fits_tablet_and_ultrawide() {
        for (w,h) in [(1024.0,768.0),(1920.0,1080.0),(2400.0,1080.0)]{
            let l=UiLayout::new(w,h);
            let r=l.source_rect(Rect::new(0.0,0.0,960.0,540.0));
            assert!(r.x>=0.0&&r.y>=0.0);
            assert!(r.x+r.w<=l.width+0.01);
            assert!(r.y+r.h<=l.height+0.01);
        }
    }
    #[test] fn controls_and_hit_rects_share_geometry() {
        let prefs=ControlPrefs::default();
        for (w,h) in [(1024.0,768.0),(1920.0,1080.0),(2400.0,1080.0)]{
            let l=UiLayout::new(w,h);
            for (id,r) in [("bl",Rect::new(10.0,454.0,74.0,74.0)),
                ("bm",Rect::new(860.0,440.0,90.0,90.0)),
                ("pause",Rect::new(904.0,2.0,54.0,54.0))]{
                let vr=l.control_rect(id,r,&prefs);
                let hit=l.hit_rect(vr);
                assert!(hit.contains(vec2(vr.x+vr.w*0.5,vr.y+vr.h*0.5)));
                assert!(vr.x>=0.0&&vr.x+vr.w<=l.width+0.01);
            }
        }
    }
}