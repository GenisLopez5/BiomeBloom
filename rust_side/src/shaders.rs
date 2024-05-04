use std::collections::HashMap;

use crate::*;

pub struct AttachmentsForApply {
    primitive_rules: HashMap<EntityTag, Vec<Rule>>,
    buffers: Vec<*const f64>,
    logic_buffer: Box<Vec<Atom>>,
    mouse_pos: Position,
    width: usize,
    height: usize,
}
pub struct Rule(([Option<EntityTag>; 8] ,[Option<EntityTag>; 9]));

impl Rule {
    fn pre(&self) -> &[Option<EntityTag>; 8] { &self.0.0 }
    fn post(&self) -> &[Option<EntityTag>; 9] { &self.0.1 }
}

impl From<([Option<EntityTag>; 8] ,[Option<EntityTag>; 9])> for Rule {
    fn from(value: ([Option<EntityTag>; 8] ,[Option<EntityTag>; 9])) -> Self {
        Self(value)
    }
}

pub fn apply_rule(rule: &Rule, logic_buffer: &Vec<Atom>, new_buf: &mut Vec<Atom>, index: usize, width: usize, height: usize) {
    let neighs = find_neighbours( index, logic_buffer.as_ptr(), width, height);
    // Does rule apply?
    if !neighs.into_iter().zip(rule.pre())
        .filter(|(_n, mb_r)| mb_r.is_some())
        .map(|(n, mb_r)| (n, mb_r.unwrap()))
        .all(|(n, r)| n == r) { return; }

    // If we're here, the rule does apply: we will apply it
    let current_pos = Position::from_index(index, width, height);
    let new_atoms = rule.post();
    let mut cnt = 0;
    for i in 0..3 {
        for j in 0..3 {
            let new_pos = Position {
                x:  (current_pos.x + j + width - 1) % width,
                y:  (current_pos.y + i + height - 1) % height,
            };
            if let Some(new_atom) = new_atoms[cnt] {
                new_buf[new_pos.as_idx(width, height)].entity_tag = new_atom;
                new_buf[new_pos.as_idx(width, height)].obsolete = true;
                new_buf[new_pos.as_idx(width, height)].priority = 2;
            }
            cnt += 1;
        }
    }
}


// ------------- Shaders --------------
pub fn ant_shader(attach: &AttachmentsForApply) -> Result<(), ()> {


    Ok(())
}