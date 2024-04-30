use resvg::tiny_skia;

pub enum FitTo {
    /// Zoom by factor.
    Zoom(f32),
}

impl FitTo {
    pub fn fit_to_size(&self, size: tiny_skia::IntSize) -> Option<tiny_skia::IntSize> {
        match *self {
            FitTo::Zoom(z) => size.scale_by(z),
        }
    }

    pub fn fit_to_transform(&self, size: tiny_skia::IntSize) -> tiny_skia::Transform {
        let size1 = size.to_size();
        let size2 = match self.fit_to_size(size) {
            Some(v) => v.to_size(),
            None => return tiny_skia::Transform::default(),
        };
        tiny_skia::Transform::from_scale(
            size2.width() / size1.width(),
            size2.height() / size1.height(),
        )
    }
}

pub fn calculate_scaling_factor(x1: f32, y1: f32, x2: f32) -> f32 {
    if x1 == 0.0 || y1 == 0.0 {
        panic!("Neither x1 nor y1 can be zero to avoid division by zero!");
    }
    let kx = x2 / x1;

    kx
}