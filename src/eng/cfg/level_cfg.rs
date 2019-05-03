use crate::{
    eng::{
        utils::{
            constants
        }
    }
};
use std::{
    fs::{
        File
    },
    io::{
        Write
    },
    path::{
        Path,
        PathBuf
    }
};
use serde::{
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LevelConfig {
    pub name: String,
    pub level_dir: PathBuf
}

impl LevelConfig {
    pub fn load(level_dir: PathBuf) -> Result<Self, ron::de::Error> {
        assert!(level_dir.is_dir());
        let config_file = File::open(level_dir.join(constants::path::GAME_LEVELS_CFG_NAME.as_path())).map_err(|err| ron::de::Error::from(err))?;
        ron::de::from_reader::<File, Self>(config_file)
    }
    pub fn save(&self) {
        let mut config_file = File::create(self.level_dir.join(constants::path::GAME_LEVELS_CFG_NAME.as_path())).expect("Could not create folder for the level config file");
        let content = ron::ser::to_string_pretty(&self, Default::default()).expect("Could not serialise the configuration for the level");
        config_file.write_all(content.as_bytes()).expect("Could not save the configuration file for the level");
    }
    pub fn world_data_path(&self) -> PathBuf {
        self.level_dir.join(constants::path::GAME_LEVELS_DATA_FILE.as_path())
    }
}