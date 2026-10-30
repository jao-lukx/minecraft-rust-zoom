use super::rust_zoom_core::Zoom;
use super::{
    Game,
    native_hooks::{NativeMod, NativeZoom},
};
use std::cell::Cell;
thread_local! { static STATE: Cell<Zoom> = Cell::new(Zoom::default()); }
const ID: &str = "rust_zoom";
impl Game<'_> {
    pub(super) fn rust_zoom_frame(&mut self) {
        if !self.native_window() {
            return;
        }
        if !self.native_on(ID) || !self.native_playing() || !self.focused {
            STATE.with(|s| {
                let mut z = s.get();
                z.held = false;
                s.set(z);
            });
        }
    }
    pub(super) fn rust_zoom_key(&mut self, key: &str, pressed: bool) {
        if key != self.native_mod_key(ID, "zoom", "key.keyboard.c") {
            return;
        }
        STATE.with(|s| {
            let mut z = s.get();
            z.held = pressed && self.native_on(ID) && self.native_playing() && self.focused;
            s.set(z);
        });
    }
    pub(super) fn rust_zoom_wheel(&mut self, steps: f32, claimed: &mut bool) {
        if *claimed || !self.native_on(ID) || !self.native_playing() || !self.focused {
            return;
        }
        STATE.with(|s| {
            let mut z = s.get();
            if z.held {
                z.wheel(steps);
                *claimed = true;
                s.set(z);
            }
        });
    }
    pub(super) fn rust_zoom_view(&self, zoom: &mut Option<NativeZoom>) {
        if zoom.is_some() || !self.native_on(ID) || !self.native_playing() || !self.focused {
            return;
        }
        STATE.with(|s| {
            let z = s.get();
            if z.held {
                *zoom = Some(NativeZoom {
                    factor: z.factor,
                    smooth: true,
                    turn: z.sensitivity(),
                });
            }
        });
    }
    pub(super) fn rust_zoom_list(&mut self, mods: &mut Vec<NativeMod>) {
        mods.push(NativeMod {
            id: ID.into(),
            title: "Zoom Rust 0.1.0".into(),
            description: "Segure C; ajuste com a roda.".into(),
            keys: vec![["zoom".into(), "Zoom".into(), "key.keyboard.c".into()]],
        });
    }
}
