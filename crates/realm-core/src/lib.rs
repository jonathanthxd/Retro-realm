//! Simulación determinista de Retro Realm. No depende de Macroquad ni de Android.
//! Las magnitudes de movimiento y combate se expresan en ticks de 1/60 s.
//! Referencia de paridad: \`Retro Realm.html\` (versión original preservada en main).

pub const WORLD_WIDTH: f32 = 320.0 * 16.0;
pub const TICK_RATE: f32 = 60.0;
pub const SKILL_COST: [f32; 3] = [10.0, 25.0, 50.0];
pub const SKILL_COOLDOWN: [u16; 4] = [16, 45, 80, 60];
pub const CHESTS: [&[f32]; 2] = [
    &[520.0, 1180.0, 1960.0, 2740.0, 3500.0, 4300.0],
    &[380.0, 1020.0, 1700.0, 2500.0, 3300.0, 4100.0, 4700.0],
];
pub const TORCHES: [&[f32]; 2] = [
    &[140.0, 1500.0, 2900.0, 4960.0, 5090.0],
    &[160.0, 1300.0, 2800.0, 4870.0, 5080.0],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeroClass {
    LunarMage,
    Huntress,
    CyberRaccoon,
}
impl HeroClass {
    pub fn from_index(n: usize) -> Self {
        match n {
            1 => Self::Huntress,
            2 => Self::CyberRaccoon,
            _ => Self::LunarMage,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::LunarMage => "Maga Lunar",
            Self::Huntress => "Cazadora",
            Self::CyberRaccoon => "Mapache Cyber",
        }
    }
    pub fn color(self) -> [u8; 3] {
        match self {
            Self::LunarMage => [160, 110, 240],
            Self::Huntress => [88, 160, 92],
            Self::CyberRaccoon => [39, 224, 232],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapId {
    NightForest,
    DarkForest,
}
impl MapId {
    pub fn index(self) -> usize {
        match self {
            Self::NightForest => 0,
            Self::DarkForest => 1,
        }
    }
    pub fn quota(self) -> u16 {
        if self == Self::NightForest {
            20
        } else {
            30
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::NightForest => "BOSQUE NOCTURNO",
            Self::DarkForest => "BOSQUE OSCURO",
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    /// -1 (izquierda), 0 o 1 (derecha); teclas/joystick independientes del hardware.
    pub movement: f32,
    pub jump: bool,
    pub melee: bool,
    pub skills: [bool; 3],
    pub dash: bool,
    pub shield: bool,
    pub interact: bool,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub class: HeroClass,
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub ground: bool,
    pub facing: f32,
    pub moving: bool,
    pub hp: f32,
    pub max_hp: f32,
    pub mp: f32,
    pub max_mp: f32,
    pub level: u16,
    pub xp: u32,
    pub gold: u32,
    pub gems: u32,
    pub inv: u16,
    pub melee_ticks: u16,
    pub melee_cd: u16,
    pub cooldowns: [u16; 4],
    pub dash_ticks: u16,
    pub shield_ticks: u16,
    pub shield_cd: u16,
    pub extra_jump: bool,
    pub jump_ticks: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KingState {
    Idle,
    Combo,
    Leap,
    Dash,
}

#[derive(Clone, Debug)]
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub ground: bool,
    pub kind: u8,
    pub level: u16,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub base_speed: f32,
    pub damage: f32,
    pub reward: f32,
    pub scale: f32,
    pub knockback: f32,
    pub facing: f32,
    pub flash: u16,
    pub attack: u16,
    pub attack_cd: u16,
    pub boss: bool,
    pub action: u16,
    pub phase: u16,
    pub king_state: KingState,
    pub king_pose: &'static str,
    pub king_ticks: u16,
    pub king_move_ticks: u32,
    pub king_cooldown: u16,
    pub king_combo_left: u8,
    pub king_dir: f32,
    pub king_air: bool,
}
impl Enemy {
    pub fn name(&self) -> &'static str {
        match self.kind {
            0 => "Slime",
            1 => "Segador",
            2 => "Bruto",
            3 => "Lobo",
            4 => "Esqueleto",
            40 => "Zombi",
            41 => "Bárbaro",
            42 => "Pícaro",
            43 => "Arquero",
            44 => "Segador oscuro",
            45 => "Caballero",
            46 => "Ninja",
            47 => "Sacerdote carmesí",
            48 => "Cruzado",
            49 => "Guerrero bruto",
            60 => "Ogro pantanoso",
            61 => "Ogro de tres calaveras",
            62 => "Rey de las Tumbas",
            _ => "Criatura",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectileKind {
    Orb,
    Knife,
    Explosive,
    EnemyArrow,
}
#[derive(Clone, Debug)]
pub struct Projectile {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub damage: f32,
    pub radius: f32,
    pub life: u16,
    pub kind: ProjectileKind,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PickupKind {
    Gold,
    Heart,
    Gem,
    Mana,
}
#[derive(Clone, Debug)]
pub struct Pickup {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub kind: PickupKind,
    pub amount: u32,
    pub life: u16,
}
#[derive(Clone, Debug)]
pub struct Chest {
    pub x: f32,
    pub opened: bool,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BossStage {
    Locked,
    Alive,
    Defeated,
}

#[derive(Clone, Debug)]
struct Rng {
    seed: u64,
}
impl Rng {
    fn new(seed: u64) -> Self {
        Self { seed }
    }
    fn next(&mut self) -> f32 {
        self.seed = (self.seed * 16807) % 2147483647;
        self.seed as f32 / 2147483647.0
    }
    fn range(&mut self, a: f32, b: f32) -> f32 {
        a + (b - a) * self.next()
    }
    fn pick(&mut self, items: &[u8]) -> u8 {
        items[((self.next() * items.len() as f32) as usize).min(items.len() - 1)]
    }
}

#[derive(Debug)]
pub struct World {
    pub terrain: [f32; 320],
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    pub pickups: Vec<Pickup>,
    pub chests: [Vec<Chest>; 2],
    pub map: MapId,
    pub cleared: [bool; 2],
    pub boss_stages: [BossStage; 2],
    pub killed: u16,
    pub spawned: u16,
    pub spawn_timer: i32,
    pub tick: u64,
    pub game_over: bool,
    pub next_map_pending: bool,
    pub message: String,
    pub message_ticks: u16,
    rng: Rng,
}
impl World {
    pub fn new(class: HeroClass) -> Self {
        let mut rng = Rng::new(11);
        let mut terrain = [0.0; 320];
        let mut height: i32 = 4;
        for (i, y) in terrain.iter_mut().enumerate() {
            if i > 14 && rng.next() < 0.13 {
                height += if rng.next() < 0.5 { -1 } else { 1 };
                height = height.clamp(3, 6);
            }
            *y = 180.0 - (height * 16) as f32;
        }
        let start_y = terrain[3];
        Self {
            terrain,
            player: Player {
                class,
                x: 60.0,
                y: start_y,
                vy: 0.0,
                ground: true,
                facing: 1.0,
                moving: false,
                hp: 60.0,
                max_hp: 60.0,
                mp: 100.0,
                max_mp: 100.0,
                level: 1,
                xp: 0,
                gold: 0,
                gems: 0,
                inv: 0,
                melee_ticks: 0,
                melee_cd: 0,
                cooldowns: [0; 4],
                dash_ticks: 0,
                shield_ticks: 0,
                shield_cd: 0,
                extra_jump: true,
                jump_ticks: 99,
            },
            enemies: Vec::new(),
            projectiles: Vec::new(),
            pickups: Vec::new(),
            chests: std::array::from_fn(|m| {
                CHESTS[m]
                    .iter()
                    .map(|x| Chest {
                        x: *x,
                        opened: false,
                    })
                    .collect()
            }),
            map: MapId::NightForest,
            cleared: [false; 2],
            boss_stages: [BossStage::Locked; 2],
            killed: 0,
            spawned: 0,
            spawn_timer: 30,
            tick: 0,
            game_over: false,
            next_map_pending: false,
            message: String::new(),
            message_ticks: 0,
            rng,
        }
    }
    pub fn reset(&mut self) {
        *self = Self::new(self.player.class);
    }
    pub fn floor(&self, x: f32) -> f32 {
        floor_at(&self.terrain, x)
    }
    pub fn damage(&self) -> f32 {
        12.0 + self.player.level as f32 * 5.0
    }
    pub fn boss(&self) -> Option<&Enemy> {
        self.enemies.iter().find(|e| e.boss)
    }
    pub fn portal_available(&self) -> bool {
        self.cleared[self.map.index()]
    }
    pub fn player_near_portal(&self) -> bool {
        self.portal_available() && (self.player.x - 5040.0).abs() < 22.0
            || (self.map == MapId::DarkForest
                && self.portal_available()
                && (self.player.x - 4920.0).abs() < 22.0)
    }
    fn say(&mut self, msg: &str, ticks: u16) {
        self.message = msg.to_owned();
        self.message_ticks = ticks;
    }
    fn move_player(&mut self, velocity: f32) {
        let p = &mut self.player;
        walk(
            &self.terrain,
            &mut p.x,
            &mut p.y,
            &mut p.vy,
            &mut p.ground,
            velocity,
            false,
        );
    }
    fn hurt_player(&mut self, amount: f32) {
        if self.player.inv != 0 || self.game_over {
            return;
        }
        let p = &mut self.player;
        p.hp = (p.hp - amount).max(0.0);
        p.inv = 50;
        p.vy = -2.2;
        if p.hp == 0.0 {
            self.game_over = true;
            self.say("Has caido", 160);
        }
    }
    fn damage_enemy(&mut self, index: usize, amount: f32) {
        if index >= self.enemies.len() {
            return;
        }
        let damage = (amount * self.rng.range(0.85, 1.15)).round();
        let e = &mut self.enemies[index];
        e.hp -= damage;
        e.flash = 6;
        e.knockback = (e.x - self.player.x).signum() * 1.4;
    }
    fn explode(&mut self, x: f32, y: f32, damage: f32, radius: f32) {
        let targets: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                ((e.x - x).powi(2) + (e.y - 10.0 * e.scale - y).powi(2)).sqrt()
                    < radius + 8.0 * (e.scale - 1.0)
            })
            .map(|(i, _)| i)
            .collect();
        for i in targets {
            self.damage_enemy(i, damage);
        }
    }
    fn cast(&mut self, index: usize) {
        if index > 2 || self.player.cooldowns[index] > 0 || self.player.mp < SKILL_COST[index] {
            return;
        }
        self.player.mp -= SKILL_COST[index];
        self.player.cooldowns[index] = SKILL_COOLDOWN[index];
        let (x, y, f, class) = (
            self.player.x,
            self.player.y,
            self.player.facing,
            self.player.class,
        );
        let damage = self.damage();
        match index {
            0 => {
                let (vx, factor, radius, life, kind) = match class {
                    HeroClass::CyberRaccoon => (4.5, 1.2, 4.0, 60, ProjectileKind::Orb),
                    HeroClass::Huntress => (5.0, 1.3, 3.0, 50, ProjectileKind::Knife),
                    HeroClass::LunarMage => (4.0, 1.0, 3.0, 60, ProjectileKind::Orb),
                };
                self.projectiles.push(Projectile {
                    x: x + f * 9.0,
                    y: y - 13.0,
                    vx: vx * f,
                    vy: 0.0,
                    damage: damage * factor,
                    radius,
                    life,
                    kind,
                });
            }
            1 => {
                let best = self
                    .enemies
                    .iter()
                    .enumerate()
                    .filter(|(_, e)| (e.x - x) * f > -20.0 && (e.x - x).abs() < 180.0)
                    .min_by(|(_, a), (_, b)| (a.x - x).abs().total_cmp(&(b.x - x).abs()))
                    .map(|(i, _)| i);
                if let Some(i) = best {
                    self.damage_enemy(i, damage * 3.0);
                }
            }
            2 => match class {
                HeroClass::Huntress => {
                    for step in -2..=2 {
                        self.projectiles.push(Projectile {
                            x: x + f * 8.0,
                            y: y - 12.0,
                            vx: 4.2 * f,
                            vy: step as f32 * 0.5,
                            damage: damage * 1.1,
                            radius: 3.0,
                            life: 45,
                            kind: ProjectileKind::Knife,
                        });
                    }
                }
                HeroClass::CyberRaccoon => self.explode(x, y - 10.0, damage * 2.6, 75.0),
                HeroClass::LunarMage => self.explode(x, y - 10.0, damage * 2.5, 70.0),
            },
            _ => unreachable!(),
        }
    }
    fn melee(&mut self) {
        if self.player.melee_cd == 0 && self.player.melee_ticks == 0 {
            self.player.melee_cd = 20;
            self.player.melee_ticks = 14;
        }
    }
    fn melee_hit(&mut self) {
        let (x, y, f, c) = (
            self.player.x,
            self.player.y,
            self.player.facing,
            self.player.class,
        );
        let targets: Vec<usize> = self
            .enemies
            .iter()
            .enumerate()
            .filter(|(_, e)| {
                let dx = (e.x - x) * f;
                dx > -8.0 - 5.0 * e.scale
                    && dx < 32.0 + 12.0 * (e.scale - 1.0) + 4.0
                    && (e.y - y).abs() < 24.0 * e.scale.max(1.0)
            })
            .map(|(i, _)| i)
            .collect();
        for i in targets {
            self.damage_enemy(
                i,
                self.damage()
                    * if c == HeroClass::CyberRaccoon {
                        1.5
                    } else {
                        1.2
                    },
            );
            self.enemies[i].knockback = f * 2.6;
        }
        let close: Vec<usize> = self.chests[self.map.index()]
            .iter()
            .enumerate()
            .filter(|(_, ch)| {
                !ch.opened
                    && (ch.x - x).abs() < 28.0
                    && (ch.x - x) * f > -10.0
                    && (self.floor(ch.x) - y).abs() < 22.0
            })
            .map(|(i, _)| i)
            .collect();
        for i in close {
            self.open_chest(i);
        }
    }
    fn open_chest(&mut self, index: usize) {
        let map = self.map.index();
        if self.chests[map][index].opened {
            return;
        }
        self.chests[map][index].opened = true;
        let x = self.chests[map][index].x;
        let y = self.floor(x) - 14.0;
        let coins = 5 + (self.rng.next() * 4.0) as usize;
        for _ in 0..coins {
            let px = x + self.rng.range(-5.0, 5.0);
            let vy = self.rng.range(-4.6, -3.0);
            self.pickups.push(Pickup {
                x: px,
                y,
                vy,
                kind: PickupKind::Gold,
                amount: 6 + self.player.level as u32 * 3,
                life: 450,
            });
        }
        for (kind, chance) in [
            (PickupKind::Heart, 0.55),
            (PickupKind::Gem, 0.4),
            (PickupKind::Mana, 0.4),
        ] {
            if self.rng.next() < chance {
                self.pickups.push(Pickup {
                    x,
                    y,
                    vy: -3.8,
                    kind,
                    amount: 1,
                    life: 450,
                });
            }
        }
    }
    fn spawn_enemy(&mut self) {
        if self.spawned >= self.map.quota() || self.cleared[self.map.index()] {
            return;
        }
        let side = if self.rng.next() < 0.5 { -1.0 } else { 1.0 };
        let mut x = self.player.x + side * (190.0 + self.rng.range(0.0, 40.0));
        if !(20.0..=WORLD_WIDTH - 20.0).contains(&x) {
            x = self.player.x - side * 200.0;
        }
        if !(20.0..=WORLD_WIDTH - 20.0).contains(&x) {
            return;
        }
        let l = self.player.level;
        let k = if l >= 4 && self.rng.next() < 0.2 {
            2
        } else if l >= 2 && self.rng.next() < 0.45 {
            1
        } else {
            0
        };
        let el = (l as i32 - 1 + (self.rng.next() * 3.0) as i32).max(1) as u16;
        let b = 25.0 + el as f32 * 18.0;
        let d = 3.0 + el as f32 * 2.0;
        let mut e = self.enemy(x, k, el, b, d);
        // Redistribucion original de Bosque Oscuro, antes del ajuste de variantes.
        if self.map == MapId::DarkForest {
            let r = self.rng.next();
            if r < 0.38 {
                e.kind = 3;
                e.hp = b * 1.3;
                e.speed = 0.85;
                e.damage = d * 1.2;
                e.reward = 1.8
            } else if r < 0.72 {
                e.kind = 4;
                e.hp = b * 1.1;
                e.speed = 0.3;
                e.damage = d * 1.1;
                e.reward = 1.6
            } else {
                e.kind = 1;
                e.hp = b * 1.6;
                e.speed = 0.55;
                e.damage = d * 1.3;
                e.reward = 1.6
            }
            e.max_hp = e.hp;
            e.base_speed = e.speed;
        }
        // La revisión final del HTML agrega slimes y humanos, además de los iniciales.
        let r = self.rng.next();
        if r < if self.map == MapId::NightForest {
            0.58
        } else {
            0.30
        } {
            e.kind = self.rng.pick(if self.map == MapId::NightForest {
                &[20, 20, 21, 22, 22, 23, 23, 24, 27, 30, 31, 32, 36, 26, 33]
            } else {
                &[25, 26, 28, 29, 33, 34, 35, 37, 30, 24, 21, 23]
            });
            e.scale = 1.0 + self.rng.range(0.0, 0.6);
        } else if r < if self.map == MapId::NightForest {
            0.88
        } else {
            0.60
        } {
            e.kind = self.rng.pick(if self.map == MapId::NightForest {
                &[41, 42, 43, 48, 42, 41]
            } else {
                &[44, 45, 46, 47, 49, 40, 40, 46]
            });
            e.scale = 1.0 + self.rng.range(0.0, 0.35);
        } else if r > 0.94 {
            e.kind = if self.map == MapId::NightForest {
                60
            } else {
                61
            };
            e.scale = 1.5;
            e.hp = b * 5.0;
            e.max_hp = e.hp;
            e.damage = d * 2.0;
            e.reward = 5.0;
        }
        self.enemies.push(e);
        self.spawned += 1;
    }
    fn enemy(&self, x: f32, kind: u8, level: u16, hp: f32, damage: f32) -> Enemy {
        let mult = match kind {
            1 => 1.6,
            2 => 3.0,
            _ => 1.0,
        };
        let speed = match kind {
            1 => 0.55,
            2 => 0.35,
            _ => 0.4,
        };
        Enemy {
            x,
            y: self.floor(x) - 2.0,
            vy: 0.0,
            ground: false,
            kind,
            level,
            hp: hp * mult,
            max_hp: hp * mult,
            speed,
            base_speed: speed,
            damage: damage
                * match kind {
                    1 => 1.3,
                    2 => 2.0,
                    _ => 1.0,
                },
            reward: mult,
            scale: 1.0,
            knockback: 0.0,
            facing: 1.0,
            flash: 0,
            attack: 0,
            attack_cd: 0,
            boss: false,
            action: 0,
            phase: 0,
            king_state: KingState::Idle,
            king_pose: "i",
            king_ticks: 0,
            king_move_ticks: 0,
            king_cooldown: 0,
            king_combo_left: 0,
            king_dir: 1.0,
            king_air: false,
        }
    }
    fn spawn_boss(&mut self) {
        let is_dark = self.map == MapId::DarkForest;
        let side = if self.rng.next() < 0.5 { -1.0 } else { 1.0 };
        let x = (self.player.x + side * 200.0).clamp(30.0, WORLD_WIDTH - 30.0);
        let level = self.player.level + 2;
        // Ajustes exactos del override spawnBoss original para ambos bosques.
        let hp_mult = if is_dark { 17.0 } else { 15.0 };
        let damage_mult = if is_dark { 3.4 } else { 3.6 };
        let mut e = self.enemy(
            x,
            if is_dark { 62 } else { 60 },
            level,
            ((25.0 + level as f32 * 18.0) * hp_mult).round(),
            ((3.0 + level as f32 * 2.0) * damage_mult).round(),
        );
        e.boss = true;
        e.scale = if is_dark { 1.4 } else { 1.55 };
        e.speed = if is_dark { 0.65 } else { 0.45 };
        e.base_speed = e.speed;
        e.reward = 8.0;
        self.enemies.push(e);
        self.boss_stages[self.map.index()] = BossStage::Alive;
        self.say("¡JEFE FINAL!", 150);
    }
    fn portal(&mut self) {
        if !self.portal_available() {
            return;
        }
        match self.map {
            MapId::NightForest if (self.player.x - 5040.0).abs() < 22.0 => {
                self.switch_map(MapId::DarkForest, 70.0)
            }
            MapId::DarkForest if (self.player.x - 4920.0).abs() < 22.0 => {
                self.switch_map(MapId::NightForest, 4960.0)
            }
            MapId::DarkForest if (self.player.x - 5040.0).abs() < 22.0 => {
                self.next_map_pending = true;
                self.say("Proximamente", 150);
            }
            _ => {}
        }
    }
    fn switch_map(&mut self, map: MapId, x: f32) {
        self.map = map;
        self.enemies.clear();
        self.projectiles.clear();
        self.pickups.clear();
        self.killed = if self.cleared[map.index()] {
            map.quota()
        } else {
            0
        };
        self.spawned = self.killed;
        self.player.x = x;
        self.player.y = self.floor(x);
        self.player.vy = 0.0;
        self.spawn_timer = 40;
        self.say(map.name(), 160);
    }
    fn update_enemy_ai(&mut self) {
        let (px, py) = (self.player.x, self.player.y);
        let rng = &mut self.rng;
        let mut attacks = Vec::new();
        let mut arrows = Vec::new();
        for e in &mut self.enemies {
            if e.flash > 0 {
                e.flash -= 1;
            }
            if e.attack > 0 {
                e.attack -= 1;
            }
            if e.attack_cd > 0 {
                e.attack_cd -= 1;
            }
            let dx = px - e.x;
            let dist = dx.abs();
            e.facing = if dx < 0.0 { -1.0 } else { 1.0 };
            match e.kind {
                0 | 3 | 20..=39 => {
                    if e.ground && e.attack_cd == 0 && dist > 14.0 && dist < 90.0 {
                        e.vy = -4.4;
                        e.attack_cd = 70;
                        e.attack = 22;
                    }
                    e.speed = if e.ground {
                        e.base_speed
                    } else {
                        e.base_speed * 2.6
                    };
                }
                4 | 43 | 44 | 47 => {
                    e.speed = if dist < 75.0 {
                        -e.base_speed
                    } else if dist > 140.0 {
                        e.base_speed
                    } else {
                        0.0
                    };
                    if e.attack_cd == 0 && dist > 40.0 && dist < 190.0 {
                        e.attack = 26;
                        e.attack_cd = 110;
                    }
                    if e.attack == 10 {
                        arrows.push(Projectile {
                            x: e.x + e.facing * 8.0,
                            y: e.y - 14.0,
                            vx: e.facing * 2.8,
                            vy: -0.15,
                            damage: e.damage * 0.8,
                            radius: 2.0,
                            life: 160,
                            kind: ProjectileKind::EnemyArrow,
                        });
                    }
                }
                62 if e.boss => {
                    // Port de zKingAI (script final del HTML original):
                    // combo encadenado, salto de área y dash; fase de furia <50% HP.
                    let rage = e.hp < e.max_hp * 0.5;
                    let combo_size = if rage { 4 } else { 3 };
                    e.king_move_ticks = e.king_move_ticks.saturating_add(1);
                    e.king_cooldown = e.king_cooldown.saturating_sub(1);
                    e.speed = 0.0;
                    match e.king_state {
                        KingState::Idle => {
                            e.speed = e.base_speed;
                            e.king_pose = if !e.ground {
                                if e.vy < 0.0 {
                                    "j"
                                } else {
                                    "f"
                                }
                            } else if dist > 6.0 {
                                "w"
                            } else {
                                "i"
                            };
                            if e.king_cooldown == 0 {
                                if dist < 56.0 {
                                    e.king_state = KingState::Combo;
                                    e.king_combo_left = combo_size;
                                    e.king_ticks = 0;
                                } else if e.king_move_ticks > (if rage { 80 } else { 120 })
                                    && dist < 300.0
                                {
                                    let r = rng.next();
                                    let leap = if dist > 150.0 { r >= 0.5 } else { r < 0.5 };
                                    e.king_state = if leap {
                                        KingState::Leap
                                    } else {
                                        KingState::Dash
                                    };
                                    e.king_ticks = 0;
                                    e.king_air = false;
                                }
                            }
                        }
                        KingState::Combo => {
                            e.king_ticks = e.king_ticks.saturating_add(1);
                            let i = (combo_size - e.king_combo_left).min(2);
                            e.king_pose = match i {
                                0 => "s1",
                                1 => "s2",
                                _ => "s3",
                            };
                            if e.king_ticks == 1 {
                                e.king_dir = e.facing;
                            }
                            if e.king_ticks < 10 {
                                e.knockback = e.king_dir * 1.3;
                            }
                            if e.king_ticks == 12 && dist < 64.0 && (py - e.y).abs() < 32.0 {
                                attacks.push((e.damage * (if i == 2 { 1.4 } else { 1.0 })).round());
                            }
                            if e.king_ticks >= 22 {
                                e.king_combo_left = e.king_combo_left.saturating_sub(1);
                                e.king_ticks = 0;
                                if e.king_combo_left == 0 {
                                    e.king_state = KingState::Idle;
                                    e.king_cooldown = if rage { 45 } else { 70 };
                                    e.king_move_ticks = 0;
                                    e.king_pose = "i";
                                }
                            }
                        }
                        KingState::Leap => {
                            e.king_ticks = e.king_ticks.saturating_add(1);
                            if e.king_ticks < 16 {
                                e.king_pose = "a";
                            } else if e.king_ticks == 16 {
                                e.vy = -6.2;
                                e.king_air = true;
                                e.king_pose = "j";
                                e.knockback = e.facing * (dist / 20.0 + 0.6).min(4.4);
                            } else if e.king_air {
                                e.king_pose = if e.vy < 0.0 { "j" } else { "f" };
                                if e.king_ticks < 40 {
                                    e.knockback = e.facing * (dist / 22.0).abs().max(0.6).min(4.2);
                                }
                                if e.ground && e.king_ticks > 20 {
                                    e.king_air = false;
                                    e.king_ticks = 100;
                                    if dist < 95.0 && py > e.y - 30.0 {
                                        attacks.push((e.damage * 1.2).round());
                                    }
                                }
                            } else if e.king_ticks >= 100 {
                                e.king_pose = "sl";
                                if e.king_ticks >= 124 {
                                    e.king_state = KingState::Idle;
                                    e.king_cooldown = if rage { 50 } else { 80 };
                                    e.king_move_ticks = 0;
                                    e.king_pose = "i";
                                }
                            }
                        }
                        KingState::Dash => {
                            e.king_ticks = e.king_ticks.saturating_add(1);
                            if e.king_ticks < 12 {
                                e.king_pose = "a";
                                e.king_dir = e.facing;
                            } else if e.king_ticks < 30 {
                                e.king_pose = "d";
                                e.knockback = e.king_dir * 5.4;
                                if e.king_ticks == 24 && dist < 52.0 && (py - e.y).abs() < 32.0 {
                                    attacks.push((e.damage * 1.3).round());
                                }
                            } else if e.king_ticks < 44 {
                                e.king_pose = "s2";
                            } else {
                                e.king_state = KingState::Idle;
                                e.king_cooldown = 60;
                                e.king_move_ticks = 0;
                                e.king_pose = "i";
                            }
                        }
                    }
                }
                _ => {
                    let reach = if e.boss { 40.0 } else { 28.0 };
                    if e.attack == 0 && e.attack_cd == 0 && dist < reach + 6.0 {
                        e.attack = if e.boss { 36 } else { 30 };
                        e.attack_cd = if e.boss { 90 } else { 75 };
                    }
                    if e.attack == if e.boss { 14 } else { 12 }
                        && dist < reach
                        && (py - e.y).abs() < 24.0
                    {
                        attacks.push((e.damage * 1.2).round());
                    }
                    if e.attack > 0 {
                        e.speed = 0.0
                    } else {
                        e.speed = e.base_speed
                    }
                }
            }
            e.knockback *= 0.8;
            let speed = if dist > 6.0 { e.facing * e.speed } else { 0.0 };
            walk(
                &self.terrain,
                &mut e.x,
                &mut e.y,
                &mut e.vy,
                &mut e.ground,
                speed + e.knockback,
                true,
            );
        }
        self.projectiles.extend(arrows);
        for attack in attacks {
            self.hurt_player(attack);
        }
    }
    fn update_projectiles(&mut self) {
        // Los impactos conservan el daño del proyectil concreto. Nunca se
        // codifican índices como números negativos ni se busca otro proyectil.
        let mut explosions = Vec::new();
        let mut enemy_hits: Vec<(usize, f32)> = Vec::new();
        let mut player_hits: Vec<f32> = Vec::new();
        let mut chest_hits: Vec<usize> = Vec::new();
        let active_chests = &self.chests[self.map.index()];
        for q in &mut self.projectiles {
            q.x += q.vx;
            q.y += q.vy;
            q.life = q.life.saturating_sub(1);
            if q.kind == ProjectileKind::EnemyArrow {
                q.vy += 0.012;
            }
            let mut dead =
                q.life == 0 || q.x < 0.0 || q.x > WORLD_WIDTH || q.y > floor_at(&self.terrain, q.x);
            if !dead && q.kind == ProjectileKind::EnemyArrow {
                if (q.x - self.player.x).abs() < 6.0 && (q.y - (self.player.y - 12.0)).abs() < 13.0
                {
                    dead = true;
                    player_hits.push(q.damage);
                }
            } else if !dead {
                for (i, e) in self.enemies.iter().enumerate() {
                    if (e.x - q.x).abs() < q.radius + 6.0 * e.scale
                        && q.y > e.y - 24.0 * e.scale
                        && q.y < e.y + 2.0
                    {
                        dead = true;
                        if q.kind != ProjectileKind::Explosive {
                            enemy_hits.push((i, q.damage));
                        }
                        break;
                    }
                }
                if !dead {
                    for (i, ch) in active_chests.iter().enumerate() {
                        let y = floor_at(&self.terrain, ch.x);
                        if !ch.opened && (ch.x - q.x).abs() < 9.0 && (q.y - (y - 8.0)).abs() < 10.0
                        {
                            chest_hits.push(i);
                            dead = true;
                            break;
                        }
                    }
                }
            }
            if dead {
                q.life = 0;
                if q.kind == ProjectileKind::Explosive {
                    explosions.push((q.x, q.y, q.damage));
                }
            }
        }
        for (index, damage) in enemy_hits {
            self.damage_enemy(index, damage);
        }
        for damage in player_hits {
            self.hurt_player(damage);
        }
        chest_hits.sort_unstable();
        chest_hits.dedup();
        for index in chest_hits {
            if !self.chests[self.map.index()][index].opened {
                self.open_chest(index);
            }
        }
        for (x, y, damage) in explosions {
            self.explode(x, y, damage, 45.0);
        }
        self.projectiles.retain(|q| q.life > 0);
    }
    fn update_deaths(&mut self) {
        let mut fallen = Vec::new();
        self.enemies.retain(|e| {
            if e.hp <= 0.0 {
                fallen.push(e.clone());
                false
            } else {
                true
            }
        });
        for e in fallen {
            if !e.boss {
                self.killed = self.killed.saturating_add(1);
            }
            self.player.gold += ((e.level as f32 * 8.0 * e.reward).round() as u32).max(1);
            self.player.xp += ((e.level as f32 * 3.0 * e.reward).round() as u32).max(1);
            if self.rng.next() < 0.10 {
                self.player.gems += 1;
            }
            let need = self.player.level as u32 * 30;
            if self.player.xp >= need {
                self.player.xp -= need;
                self.player.level += 1;
                self.player.max_hp += 20.0;
                self.player.hp = self.player.max_hp;
                self.player.max_mp += 10.0;
                self.player.mp = self.player.max_mp;
                self.say("¡NIVEL!", 90);
            }
        }
        let m = self.map.index();
        if self.killed >= self.map.quota()
            && self.enemies.is_empty()
            && self.boss_stages[m] == BossStage::Locked
            && !self.cleared[m]
        {
            self.spawn_boss();
        } else if self.boss_stages[m] == BossStage::Alive && self.boss().is_none() {
            self.boss_stages[m] = BossStage::Defeated;
            if self.enemies.is_empty() {
                self.cleared[m] = true;
                self.say("Portal desbloqueado", 150);
            }
        }
    }
    fn update_pickups(&mut self) {
        for p in &mut self.pickups {
            p.vy += 0.12;
            p.y += p.vy;
            p.life = p.life.saturating_sub(1);
            if p.y > floor_at(&self.terrain, p.x) - 4.0 {
                p.y = floor_at(&self.terrain, p.x) - 4.0;
                p.vy = 0.0;
            }
            if (p.x - self.player.x).abs() < 12.0 && (p.y - (self.player.y - 10.0)).abs() < 16.0 {
                match p.kind {
                    PickupKind::Gold => self.player.gold += p.amount,
                    PickupKind::Heart => {
                        self.player.hp = (self.player.hp + 25.0).min(self.player.max_hp)
                    }
                    PickupKind::Gem => self.player.gems += p.amount,
                    PickupKind::Mana => {
                        self.player.mp = (self.player.mp + 30.0).min(self.player.max_mp)
                    }
                }
                p.life = 0;
            }
        }
        self.pickups.retain(|p| p.life > 0);
    }
    /// Ejecuta exactamente un tick lógico de 1/60s. No llamar con delta time variable.
    pub fn tick(&mut self, input: Input) {
        self.tick += 1;
        if self.message_ticks > 0 {
            self.message_ticks -= 1;
        }
        for cd in &mut self.player.cooldowns {
            *cd = cd.saturating_sub(1);
        }
        if self.game_over {
            return;
        }
        for i in 0..3 {
            if input.skills[i] {
                self.cast(i);
            }
        }
        if input.dash && self.player.cooldowns[3] == 0 && self.player.dash_ticks == 0 {
            self.player.cooldowns[3] = 60;
            self.player.dash_ticks = 14;
            self.player.vy = self.player.vy.min(-1.6);
        }
        if input.shield
            && self.player.shield_ticks == 0
            && self.player.shield_cd == 0
            && self.player.mp >= 30.0
        {
            self.player.mp -= 30.0;
            self.player.shield_ticks = 180;
            self.player.shield_cd = 480;
        }
        if input.melee {
            self.melee();
        }
        let movement = input.movement.clamp(-1.0, 1.0);
        self.player.moving = movement.abs() > 0.1;
        if movement.abs() > 0.1 {
            self.player.facing = movement.signum();
        }
        self.player.jump_ticks = self.player.jump_ticks.saturating_add(1);
        if self.player.ground {
            self.player.extra_jump = true;
        }
        if input.jump {
            if self.player.ground {
                self.player.vy = -4.8;
                self.player.ground = false;
                self.player.jump_ticks = 0;
            } else if self.player.extra_jump && self.player.jump_ticks > 14 {
                self.player.vy = -4.2;
                self.player.extra_jump = false;
                self.player.jump_ticks = 0;
            }
        }
        if self.player.dash_ticks > 0 {
            self.player.dash_ticks -= 1;
            self.move_player(self.player.facing * 4.6);
            self.player.inv = self.player.inv.max(3);
        }
        if self.player.shield_ticks > 0 {
            self.player.shield_ticks -= 1;
            self.player.inv = self.player.inv.max(2);
        }
        self.player.shield_cd = self.player.shield_cd.saturating_sub(1);
        self.move_player(
            movement
                * if self.player.class == HeroClass::CyberRaccoon {
                    1.6
                } else {
                    1.25
                },
        );
        self.player.mp = (self.player.mp + 0.12).min(self.player.max_mp);
        self.player.melee_cd = self.player.melee_cd.saturating_sub(1);
        if self.player.melee_ticks > 0 {
            self.player.melee_ticks -= 1;
            if self.player.melee_ticks == 9 {
                self.melee_hit();
            }
        }
        self.player.inv = self.player.inv.saturating_sub(1);
        self.spawn_timer -= 1;
        if self.spawn_timer <= 0 && self.enemies.len() < 5 + self.player.level as usize {
            self.spawn_enemy();
            self.spawn_timer = (130 - self.player.level as i32 * 8).max(40);
        }
        self.update_enemy_ai();
        let hits: Vec<f32> = self
            .enemies
            .iter()
            .filter(|e| (self.player.x - e.x).abs() < 9.0 && (self.player.y - e.y).abs() < 14.0)
            .map(|e| e.damage)
            .collect();
        for amount in hits {
            self.hurt_player(amount);
        }
        self.update_projectiles();
        self.update_deaths();
        self.update_pickups();
        if input.interact {
            self.portal();
        }
    }
}

fn floor_at(terrain: &[f32; 320], x: f32) -> f32 {
    let at = |px: f32| terrain[((px / 16.0).floor() as i32).clamp(0, 319) as usize];
    at(x - 3.0).min(at(x + 3.0))
}
fn walk(
    terrain: &[f32; 320],
    x: &mut f32,
    y: &mut f32,
    vy: &mut f32,
    ground: &mut bool,
    step: f32,
    hop: bool,
) {
    if step != 0.0 {
        let nx = *x + step;
        if floor_at(terrain, nx + step.signum() * 4.0) >= *y - 5.0 {
            *x = nx.clamp(8.0, WORLD_WIDTH - 8.0);
        } else if hop && *ground {
            *vy = -4.0;
        }
    }
    *vy += 0.25;
    *y += *vy;
    let bottom = floor_at(terrain, *x);
    if *y >= bottom {
        *y = bottom;
        *vy = 0.0;
        *ground = true;
    } else {
        *ground = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn starting_values_match_legacy() {
        let w = World::new(HeroClass::LunarMage);
        assert_eq!(w.player.hp, 60.0);
        assert_eq!(w.player.mp, 100.0);
        assert_eq!(w.player.level, 1);
        assert_eq!(w.map.quota(), 20);
        assert_eq!(MapId::DarkForest.quota(), 30);
        assert_eq!(w.chests[0].len(), 6);
        assert_eq!(w.chests[1].len(), 7);
    }
    #[test]
    fn world_is_reproducible() {
        let a = World::new(HeroClass::Huntress);
        let b = World::new(HeroClass::Huntress);
        assert_eq!(a.terrain, b.terrain);
    }
    #[test]
    fn jumping_and_mana() {
        let mut w = World::new(HeroClass::LunarMage);
        let y = w.player.y;
        w.tick(Input {
            jump: true,
            skills: [true, false, false],
            ..Input::default()
        });
        assert!(w.player.y < y);
        assert!(w.player.mp < 100.0);
        assert_eq!(w.player.cooldowns[0], 16);
    }
    #[test]
    fn a_boss_gates_each_portal() {
        let mut w = World::new(HeroClass::CyberRaccoon);
        w.killed = 20;
        w.spawned = 20;
        w.tick(Input::default());
        assert_eq!(w.boss_stages[0], BossStage::Alive);
        assert_eq!(w.boss().unwrap().kind, 60);
        assert!(!w.portal_available());
        w.enemies.clear();
        w.tick(Input::default());
        assert!(w.portal_available());
    }
    #[test]
    fn dash_and_shield_keep_their_original_costs() {
        let mut w = World::new(HeroClass::Huntress);
        w.tick(Input {
            dash: true,
            shield: true,
            ..Input::default()
        });
        assert_eq!(w.player.cooldowns[3], 60);
        assert_eq!(w.player.shield_cd, 479);
        assert!(w.player.mp < 71.0);
    }
    #[test]
    fn projectile_hits_use_each_projectiles_own_damage() {
        let mut w = World::new(HeroClass::Huntress);
        let x = w.player.x + 42.0;
        let y = w.floor(x);
        let mut e = w.enemy(x, 0, 1, 120.0, 3.0);
        e.y = y;
        w.enemies.push(e);
        // Dos proyectiles alcanzan el mismo enemigo en el mismo tick.
        w.projectiles.push(Projectile {
            x: x - 4.0,
            y: y - 12.0,
            vx: 4.0,
            vy: 0.0,
            damage: 10.0,
            radius: 3.0,
            life: 30,
            kind: ProjectileKind::Knife,
        });
        w.projectiles.push(Projectile {
            x: x - 4.0,
            y: y - 12.0,
            vx: 4.0,
            vy: 0.0,
            damage: 30.0,
            radius: 3.0,
            life: 30,
            kind: ProjectileKind::Knife,
        });
        w.update_projectiles();
        assert!(
            w.enemies[0].hp < 94.0,
            "both distinct projectiles must deal damage"
        );
        assert!(w.projectiles.is_empty());
    }
    #[test]
    fn projectile_opens_chest_without_changing_player_level() {
        let mut w = World::new(HeroClass::LunarMage);
        let x = w.chests[0][0].x;
        let y = w.floor(x) - 8.0;
        w.projectiles.push(Projectile {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            damage: 2.0,
            radius: 3.0,
            life: 30,
            kind: ProjectileKind::Orb,
        });
        w.update_projectiles();
        assert!(w.chests[0][0].opened);
        assert_eq!(w.player.level, 1);
        assert!(!w.pickups.is_empty());
    }

    #[test]
    fn king_hp_scale_and_combo_match_original_final_override() {
        let mut w = World::new(HeroClass::Huntress);
        w.switch_map(MapId::DarkForest, 70.0);
        w.spawn_boss();
        let boss = &w.enemies[0];
        assert_eq!(boss.kind, 62);
        assert_eq!(boss.scale, 1.4);
        assert_eq!(boss.base_speed, 0.65);
        let level = boss.level as f32;
        assert_eq!(boss.hp, ((25.0 + level * 18.0) * 17.0).round());
        w.enemies[0].x = w.player.x + 35.0;
        w.enemies[0].y = w.player.y;
        w.update_enemy_ai();
        assert_eq!(w.enemies[0].king_state, KingState::Combo);
        for _ in 0..12 {
            w.update_enemy_ai();
        }
        assert_eq!(w.enemies[0].king_pose, "s1");
    }
}
