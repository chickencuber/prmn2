use std::sync::{Arc, Mutex};

use cursive::{View, view::ViewWrapper};

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Category,
}

pub trait Modeable: View + Sized {
    fn with_mode(self, mode: Mode) -> ModeView {
        ModeView::new(mode, self)
    }
}

impl<T: View+Sized> Modeable for T {}

///Type erasure
pub struct ModeView {
    view: Arc<Mutex<dyn View>>,
    pub mode: Mode,
}

impl ViewWrapper for ModeView {
    type V = dyn View;
    fn with_view<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Self::V) -> R,
    {
        self.view.try_lock().map(|v| f(&*v)).ok()
    }

    fn with_view_mut<F, R>(&mut self, f: F) -> Option<R>
    where
        F: FnOnce(&mut Self::V) -> R,
    {
        self.view.try_lock().map(|mut v| f(&mut *v)).ok()
    }
}

impl ModeView {
    pub fn new<T: View>(mode: Mode, view: T) -> Self {
        Self {
            view: Arc::new(Mutex::new(view)),
            mode,
        }
    }
}

