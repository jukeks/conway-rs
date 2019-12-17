use rand;
use std::{thread, time};
use term_size;

struct Cell {
    now: bool,
    next: bool,
}

impl Clone for Cell {
    fn clone(&self) -> Self {
        Self {
            now: self.now,
            next: self.next,
        }
    }
}

struct World {
    w: usize,
    h: usize,
    state: Vec<Vec<Cell>>,
}

impl World {
    fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            state: vec![vec![Cell{now: false, next: false}; w]; h],
        }
    }

    fn fill(&mut self) {
        for j in 0..self.h  {
            for i in 0..self.w {
                self.state[j][i].now = rand::random()
            }
        }
    }

    fn alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;

        let i_x = x as isize;
        let i_y = y as isize;
        let i_w = self.w as isize;
        let i_h = self.h as isize;

        for j in i_y-1..i_y+2  {
            for i in i_x-1..i_x+2 {
                if i < 0 || i > i_w-1 {
                    continue
                }
                if j < 0 || j > i_h-1 {
                    continue
                }

                if self.state[j as usize][i as usize].now {
                    count += 1;
                }
            }
        }

        count
    }

    fn as_string(&self) -> String {
        let mut text = String::from("");

        for j in 0..self.h  {
            for i in 0..self.w {
                if self.state[j][i].now {
                    text.push_str("+");
                } else {
                    text.push_str(" ");
                }
            }

            text.push_str("\n");
        }

        text
    }

    fn update(&mut self) -> (bool) {
        let mut changed = false;

        for j in 0..self.h  {
            for i in 0..self.w {
                let was_alive = self.state[j][i].now;
                let alive_neighbors = self.alive_neighbors(i, j);
                let alive: bool;

                if was_alive {
                    alive = match alive_neighbors {
                        0 | 1 => false,
                        2 | 3 => true,
                        _ => false
                    };
                } else {
                    alive = alive_neighbors == 3;
                }

                self.state[j][i].next = alive;

                changed |= was_alive ^ alive;
            }
        }

        for j in 0..self.h  {
            for i in 0..self.w {
                self.state[j][i].now = self.state[j][i].next;
            }
        }

        changed
    }
}

fn main() {
    let mut world: World;

    if let Some((w, h)) = term_size::dimensions() {
         world = World::new(w, h-2);
    } else {
        println!("Unable to get term size :(");
        return;
    }

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
        println!("{} ms", ((frame_end-frame_start).as_micros() as f64) / 1000.0);

        let now = time::Instant::now();
        if now < next {
            thread::sleep(next - now);
        }

        next = next + interval;
    }
}
