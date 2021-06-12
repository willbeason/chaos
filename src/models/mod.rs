pub struct Spring {
    pub a: f64,
    pub b: f64,
    pub d: f64,
    pub y: f64,
    pub w: f64,
}

impl Spring {
    fn xpp(&self, x: f64, v: f64, t: f64) -> f64 {
        -self.d * v - self.a * x - self.b * x * x * x + self.y * (self.w * t).cos()
    }

    pub fn forward(&self, x0: f64, v0: f64, t0: f64, h: f64) -> (f64, f64) {
        let k1 = self.xpp(x0, v0, t0);
        let v1 = v0 + k1*(h/2.0);
        let x1 = x0 + (v0 + v1)*(h/4.0);

        let k2 = self.xpp(x1, v1, t0+h/2.0);
        let v2 = v0 + k2*(h/2.0);
        let x2 = x0 + (v0 + (v1+v2)/2.0)*(h/4.0);

        let k3 = self.xpp(x2, v2, t0+h/2.0);
        let v3 = v0 + k3*h;
        let x3 = x0 + (v0 + 2.0*v1 + 2.0*v2 + v3)*(h/6.0);

        let k4 = self.xpp(x3, v3, t0+h);
        let v4 = v0 + (k1 + 2.0*k2 + 2.0*k3 + k4)*(h/6.0);
        let x4 = x0 + (v0 + 2.0*v1 + 2.0*v2 + (v3+v4)/2.0)*(h/6.0);

        (x4, v4)
    }

    pub fn forward2(&self, x0: f64, v0: f64, t0: f64, h: f64) -> (f64, f64) {
        let k1 = self.xpp(x0, v0, t0);
        let v1 = v0 + k1*h;
        let x1 = x0 + (v0 +v1)*(h/2.0);

        (x1, v1)
    }
}

pub fn render(out: String, weight: u8, points: &Vec<(f64, f64)>) {
    let dim = 1000;

    let mut image: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        image::ImageBuffer::new(dim, dim);

    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    // for p in points {
    //     if p.0 < min_x {
    //         min_x = p.0;
    //     }
    //     if p.0 > max_x {
    //         max_x = p.0;
    //     }
    //     if p.1 < min_y {
    //         min_y = p.1;
    //     }
    //     if p.1 > max_y {
    //         max_y = p.1;
    //     }
    // }
    min_x = -3.0;
    max_x = 3.0;
    min_y = -8.0;
    max_y = 8.0;

    let mut idx = 0;
    // let len = points.len();
    for p in points {
        let px = (((p.0 - min_x) / (max_x - min_x)) * (dim as f64)) as i32;
        if px < 0 || px >= (dim as i32) {
            continue;
        }
        let py = (((p.1 - min_y) / (max_y - min_y)) * (dim as f64)) as i32;
        if py < 0 || py >= (dim as i32) {
            continue;
        }
        idx += 1;
        // let brightness = (idx * 255 / len) as u8;
        let brightness = image.get_pixel(px as u32, py as u32).0[0];
        if brightness < 255 {
            let brightness = brightness + weight;
            image.get_pixel_mut(px as u32, py as u32).0 = [brightness, brightness, brightness];
        }
    }

    image.save(out).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conserve_energy() {
        let mut x: f64 = 1.0;
        let mut v: f64 = 0.0;

        let s = Spring {
            a: 1.0,
            b: 0.0,
            d: 0.0,
            y: 0.0,
            w: 0.0
        };

        for i in 0..200 {
            let energy = x*x + v*v;
            println!("{:.1}: {}", (i as f64)*0.1, x);

            if energy < 0.99 || energy > 1.01 {
                assert!(false, "Lost or gained too much energy: {}", energy)
            }

            let z = s.forward(x, v, 0.0, 0.1);
            x = z.0;
            v = z.1;
        }

        panic!("")
    }
}
