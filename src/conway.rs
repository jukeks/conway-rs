use rand;

pub struct World {
    w: usize,
    h: usize,
    state: Vec<bool>,
    next_state: Vec<bool>,
}

impl World {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w: w,
            h: h,
            state: vec![false; w * h],
            next_state: vec![false; w * h],
        }
    }

    pub fn fill(&mut self) {
        for j in 0..self.h  {
            for i in 0..self.w {
                self.set_cell(i, j, rand::random())
            }
        }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        j * self.w + i
    }

    fn cell(&self, i: usize, j: usize) -> bool {
        self.state[self.index(i, j)]
    }

    fn set_cell(&mut self, i: usize, j: usize, cell: bool) {
        let idx = self.index(i, j);
        self.state[idx] = cell
    }

    fn set_next_cell(&mut self, i: usize, j: usize, cell: bool) {
        let idx = self.index(i, j);
        self.next_state[idx] = cell
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

                if self.cell(i as usize, j as usize) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn update(&mut self) -> (bool) {
        let mut changed = false;

        for j in 0..self.h  {
            for i in 0..self.w {
                let was_alive = self.cell(i, j);
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

                self.set_next_cell(i, j, alive);

                changed |= was_alive ^ alive;
            }
        }

        std::mem::swap(&mut self.next_state, &mut self.state);

        changed
    }

    pub fn as_string(&self) -> String {
        let mut text = String::from("");

        for j in 0..self.h  {
            for i in 0..self.w {
                if self.cell(i, j) {
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
                if self.cell(i, j) {
                    value = (255 << 16) + (255 << 8) + 255;
                }

                buff[j * self.w + i] = value
            }
        }
    }
}
