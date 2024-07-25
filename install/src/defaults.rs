pub const JSON_RPC_URL: &str = "http://api.devnet.lumos.com";

lazy_static! {
    pub static ref CONFIG_FILE: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".config", "lumos", "install", "config.yml"]);
            path.to_str().unwrap().to_string()
        })
    };
    pub static ref USER_KEYPAIR: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".config", "lumos", "id.json"]);
            path.to_str().unwrap().to_string()
        })
    };
    pub static ref DATA_DIR: Option<String> = {
        dirs_next::home_dir().map(|mut path| {
            path.extend([".local", "share", "lumos", "install"]);
            path.to_str().unwrap().to_string()
        })
    };
}
