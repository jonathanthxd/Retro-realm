//! Sprites originales extraídos de los Canvas del HTML sin redibujar.
use macroquad::prelude::*;
use realm_core::HeroClass;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Decoration {
    pub x: f32,
    pub key: String,
    pub ax: f32,
    pub ay: f32,
    pub scale: f32,
    pub glow: bool,
    pub layer: u8,
}
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
    forest_layers: [[Option<Texture2D>; 3]; 2],
    decor_textures: HashMap<String, Texture2D>,
    decor_layout: [Vec<Decoration>; 2],
    // Recursos artísticos de las capas zSky, zRidge, zForest, zCloud
    // exportados desde el HTML, sin recreaciones de terceros.
    scene_layers: HashMap<String, Texture2D>,
    portraits: [Option<Texture2D>; 3],
    ui_icons: HashMap<(String, usize), Texture2D>,
}

async fn texture(name: String) -> Option<Texture2D> {
    match load_texture(&format!("sprites/{}.png", name)).await {
        Ok(texture) => {
            texture.set_filter(FilterMode::Nearest);
            Some(texture)
        }
        Err(_) => None,
    }
}

impl Assets {
    pub async fn load() -> Self {
        let mut result = Self {
            heroes: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            attacks: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            enemies: HashMap::new(),
            humanoid_attacks: HashMap::new(),
            king: HashMap::new(),
            grass: std::array::from_fn(|_| None),
            soil: std::array::from_fn(|_| None),
            dark_grass: std::array::from_fn(|_| None),
            dark_soil: std::array::from_fn(|_| None),
            chests: std::array::from_fn(|_| None),
            forest_layers: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            decor_textures: HashMap::new(),
            decor_layout: std::array::from_fn(|_| Vec::new()),
            scene_layers: HashMap::new(),
            portraits: std::array::from_fn(|_| None),
            ui_icons: HashMap::new(),
        };
        for class in 0..3 {
            for phase in 0..9 {
                result.heroes[class][phase] = texture(format!("hero_{}_{}", class, phase)).await;
            }
            for phase in 0..3 {
                result.attacks[class][phase] =
                    texture(format!("hero_attack_{}_{}", class, phase)).await;
            }
        }
        // Enemigos de la versión original (0..15) y de las extensiones.
        for k in 0..16 {
            for frame in 0..2 {
                if let Some(t) = texture(format!("enemy_{}_{}", k, frame)).await {
                    result.enemies.insert((k, frame), t);
                }
            }
        }
        for kind in 20..38 {
            for frame in 0..2 {
                if let Some(t) = texture(format!("enemy_{}_{}", kind, frame)).await {
                    result.enemies.insert((kind, frame), t);
                }
            }
        }
        for kind in 40..50 {
            for frame in 0..9 {
                if let Some(t) = texture(format!("enemy_{}_{}", kind, frame)).await {
                    result.enemies.insert((kind, frame), t);
                }
            }
            for frame in 0..3 {
                if let Some(t) = texture(format!("humanoid_attack_{}_{}", kind, frame)).await {
                    result.humanoid_attacks.insert((kind, frame), t);
                }
            }
        }
        for kind in 60..63 {
            let n = if kind == 62 { 2 } else { 4 };
            for frame in 0..n {
                if let Some(t) = texture(format!("enemy_{}_{}", kind, frame)).await {
                    result.enemies.insert((kind, frame), t);
                }
            }
        }
        for pose in ["i", "w", "s1", "s2", "s3", "a", "j", "f", "d", "sl"] {
            for frame in 0..if pose == "w" { 4 } else { 2 } {
                if let Some(t) = texture(format!("king_{}_{}", pose, frame)).await {
                    result.king.insert((pose, frame), t);
                }
            }
        }
        for i in 0..4 {
            result.grass[i] = texture(format!("terrain_grass_{}", i)).await;
            result.soil[i] = texture(format!("terrain_soil_{}", i)).await;
            result.dark_grass[i] = texture(format!("dark_grass_{}", i)).await;
            result.dark_soil[i] = texture(format!("dark_soil_{}", i)).await;
        }
        for i in 0..2 {
            result.chests[i] = texture(format!("chest_{}", i)).await;
        }
        for cl in 0..3 {
            result.portraits[cl] = texture(format!("ui_portrait_{}",cl)).await;
            for id in ["s0","s1","s2","s3","bj","bm","bl","br"] {
                if let Some(t)=texture(format!("ui_{}_{}",id,cl)).await {
                    result.ui_icons.insert((id.to_string(),cl),t);
                }
            }
        }
        if let Some(moon)=texture("scene_moon".to_string()).await {
            result.scene_layers.insert("scene_moon".to_string(),moon);
        }
        for scene in 0..2 {
            for part in ["sky","mf","mn","ff","fm"] {
                let key=format!("scene_{}_{}",part,scene);
                if let Some(t)=texture(key.clone()).await {
                    result.scene_layers.insert(key,t);
                }
            }
            for cloud in 0..4 {
                let key=format!("scene_cloud_{}_{}",scene,cloud);
                if let Some(t)=texture(key.clone()).await {
                    result.scene_layers.insert(key,t);
                }
            }
        }
        // Mapas de colocación de vegetación generados por el HTML original.
        // Se almacena como CSV simple para no requerir otro parser en Android.
        for map in 0..2 {
            for layer in 0..3 {
                result.forest_layers[map][layer] =
                    texture(format!("forest_layer_{}_{}",map,layer)).await;
            }
        }
        if let Ok(data)=macroquad::file::load_string("sprites/decor_layout.csv").await {
            for row in data.lines() {
                let parts: Vec<&str> = row.split(',').collect();
                if parts.len()!=8 {continue;}
                let map:usize=match parts[0].parse(){Ok(i) if i<2=>i,_=>continue};
                let parse=|i:usize|parts[i].parse::<f32>().ok();
                let (Some(x),Some(ax),Some(ay),Some(scale)) =
                    (parse(1),parse(3),parse(4),parse(5)) else {continue};
                let key=parts[2].to_string();
                if key.is_empty() || !key.bytes().all(|b|b.is_ascii_alphanumeric()||b==b'_') {
                    continue;
                }
                let layer=parts[7].parse::<u8>().unwrap_or(2).min(2);
                if !result.decor_textures.contains_key(&key) {
                    if let Some(t)=texture(key.clone()).await {
                        result.decor_textures.insert(key.clone(),t);
                    }
                }
                result.decor_layout[map].push(Decoration {
                    x,key,ax,ay,scale:scale.clamp(0.2,3.0),glow:parts[6]=="1",layer,
                });
            }
        }
        result
    }
    pub fn scene(&self,key:&str)->Option<&Texture2D> {
        self.scene_layers.get(key)
    }
    pub fn portrait(&self,hero:HeroClass)->Option<&Texture2D> {
        self.portraits[Self::class_index(hero)].as_ref()
    }
    pub fn ui_icon(&self,id:&str,hero:HeroClass)->Option<&Texture2D> {
        self.ui_icons.get(&(id.to_string(),Self::class_index(hero)))
    }
    pub fn forest_layer(&self,map:usize,layer:usize)->Option<&Texture2D> {
        self.forest_layers.get(map)?.get(layer)?.as_ref()
    }
    pub fn decor(&self,map:usize)->&[Decoration] {
        self.decor_layout.get(map).map(|d|d.as_slice()).unwrap_or(&[])
    }
    pub fn decor_texture(&self,key:&str)->Option<&Texture2D> {
        self.decor_textures.get(key)
    }
    fn class_index(class: HeroClass) -> usize {
        match class {
            HeroClass::LunarMage => 0,
            HeroClass::Huntress => 1,
            HeroClass::CyberRaccoon => 2,
        }
    }
    pub fn hero(&self, class: HeroClass, phase: usize) -> Option<&Texture2D> {
        self.heroes[Self::class_index(class)][phase.min(8)].as_ref()
    }
    pub fn hero_attack(&self, class: HeroClass, phase: usize) -> Option<&Texture2D> {
        self.attacks[Self::class_index(class)][phase.min(2)].as_ref()
    }
    pub fn enemy(&self, kind: u8, frame: usize) -> Option<&Texture2D> {
        self.enemies
            .get(&(kind, frame))
            .or_else(|| self.enemies.get(&(kind, 0)))
    }
    pub fn humanoid_attack(&self, kind: u8, frame: usize) -> Option<&Texture2D> {
        self.humanoid_attacks.get(&(kind, frame.min(2)))
    }
    pub fn king(&self, pose: &'static str, frame: usize) -> Option<&Texture2D> {
        self.king.get(&(pose, frame))
    }
    pub fn grass(&self, index: usize, dark: bool) -> Option<&Texture2D> {
        if dark {
            self.dark_grass[index % 4].as_ref()
        } else {
            self.grass[index % 4].as_ref()
        }
    }
    pub fn soil(&self, index: usize, dark: bool) -> Option<&Texture2D> {
        if dark {
            self.dark_soil[index % 4].as_ref()
        } else {
            self.soil[index % 4].as_ref()
        }
    }
    pub fn chest(&self, opened: bool) -> Option<&Texture2D> {
        self.chests[usize::from(opened)].as_ref()
    }
}
