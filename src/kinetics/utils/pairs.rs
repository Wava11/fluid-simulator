use bevy::prelude::*;

#[derive(Eq, Hash, PartialEq)]
pub struct UnorderedEntitiesPair {
    entities: (Entity, Entity),
}

impl UnorderedEntitiesPair {
    pub fn new(e1: Entity, e2: Entity) -> UnorderedEntitiesPair {
        if e1 < e2 {
            UnorderedEntitiesPair { entities: (e1, e2) }
        } else {
            UnorderedEntitiesPair { entities: (e2, e1) }
        }
    }
}