fn hook(game: &mut crate::game::Game<'_>, steps: f32, claimed: &mut bool) {
    game.rust_zoom_wheel(steps, claimed);
}
