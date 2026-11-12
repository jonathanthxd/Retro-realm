//! Versioned, validated profile preferences. Never touches the HTML app data.
use crate::layout::ControlPrefs;
use macroquad::prelude::vec2;
use serde::{Deserialize,Serialize};
use std::{collections::HashMap,fs::{self,File},io::{self,Write},path::PathBuf};

#[derive(Clone,Debug,Serialize,Deserialize,PartialEq)]
#[serde(default)]
pub struct Settings {
    pub schema_version:u32,
    pub player_name:String,
    /// Highest level ever reached, never the level of a new run.
    pub best_level:u16,
    pub selected_class:u8,
    pub music_volume:f32,
    pub sfx_volume:f32,
    pub music_style:u8,
    pub controls_joystick:bool,
    pub controls_scale:f32,
    pub control_individual:HashMap<String,f32>,
    pub control_offsets:HashMap<String,[f32;2]>,
}
impl Default for Settings {
    fn default()->Self{
        Self{
            schema_version:1,
            player_name:"Jugador".to_string(),best_level:1,selected_class:0,
            music_volume:0.70,sfx_volume:0.80,music_style:2,
            controls_joystick:false,controls_scale:1.35,
            control_individual:HashMap::new(),control_offsets:HashMap::new()
        }
    }
}
impl Settings {
    pub fn validate(&mut self) {
        self.schema_version=1;
        let cleaned:String=self.player_name.trim().chars()
            .filter(|c|!c.is_control()).take(14).collect();
        self.player_name=if cleaned.is_empty(){"Jugador".to_string()}else{cleaned};
        self.best_level=self.best_level.clamp(1,9999);
        self.selected_class=self.selected_class.min(2);
        self.music_volume=finite_clamp(self.music_volume,0.0,1.0,0.7);
        self.sfx_volume=finite_clamp(self.sfx_volume,0.0,1.0,0.8);
        self.music_style=self.music_style.min(2);
        self.controls_scale=finite_clamp(self.controls_scale,0.75,1.9,1.35);
        const KEYS:[&str;11]=["bl","br","bj","bm","bs","s0","s1","s2","s3","ap","pause"];
        self.control_individual.retain(|k,v|{
            if !KEYS.contains(&k.as_str()){return false;}
            *v=finite_clamp(*v,0.4,2.2,1.0);
            true
        });
        self.control_offsets.retain(|k,v|{
            if !KEYS.contains(&k.as_str()){return false;}
            v[0]=finite_clamp(v[0],-500.0,500.0,0.0);
            v[1]=finite_clamp(v[1],-500.0,500.0,0.0);
            true
        });
    }
    pub fn controls(&self)->ControlPrefs {
        ControlPrefs {
            global_scale:self.controls_scale,
            individual:self.control_individual.clone(),
            offsets:self.control_offsets.iter()
                .map(|(k,v)|(k.clone(),vec2(v[0],v[1]))).collect(),
            joystick:self.controls_joystick,
        }
    }
    pub fn update_controls(&mut self,ctrl:&ControlPrefs) {
        self.controls_scale=ctrl.global_scale;
        self.control_individual=ctrl.individual.clone();
        self.control_offsets=ctrl.offsets.iter().map(|(k,v)|(k.clone(),[v.x,v.y])).collect();
        self.controls_joystick=ctrl.joystick;
    }
    pub fn load()->Self{
        let mut settings=preferences_file().and_then(|p|fs::read_to_string(p).ok())
            .and_then(|s|serde_json::from_str::<Self>(&s).ok())
            .unwrap_or_default();
        settings.validate();
        settings
    }
    pub fn save(&self)->io::Result<()> {
        let Some(path)=preferences_file() else {return Err(io::Error::other("No settings dir"));};
        if let Some(parent)=path.parent(){fs::create_dir_all(parent)?;}
        let mut safe=self.clone();safe.validate();
        let serialized=serde_json::to_vec_pretty(&safe).map_err(io::Error::other)?;
        let temp=path.with_extension("tmp");
        {
            let mut f=File::create(&temp)?;
            f.write_all(&serialized)?;
            f.sync_all()?;
        }
        fs::rename(temp,path)?;
        Ok(())
    }
}
fn finite_clamp(v:f32,min:f32,max:f32,fallback:f32)->f32 {
    if v.is_finite(){v.clamp(min,max)}else{fallback}
}
fn preferences_file()->Option<PathBuf>{
    #[cfg(target_os="android")]
    {
        // The preview APK's app-private Android sandbox. A production change of
        // applicationId must update this path or replace it with JNI filesDir.
        Some(PathBuf::from("/data/data/com.pixelquest.game.preview/files/retro_realm_settings.json"))
    }
    #[cfg(not(target_os="android"))]
    {
        let base=std::env::var_os("APPDATA").map(PathBuf::from)
            .or_else(||std::env::var_os("XDG_CONFIG_HOME").map(PathBuf::from))
            .or_else(||std::env::var_os("HOME").map(|s|PathBuf::from(s).join(".config")))
            .or_else(||std::env::current_dir().ok())?;
        Some(base.join("RetroRealm").join("settings.json"))
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test] fn corrupt_json_does_not_change_defaults(){
        assert!(serde_json::from_str::<Settings>("{bad").is_err());
        let mut bad=Settings{player_name:" ".to_string(),music_volume:f32::NAN,
            controls_scale:99.0,best_level:0,selected_class:255,..Settings::default()};
        bad.validate();
        assert_eq!(bad.player_name,"Jugador");
        assert_eq!(bad.best_level,1);
        assert_eq!(bad.selected_class,2);
        assert_eq!(bad.music_volume,0.7);
        assert_eq!(bad.controls_scale,1.9);
    }
    #[test] fn settings_roundtrip(){
        let s=Settings{player_name:"Ñandú".into(),best_level:17,..Settings::default()};
        let json=serde_json::to_string(&s).unwrap();
        let parsed:Settings=serde_json::from_str(&json).unwrap();
        assert_eq!(parsed,s);
    }
    #[test] fn control_offsets_survive_roundtrip(){
        let mut c=ControlPrefs::default();
        c.offsets.insert("bm".to_string(),vec2(-21.0,-43.0));
        c.individual.insert("bm".to_string(),1.8);
        let mut s=Settings::default();
        s.update_controls(&c);
        let loaded:Settings=serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(loaded.controls().offsets.get("bm").unwrap().x,-21.0);
    }
}
