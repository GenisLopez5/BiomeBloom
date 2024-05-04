use crate::*;

pub struct AttachmentsForApply<'a> {
    pub buffers: *mut i64,
    pub old_logic_buffer: &'a mut Vec<Atom>,
    pub new_logic_buffer: &'a mut Vec<Atom>,
    pub mouse_pos: MouseInfo,
    pub width: usize,
    pub height: usize,
}
pub struct Rule(([Option<EntityTag>; 8], [Option<EntityTag>; 9]));

impl Rule {
    fn pre(&self) -> &[Option<EntityTag>; 8] {
        &self.0 .0
    }
    fn post(&self) -> &[Option<EntityTag>; 9] {
        &self.0 .1
    }
}

impl From<([Option<EntityTag>; 8], [Option<EntityTag>; 9])> for Rule {
    fn from(value: ([Option<EntityTag>; 8], [Option<EntityTag>; 9])) -> Self {
        Self(value)
    }
}

pub fn apply_rule(
    rule: &Rule,
    logic_buffer: &Vec<Atom>,
    new_buf: &mut Vec<Atom>,
    index: usize,
    width: usize,
    height: usize,
) {
    let neighs = find_neighbours(index, logic_buffer.as_ptr(), width, height);
    // Does rule apply?
    if !neighs
        .into_iter()
        .zip(rule.pre())
        .filter(|(_n, mb_r)| mb_r.is_some())
        .map(|(n, mb_r)| (n, mb_r.unwrap()))
        .all(|(n, r)| n == r)
    {
        return;
    }

    // If we're here, the rule does apply: we will apply it
    let current_pos = Position::from_index(index, width, height);
    let new_atoms = rule.post();
    let mut cnt = 0;
    for i in 0..3 {
        for j in 0..3 {
            let new_pos = Position {
                x: (current_pos.x + j + width - 1) % width,
                y: (current_pos.y + i + height - 1) % height,
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
pub fn nothing_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    Ok(())
}

pub fn ant_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    use Entity as E;
    let walk_right: Rule = (
        [
            None,
            None,
            None,
            None,
            None,
            None,
            Some(E::Nothing.into()),
            None,
        ],
        [
            None,
            None,
            None,
            None,
            Some(E::Nothing.into()),
            None,
            None,
            Some(E::Ant.into()),
            None,
        ],
    )
        .into();

    let walk_down: Rule = (
        [
            None,
            None,
            None,
            None,
            Some(E::Nothing.into()),
            None,
            None,
            None,
        ],
        [
            None,
            None,
            None,
            None,
            Some(E::Nothing.into()),
            Some(E::Ant.into()),
            None,
            None,
            None,
        ],
    )
        .into();

    apply_rule(
        &walk_right,
        &mut *attach.old_logic_buffer,
        &mut attach.new_logic_buffer,
        index,
        attach.width,
        attach.height,
    );

    Ok(())
}

pub fn tnt_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    todo!()
}

pub fn fire_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    todo!()
}

