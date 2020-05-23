use nalgebra::Vector3;

#[derive(Debug)]
pub struct BoundingBox {
    min: Vector3<f32>,
    max: Vector3<f32>,
}

#[derive(Debug)]
pub struct Triangle {
    pub a: Vector3<f32>,
    pub b: Vector3<f32>,
    pub c: Vector3<f32>,
}

impl Triangle {
    pub fn new(a: Vector3<f32>, b: Vector3<f32>, c: Vector3<f32>) -> Self {
        Triangle { a, b, c }
    }
}

impl Triangle {
    fn is_inside(&self, point: &Vector3<f32>) -> bool {
        let box_min = Vector3::new(point.x.floor(), point.y.floor(), point.z.floor());
        let box_half_size = Vector3::new(0.5, 0.5, 0.5);
        let box_center = box_min + &box_half_size;

        // Move the triangle so that the box is centered around the origin.
        let v0 = self.a - &box_center;
        let v1 = self.b - &box_center;
        let v2 = self.c - &box_center;

        // The edges of the triangle.
        let e0 = v1 - &v0;
        let e1 = v2 - &v1;
        let e2 = v0 - &v2;

        // 1. Test the AABB against the minimal AABB around the triangle.
        if min_max_overlaps(box_half_size.x, v0.x, v1.x, v2.x) {
            return false;
        }

        if min_max_overlaps(box_half_size.y, v0.y, v1.y, v2.y) {
            return false;
        }

        if min_max_overlaps(box_half_size.z, v0.z, v1.z, v2.z) {
            return false;
        }

        // 2. Test if the box intersects the plane of the triangle.
        let normal = e0.cross(&e1);
        let d = -normal.dot(&v0);

        let mut v_min = Vector3::new(0.0, 0.0, 0.0);
        let mut v_max = Vector3::new(0.0, 0.0, 0.0);

        if normal.x > 0.0 {
            v_min.x -= box_half_size.x;
            v_max.x += box_half_size.x;
        } else {
            v_min.x += box_half_size.x;
            v_max.x -= box_half_size.x;
        }

        if normal.y > 0.0 {
            v_min.y -= box_half_size.y;
            v_max.y += box_half_size.y;
        } else {
            v_min.y += box_half_size.y;
            v_max.y -= box_half_size.y;
        }

        if normal.z > 0.0 {
            v_min.z -= box_half_size.z;
            v_max.z += box_half_size.z;
        } else {
            v_min.z += box_half_size.z;
            v_max.z -= box_half_size.z;
        }

        if normal.dot(&v_min) + d > 0.0 {
            return false;
        }

        if normal.dot(&v_max) < 0.0 {
            return false;
        }

        // 3. Axis test

        // Edge #1
        if !axis_test_zy(&v0, &v2, &box_half_size, &e0) {
            return false;
        }

        if !axis_test_mzx(&v0, &v2, &box_half_size, &e0) {
            return false;
        }

        if !axis_test_yx(&v1, &v2, &box_half_size, &e0) {
            return false;
        }

        // Edge #2
        if !axis_test_zy(&v0, &v2, &box_half_size, &e1) {
            return false;
        }

        if !axis_test_mzx(&v0, &v2, &box_half_size, &e1) {
            return false;
        }

        if !axis_test_yx(&v0, &v1, &box_half_size, &e1) {
            return false;
        }

        // Edge #3
        if !axis_test_zy(&v0, &v1, &box_half_size, &e2) {
            return false;
        }

        if !axis_test_mzx(&v0, &v1, &box_half_size, &e2) {
            return false;
        }

        if !axis_test_yx(&v1, &v2, &box_half_size, &e2) {
            return false;
        }

        true
    }
}

fn min_max_overlaps(box_half_size: f32, v0: f32, v1: f32, v2: f32) -> bool {
    let min = v0.min(v1).min(v2);
    let max = v0.max(v1).max(v2);

    min > box_half_size || max < -box_half_size
}

fn axis_test_zy(point1: &Vector3<f32>, point2: &Vector3<f32>, box_half_size: &Vector3<f32>, edge: &Vector3<f32>) -> bool {
    axis_test(
        edge.z,
        edge.y,
        point1.y,
        point1.z,
        point2.y,
        point2.z,
        box_half_size.y,
        box_half_size.z,
        false,
    )
}

fn axis_test_mzx(point1: &Vector3<f32>, point2: &Vector3<f32>, box_half_size: &Vector3<f32>, edge: &Vector3<f32>) -> bool {
    axis_test(
        -edge.z,
        edge.x,
        point1.x,
        point1.z,
        point2.x,
        point2.z,
        box_half_size.x,
        box_half_size.z,
        true,
    )
}

fn axis_test_yx(point1: &Vector3<f32>, point2: &Vector3<f32>, box_half_size: &Vector3<f32>, edge: &Vector3<f32>) -> bool {
    axis_test(
        edge.y,
        edge.x,
        point1.x,
        point1.y,
        point2.x,
        point2.y,
        box_half_size.x,
        box_half_size.y,
        false,
    )
}

fn axis_test(
    edge_axis1: f32,
    edge_axis2: f32,
    point1_axis1: f32,
    point1_axis2: f32,
    point2_axis1: f32,
    point2_axis2: f32,
    box_half_size_axis1: f32,
    box_half_size_axis2: f32,
    sign: bool,
) -> bool {
    let p1 = if sign {
        edge_axis1 * point1_axis1 + edge_axis2 * point1_axis2
    } else {
        edge_axis1 * point1_axis1 - edge_axis2 * point1_axis2
    };

    let p2 = if sign {
        edge_axis1 * point2_axis1 + edge_axis2 * point2_axis2
    } else {
        edge_axis1 * point2_axis1 - edge_axis2 * point2_axis2
    };

    let min = p1.min(p2);
    let max = p1.max(p2);

    let radius = edge_axis1.abs() * box_half_size_axis1 + edge_axis2.abs() * box_half_size_axis2;

    if min > radius || max < -radius {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::geometry::Triangle;
    use nalgebra::Vector3;

    #[test]
    fn test_is_inside() {
        let a = Vector3::new(0.0, 0.0, 0.0);
        let b = Vector3::new(5.0, 5.0, 5.0);
        let c = Vector3::new(-5.0, 5.0, -5.0);

        let triangle = Triangle::new(a, b, c);

        assert!(triangle.is_inside(&Vector3::new(0.0, 0.0, 0.0)));
        assert!(triangle.is_inside(&Vector3::new(5.0, 5.0, 5.0)));
        assert!(triangle.is_inside(&Vector3::new(-5.0, 5.0, -5.0)));

        assert!(!triangle.is_inside(&Vector3::new(-3.0, 6.0, -2.0)));
        assert!(!triangle.is_inside(&Vector3::new(10.0, 5.0, 0.0)));
    }
}
