pub mod path {
    use std::path::{Path, PathBuf};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref BASE_DIR: PathBuf = Path::new("resources/").to_owned();
        pub static ref GAME_CONFIG_FILE_LOC: PathBuf = BASE_DIR.join("cfg/config.ron").to_owned();
        pub static ref GAME_SHADER_BASE_DIR: PathBuf = BASE_DIR.join("shaders/").to_owned();
        pub static ref GAME_ASSETS_BASE_DIR: PathBuf = BASE_DIR.join("assets/").to_owned();
        pub static ref GAME_LEVELS_CFG_NAME: PathBuf = Path::new("level.ron").to_owned();
        pub static ref GAME_LEVELS_BASE_DIR: PathBuf = BASE_DIR.join("levels/").to_owned();
        pub static ref GAME_LEVELS_DATA_FILE: PathBuf = Path::new("world.dat").to_owned();
    }
}