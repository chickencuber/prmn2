use ron::{
    de::from_str,
    ser::{PrettyConfig, to_string_pretty},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, os::unix::fs::PermissionsExt, path::PathBuf};

use dirs::{config_dir, data_dir};

fn get_dir<T: Fn() -> Option<PathBuf>>(f: T) -> PathBuf {
    let mut s = f().unwrap();
    s.push("prmn");
    return s;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Data {
    pub categories: HashMap<String, Category>,
    #[serde(default = "default_editor", skip_serializing_if = "editor_is_default")]
    pub editor: String,
    #[serde(default = "bool_true", skip_serializing_if = "bool_is_true")]
    pub show_menubar: bool,
    #[serde(default = "bool_true", skip_serializing_if = "bool_is_true")]
    pub show_hint: bool,

    #[serde(skip)]
    pub project_types: Vec<String>,
    #[serde(skip)]
    pub last: Option<PathBuf>,
}

fn default_editor() -> String {
    "nvim".to_string()
}

fn editor_is_default(s: &String) -> bool {
    s == "nvim"
}
fn bool_true() -> bool {
    true
}
fn bool_is_true(b: &bool) -> bool {
    return *b;
}

impl Data {
    pub fn types_dir() -> PathBuf {
        let mut path = get_dir(config_dir);
        path.push("types");
        return path;
    }
    fn new_default() -> Self {
        Self {
            editor: "nvim".to_string(),
            categories: HashMap::new(),
            project_types: vec!["Blank".to_string()],
            last: None,
            show_hint: true,
            show_menubar: true,
        }
    }
    pub fn save(&self, save_config: bool) -> Result<(), anyhow::Error> {
        if save_config {
            let mut conf = get_dir(config_dir);
            conf.push("data.ron");
            fs::write(&conf, to_string_pretty(self, PrettyConfig::default())?)?;
        }
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
            let d = Self::new_default();
            d.save(true)?;
            return Ok(d);
        }
        let mut config = conf.clone();
        config.push("data.ron");
        if !config.exists() {
            let d = Self::new_default();
            d.save(true)?;
            return Ok(d);
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
        for (_, cat) in &mut s.categories {
            cat.types.retain(|t| s.project_types.contains(t));
        }
        return Ok(s);
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Category {
    pub types: Vec<String>,
    pub dir: PathBuf,
}
