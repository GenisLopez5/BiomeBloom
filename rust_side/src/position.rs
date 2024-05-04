
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    pub fn move_down(&self, height: usize) -> Self {
        let y = (self.y + height + 1) % height;
        Self { y, ..*self }
    }
    pub fn move_up(&self, height: usize) -> Self {
        let y = (self.y + height - 1) % height;
        Self { y, ..*self }
    }
    pub fn move_right(&self, width: usize) -> Self {
        let x = (self.x + width + 1) % width;
        Self { x, ..*self }
    }
    pub fn move_left(&self, width: usize) -> Self {
        let x = (self.x + width - 1) % width;
        Self { x, ..*self }
    }
    pub fn neighbours(&self, width: usize, height: usize) -> [Position; 8]{

        let mut counter = 0;
        let mut result = [Position::new(0,0);8];
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    let x1:usize = (self.x + j + width - 1)%width;
                    let y1:usize = (self.y + i + height - 1)%height;
                    result[counter] = Position::new(x1,y1);
                    counter += 1;
                }
            }
        }
        result
    }
}

