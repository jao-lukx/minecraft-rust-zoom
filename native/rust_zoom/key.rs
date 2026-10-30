fn hook(game: &mut crate::game::Game<'_>, key: &str, pressed: bool) {
    game.rust_zoom_key(key, pressed);
}
