use cursive::Cursive;

pub trait GetCursive {
    fn get_cursive(self) -> Option<Cursive>;
}

impl GetCursive for Cursive {
    fn get_cursive(self) -> Option<Cursive> {
        Some(self)
    }
}

impl GetCursive for Option<Cursive> {
    fn get_cursive(self) -> Option<Cursive> {
        self
    }
}
