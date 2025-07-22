use anyhow::anyhow;
use directories_next::ProjectDirs;
use murack_core_app::Config;

pub fn load_config() -> anyhow::Result<Config> {
    let proj_dirs = ProjectDirs::from("", "murack", "murack-sync").ok_or_else(|| {
        anyhow!(
            "Failed to determine config directory path. \
           This usually happens when the HOME environment variable is not 
  set. \
           Config file should be located at 
  ~/.config/murack-sync/config.toml"
        )
    })?;

    let config_dir = proj_dirs.config_dir();
    // ~/.config/murack-sync/config.toml
    let config_file_path = config_dir.join("config.toml");

    Config::load(&config_file_path)
}
