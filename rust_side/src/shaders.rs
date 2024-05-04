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
        [ None, None, None, None, None, None, Some(E::Nothing.into()), None, ],
        [ None, None, None, None, Some(E::Nothing.into()), None, None, Some(E::Ant.into()), None, ],
    ) .into();

    let walk_down: Rule = (
        [ None, None, None, None, Some(E::Nothing.into()), None, None, None, ],
        [ None, None, None, None, Some(E::Nothing.into()), Some(E::Ant.into()), None, None, None, ],
    ) .into();

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

pub fn water_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    let water_neighs = find_neighbours_of_buffer(
        0,
        attach.buffers,
        Position::from_index(index, attach.width, attach.height),
        attach.width,
        attach.height
    );
    let index_of_min: usize = water_neighs
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();

    let mut tl_rule: Rule = ([None; 8], [None; 9]).into();
    let mut tr_rule: Rule = ([None; 8], [None; 9]).into();
    let mut bl_rule: Rule = ([None; 8], [None; 9]).into();
    let mut br_rule: Rule = ([None; 8], [None; 9]).into();
    let mut left_rule: Rule = ([None; 8], [None; 9]).into();
    let mut right_rule: Rule = ([None; 8], [None; 9]).into();
    let mut top_rule: Rule = ([None; 8], [None; 9]).into();
    let mut down_rule: Rule = ([None; 8], [None; 9]).into();

    tl_rule.0.1[0]   = Some(Entity::Water.into());
    tr_rule.0.1[2]   = Some(Entity::Water.into());
    bl_rule.0.1[5]   = Some(Entity::Water.into());
    br_rule.0.1[7]   = Some(Entity::Water.into());
    top_rule.0.1[1]   = Some(Entity::Water.into());
    left_rule.0.1[3]  = Some(Entity::Water.into());
    right_rule.0.1[4] = Some(Entity::Water.into());
    down_rule.0.1[6]  = Some(Entity::Water.into());

    match index_of_min {
        0 => apply_rule(&tl_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        1 => apply_rule(&top_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        2 => apply_rule(&tr_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        3 => apply_rule(&left_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        4 => apply_rule(&right_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        5 => apply_rule(&bl_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        6 => apply_rule(&down_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        7 => apply_rule(&br_rule, &attach.old_logic_buffer, attach.new_logic_buffer, index, attach.width, attach.height),
        e => unreachable!("unreachable index of neighbour (isn't 0..8): {e}"),
    }

    Ok(())
}


fn find_neighbours_of_buffer(
    buffer_idx: usize,
    buffers: *const i64,
    pos: Position,
    width: usize,
    height: usize
) -> [i64; 8] {
    let delta = buffer_idx * width * height;
    let mut counter = 0;
    let mut result = [0; 8];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; }
            let x1: usize = (pos.x + width  - 1 + j) % width;
            let y1: usize = (pos.y + height - 1 + i) % height;
            let idx = Position::new(x1, y1).as_idx(width, height) + delta;
            result[counter] = unsafe { *buffers.add(idx) };
            counter += 1;
        }
    }
    result

}

