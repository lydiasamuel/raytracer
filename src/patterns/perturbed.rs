use crate::patterns::pattern::Pattern;
use crate::patterns::solid::Solid;
use crate::tuples::tuple::Tuple;
use crate::Color;
use crate::Matrix;

const PERMUTATIONS: [i64; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, 151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194,
    233, 7, 225, 140, 36, 103, 30, 69, 142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234,
    75, 0, 26, 197, 62, 94, 252, 219, 203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174,
    20, 125, 136, 171, 168, 68, 175, 74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83,
    111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25,
    63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188,
    159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147,
    118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170,
    213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253,
    19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193,
    238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
    181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93,
    222, 114, 67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180,
];

pub struct Perturbed {
    delegate: Box<dyn Pattern>,
    scale: f64,
    transform: Matrix,
}

impl Perturbed {
    pub fn new(delegate: Box<dyn Pattern>, scale: f64, transform: Matrix) -> Perturbed {
        Perturbed {
            delegate,
            scale,
            transform,
        }
    }

    pub fn default() -> Perturbed {
        Perturbed::new(
            Box::new(Solid::new(Color::white())),
            0.2,
            Matrix::identity(4),
        )
    }

    fn fade(t: f64) -> f64 {
        t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
    }

    fn lerp(t: f64, a: f64, b: f64) -> f64 {
        a + t * (b - a)
    }

    // Convert lo 4 bits of hash code into 12 gradient directions
    fn grad(hash: i64, x: f64, y: f64, z: f64) -> f64 {
        let h: i64 = hash & 15;

        let u: f64 = if h < 8 { x } else { y };

        let v: f64 = if h < 4 {
            y
        } else if h == 12 || h == 14 {
            x
        } else {
            z
        };

        (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
    }

    #[allow(non_snake_case)]
    pub fn noise(point: Tuple) -> f64 {
        // Find unit cube that contains point
        let X = (point.x.floor() as i64) & 255;
        let Y = (point.y.floor() as i64) & 255;
        let Z = (point.z.floor() as i64) & 255;

        // Find relative X, Y, Z of point in cube
        let x = point.x - point.x.floor();
        let y = point.y - point.y.floor();
        let z = point.z - point.z.floor();

        // Compute fade curves for each X, Y, Z
        let u = Perturbed::fade(x);
        let v = Perturbed::fade(y);
        let w = Perturbed::fade(z);

        // Hash coordinates of the 8 cube corners,
        let A = PERMUTATIONS[X as usize] + Y;
        let AA = PERMUTATIONS[A as usize] + Z;
        let AB = PERMUTATIONS[(A + 1) as usize] + Z;
        let B = PERMUTATIONS[(X + 1) as usize] + Y;
        let BA = PERMUTATIONS[B as usize] + Z;
        let BB = PERMUTATIONS[(B + 1) as usize] + Z;

        // And add blended results from 8 corners of cube
        Perturbed::lerp(
            w,
            Perturbed::lerp(
                v,
                Perturbed::lerp(
                    u,
                    Perturbed::grad(PERMUTATIONS[AA as usize], x, y, z),
                    Perturbed::grad(PERMUTATIONS[BA as usize], x - 1.0, y, z),
                ),
                Perturbed::lerp(
                    u,
                    Perturbed::grad(PERMUTATIONS[AB as usize], x, y - 1.0, z),
                    Perturbed::grad(PERMUTATIONS[BB as usize], x - 1.0, y - 1.0, z),
                ),
            ),
            Perturbed::lerp(
                v,
                Perturbed::lerp(
                    u,
                    Perturbed::grad(PERMUTATIONS[(AA + 1) as usize], x, y, z - 1.0),
                    Perturbed::grad(PERMUTATIONS[(BA + 1) as usize], x - 1.0, y, z - 1.0),
                ),
                Perturbed::lerp(
                    u,
                    Perturbed::grad(PERMUTATIONS[(AB + 1) as usize], x, y - 1.0, z - 1.0),
                    Perturbed::grad(PERMUTATIONS[(BB + 1) as usize], x - 1.0, y - 1.0, z - 1.0),
                ),
            ),
        )
    }
}

impl Pattern for Perturbed {
    fn pattern_at(&self, pattern_point: Tuple) -> Color {
        assert!(pattern_point.is_point());

        // Jitter each point (i.e. move it a bit) before delegating it to the given pattern.
        // Generate 3 values from the perlin noise, scale each and add it to the corresponding point

        let noise = Perturbed::noise(pattern_point) * self.scale;

        let a = pattern_point.x + noise;
        let b = pattern_point.y + noise;
        let c = pattern_point.z + noise;

        let perturbed_point = Tuple::point(a, b, c);

        self.delegate.local_pattern_at(perturbed_point)
    }

    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }
}
