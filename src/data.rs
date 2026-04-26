use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, os::unix::fs::PermissionsExt, path::PathBuf};
use ron::{
    de::from_str,
    ser::{PrettyConfig, to_string_pretty},
};

use dirs::{config_dir, data_dir};

fn get_dir<T: Fn() -> Option<PathBuf>>(f: T) -> PathBuf {
    let mut s = f().unwrap();
    s.push("prmn");
    return s;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub editor: String,
    pub categories: HashMap<String, Category>,
    #[serde(skip)]
    pub project_types: Vec<String>,
    #[serde(skip)]
    pub last: Option<PathBuf>,
}
impl Data {
    fn new_default() -> Self {
        Self {
            editor: "nvim".to_string(),
            categories: HashMap::new(),
            project_types: vec!["Blank".to_string()],
            last: None,
        }
    }
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let mut conf = get_dir(config_dir);
        conf.push("data.ron");
        fs::write(&conf, to_string_pretty(self, PrettyConfig::default())?)?;

        if let Some(last) = &self.last {
            let mut data = get_dir(data_dir);
            data.push("last");
            fs::write(&data, last.to_str().unwrap())?;
        }
        Ok(())
    }
    pub fn new() -> Result<Self, anyhow::Error> {
        let conf = get_dir(config_dir);
        if !conf.exists() {
            let mut m = conf.clone();
            m.push("types");
            fs::create_dir_all(&m)?;
            m.push("Blank.sh");
            fs::write(&m, "#!/bin/bash\ngit init")?;
            let mut perms = fs::metadata(&m)?.permissions();
            perms.set_mode(0o755);
            return Ok(Self::new_default());
        }
        let mut config = conf.clone();
        config.push("data.ron");
        if !config.exists() {
            return Ok(Self::new_default());
        }
        let mut s = from_str::<Self>(&fs::read_to_string(&config)?)?;
        let mut data = get_dir(data_dir);
        if !data.exists() {
            fs::create_dir_all(&data)?;
        }
        data.push("last");
        s.last = fs::read_to_string(&data).map(|v| PathBuf::from(v)).ok();
        if s.last.is_some() {
            if !fs::exists(s.last.as_ref().unwrap())? {
                s.last = None;
            }
        }
        config.pop();
        config.push("types");
        let types = fs::read_dir(&config)?;
        for file in types {
            let file = file?;
            let mut name = file.file_name().into_string().unwrap();
            if !name.ends_with(".sh") {
                continue;
            }
            let perms = file.metadata().unwrap().permissions();
            let is_ext = perms.mode() & 0o111 != 0;
            if !is_ext {
                continue;
            }
            name.pop();
            name.pop();
            name.pop();
            s.project_types.push(name);
        }
        return Ok(s);
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub supported_types: Vec<String>,
    pub parent_dir: PathBuf,
}
