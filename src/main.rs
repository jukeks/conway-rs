mod conway;

use std::{time, thread};
use term_size;
use minifb::{Key, Window, WindowOptions, Scale};
use clap::{Arg, App};

use conway::World;

fn print_timing(start: time::Instant, calculated: time::Instant,
                done: time::Instant) {

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
    let args = App::new("conway-rs")
        .version("0.1.0")
        .about("Simulate a game of life")
        .arg(Arg::with_name("terminal")
                 .short("t")
                 .long("terminal")
                 .takes_value(false)
                 .help("Render in terminal (default)")
                 .display_order(0))
        .arg(Arg::with_name("window")
                 .short("w")
                 .long("window")
                 .takes_value(false)
                 .help("Render in window")
                 .display_order(1))
        .arg(Arg::with_name("size")
                 .long("size")
                 .takes_value(true)
                 .help("Simulation size")
                 .display_order(2))
        .arg(Arg::with_name("frametime")
                 .short("f")
                 .long("frame-time")
                 .takes_value(false)
                 .help("Display frame calculation time")
                 .display_order(3))
        .get_matches();

    if args.is_present("window") {
        render_window(&args);
        return
    }

    render_terminal(&args);
}

fn parse_size(size: &String) -> Option<(usize, usize)> {
    let components: Vec<&str> = size.split("x").collect();
    if components.len() != 2 {
        return None
    }

    let w: usize;
    let h: usize;

    let w_o = components[0].parse::<usize>();
    let h_o = components[1].parse::<usize>();

    match w_o {
        Ok(x) => w = x,
        _ => return None,
    }

    match h_o {
        Ok(x) => h = x,
        _ => return None,
    }

    Some((w, h))
}

fn window_size(args: &clap::ArgMatches) -> (usize, usize) {
    match args.value_of("size") {
        Some(s) => match parse_size(&String::from(s)) {
                Some((w, h)) => (w, h),
                _ => (1024, 768),
                },
        _ => (1024, 768)
    }
}

fn render_window(args: &clap::ArgMatches) {
    let (w, h) = window_size(args);

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

    let timings = args.is_present("frametime");

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame_start = time::Instant::now();
        world.update();

        let frame_updated = time::Instant::now();
        world.to_buff(&mut frame);

        let frame_end = time::Instant::now();
        if timings {
            print_timing(frame_start, frame_updated, frame_end);
        }

        window
            .update_with_buffer(&frame, w, h)
            .unwrap();
    }
}

fn terminal_size(args: &clap::ArgMatches) -> (usize, usize) {
    let (w, h) = if let Some((w, h)) = term_size::dimensions() {
        (w, h)
    } else {
        (80, 24)
    };

    match args.value_of("size") {
        Some(s) => match parse_size(&String::from(s)) {
                Some((w, h)) => (w, h),
                _ => (w, h),
                },
        _ => (w, h)
    }
}

fn render_terminal(args: &clap::ArgMatches) {
    let timings = args.is_present("frametime");

    let (w, h) = terminal_size(args);
    let mut world = if timings {
        World::new(w, h-2)
    } else {
       World::new(w, h-1)
    };

    world.fill();
 
    let interval = time::Duration::from_millis(80);
    let now = time::Instant::now();
    let mut next = now + interval;

    loop {
        let frame_start = time::Instant::now();
        let frame = world.as_string();

        let changed = world.update();
        if !changed {
            break
        }

        print!("{}[2J", 27 as char);
        print!("{}", frame);

        let frame_end = time::Instant::now();
        if timings {
            println!("{} ms",
                ((frame_end-frame_start).as_micros() as f64) / 1000.0);
        }

        let now = time::Instant::now();
        if now < next {
            thread::sleep(next - now);
        }

        next = next + interval;
    }
}
