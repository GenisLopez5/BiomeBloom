use std::sync::Mutex;

mod rules;
use rules::*;
mod position;
use position::*;
mod shaders;
use shaders::*;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DAtom {
    material: i64,
    obsolete: bool,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct MouseInfo {
    posx: i64,
    posy: i64,
    selected_tag: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CFloatVector {
    ptr: *mut f64,
    size: u64
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
    mouse: &MouseInfo,
    shader_buffers: CVector,
) {
    let (buffer_width, buffer_height): (usize, usize) = (buffer_width.try_into().unwrap(), buffer_height.try_into().unwrap());
    let buffer_size = buffer_height * buffer_width;

    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    init_logic_buffer_if_needed(&mut *logic_buffer, buffer_width, buffer_height);
    let mut new_logic_buffer = logic_buffer.clone();


    for i in 0..buffer_size {
        for p in 0..=u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p { continue }

            let shader = match current_atom.entity_tag.into() {
                0 => nothing_shader,
                1 => ant_shader,
                2 => tnt_shader,
                3 => fire_shader,
            };
            let attach = AttachmentsForApply {
                buffers: todo!(),
                old_logic_buffer: Box::new(*logic_buffer),
                new_logic_buffer: Box::new(new_logic_buffer),
                mouse_pos: todo!(),
                width:  buffer_width,
                height: buffer_height
            };

            shader(i, &mut attach).unwrap();
        }
    }
    println!("Finished calculating frame");

    *logic_buffer = new_logic_buffer;
    std::mem::drop(logic_buffer); // Drop guard, parity needs it unlocked
    get_buffer_parity(drawing_buffer, buffer_width, buffer_height)
}

#[no_mangle]
pub extern "C" fn update_mouse(mouse: MouseInfo, drawing_buffer: *mut DAtom, buffer_width: i64, buffer_height: i64) {
    let buffer_width: usize = buffer_width.try_into().unwrap();
    let buffer_height: usize = buffer_height.try_into().unwrap();
    if mouse.posx >= 0_i64 && mouse.posy >= 0_i64 {
        dbg!(mouse);
        let pos = Position::new(mouse.posx as usize, mouse.posy as usize);
        let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
        init_logic_buffer_if_needed(&mut *logic_buffer, buffer_width, buffer_height);
        logic_buffer[pos.as_idx(buffer_width, buffer_height)] = Atom {
                entity_tag: mouse.selected_tag,
                priority: 2,
                obsolete: true,
        };
    }

    get_buffer_parity(drawing_buffer, buffer_width, buffer_height)
}

/// pre: LOGIC_BUFFER must not be locked when this function is called
fn get_buffer_parity(drawing_buffer: *mut DAtom, buffer_width: usize, buffer_height: usize) {
    let logic_buffer = LOGIC_BUFFER.lock().unwrap();
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
