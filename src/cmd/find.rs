use crate::{cmd::Commands, data::Data};


pub fn find(mut args: Vec<String>) {
    args.pop(); //remove the command name
    let cmd = Commands::from(args);
    let conf = Data::new().expect("failed to load config");
}
