#[derive(Clone, Copy)]
pub struct Zoom {
    pub held: bool,
    pub factor: f32,
}
impl Default for Zoom {
    fn default() -> Self {
        Self {
            held: false,
            factor: 4.0,
        }
    }
}
impl Zoom {
    pub fn wheel(&mut self, steps: f32) {
        if steps.is_finite() {
            self.factor = (self.factor * 1.15_f32.powf(steps.clamp(-20.0, 20.0))).clamp(1.25, 10.0);
        }
    }
    pub fn sensitivity(&self) -> f32 {
        (1.0 / self.factor).clamp(0.1, 1.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wheel_is_bounded_and_reversible() {
        let mut z = Zoom::default();
        z.wheel(2.0);
        z.wheel(-2.0);
        assert!((z.factor - 4.0).abs() < 0.001);
        z.wheel(1000.0);
        assert_eq!(z.factor, 10.0);
        z.wheel(f32::NAN);
        assert_eq!(z.factor, 10.0);
        for _ in 0..10 {
            z.wheel(-20.0);
        }
        assert_eq!(z.factor, 1.25);
    }
}
