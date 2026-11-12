//! Partículas, números de daño, ondas y rayos del Canvas original.
 //! Visuales solamente: los efectos NO alteran combate, RNG ni físicas de realm-core.
use macroquad::prelude::*;
use realm_core::{HeroClass,Input,MapId,World};

const MAX_PARTICLES:usize=240;
const MAX_NUMBERS:usize=32;
const MAX_RINGS:usize=18;
const MAX_BOLTS:usize=8;

fn rgba(r:u8,g:u8,b:u8,a:u8)->Color {Color::from_rgba(r,g,b,a)}
fn rgb(r:u8,g:u8,b:u8)->Color {Color::from_rgba(r,g,b,255)}

#[derive(Clone)]
struct Particle {
    pos:Vec2, velocity:Vec2, color:Color, size:f32,
    life:u16,max_life:u16,gravity:f32,
}
#[derive(Clone)]
struct Number {
    pos:Vec2, label:String, color:Color, life:u16,
}
#[derive(Clone)]
struct Ring {
    pos:Vec2, radius:f32, color:Color, life:u16, duration:u16,
}
#[derive(Clone)]
struct Bolt {
    from:Vec2,to:Vec2,color:Color,life:u16,seed:u32,
}
#[derive(Clone,Copy)]
struct EnemyMark {
    x:f32,y:f32,hp:f32,kind:u8,generation:u8,
    max_hp:f32,scale:f32,boss:bool,
}
pub struct FrameSnapshot {
    map:MapId,
    player:Vec2,
    hp:f32,level:u16,gold:u32,gems:u32,
    cooldowns:[u16;4],shield:u16,dash:u16,
    enemies:Vec<EnemyMark>,
}
impl FrameSnapshot {
    pub fn capture(world:&World)->Self {
        Self{
            map:world.map,
            player:vec2(world.player.x,world.player.y),
            hp:world.player.hp,level:world.player.level,
            gold:world.player.gold,gems:world.player.gems,
            cooldowns:world.player.cooldowns,
            shield:world.player.shield_ticks,
            dash:world.player.dash_ticks,
            enemies:world.enemies.iter().map(|e|EnemyMark{
                x:e.x,y:e.y,hp:e.hp,kind:e.kind,generation:e.generation,
                max_hp:e.max_hp,scale:e.scale,boss:e.boss,
            }).collect(),
        }
    }
}
pub struct VisualFx {
    particles:Vec<Particle>,
    numbers:Vec<Number>,
    rings:Vec<Ring>,
    bolts:Vec<Bolt>,
    seed:u32,
    pub cast_ticks:u8,
}
impl Default for VisualFx {
    fn default()->Self {
        Self{particles:Vec::new(),numbers:Vec::new(),
            rings:Vec::new(),bolts:Vec::new(),seed:0x5a26_9c39,cast_ticks:0}
    }
}
impl VisualFx {
    pub fn clear(&mut self){
        self.particles.clear();self.numbers.clear();
        self.rings.clear();self.bolts.clear();self.cast_ticks=0;
    }
    fn rnd(&mut self)->f32 {
        // RNG propio para que los efectos no perturben la simulación del juego.
        self.seed ^=self.seed<<13;
        self.seed ^=self.seed>>17;
        self.seed ^=self.seed<<5;
        (self.seed & 0x00ff_ffff) as f32/16_777_216.0
    }
    fn spark(&mut self,pos:Vec2,color:Color,count:usize,force:f32) {
        for _ in 0..count.min(24) {
            if self.particles.len()>=MAX_PARTICLES{self.particles.remove(0);}
            let vx=(self.rnd()-0.5)*3.0*force;
            let vy=-self.rnd()*2.5*force;
            let size=if self.rnd()<0.25 {1.0}else{2.0};
            self.particles.push(Particle{pos,velocity:vec2(vx,vy),
                color,size,life:20,max_life:20,gravity:0.12});
        }
    }
    fn number(&mut self,pos:Vec2,label:String,color:Color){
        if self.numbers.len()>=MAX_NUMBERS{self.numbers.remove(0);}
        self.numbers.push(Number{pos,label,color,life:45});
    }
    fn ring(&mut self,pos:Vec2,radius:f32,color:Color,life:u16) {
        if self.rings.len()>=MAX_RINGS{self.rings.remove(0);}
        self.rings.push(Ring{pos,radius,color,life,duration:life});
    }
    fn bolt(&mut self,to:Vec2,color:Color){
        if self.bolts.len()>=MAX_BOLTS{self.bolts.remove(0);}
        let seed=self.seed;
        self.rnd();
        self.bolts.push(Bolt{from:vec2(to.x+8.0,-50.0),to,color,life:10,seed});
    }
    fn advance(&mut self) {
        self.cast_ticks=self.cast_ticks.saturating_sub(1);
        for p in &mut self.particles {
            p.pos+=p.velocity;
            p.velocity.y+=p.gravity;
            p.life=p.life.saturating_sub(1);
        }
        self.particles.retain(|p|p.life>0);
        for n in &mut self.numbers {n.pos.y-=0.4;n.life=n.life.saturating_sub(1);}
        self.numbers.retain(|n|n.life>0);
        for r in &mut self.rings {r.life=r.life.saturating_sub(1);}
        self.rings.retain(|r|r.life>0);
        for b in &mut self.bolts {b.life=b.life.saturating_sub(1);}
        self.bolts.retain(|b|b.life>0);
    }
    /// Comparar estados pre/post de un solo tick 60 Hz. Siempre después de World::tick.
    pub fn observe(&mut self,before:&FrameSnapshot,world:&World,input:Input) {
        if before.map!=world.map{self.clear();return;}
        self.advance();
        let p=&world.player;
        let pos=vec2(p.x,p.y);
        if before.hp>p.hp {
            self.spark(pos+vec2(0.0,-10.0),rgb(255,80,80),6,1.0);
            self.number(pos+vec2(0.0,-30.0),format!("{}",(before.hp-p.hp).round() as i32),
                rgb(255,90,90));
        }
        if p.level>before.level {
            self.spark(pos+vec2(0.0,-10.0),rgb(255,218,75),20,1.6);
            self.ring(pos+vec2(0.0,-15.0),32.0,rgb(255,218,75),18);
            self.number(pos+vec2(0.0,-36.0),format!("NIVEL {}!",p.level),
                rgb(255,218,75));
        } else if p.gold>before.gold && p.gold-before.gold < 300 {
            self.number(pos+vec2(-5.0,-32.0),format!("+{}",p.gold-before.gold),
                rgb(248,215,93));
        } else if p.gems>before.gems {
            self.number(pos+vec2(4.0,-31.0),format!("+{}",p.gems-before.gems),
                rgb(88,204,255));
        }
        if input.jump && p.y<before.player.y && p.vy<0.0 {
            self.spark(pos+vec2(0.0,-2.0),rgb(200,221,246),3,0.65);
        }
        if before.dash==0 && p.dash_ticks>0 {
            self.spark(pos+vec2(-p.facing*5.0,-9.0),rgb(255,174,93),12,1.0);
        }
        if before.shield==0 && p.shield_ticks>0 {
            self.ring(pos+vec2(0.0,-17.0),22.0,rgb(135,224,255),15);
        }
        for index in 0..3 {
            if input.skills[index] && before.cooldowns[index]==0
                && p.cooldowns[index]>0
            {
                self.cast_ticks=10;
                let power=pos+vec2(p.facing*9.0,-20.0);
                match index {
                    0=>self.spark(power,
                        if p.class==HeroClass::CyberRaccoon {
                            rgb(39,224,232)
                        }else if p.class==HeroClass::Huntress{
                            rgb(225,238,249)
                        }else{rgb(192,123,255)},6,0.8),
                    1=>{
                        let target=world.enemies.iter()
                            .filter(|e|(e.x-p.x)*p.facing>-20.0
                                &&(e.x-p.x).abs()<180.0)
                            .min_by(|a,b|(a.x-p.x).abs()
                                .total_cmp(&(b.x-p.x).abs()));
                        let dest=target.map(|e|vec2(e.x,e.y-15.0))
                            .unwrap_or(pos+vec2(p.facing*60.0,-36.0));
                        self.bolt(dest,rgb(162,223,255));
                        self.spark(dest,rgb(170,210,255),6,1.0);
                    },
                    2=>{
                        self.ring(pos+vec2(0.0,-10.0),
                            if p.class==HeroClass::CyberRaccoon{75.0}else{70.0},
                            rgb(154,181,255),16);
                        self.spark(pos+vec2(0.0,-10.0),
                            rgb(180,160,255),14,1.5);
                    },
                    _=>{}
                }
            }
        }
        // Emparejamiento por tipo/generación/proximidad; soporta el retain
        // de enemigos derrotados sin introducir IDs nuevos en realm-core.
        let mut used=vec![false;world.enemies.len()];
        for prev in &before.enemies {
            let current=world.enemies.iter().enumerate()
                .filter(|(i,e)|!used[*i] && e.kind==prev.kind
                    &&e.generation==prev.generation
                    &&(e.x-prev.x).abs()<22.0
                    &&(e.y-prev.y).abs()<30.0)
                .min_by(|(_,a),(_,b)|{
                    let da=(a.x-prev.x).abs()+(a.y-prev.y).abs()*0.2;
                    let db=(b.x-prev.x).abs()+(b.y-prev.y).abs()*0.2;
                    da.total_cmp(&db)
                });
            if let Some((index,e))=current {
                used[index]=true;
                if prev.hp>e.hp {
                    let damage=(prev.hp-e.hp).round() as i32;
                    let hit=vec2(e.x,e.y-10.0*e.scale);
                    self.spark(hit,rgb(217,168,255),4,1.0);
                    self.number(vec2(e.x,e.y-26.0*e.scale),
                        damage.to_string(),if damage as f32 > 50.0 {
                            rgb(255,155,29)
                        }else{rgb(255,224,99)});
                }
            } else if prev.hp>0.0 {
                let death=vec2(prev.x,prev.y-12.0*prev.scale);
                let color=if prev.boss{rgb(255,235,179)}
                    else if prev.kind>=20&&prev.kind<40{rgb(126,226,149)}
                    else{rgb(230,210,243)};
                self.spark(death,color,if prev.boss{20}else{12},1.7);
                if !prev.boss&&prev.max_hp>60.0{
                    self.ring(death,22.0,color,12);
                }
            }
        }
    }
    /// Pases visuales que deben estar dentro de la cámara del mundo.
    pub fn draw_world(&self,cam:f32,assets:&crate::assets::Assets,vw:f32){
        for ring in &self.rings {
            let x=ring.pos.x-cam;
            if x< -ring.radius || x>vw+ring.radius{continue;}
            let progress=1.0-ring.life as f32/ring.duration.max(1) as f32;
            let radius=(ring.radius*progress).max(0.1);
            let alpha=(1.0-progress).clamp(0.0,1.0);
            let col=Color::new(ring.color.r,ring.color.g,ring.color.b,alpha);
            draw_circle_lines(x,ring.pos.y,radius,2.0,col);
        }
        for beam in &self.bolts {
            let x=beam.to.x-cam;
            if x< -20.0 || x>vw+20.0 {continue;}
            let a=(beam.life as f32/10.0).clamp(0.0,1.0);
            let mut last=vec2(beam.from.x-cam,-40.0);
            for step in 1..=14 {
                let u=step as f32/14.0;
                let wobble=((beam.seed as f32*.000_01+step as f32*7.0).sin()*7.0)
                    *u*(1.0-u);
                let p=vec2(x+8.0*(1.0-u)+wobble,
                    -40.0+(beam.to.y+40.0)*u);
                draw_line(last.x,last.y,p.x,p.y,2.0,
                    Color::new(beam.color.r,beam.color.g,beam.color.b,a));
                draw_line(last.x,last.y,p.x,p.y,0.8,
                    Color::new(1.0,1.0,1.0,a));
                last=p;
            }
        }
        for p in &self.particles {
            let x=p.pos.x-cam;
            if x< -8.0 || x>vw+8.0{continue;}
            let a=(p.life as f32/p.max_life.max(1) as f32).clamp(0.0,1.0);
            draw_rectangle(x.round(),p.pos.y.round(),p.size,p.size,
                Color::new(p.color.r,p.color.g,p.color.b,p.color.a*a));
        }
        for n in &self.numbers {
            let x=n.pos.x-cam;
            if x< -80.0 || x>vw+80.0{continue;}
            crate::ui::draw_world_text(assets,&n.label,x,n.pos.y,8.0,n.color);
        }
    }
    pub fn particle_count(&self)->usize {self.particles.len()}
    pub fn number_count(&self)->usize {self.numbers.len()}
}

// Superposiciones del HTML zAtkFx / zKingFx sin alterar decisiones de IA.
pub fn enemy_attack_fx(e:&realm_core::Enemy,x:f32,tick:u64){
    let sc=e.scale;
    if e.boss{
        let pulse=(tick as f32*0.2).sin()*3.0;
        draw_circle(x,e.y-26.0*sc,38.0*sc+pulse,
            rgba(255,40,70,35));
    }
    if e.attack==0 && e.king_pose!="a" && e.king_pose!="s1"
        && e.king_pose!="s2" && e.king_pose!="s3" && e.king_pose!="d" {
        return;
    }
    let phase=(1.0-e.attack as f32/if e.boss{36.0}else{30.0})
        .clamp(0.0,1.0);
    if e.attack>0 && phase<0.4&&!e.boss {
        let alpha=(55.0+38.0*(tick as f32*.8).sin()) as u8;
        draw_circle(x,e.y-12.0*sc,10.0*sc,
            rgba(255,60,60,alpha));
        draw_text("!",x-2.0,e.y-38.0*sc,12.0,rgb(255,105,105));
    }
    let caster=matches!(e.kind,43|44|47);
    if (e.attack>0 && phase>0.35 && phase<0.82&&!caster)
        ||matches!(e.king_pose,"s1"|"s2"|"s3") {
        let progress=if e.kind==62 {
            (e.king_ticks as f32/24.0).clamp(0.0,1.0)
        }else{((phase-0.35)/0.47).clamp(0.0,1.0)};
        let radius=if e.boss{36.0*sc}else{24.0*sc};
        let center=vec2(x+e.facing*5.0*sc,e.y-18.0*sc);
        let start=-1.5;
        let end=start+progress*2.7;
        let alpha=((1.0-progress*progress)*255.0).clamp(0.0,255.0) as u8;
        let color=if e.kind==62 && e.king_pose=="s3" {
            rgba(255,120,105,alpha)
        }else{rgba(223,233,223,alpha)};
        let mut previous=vec2(center.x+e.facing*radius*start.cos(),
            center.y+radius*start.sin());
        for i in 1..=18 {
            let ang=start+(end-start)*i as f32/18.0;
            let next=vec2(center.x+e.facing*radius*ang.cos(),
                center.y+radius*ang.sin());
            draw_line(previous.x,previous.y,next.x,next.y,2.5,color);
            previous=next;
        }
    }
    if caster && e.attack>0 && phase<0.9 {
        let pos=vec2(x+e.facing*11.0*sc,e.y-25.0*sc);
        let radius=(3.0+phase*7.0)*sc;
        let col=if e.kind==47 {rgba(255,110,110,170)}
            else if e.kind==44 {rgba(110,255,160,170)}
            else{rgba(245,220,161,170)};
        draw_circle(pos.x,pos.y,radius,col);
        draw_circle(pos.x,pos.y,2.0,rgb(255,255,236));
        draw_ellipse_lines(x,e.y-1.0,6.0+phase*12.0,2.0+phase*3.0,
            1.2,rgba(210,192,255,160));
    }
    if e.king_pose=="a"{
        let x=x-e.facing*10.0*sc;
        let y=e.y-60.0*sc;
        draw_circle(x,y,13.0,rgba(255,60,60,105));
        draw_circle(x,y,4.0,rgb(255,168,126));
    }
    if e.king_pose=="d"{
        for i in 1..5{
            draw_rectangle(x-e.facing*(14.0+i as f32*9.0)*sc,
                e.y-(18.0+i as f32*5.0)*sc,10.0*sc,2.0,
                rgba(255,106,90,105));
        }
    }
}
pub fn hero_attack_fx(x:f32,y:f32,dir:f32,melee_ticks:u16,class:HeroClass) {
    if !(5..=11).contains(&melee_ticks){return;}
    let u=(11-melee_ticks) as f32/6.0;
    let start=-1.1;
    let radius=if class==HeroClass::CyberRaccoon{25.0}else{30.0};
    let color=if class==HeroClass::CyberRaccoon{rgb(127,252,255)}
        else{rgb(234,240,255)};
    let center=vec2(x+dir*4.0,y-12.0);
    let mut prev=vec2(center.x+start.cos()*radius*dir,
        center.y+start.sin()*radius);
    for n in 1..=16 {
        let a=start+2.2*u*n as f32/16.0;
        let next=vec2(center.x+a.cos()*radius*dir,center.y+a.sin()*radius);
        draw_line(prev.x,prev.y,next.x,next.y,3.0,
            Color::new(color.r,color.g,color.b,(1.0-u*.65).max(0.15)));
        prev=next;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn muted_graphics_do_not_change_game_rules() {
        let mut first=World::new(HeroClass::LunarMage);
        let mut second=World::new(HeroClass::LunarMage);
        let mut fx=VisualFx::default();
        for step in 0..160{
            let input=Input{movement:if step%50<25{1.0}else{-1.0},
                jump:step%38==0,melee:step%15==0,
                skills:[step==20,false,step==50],..Default::default()};
            let prior=FrameSnapshot::capture(&second);
            first.tick(input);
            second.tick(input);
            fx.observe(&prior,&second,input);
        }
        assert_eq!(first.tick,second.tick);
        assert_eq!(first.player.hp,second.player.hp);
        assert_eq!(first.player.x,second.player.x);
        assert_eq!(first.player.gold,second.player.gold);
    }
    #[test]
    fn particle_pool_is_bounded() {
        let mut fx=VisualFx::default();
        for _ in 0..80 {fx.spark(vec2(25.0,50.0),WHITE,24,1.0);}
        assert_eq!(fx.particle_count(),MAX_PARTICLES);
        for _ in 0..20 {fx.advance();}
        assert_eq!(fx.particle_count(),0);
    }
}
