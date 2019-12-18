mod conway;

use std::time;
use conway::World;

use minifb::{Key, Window, WindowOptions, Scale};

fn print_timing(start: time::Instant, calculated: time::Instant, done: time::Instant) {
    fn in_ms(d: time::Duration) -> f64 {
        (d.as_micros() as f64) / 1000.0
    }

    println!("{} ms {} ms {} ms",
        in_ms(done-start),
        in_ms(calculated - start),
        in_ms(done - calculated),
    );
}

fn main() {
    let w: usize = 1920;
    let h: usize = 1080;

    let mut world = World::new(w, h);
    world.fill();

    let mut options = WindowOptions::default();
    options.scale = Scale::X1;

    let mut window = Window::new(
        "Conway's Game of Life",
        w,
        h,
        options,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.limit_update_rate(Some(std::time::Duration::from_millis(80)));

    let mut frame: Vec<u32> = vec![0; w * h];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = time::Instant::now();
        world.update();

        let frame_updated = time::Instant::now();
        world.to_buff(&mut frame);

        let frame_end = time::Instant::now();
        print_timing(frame_start, frame_updated, frame_end);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&frame, w, h)
            .unwrap();
    }
}
