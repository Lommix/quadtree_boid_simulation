use super::coord::Coord;

#[derive(Debug, Clone)]
pub struct Region {
    pub min: Coord,
    pub max: Coord,
}

impl Region {
    pub fn new(min: Coord, max: Coord) -> Self {
        Self { min, max }
    }

    pub fn with_margin(&self, margin: i32) -> Self {
        Self::new(
            Coord::new(self.min.x - margin, self.min.y - margin),
            Coord::new(self.max.x + margin, self.max.y + margin),
        )
    }

    pub fn intersects(&self, other: &Region) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
    }

    pub fn size_f32(&self) -> (f32, f32) {
        (
            (self.max.x - self.min.x) as f32,
            (self.max.y - self.min.y) as f32,
        )
    }

    pub fn into_f32(&self) -> (f32, f32, f32, f32) {
        (
            self.min.x as f32,
            self.min.y as f32,
            self.max.x as f32,
            self.max.y as f32,
        )
    }

    pub fn quad_divide(&self) -> [Region; 4] {
        let diag = self.max - self.min;
        let width = diag.project_x();
        let height = diag.project_y();
        [
            Region::new(self.min + height / 2, self.max - width / 2),
            Region::new(self.min + diag / 2, self.max),
            Region::new(self.min, self.min + diag / 2),
            Region::new(self.min + width / 2, self.max - height / 2),
        ]
    }
}
