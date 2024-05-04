use std::sync::Mutex;

mod rules;
use rules::*;
mod position;
use position::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Atom {
    entity_tag: u64, // Doubles as material
    priority: u8,
    obsolete: bool,
}

impl Atom {
    const NULL: Self = Self {
        entity_tag: 0,
        priority: u8::MAX,
        obsolete: true,
    };
}

#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Entity {
    Nothing,
    Ant,
    Tnt,
    Fire
}

/// First time set up of logical buffer (initial state of simulation). All Atoms should be marked as obsolete, here
fn init_logic_buffer(logic_buffer: &mut Vec<Atom>, buffer_width: usize, buffer_height: usize) {
    println!("Initializing ");
    for _ in 0..buffer_width*buffer_height {
        logic_buffer.push(Atom::NULL)
    }
    let ant_pos = Position::new(buffer_width / 2, buffer_height / 2, buffer_height);
    logic_buffer[ant_pos.as_idx(buffer_width, buffer_height)] = Atom {
        entity_tag: Entity::Ant as u64,
        priority: 1,
        obsolete: true,
    };
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
    let buffer_height = buffer_height as usize;
    let buffer_width = buffer_width as usize;
    let buffer_size = buffer_height * buffer_width;
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    if logic_buffer.is_empty() {
        init_logic_buffer(&mut *logic_buffer, buffer_width, buffer_height);
    }

    let mut new_logic_buffer = logic_buffer.clone();

    for i in 0..buffer_size {
        for p in 0..=u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p { continue }

            apply_rules(&mut logic_buffer, &mut new_logic_buffer, i, buffer_width, buffer_height);
        }
    }
    println!("Finished calculating frame");

    *logic_buffer = new_logic_buffer;
    // Update drawing buffer with the logic one
    for i in 0..buffer_size {
        unsafe {
            *drawing_buffer.add(i) = logic_buffer[i].into();
        }
    }
}
