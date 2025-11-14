use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};
use std::io::Cursor;

use anyhow::{ Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub enum SoundType {
    Fire,
    Explode,
    MultiFire,
    Wave,
    Load,
}

pub struct SoundPlayer {
    manager: AudioManager,
    // sounds for different sound types
    sounds: HashMap<String, StaticSoundData>,
}

fn read_assets_dir() -> Result<Vec<PathBuf>> {
    let mut ogg_files:Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir("assets") {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.as_path().extension().and_then(|s| s.to_str()) == Some("ogg") {
                ogg_files.push(path);
            }
        }
    }
    Ok(ogg_files)
}

fn random_name(prefix: &str, min: u8, max: u8) -> String {
    let index = min + rand::random::<u8>() % (max - min);
    format!("{}{:02}", prefix, index)
}

impl SoundPlayer {
    pub fn new() -> Result<Self> {
        let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

        // load sounds from assets returning a vector of paths
        if let Ok(ogg_files) = read_assets_dir() {
            let mut sounds = HashMap::new();
            for path in &ogg_files {
                println!("Found sound file: {}", path.display());
                let file_name = path.file_stem().unwrap().to_string_lossy().to_string();
                let data = fs::read(path).unwrap();
                let sound_data = StaticSoundData::from_cursor(Cursor::new(data)).unwrap();
                sounds.insert(file_name, sound_data);
            }
            Ok(SoundPlayer { manager, sounds })
        } else {
            Err(anyhow::anyhow!("Failed to read assets directory"))
        }
    }

    pub fn play(&mut self, sound_type: SoundType) {
        // select a random sound from self.sounds based on the sound type
        let (sound_name, volume) = match sound_type {
            SoundType::Fire => (random_name("Laser_", 7, 10), -6.0),
            SoundType::Explode => (random_name("Laser_", 3, 7), 1.0),
            SoundType::MultiFire => (random_name("Laser_", 0, 3), -8.0),
            SoundType::Wave => (random_name("incoming_", 0, 5), 10.0),
            SoundType::Load => ("part".to_string(), -8.0),
        };
       
        let sound = match self.sounds.get(&sound_name) {
            Some(sound) => sound.clone().volume(volume),
            None => {
                eprintln!("Sound not found: {}", sound_name);
                return;
            }
        };
        let _ = self.manager.play(sound);
    }
}
