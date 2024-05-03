use std::sync::Mutex;

#[repr(C)]
pub struct DAtom {
    material: u64,
    obsolete: bool,
}


#[repr(C)]
type EntityTag = u64;
pub struct Atom {
    entity_tag: EntityTag,
}

static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(drawing_buffer: *mut DAtom, buffer_size: u64) {
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    for i in 0..buffer_size {
        logic_buffer[i as usize] = Atom { entity_tag: 42 };
        unsafe { *drawing_buffer.add(i as usize)  = DAtom { material: 42, obsolete: true }};
    }
}

/// Pre: Index refers to element in grid
/// Post: Eight element array of the eight neighbours (toroidal geometry) that surrounds the pixel at the given index
/// The indexes of the array look like:
/// ```txt
/// 012
/// 3X4
/// 567```
fn find_neighbors(index: u64, buffer: *mut Atom, width: usize, height: usize) -> [EntityTag; 8] {
    let x = index%width;
    let y = index/width;
    let mut counter = 0;
    let mut result = [0 ;8];
    for i in 0..3 {
        for j in 0..3 {
            let x1 = (x-1 + i)%width;
            let y1 = (y-1 + j)%height;
            if i != 1 || j != 1 {
                result[counter] = buffer[x1*height + y1];
                ++counter;
            }
        }
    }
}

#[test]
fn test_neighbours() {
}