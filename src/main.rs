use std::f64::consts::PI;
use rand::Rng;

mod foo;
mod bar;
mod models;

fn main() {
    let mut rng = rand::thread_rng();

    let spring = models::Spring{
        a: 1.0,
        b: 5.0,
        d: 0.02,
        y: 12.0,
        w: 0.5,
    };

    let mut points: Vec<(f64, f64)> = Vec::new();
    for x in 0..1000 {
        for y in 0..1000 {
            let xp = ((x - 500) as f64 + rng.gen::<f64>() - 0.5) / 500.0;
            let yp = ((y - 500) as f64 + rng.gen::<f64>() - 0.5) / 500.0;
            if xp*xp + yp*yp > 1.0 {
                continue
            }
            points.push((xp, yp))
        }
    }

    let dt = 4.0*PI/(1000.0);
    let mut step: i64 = 0;
    while step <= 26000 {
        if step % 1000 == 0 {
            let out = format!("folding-e/{}.png", step);
            println!("{}", out);
            models::render(out, 5, &points);
        }

        let t = (step as f64)*dt;
        points.iter_mut().for_each(|p| *p = spring.forward(p.0, p.1, t, dt));

        step += 1;
    }
}
