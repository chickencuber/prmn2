use cursive::Cursive;

use crate::{
    cmd::{
        Commands,
        shared::{Conf, output},
        start,
    },
    data::Data,
};

pub fn last(cmd: Commands, mut siv: Cursive) -> Option<Cursive> {
    let conf = siv.user_data::<Data>().unwrap();
    let c = conf.last.clone();
    if let Some(last) = c {
        if last.is_dir() {
            output(Conf::Data(conf), cmd.out, last.to_string_lossy());
            return None;
        }
    }
    Some(start(cmd, siv))
}
