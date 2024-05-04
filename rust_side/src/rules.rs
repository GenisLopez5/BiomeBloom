use std::{collections::HashMap, hash::Hash};

use crate::*;
impl From<i64> for Entity {
    fn from(value: i64) -> Self {
        let x: Entity = unsafe { std::mem::transmute(value as u64) }; // Assumes it's in range
        x
    } 
}

impl From<Entity> for i64 {
    fn from(value: Entity) -> Self {
        value as i64
    } 
}


impl From<Atom> for DAtom {
    fn from(value: Atom) -> Self {
        Self {
            obsolete: value.obsolete,
            material: value.entity_tag
        }
    }
}

/// Map from Entity -> Vec<(Neighbours, [Option<Entity>; 9])>
/// Maps entity with its (needed) neighbours to its (maybe) changing surrounding neighbours
/// NOTE: They are both reversed!
fn primitive_rules() -> HashMap<EntityTag, Vec<([Option<EntityTag>; 8] ,[Option<EntityTag>; 9])>> {
    use Entity as E;
    HashMap::from([
        (E::Ant as i64, 
            vec![
                ([None,          None,           None,
                 None,                          None,
                 None, Some(E::Nothing.into()), None],

                [None,          None,           None,
                 None, Some(E::Nothing.into()), None,
                 None, Some(E::Ant.into()),     None]),


                ([None,         None,             None,
                 None,                   Some(E::Nothing.into()),
                 None,          None,             None],

                [None,          None,               None,
                 None, Some(E::Nothing.into()), Some(E::Ant.into()),
                 None,          None,                None])
            ]
        ),
        (E::Tnt.into(),
            vec![
                ([Some(E::Ant.into()), None, None, None, None, None, None, None], [None; 9]),
                ([None, Some(E::Ant.into()), None, None, None, None, None, None], [None; 9]),
                ([None, None, Some(E::Ant.into()), None, None, None, None, None], [None; 9]),
                ([None, None, None, Some(E::Ant.into()), None, None, None, None], [None; 9]),
                ([None, None, None, None, Some(E::Ant.into()), None, None, None], [None; 9]),
                ([None, None, None, None, None, Some(E::Ant.into()), None, None], [None; 9]),
                ([None, None, None, None, None, None, Some(E::Ant.into()), None], [None; 9]),
                ([None, None, None, None, None, None, None, Some(E::Ant.into())], [None; 9]),
            ]
        )
    ])
}

// Rules abstrction
pub fn apply_rules(logic_buffer: &Vec<Atom>, new_buf: &mut Vec<Atom>, index: usize, width: usize, height: usize) {
    let rules = primitive_rules();
    let rules = rules.get(&logic_buffer[index].entity_tag);
    if let Some(rules) = rules {
        let neighs = find_neighbours( index, logic_buffer.as_ptr(), width, height);

        for rule in rules {
            // Does rule apply?
            if !neighs.into_iter().zip(rule.0)
                .filter(|(_n, mb_r)| mb_r.is_some())
                .map(|(n, mb_r)| (n, mb_r.unwrap()))
                .all(|(n, r)| n == r) { continue; }

            // If we're here, the rule does apply
            let current_pos = Position::from_index(index, width, height);
            let new_atoms = rule.1;
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
    }
}


fn neighbour_count(buffer: &[Atom], index: usize) -> HashMap<EntityTag, usize> {
    let mut result = HashMap::new();
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; }
            let current = buffer[index];
            match result.get_mut(&current.entity_tag) {
                Some(v) => *v += 1,
                None => { result.insert(current.entity_tag, 1); },
            }
        }
    }

    result

}

/// Pre: Index refers to element in grid
/// Post: Eight element array of the eight neighbours (toroidal geometry) that surrounds the pixel at the given index
/// The indexes of the array look like:
/// ```txt
/// 012
/// 3X4
/// 567
/// ```
pub fn find_neighbours(index: usize, buffer: *const Atom, width: usize, height: usize) -> [EntityTag; 8] {
    let Position {x, y} = Position::from_index(index, width, height);
    let mut counter = 0;
    let mut result = [0; 8];
    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 { continue; }
            let x1: usize = (x + width  - 1 + j) % width;
            let y1: usize = (y + height - 1 + i) % height;
            let idx = Position::new(x1, y1).as_idx(width, height);
            result[counter] = unsafe { (*buffer.add(idx)).entity_tag };
            counter += 1;
        }
    }
    result
}


#[test]
fn test_neighbours() {
    let mut array = [Atom{entity_tag:0.into(), priority: 1, obsolete: false};12];
    for i in 0..12 {
        array[i] = Atom{entity_tag:i as i64, priority: 1, obsolete: false};
    }
    let tags = find_neighbours(2,  array.as_mut_ptr(), 4, 3);

    dbg!(tags);
}
