fn hook(game: &crate::game::Game<'_>, zoom: &mut Option<crate::game::native_hooks::NativeZoom>) {
    game.rust_zoom_view(zoom);
}
