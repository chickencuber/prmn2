use std::{fs, os::unix::process::CommandExt, path::PathBuf, process::Command, sync::Mutex};

use cursive::{
    Cursive, View,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, OnEventView, SelectView},
};

use crate::{
    cmd::Commands,
    data::{Category, Data},
    ui,
};

pub enum Conf<'a> {
    Cursive(&'a mut Cursive),
    Data(&'a mut Data),
}

static OUTPUT: Mutex<String> = Mutex::new(String::new());

pub fn print_output() {
    let out = OUTPUT.lock().unwrap();
    if *out == "" {
        return;
    }
    print!("{out}");
}

pub fn output<T: ToString>(mut conf: Conf, out: bool, v: T) {
    match &mut conf {
        Conf::Cursive(siv) => {
            let mut conf = siv.user_data::<Data>().unwrap().clone();
            conf.last = Some(PathBuf::from(v.to_string()));
            conf.save().expect("failed to save");

            if out {
                siv.quit();
                let mut out = OUTPUT.lock().unwrap();
                *out = v.to_string();
                return;
            }
            let _ = Command::new(&conf.editor).arg(v.to_string()).exec();
            siv.quit();
        }
        Conf::Data(conf) => {
            conf.last = Some(PathBuf::from(v.to_string()));
            conf.save().expect("failed to save");
            if out {
                println!("{}", v.to_string());
                return;
            }
            let _ = Command::new(&conf.editor).arg(v.to_string()).exec();
        }
    }
}

pub fn use_category(
    cat: &Category,
    cmd: &Commands,
    name: String,
) -> Result<impl View, anyhow::Error> {
    let out = cmd.out;
    let mut select = SelectView::new().on_submit(move |siv, v: &PathBuf| {
        output(Conf::Cursive(siv), out, v.to_string_lossy());
    });
    for file in fs::read_dir(&cat.dir)? {
        let path = file?.path();
        if !path.is_dir() {
            continue;
        }
        let name = path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        select.add_item(name, path);
    }
    select.sort_by_label();
    let select = select.with_name("selector");
    let cate = cat.clone();
    let types = cate.types.clone();
    return Ok(OnEventView::new(select.scrollable())
        .on_event('f', move |siv| {
            let n = name.clone();
            let select = ui::fuzzy_picker(
                get_all_files(siv.user_data().unwrap(), Some(&cate))
                    .expect("couldn't get the files"),
                move |siv, e| {
                    let conf = siv.user_data::<Data>().unwrap();

                    let mut path = conf.categories[&n].dir.clone();
                    path.push(e);
                    output(Conf::Cursive(siv), out, path.to_string_lossy());
                },
            );
            siv.add_layer(
                Dialog::new()
                    .content(select)
                    .title(format!("Search : {}", name)),
            );
        })
        .on_event('a', move |siv| {
            //TASK(20260427-141604-587-n6-239): finish logic for adding dialogs
            let mut select = SelectView::new();
            for ty in &types {
                select.add_item(ty, ty.clone());
            }
            siv.add_layer(Dialog::new().content(select).title("Create"));
        })
        .full_screen());
}

pub fn get_all_files(conf: &Data, cat: Option<&Category>) -> Result<Vec<String>, anyhow::Error> {
    let mut v = vec![];
    if let Some(cat) = cat {
        for file in fs::read_dir(&cat.dir)? {
            let path = file?.path();
            if !path.is_dir() {
                continue;
            }
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            v.push(name);
        }
        return Ok(v);
    }

    for (k, cat) in &conf.categories {
        for file in fs::read_dir(&cat.dir)? {
            let path = file?.path();
            if !path.is_dir() {
                continue;
            }
            let name = path
                .clone()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            v.push(format!("{}/{}", k, name,));
        }
    }
    return Ok(v);
}
