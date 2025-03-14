use std::{env, fs, path};

fn get_home_dir() -> Option<path::PathBuf> {
    let home_dir = home::home_dir()?;
    Some(home_dir)
}

fn get_config_dir() -> Option<path::PathBuf> {
    // first try $FEND_CONFIG_DIR
    if let Some(env_var_config_dir) = env::var_os("FEND_CONFIG_DIR") {
        return Some(path::PathBuf::from(env_var_config_dir));
    }

    // otherwise try $XDG_CONFIG_HOME/fend/
    if let Some(env_var_xdg_config_dir) = env::var_os("XDG_CONFIG_HOME") {
        let mut res = path::PathBuf::from(env_var_xdg_config_dir);
        res.push("fend");
        return Some(res);
    }

    // otherwise use $HOME/.config/fend/
    let mut res = get_home_dir()?;
    res.push(".config");
    res.push("fend");
    Some(res)
}

pub fn get_config_file_location() -> Option<path::PathBuf> {
    let mut config_path = get_config_dir()?;
    config_path.push("config.toml");
    Some(config_path)
}

fn get_history_dir() -> Option<path::PathBuf> {
    // first try $FEND_STATE_DIR
    if let Some(env_var_history_dir) = env::var_os("FEND_STATE_DIR") {
        return Some(path::PathBuf::from(env_var_history_dir));
    }

    // otherwise try $XDG_STATE_HOME/fend/
    if let Some(env_var_xdg_state_dir) = env::var_os("XDG_STATE_HOME") {
        let mut res = path::PathBuf::from(env_var_xdg_state_dir);
        res.push("fend");
        return Some(res);
    }

    // otherwise use $HOME/.local/state/fend/
    let mut res = get_home_dir()?;
    res.push(".local");
    res.push("state");
    res.push("fend");
    Some(res)
}

pub fn get_history_file_location() -> Option<path::PathBuf> {
    let mut history_path = get_history_dir()?;
    match fs::create_dir_all(history_path.as_path()) {
        Ok(_) => (),
        Err(_) => return None,
    }
    history_path.push("history");
    Some(history_path)
}
