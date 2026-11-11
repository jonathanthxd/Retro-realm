//! Efectos sintetizados sin samples de terceros. Inspirados en el WebAudio
//! original, pero aún requieren comparación auditiva para paridad exacta.
use std::f32::consts::TAU;
use macroquad::audio::{load_sound_from_bytes,play_sound,PlaySoundParams,Sound};

#[derive(Clone,Copy,Debug)]
pub enum Cue {Jump, Attack, Cast, Dash, Shield, Hurt, Kill, Level, Portal, Boss, Chest}
impl Cue {
    fn index(self)->usize {self as usize}
}
pub struct Sfx { clips:Vec<Option<Sound>> }
impl Sfx {
    pub async fn load()->Self {
        let mut clips=Vec::new();
        // Frecuencia inicial, final, duración en segundos y forma de onda.
        let definitions=[
            (450.0,650.0,0.12,0),   // jump
            (240.0,75.0,0.09,2),    // attack
            (730.0,350.0,0.20,0),   // cast
            (120.0,810.0,0.14,1),   // dash
            (300.0,190.0,0.26,0),   // shield
            (200.0,70.0,0.25,2),    // hurt
            (580.0,300.0,0.16,1),   // kill
            (420.0,990.0,0.36,0),   // level
            (270.0,780.0,0.34,0),   // portal
            (140.0,65.0,0.60,1),    // boss
            (670.0,930.0,0.19,0),   // chest
        ];
        for &(a,b,d,w) in &definitions {
            clips.push(load_sound_from_bytes(&wav_tone(a,b,d,w)).await.ok());
        }
        Self{clips}
    }
    pub fn play(&self,cue:Cue){
        if let Some(Some(sound))=self.clips.get(cue.index()){
            play_sound(sound,PlaySoundParams{looped:false,volume:0.24});
        }
    }
}
fn wav_tone(start:f32,end:f32,duration:f32,wave:u8)->Vec<u8> {
    let sample_rate=22050u32;
    let samples=(duration*sample_rate as f32) as u32;
    let data_len=samples*2;
    let mut bytes=Vec::with_capacity(44+data_len as usize);
    bytes.extend_from_slice(b"RIFF");
    bytes.extend_from_slice(&(36+data_len).to_le_bytes());
    bytes.extend_from_slice(b"WAVEfmt ");
    bytes.extend_from_slice(&16u32.to_le_bytes()); // fmt size
    bytes.extend_from_slice(&1u16.to_le_bytes());  // PCM
    bytes.extend_from_slice(&1u16.to_le_bytes());  // mono
    bytes.extend_from_slice(&sample_rate.to_le_bytes());
    bytes.extend_from_slice(&(sample_rate*2).to_le_bytes()); // bytes/sec
    bytes.extend_from_slice(&2u16.to_le_bytes()); // frame size
    bytes.extend_from_slice(&16u16.to_le_bytes()); // bit depth
    bytes.extend_from_slice(b"data");
    bytes.extend_from_slice(&data_len.to_le_bytes());
    let mut phase=0.0f32;
    let mut noise=19421u32;
    for i in 0..samples {
        let t=i as f32/samples as f32;
        let freq=start+(end-start)*t;
        phase +=TAU*freq/sample_rate as f32;
        noise ^=noise<<13; noise ^=noise>>17; noise ^=noise<<5;
        let v=match wave {
            1=>if phase.sin()>0.0 {1.0}else{-1.0},
            2=>(noise as f32/u32::MAX as f32)*2.0-1.0,
            _=>phase.sin()
        };
        let envelope=(1.0-t).powf(1.8);
        let amp=(v*envelope*0.28*i16::MAX as f32) as i16;
        bytes.extend_from_slice(&amp.to_le_bytes());
    }
    bytes
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]fn wav_is_pcm_mono() {
        let wave=wav_tone(300.0,400.0,0.2,0);
        assert!(wave.len()>44);
        assert_eq!(&wave[0..4],b"RIFF");
        assert_eq!(&wave[8..12],b"WAVE");
        assert_eq!(&wave[36..40],b"data");
        assert_eq!((wave.len()-44)%2,0);
    }
}
