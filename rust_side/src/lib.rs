use std::sync::Mutex;

mod rules;
use rules::*;
mod position;
use position::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DAtom {
    material: i64,
    obsolete: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Mouse {
    posx: i64,
    posy: i64,
    selected_tag: i64,
    clicked: bool,
}

type EntityTag = i64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Atom {
    entity_tag: i64, // Doubles as material
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

// Internal buffer, in case we need to add things like Lifetimes or whatever
static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(
    drawing_buffer: *mut DAtom,
    buffer_width: i64,
    buffer_height: i64,
) {
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    let mut new_logic_buffer = logic_buffer.clone();

    let (buffer_width, buffer_height): (usize, usize) = (buffer_width.try_into().unwrap(), buffer_height.try_into().unwrap());
    let buffer_size = buffer_height * buffer_width;

    init_logic_buffer_if_needed(&mut *logic_buffer, buffer_width, buffer_height);

    for i in 0..buffer_size {
        for p in 0..=u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p { continue }

            apply_rules(&mut logic_buffer, &mut new_logic_buffer, i, buffer_width, buffer_height);
        }
    }
    println!("Finished calculating frame");

    *logic_buffer = new_logic_buffer;
    get_buffer_parity(drawing_buffer, &mut logic_buffer, buffer_width, buffer_height)
}

#[no_mangle]
pub extern "C" fn update_mouse(mouse: Mouse, drawing_buffer: *mut DAtom, logic_buffer: &mut Vec<Atom>, buffer_width: usize, buffer_height: usize) {
    println!("Click is: {}", mouse.clicked);
    if mouse.clicked && mouse.posx >= 0_i64 && mouse.posy >= 0_i64 {
        dbg!(mouse);
        let pos = Position::new(mouse.posx as usize, mouse.posy as usize);
        logic_buffer[pos.as_idx(buffer_width, buffer_height)] = Atom {
                entity_tag: mouse.selected_tag,
                priority: 2,
                obsolete: true,
        };
    }

    get_buffer_parity(drawing_buffer, logic_buffer, buffer_width, buffer_height)
}

fn get_buffer_parity(drawing_buffer: *mut DAtom, logic_buffer: &mut Vec<Atom>, buffer_width: usize, buffer_height: usize) {
    for i in 0..buffer_height * buffer_width {
        unsafe {
            *drawing_buffer.add(i) = logic_buffer[i].into();
        }
    }
}

/// First time set up of logical buffer (initial state of simulation). All Atoms should be marked as obsolete, here
fn init_logic_buffer_if_needed(logic_buffer: &mut Vec<Atom>, buffer_width: usize, buffer_height: usize) {
    if !logic_buffer.is_empty() { return; }
    println!("Initializing ");
    for _ in 0..buffer_width*buffer_height {
        logic_buffer.push(Atom::NULL)
    }
    let ant_pos = Position::new(buffer_width / 2, buffer_height / 2);
    logic_buffer[ant_pos.as_idx(buffer_width, buffer_height)] = Atom {
        entity_tag: Entity::Ant as i64,
        priority: 1,
        obsolete: true,
    };
}
