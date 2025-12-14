use std::collections::VecDeque;

pub struct Snake {
    x_dir: i8, // +1, 0, or -1
    y_dir: i8, // +1, 0 , or -1
    head: (u16, u16),
    tail: VecDeque<(u16, u16)>,
}

impl Snake {
    pub fn new(width: u16, height: u16, start_x: u16, start_y: u16) -> Self {
        Snake {
            tail: VecDeque::with_capacity(width as usize * height as usize),
            x_dir: 0,
            y_dir: 0,
            head: (start_x, start_y),
        }
    }

    pub fn step(&mut self) {
        self.tail.push_front(self.head);

        if self.x_dir > 0 {
            self.head.0 += 1;
        } else if self.x_dir < 0 {
            self.head.0 -= 1;
        }

        if self.y_dir > 0 {
            self.head.1 += 1;
        } else if self.y_dir < 0 {
            self.head.1 -= 1;
        }
    }

    pub fn trim_tail(&mut self) {
        self.tail.pop_back();
    }

    pub fn face_down(&mut self) {
        self.y_dir = -1;
        self.x_dir = 0;
    }

    pub fn face_left(&mut self) {
        self.y_dir = 0;
        self.x_dir = -1;
    }

    pub fn face_right(&mut self) {
        self.y_dir = 0;
        self.x_dir = 1;
    }
}
