use rand;

pub struct World {
    w: usize,
    h: usize,
    state: Vec<Vec<bool>>,
    next_state: Vec<Vec<bool>>,
}

impl World {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            state: vec![vec![false; w]; h],
            next_state: vec![vec![false; w]; h],
        }
    }

    pub fn fill(&mut self) {
        for j in 0..self.h  {
            for i in 0..self.w {
                self.state[j][i] = rand::random()
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

                if self.state[j as usize][i as usize] {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn as_string(&self) -> String {
        let mut text = String::from("");

        for j in 0..self.h  {
            for i in 0..self.w {
                if self.state[j][i] {
                    text.push_str("+");
                } else {
                    text.push_str(" ");
                }
            }

            text.push_str("\n");
        }

        text
    }

    pub fn to_buff(&self, buff: &mut Vec<u32>) {
        for j in 0..self.h  {
            for i in 0..self.w {
                let mut value = 0;
                if self.state[j][i] {
                    value = (255 << 16) + (255 << 8) + 255;
                }

                buff[j * self.w + i] = value
            }
        }
    }

    pub fn update(&mut self) -> (bool) {
        let mut changed = false;

        for j in 0..self.h  {
            for i in 0..self.w {
                let was_alive = self.state[j][i];
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

                self.next_state[j][i] = alive;

                changed |= was_alive ^ alive;
            }
        }

        std::mem::swap(&mut self.next_state, &mut self.state);

        changed
    }
}
