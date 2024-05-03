use crate::*;

/// Pre: Index refers to element in grid
/// Post: Eight element array of the eight neighbours (toroidal geometry) that surrounds the pixel at the given index
/// The indexes of the array look like:
/// ```txt
/// 012
/// 3X4
/// 567```
pub fn find_neighbours(index: usize, buffer: *mut Atom, width: usize, height: usize) -> [EntityTag; 8] {
    let x: isize = (index%width) as isize;
    let y: isize = (index/width) as isize;
    let mut counter = 0;
    let mut result = [0 ;8];
    for i in 0..3 {
        for j in 0..3 {
            let x1:isize = (x-1 + j)%(width as isize);
            let y1:isize = (y-1 + i)%(height as isize);
            if i != 1 || j != 1 {
                unsafe {result[counter] = (*buffer.add((x1 + y1*width as isize) as usize)).entity_tag};
                counter += 1;
            }
        }
    }
    result
}

//#[test]
//fn test_neighbours() {
//    let mut array = [Atom{entity_tag: 0};12];
//    for i in 0..12 {
//        array[i] = Atom{entity_tag:i as u64};
//    }
//    let tags = find_neighbours(2,  array.as_mut_ptr(), 4, 3);
//
//    dbg!(tags);
//    unsafe { for i in 0..buffer_size {
//        *drawing_buffer.add(i as usize)  = DAtom { material: 42, obsolete: true };
//    }}
//}