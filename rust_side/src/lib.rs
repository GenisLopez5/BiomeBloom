use std::sync::Mutex;

mod rules;
use rules::*;
mod position;
use position::*;

#[repr(C)]
pub struct DAtom {
    material: u64,
    obsolete: bool,
}

#[repr(C)]
pub struct Mouse {
    posx: i64,
    posy: i64,
    clicked: bool,
}

type EntityTag = u64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Atom {
    entity_tag: u64,
    priority: u8,
    material: u64,
    obsolete: bool,
}

impl Atom {
    const NULL: Self = Self {
        entity_tag: 0,
        priority: u8::MAX,
        material: 0,
        obsolete: true,
    };
}

#[repr(u64)]
enum Entity {
    Nothing,
    Ant,
    Tnt,
}

fn init_logic_buffer(logic_buffer: &mut Vec<Atom>, buffer_size: u64) {
    println!("Initializing ");
    for _ in 0..buffer_size as usize {
        logic_buffer.push(Atom::NULL)
    }
}

// Internal buffer, in case we need to add things like Lifetimes or whatever
static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(
    drawing_buffer: *mut DAtom,
    buffer_width: u64,
    buffer_height: u64,
    mouse: Mouse,
) {
    let buffer_size = buffer_height * buffer_width;
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    if logic_buffer.is_empty() {
        init_logic_buffer(&mut *logic_buffer, buffer_size)
    }

    let mut new_logic_buffer = logic_buffer.clone();

    for i in 0..buffer_size as usize {
        for p in 0..u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p {
                continue;
            }

            let [tl, tc, tr, ll, rr, bl, bc, br] = find_neighbours(
                i,
                logic_buffer.as_mut_ptr(),
                buffer_width as usize,
                buffer_height as usize,
            );
            let curr_pos = Position::from_index(i, buffer_width as usize);
            match current_atom.entity_tag.try_into().unwrap() {
                Entity::Nothing => {}
                Entity::Ant => {
                    if bc == Entity::Nothing as u64 {
                        let new_i = curr_pos
                            .move_down(buffer_height as usize)
                            .as_idx(buffer_width as usize);
                        new_logic_buffer[new_i] = logic_buffer[i];
                        new_logic_buffer[new_i].obsolete = true;
                        new_logic_buffer[i] = Atom::NULL;
                    }
                }
                Entity::Tnt => {
                    if [tl, tc, tr, ll, rr, bl, bc, br]
                        .iter()
                        .any(|&p| p == Entity::Ant as u64)
                    {
                        for i in 0..3 {
                            for j in 0..3 {
                                //                                new_logic_buffer[i]
                            }
                        }
                    }
                }
            }
        }
    }

    // Update drawing buffer with the logic one
    for i in 0..buffer_size as usize {
        unsafe {
            *drawing_buffer.add(i) = new_logic_buffer[i].into();
        }
    }
}
