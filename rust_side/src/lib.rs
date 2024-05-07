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

// Internal buffer, in case we need to add things like Lifetimes or whatever
static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(
    drawing_buffer: *mut DAtom,
    buffer_width: i64,
    buffer_height: i64,
    mouse: MouseInfo,
    shader_buffers: *mut i64,
) {
    let (buffer_width, buffer_height): (usize, usize) = (
        buffer_width.try_into().unwrap(),
        buffer_height.try_into().unwrap(),
    );
    let buffer_size = buffer_height * buffer_width;

    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    init_logic_buffer_if_needed(&mut *logic_buffer, buffer_width, buffer_height);
    let mut new_logic_buffer = logic_buffer.clone();

    println!("Pointer of shared is: {:?}", shader_buffers);

    let y = unsafe { *shader_buffers };
    println!("{y}");

    for i in 0..buffer_size {
        for p in 0..=u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p {
                continue;
            }

            use Entity as E;
            let shader = match current_atom.entity_tag.into() {
                E::Dirt => nothing_shader,
                E::Grass => grass_shader,
                E::Ant => ant_shader,
                E::Fire => fire_shader,
                E::Water => water_shader,
                t => missing_shader,
            };
            let mut attach = AttachmentsForApply {
                buffers: shader_buffers,
                old_logic_buffer: &mut *logic_buffer,
                new_logic_buffer: &mut new_logic_buffer,
                mouse_pos: mouse,
                width: buffer_width,
                height: buffer_height,
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
pub extern "C" fn reset_buffer(draw_buffer: *mut DAtom, buffer_size: i64) {
    let mut buf = LOGIC_BUFFER.lock().unwrap();
    for i in 0..buffer_size as usize {
        buf[i] = Atom::NULL;
        unsafe { *draw_buffer.add(i) = DAtom { material: 0, obsolete: true} };
    }
}
#[no_mangle]
pub extern "C" fn update_mouse(
    mouse: MouseInfo,
    drawing_buffer: *mut DAtom,
    buffer_width: i64,
    buffer_height: i64,
) {
    let buffer_width: usize = buffer_width.try_into().unwrap();
    let buffer_height: usize = buffer_height.try_into().unwrap();
    if mouse.posx >= 0_i64 && mouse.posy >= 0_i64 && (mouse.posx as usize) < buffer_width && (mouse.posy as usize) < buffer_height {
        // dbg!(mouse);
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
fn init_logic_buffer_if_needed(
    logic_buffer: &mut Vec<Atom>,
    buffer_width: usize,
    buffer_height: usize,
) {
    if !logic_buffer.is_empty() {
        return;
    }
    printinfo("Initializing logic buffer");
    for _ in 0..buffer_width * buffer_height {
        logic_buffer.push(Atom::NULL)
    }
    printinfo("Finished initializing logic buffer");
}

fn printinfo(s: &str) {
    println!("[INFO (rs)]: {s}");
}
