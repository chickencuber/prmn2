use cursive::CursiveExt;

use crate::{cmd::{Commands, shared::{Conf, output}, start}, data::Data};

pub fn last(cmd: Commands) {
    let mut conf = Data::new().expect("failed to load config");
    let c = conf.last.clone();
    if let Some(last) = c {
        if last.is_dir() {
            output(Conf::Data(&mut conf), cmd.out, last.to_string_lossy());
            return;
        }
    }
    start(cmd, Some(conf)).run();
}
