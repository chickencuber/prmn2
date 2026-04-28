use std::{fs, os::unix::process::CommandExt, path::PathBuf, process::Command, sync::Mutex};

use cursive::{
    Cursive, View,
    view::{Nameable, Resizable, Scrollable},
    views::{Dialog, EditView, OnEventView, SelectView, TextView},
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
            let conf = siv.user_data::<Data>().unwrap();
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
    let select = select.with_name("selector-category-name");
    let cate = cat.clone();
    let types = cate.types.clone();
    let dir = cate.dir.clone();
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
        .on_event('d', |siv| {
            let s = siv
                .call_on_name(
                    "selector-category-name",
                    |view: &mut SelectView<PathBuf>| {
                        let p = view.selection().unwrap();
                        let name = p.file_name().unwrap().to_string_lossy().to_string();
                        return (name, p);
                    },
                )
                .unwrap();
            let (name, path) = s;
            siv.add_layer(
                Dialog::new()
                    .title("Confirm")
                    .content(TextView::new(format!(
                        "Are you sure you want to delete {}?",
                        name,
                    )))
                    .button("Yes", move |siv| {
                        siv.pop_layer();
                        fs::remove_dir_all(path.as_ref()).expect("failed to delete file");
                        siv.call_on_name("selector-category-name", |selector: &mut SelectView<PathBuf>| {
                            let s = selector.selected_id().unwrap();
                            selector.remove_item(s);
                        });
                    })
                    .button("No", |siv| {
                        siv.pop_layer();
                    }),
            )
        })
        .on_event('a', move |siv| {
            let d = dir.clone();
            //TASK(20260427-141604-587-n6-239): finish logic for adding dialogs
            if types.len() == 1 {
                add_project(siv, Some(types[0].clone()), d, out);
            } else if types.len() == 0 {
                add_project(siv, None, d, out);
            } else {
                let mut select = SelectView::new();
                for ty in &types {
                    select.add_item(ty, ty.clone());
                }
                let select = select
                    .on_submit(move |siv, item: &String| {
                        siv.pop_layer();
                        add_project(siv, Some(item.clone()), d.clone(), out);
                    })
                    .scrollable();
                siv.add_layer(Dialog::new().content(select).title("Create"));
            }
        })
        .full_screen());
}

fn add_project(siv: &mut Cursive, ty: Option<String>, dir: PathBuf, out: bool) {
    let input = EditView::new().on_submit(move |siv, val| {
        let val = val.trim();
        if val == "" {
            siv.pop_layer();
        }
        siv.pop_layer();
        let mut dir = dir.clone();
        dir.push(val);
        fs::create_dir(&dir).unwrap();
        let mut types = Data::types_dir();
        types.push(format!(
            "{}.sh",
            ty.as_ref().unwrap_or(&"Blank".to_string())
        ));
        let child = Command::new(types)
            .current_dir(&dir)
            .output()
            .expect("failed to run command");
        if !child.status.success() {
            fs::remove_dir_all(&dir).unwrap();
            siv.add_layer(
                Dialog::new()
                    .content(TextView::new(String::from_utf8_lossy(&child.stderr)).scrollable())
                    .title("Error")
                    .button("Ok", |siv| {
                        siv.pop_layer();
                    }),
            );
        } else {
            let val = val.to_string();
            siv.call_on_name("selector-category-name", |s: &mut SelectView<PathBuf>| {
                s.add_item(&val, dir.clone());
                s.sort_by_label();
            });
            siv.add_layer(
                Dialog::new()
                    .title("Open")
                    .button("Yes", move |siv| {
                        output(Conf::Cursive(siv), out, dir.to_str().unwrap());
                        siv.pop_layer();
                    })
                    .button("No", |siv| {
                        siv.pop_layer();
                    }),
            );
        }
    });
    siv.add_layer(
        Dialog::new()
            .content(input)
            .title("Name")
            .button("Cancel", |siv| {
                siv.pop_layer();
            }),
    );
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
