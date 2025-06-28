use std::io::Cursor;
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend,
    sound::static_sound::{StaticSoundData},
};

pub enum SoundType {
    Blip,
    SuperBlip,
    SuperBang,
}

pub struct SoundPlayer {
    manager: AudioManager,
    // sounds for different sound types
    blip: StaticSoundData,
    super_blip: StaticSoundData,
    super_bang: StaticSoundData,
}

impl SoundPlayer {

    pub fn new() -> Self{
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        let sound = StaticSoundData::from_cursor(Cursor::new(include_bytes!("../assets/blip.ogg"))).unwrap();

        let mut blip = sound.clone();
        blip = blip.reverse(true);

        let mut super_blip = sound.clone();
        super_blip = super_blip.volume(5.0);

        let mut super_bang = sound.clone();
       super_bang = super_bang.volume(2.5);

        SoundPlayer {
            manager,
            blip,
            super_blip,
            super_bang,
        }
    }

    pub fn play(&mut self, sound_type: SoundType) {
        let sound = match sound_type {
            SoundType::Blip => &self.blip,
            SoundType::SuperBlip => &self.super_blip,
            SoundType::SuperBang => &self.super_bang,
        };
        let _ = self.manager.play(sound.clone());

    }
}