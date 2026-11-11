//! Recursos PNG provenientes de los propios Canvas del HTML original.
use macroquad::prelude::*;
use realm_core::HeroClass;

pub struct Assets {
    heroes: [[Option<Texture2D>; 9]; 3],
    attacks: [[Option<Texture2D>; 3]; 3],
    enemies: Vec<[Option<Texture2D>; 2]>,
    grass: [Option<Texture2D>; 4],
    soil: [Option<Texture2D>; 4],
}

async fn texture(name: String) -> Option<Texture2D> {
    match load_texture(&format!("sprites/{}.png",name)).await {
        Ok(texture) => { texture.set_filter(FilterMode::Nearest); Some(texture) }
        Err(_) => None,
    }
}

impl Assets {
    pub async fn load() -> Self {
        let mut result = Self {
            heroes: std::array::from_fn(|_|std::array::from_fn(|_|None)),
            attacks: std::array::from_fn(|_|std::array::from_fn(|_|None)),
            enemies: (0..16).map(|_|std::array::from_fn(|_|None)).collect(),
            grass: std::array::from_fn(|_|None),
            soil: std::array::from_fn(|_|None),
        };
        for class in 0..3 {
            for phase in 0..9 {
                result.heroes[class][phase]=texture(format!("hero_{}_{}",class,phase)).await;
            }
            for phase in 0..3 {
                result.attacks[class][phase]=texture(format!("hero_attack_{}_{}",class,phase)).await;
            }
        }
        for kind in 0..16 {
            for frame in 0..2 {
                result.enemies[kind][frame]=texture(format!("enemy_{}_{}",kind,frame)).await;
            }
        }
        for i in 0..4 {
            result.grass[i]=texture(format!("terrain_grass_{}",i)).await;
            result.soil[i]=texture(format!("terrain_soil_{}",i)).await;
        }
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
        self.enemies.get(kind as usize)?.get(frame.min(1))?.as_ref()
    }
    pub fn grass(&self,index:usize)->Option<&Texture2D>{self.grass[index%4].as_ref()}
    pub fn soil(&self,index:usize)->Option<&Texture2D>{self.soil[index%4].as_ref()}
}
