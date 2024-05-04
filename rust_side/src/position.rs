
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize, height: usize) -> Self {
        Self { x, y: height - y - 1 }
    }
    pub fn from_index(val: usize, w: usize, h: usize) -> Self {
        let x = val % w;
        let y = h - val / w - 1;
        Self { x, y }
    }
    pub fn as_idx(&self, width: usize, height: usize) -> usize {
        (height - self.y - 1) * width + self.x
    }
    pub fn move_down(&self, amount: usize, height: usize) -> Self {
        let y = (self.y + height - amount) % height;
        Self { y, ..*self }
    }
    pub fn move_up(&self, amount: usize, height: usize) -> Self {
        let y = (self.y + height + amount) % height;
        Self { y, ..*self }
    }
    pub fn move_right(&self, amount: usize, width: usize) -> Self {
        let x = (self.x + width + amount) % width;
        Self { x, ..*self }
    }
    pub fn move_left(&self, amount: usize, width: usize) -> Self {
        let x = (self.x + width - amount) % width;
        Self { x, ..*self }
    }
    pub fn neighbours(&self, width: usize, height: usize) -> [Position; 8]{
        let mut counter = 0;
        let mut result = [Position::new(0,0, height);8];
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    let x1: usize = (self.x + j + width - 1) % width;
                    let y1: usize = (self.y + i + height - 1) % height;
                    result[counter] = Position::new(x1,y1, height);
                    counter += 1;
                }
            }
        }
        result
    }
}

