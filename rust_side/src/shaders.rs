use crate::*;

pub struct AttachmentsForApply<'a> {
    pub buffers: *mut i64,
    pub old_logic_buffer: &'a mut Vec<Atom>,
    pub new_logic_buffer: &'a mut Vec<Atom>,
    pub mouse_pos: MouseInfo,
    pub width: usize,
    pub height: usize,
}

#[repr(u64)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Entity {
    Dirt,
    Grass,
    Ant,
    Fire,
    Water,
}

#[derive(Clone, Copy)]
pub enum RuleEntity {
    Basic(Entity),
    Not(Entity),
    Any, 
}

type RE = RuleEntity;
type E = Entity;

impl PartialEq<Entity> for RuleEntity {
    fn eq(&self, other: &Entity) -> bool {
        use RuleEntity as RE;
        match (self, other) {
            (RE::Basic(x), e) => x == e,
            (RE::Not(x), e) => x != e,
            (RE::Any, _) => true,
        }
    }
}

pub struct Rule(([RuleEntity; 8], [Option<Entity>; 9]));

impl Rule {
    fn pre(&self) -> &[RuleEntity; 8] {
        &self.0.0
    }
    fn post(&self) -> &[Option<Entity>; 9] {
        &self.0.1
    }
}

impl From<([RuleEntity; 8], [Option<Entity>; 9])> for Rule {
    fn from(value: ([RuleEntity; 8], [Option<Entity>; 9])) -> Self {
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
        .all(|(n, &r)| r == Into::<Entity>::into(n))
    { return; }

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
                new_buf[new_pos.as_idx(width, height)].entity_tag = new_atom.into();
                new_buf[new_pos.as_idx(width, height)].obsolete = true;
                new_buf[new_pos.as_idx(width, height)].priority = 2;
            }
            cnt += 1;
        }
    }
}

// ------------- Shaders --------------
pub fn nothing_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    Ok(()) // No-op my beloved <3
}


pub fn missing_shader(_index: usize, _attach: &mut AttachmentsForApply) -> Result<(), ()> {
    printinfo("Tag wasn't defined in the protocol in the GitHub Wiki:");
    Ok(())
}

pub fn ant_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    use RuleEntity as RE;
    use Entity as E;
    let walk_right: Rule = (
        [ RE::Any, RE::Any, RE::Any, RE::Any, RE::Any, RE::Any, RE::Basic(E::Dirt), RE::Any, ],
        [ None, None, None, None, Some(E::Dirt.into()), None, None, Some(E::Ant.into()), None, ],
    ).into();

    let walk_down: Rule = (
        [ RE::Any, RE::Any, RE::Any, RE::Any, RE::Basic(E::Dirt), RE::Any, RE::Basic(E::Dirt), RE::Any, ],
        [ None, None, None, None, Some(E::Dirt.into()), Some(E::Ant.into()), None, None, None, ],
    ).into();

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

pub fn fire_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    todo!("TRIED TO APPLY FIRE SHADER, BUT NOT YET IMPLEMENTED");
    Ok(())
}

pub fn grass_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    todo!("TRIED TO APPLY FIRE SHADER, BUT NOT YET IMPLEMENTED");
    Ok(())
}


pub fn water_shader(index: usize, attach: &mut AttachmentsForApply) -> Result<(), ()> {
    let current_pos = Position::from_index(index, attach.width, attach.height);
    let water_neighs = find_neighbours_of_buffer(
        0, attach.buffers, current_pos, attach.width, attach.height
    );
    //let water_neighs = water_neighs.iter()
    //    .zip(current_pos.neighbours(attach.width, attach.height))
    //    .map(|(&h, p)| {
    //        if attach.old_logic_buffer[index].entity_tag != Entity::Nothing.into() { i64::MAX } else { h }
    //});
    let current_height = unsafe { *attach.buffers.add(index + 0*attach.width*attach.height) };
    let indexes_of_lower: Vec<usize> = water_neighs.iter()
        .enumerate()
        .filter(|(_i, &h)| h < current_height)
        .map(|(index, _)| index)
        .collect();

    let mut tl_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut tr_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut bl_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut br_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut left_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut center_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut right_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut top_rule: Rule = ([RE::Any; 8], [None; 9]).into();
    let mut down_rule: Rule = ([RE::Any; 8], [None; 9]).into();

    tl_rule.0.1[0]      = Some(Entity::Water.into());
    top_rule.0.1[1]     = Some(Entity::Water.into());
    tr_rule.0.1[2]      = Some(Entity::Water.into());
    left_rule.0.1[3]    = Some(Entity::Water.into());
    center_rule.0.1[4]  = Some(Entity::Water.into());
    right_rule.0.1[5]   = Some(Entity::Water.into());
    bl_rule.0.1[6]      = Some(Entity::Water.into());
    down_rule.0.1[7]    = Some(Entity::Water.into());
    br_rule.0.1[8]      = Some(Entity::Water.into());


    tl_rule.0.1[4]    = Some(Entity::Water.into());
    tr_rule.0.1[4]    = Some(Entity::Water.into());
    bl_rule.0.1[4]    = Some(Entity::Water.into());
    br_rule.0.1[4]    = Some(Entity::Water.into());
    top_rule.0.1[4]   = Some(Entity::Water.into());
    left_rule.0.1[4]  = Some(Entity::Water.into());
    right_rule.0.1[4] = Some(Entity::Water.into());
    down_rule.0.1[4]  = Some(Entity::Water.into());

    for indextobewater in indexes_of_lower {
        match indextobewater {
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
    }

    Ok(())
}
