use crate::*;

fn positive_mod(i: isize, n: isize) -> usize {
    ((i % n + n) % n) as usize
}

/// Pre: Index refers to element in grid
/// Post: Eight element array of the eight neighbours (toroidal geometry) that surrounds the pixel at the given index
/// The indexes of the array look like:
/// ```txt
/// 012
/// 3X4
/// 567
/// ```
pub fn find_neighbours(index: usize, buffer: *mut Atom, width: usize, height: usize) -> [EntityTag; 8] {
    let x: isize = (index%width) as isize;
    let y: isize = (index/width) as isize;
    let mut counter = 0;
    let mut result = [0 ;8];
    for i in 0..3 {
        for j in 0..3 {
            let x1:usize = positive_mod((x-1 + j) as isize, width as isize);
            let y1:usize = positive_mod((y-1 + i) as isize, height as isize);
            if i != 1 || j != 1 {
                unsafe {result[counter] = (*buffer.add(x1 + y1*width)).entity_tag};
                counter += 1;
            }
        }
    }
    result
}

pub fn i_to_xy(i: usize, width: usize) -> (usize, usize) {
    let x = i % width;
    let y = i / width;
    (x, y)
}

pub fn xy_to_i(x: usize, y: usize, width: usize) -> usize {
    y*width + x
}

pub fn move_left(i: usize, width: usize, height: usize) -> usize {
    let (mut x, y) = i_to_xy(i, width);
    x = (x + width - 1) % width;
    xy_to_i(x, y, width)
}

pub fn move_right(i: usize, width: usize, height: usize) -> usize {
    let (mut x, y) = i_to_xy(i, width);
    x = (x + width + 1) % width;
    xy_to_i(x, y, width)
}

pub fn move_down(i: usize, width: usize, height: usize) -> usize {
    let (mut x, y) = i_to_xy(i, width);
    x = (x + height + 1) % height;
    xy_to_i(x, y, width)
}

pub fn move_up(i: usize, width: usize, height: usize) -> usize {
    let (mut x, y) = i_to_xy(i, width);
    x = (x + height - 1) % height;
    xy_to_i(x, y, width)
}

impl TryFrom<u64> for Entity {
    type Error = ();
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let x: Entity = unsafe { std::mem::transmute(value as u64) }; // Assumes it's in range
        Ok(x)
    } 
}




impl From<Atom> for DAtom {
    fn from(value: Atom) -> Self {
        Self {
            material: value.material,
            obsolete: value.obsolete
        }
    }
}

#[test]
fn test_neighbours() {
    let mut array = [Atom{entity_tag:0 as u64, priority: 1, material: 1, obsolete: false};12];
    for i in 0..12 {
        array[i] = Atom{entity_tag:i as u64, priority: 1, material: 1, obsolete: false};
    }
    let tags = find_neighbours(2,  array.as_mut_ptr(), 4, 3);

    dbg!(tags);
}