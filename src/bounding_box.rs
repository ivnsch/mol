use core::f32;

#[derive(Debug)]
pub struct BoundingBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl BoundingBox {
    pub fn max_distance(&self) -> f32 {
        let x_len = self.max_x - self.min_x;
        let y_len = self.max_y - self.min_y;
        let z_len = self.max_z - self.min_z;

        (x_len.powf(2.) + y_len.powf(2.) + z_len.powf(2.)).sqrt()
    }

    pub fn dist_x(&self) -> f32 {
        self.max_x - self.min_x
    }
    pub fn dist_y(&self) -> f32 {
        self.max_y - self.min_y
    }
    pub fn dist_z(&self) -> f32 {
        self.max_z - self.min_z
    }
    /// center point in absolute coords
    pub fn mid_x(&self) -> f32 {
        self.min_x + self.dist_x() / 2.
    }
    /// center point in absolute coords
    pub fn mid_y(&self) -> f32 {
        self.min_y + self.dist_y() / 2.
    }
    /// center point in absolute coords
    pub fn mid_z(&self) -> f32 {
        self.min_z + self.dist_z() / 2.
    }
}

pub fn bounding_box_for(vertices: &[[f32; 3]]) -> BoundingBox {
    let mut min_x = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;

    let mut min_y = f32::INFINITY;
    let mut max_y = f32::NEG_INFINITY;

    let mut min_z = f32::INFINITY;
    let mut max_z = f32::NEG_INFINITY;

    for v in vertices {
        if v[0] < min_x {
            min_x = v[0];
        };
        if v[0] > max_x {
            max_x = v[0];
        };
        if v[1] < min_y {
            min_y = v[1];
        };
        if v[1] > max_y {
            max_y = v[1];
        };
        if v[2] < min_z {
            min_z = v[2];
        };
        if v[2] > max_z {
            max_z = v[2];
        };
    }
    BoundingBox {
        min_x,
        max_x,
        min_y,
        max_y,
        min_z,
        max_z,
    }
}

#[cfg(test)]
mod test {
    use super::BoundingBox;

    #[test]
    fn test_bounding_box_max_dist() {
        let bounding_box = BoundingBox {
            min_x: -2.5,
            max_x: 2.5,
            min_y: -4.4,
            max_y: 10.,
            min_z: -4.2,
            max_z: 2.4,
        };

        assert_eq!(16.61084, bounding_box.max_distance());
    }
}
