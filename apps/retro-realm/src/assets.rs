//! Sprites originales extraídos de los Canvas del HTML sin redibujar.
use std::collections::HashMap;
use macroquad::prelude::*;
use realm_core::HeroClass;

pub struct Assets {
    heroes: [[Option<Texture2D>; 9]; 3],
    attacks: [[Option<Texture2D>; 3]; 3],
    enemies: HashMap<(u8, usize), Texture2D>,
    humanoid_attacks: HashMap<(u8, usize), Texture2D>,
    king: HashMap<(&'static str, usize), Texture2D>,
    grass: [Option<Texture2D>; 4],
    soil: [Option<Texture2D>; 4],
    dark_grass: [Option<Texture2D>; 4],
    dark_soil: [Option<Texture2D>; 4],
    chests: [Option<Texture2D>; 2],
}

async fn texture(name: String) -> Option<Texture2D> {
    match load_texture(&format!("sprites/{}.png", name)).await {
        Ok(texture) => { texture.set_filter(FilterMode::Nearest); Some(texture) }
        Err(_) => None,
    }
}

impl Assets {
    pub async fn load() -> Self {
        let mut result = Self {
            heroes: std::array::from_fn(|_|std::array::from_fn(|_|None)),
            attacks: std::array::from_fn(|_|std::array::from_fn(|_|None)),
            enemies: HashMap::new(),
            humanoid_attacks: HashMap::new(),
            king: HashMap::new(),
            grass: std::array::from_fn(|_|None),
            soil: std::array::from_fn(|_|None),
            dark_grass: std::array::from_fn(|_|None),
            dark_soil: std::array::from_fn(|_|None),
            chests: std::array::from_fn(|_|None),
        };
        for class in 0..3 {
            for phase in 0..9 {
                result.heroes[class][phase]=texture(format!("hero_{}_{}",class,phase)).await;
            }
            for phase in 0..3 {
                result.attacks[class][phase]=texture(format!("hero_attack_{}_{}",class,phase)).await;
            }
        }
        // Enemigos de la versión original (0..15) y de las extensiones.
        for k in 0..16 {
            for frame in 0..2 {
                if let Some(t)=texture(format!("enemy_{}_{}",k,frame)).await {
                    result.enemies.insert((k,frame),t);
                }
            }
        }
        for kind in 20..38 {
            for frame in 0..2 {
                if let Some(t)=texture(format!("enemy_{}_{}",kind,frame)).await {
                    result.enemies.insert((kind,frame),t);
                }
            }
        }
        for kind in 40..50 {
            for frame in 0..9 {
                if let Some(t)=texture(format!("enemy_{}_{}",kind,frame)).await {
                    result.enemies.insert((kind,frame),t);
                }
            }
            for frame in 0..3 {
                if let Some(t)=texture(format!("humanoid_attack_{}_{}",kind,frame)).await {
                    result.humanoid_attacks.insert((kind,frame),t);
                }
            }
        }
        for kind in 60..63 {
            let n=if kind==62 {2}else{4};
            for frame in 0..n {
                if let Some(t)=texture(format!("enemy_{}_{}",kind,frame)).await {
                    result.enemies.insert((kind,frame),t);
                }
            }
        }
        for pose in ["i","w","s1","s2","s3","a","j","f","d","sl"] {
            for frame in 0..if pose=="w" {4}else{2} {
                if let Some(t)=texture(format!("king_{}_{}",pose,frame)).await {
                    result.king.insert((pose,frame),t);
                }
            }
        }
        for i in 0..4 {
            result.grass[i]=texture(format!("terrain_grass_{}",i)).await;
            result.soil[i]=texture(format!("terrain_soil_{}",i)).await;
            result.dark_grass[i]=texture(format!("dark_grass_{}",i)).await;
            result.dark_soil[i]=texture(format!("dark_soil_{}",i)).await;
        }
        for i in 0..2 {result.chests[i]=texture(format!("chest_{}",i)).await;}
        result
    }
    fn class_index(class: HeroClass)->usize {
        match class {
            HeroClass::LunarMage=>0,HeroClass::Huntress=>1,HeroClass::CyberRaccoon=>2
        }
    }
    pub fn hero(&self,class:HeroClass,phase:usize)->Option<&Texture2D> {
        self.heroes[Self::class_index(class)][phase.min(8)].as_ref()
    }
    pub fn hero_attack(&self,class:HeroClass,phase:usize)->Option<&Texture2D> {
        self.attacks[Self::class_index(class)][phase.min(2)].as_ref()
    }
    pub fn enemy(&self,kind:u8,frame:usize)->Option<&Texture2D> {
        self.enemies.get(&(kind,frame)).or_else(||self.enemies.get(&(kind,0)))
    }
    pub fn humanoid_attack(&self,kind:u8,frame:usize)->Option<&Texture2D> {
        self.humanoid_attacks.get(&(kind,frame.min(2)))
    }
    pub fn king(&self,pose:&'static str,frame:usize)->Option<&Texture2D> {
        self.king.get(&(pose,frame))
    }
    pub fn grass(&self,index:usize,dark:bool)->Option<&Texture2D> {
        if dark {self.dark_grass[index%4].as_ref()}else{self.grass[index%4].as_ref()}
    }
    pub fn soil(&self,index:usize,dark:bool)->Option<&Texture2D> {
        if dark {self.dark_soil[index%4].as_ref()}else{self.soil[index%4].as_ref()}
    }
    pub fn chest(&self,opened:bool)->Option<&Texture2D>{
        self.chests[usize::from(opened)].as_ref()
    }
}
