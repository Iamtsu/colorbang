use kira::{
    manager::{
        AudioManager, AudioManagerSettings,
        backend::cpal::CpalBackend,
    },
    sound::static_sound::{StaticSoundData, StaticSoundSettings},
};

pub struct SoundPlayer {
    manager: AudioManager,
    sound: StaticSoundData,
}

impl SoundPlayer {

    pub fn new() -> Self{
        let manager = AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap();
        let sound = StaticSoundData::from_file("assets/blip.ogg", StaticSoundSettings::default()).unwrap();
        SoundPlayer {
            manager,
            sound
        }
    }

    pub fn play(&mut self) {
        self.manager.play(self.sound.clone()).unwrap();
    }
}