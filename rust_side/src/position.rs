pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    pub fn from_index(val: usize, w: usize) -> Self {
        let x = val % w;
        let y = val / w;
        Self { x, y }
    }
    pub fn as_idx(&self, width: usize) -> usize {
        self.y * width + self.x
    }
    pub fn move_down(&self, amount: usize, height: usize) -> Self {
        let y = (self.y + height + amount) % height;
        Self { y, ..*self }
    }
    pub fn move_up(&self, amount: usize, height: usize) -> Self {
        let y = (self.y + height - amount) % height;
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
}

