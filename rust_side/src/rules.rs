use crate::*;
/// Pre: Index refers to element in grid
/// Post: Eight element array of the eight neighbours (toroidal geometry) that surrounds the pixel at the given index
/// The indexes of the array look like:
/// ```txt
/// 012
/// 3X4
/// 567
/// ```
pub fn find_neighbours(index: usize, buffer: *mut Atom, width: usize, height: usize) -> [EntityTag; 8] {
    let Position {x, y} = Position::from_index(index, width, height);
    let mut counter = 0;
    let mut result = [0 ;8];
    for i in 0..3 {
        for j in 0..3 {
            let x1: usize = (x + width -1 + j).rem_euclid(width);
            let y1: usize = (y + width -1 + i).rem_euclid(height);
            let idx = Position::new(x1, y1, height).as_idx(width, height);
            if i != 1 || j != 1 {
                unsafe {result[counter] = (*buffer.add(idx)).entity_tag};
                counter += 1;
            }
        }
    }
    result
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
