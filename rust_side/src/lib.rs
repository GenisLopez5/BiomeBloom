use std::sync::Mutex;

#[repr(C)]
pub struct DAtom {
    material: u64,
    obsolete: bool,
}
#[repr(C)]
pub struct Atom {
    entity_tag: u64,
}

static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(drawing_buffer: *mut DAtom, buffer_width: u64, buffer_height: u64) {
    let buffer_size = buffer_height * buffer_width;
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    unsafe { for i in 0..buffer_size {
        *drawing_buffer.add(i as usize)  = DAtom { material: 42, obsolete: true };
    }}
}