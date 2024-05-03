use std::sync::Mutex;

mod rules;
use rules::*;

#[repr(C)]
pub struct DAtom {
    material: u64,
    obsolete: bool,
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

#[repr(u64)]
enum Entity {
    Nothing,
    Ant
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

// Internal buffer, in case we need to add things like Lifetimes or whatever
static LOGIC_BUFFER: Mutex<Vec<Atom>> = Mutex::new(Vec::new());

#[no_mangle]
pub extern "C" fn compute(drawing_buffer: *mut DAtom, buffer_width: u64, buffer_height: u64) {
    let buffer_size = buffer_height * buffer_width;
    let mut logic_buffer = LOGIC_BUFFER.lock().unwrap();
    if logic_buffer.is_empty() { for i in 0..buffer_size as usize {
         logic_buffer[i] = Atom { entity_tag: 0, priority: 255, material: 0, obsolete: false }
    }} // We initialize to Nothing, for now 

    let mut new_logic_buffer = logic_buffer.clone();

    for i in 0..buffer_size as usize {
        for p in 0..u8::MAX {
            let current_atom = logic_buffer[i];
            if current_atom.priority != p { continue; }

            let [tl, tc, tr, ll, rr, bl, bb, br] = 
                 find_neighbours(i, logic_buffer.as_mut_ptr(), buffer_width as usize, buffer_height as usize);
            match logic_buffer[i].entity_tag.try_into().unwrap() {
                Entity::Nothing => {},
                Entity::Ant => {
                    if bb == Entity::Ant as u64 {
                        // Move ant down in logic buffer
                    }
                },

            }
        }
    }

    unsafe {
    for i in 0..buffer_size as usize {
        *drawing_buffer.add(i) = logic_buffer[i].into();
    }
    }
}
