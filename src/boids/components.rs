use bevy::prelude::*;

use crate::quadtree::{coord::Coord, region::Region, slot_map::SlotId};

#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}

// quad render tag
#[derive(Component)]
pub struct QuadNodeRect;

#[derive(Component, Debug)]
pub struct Boid;

#[derive(Component, Debug)]
pub struct Cursor;

#[derive(Component, Debug)]
pub struct Collider {
    pub id: Option<SlotId>,
    pub radius: f32,
    pub nearby: usize,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            id: None,
            radius: radius,
            nearby: 0,
        }
    }
    pub fn into_region(&self, origin: Vec3) -> Region {
        let min =
            Coord::from_f32(origin.x, origin.y) - Coord::from_f32(self.radius, self.radius) / 2;
        let max =
            Coord::from_f32(origin.x, origin.y) + Coord::from_f32(self.radius, self.radius) / 2;

        Region::new(min, max)
    }
}

#[derive(Debug)]
pub struct Body {
    pub entity : Entity,
    pub position: Vec3,
    pub velocity: Vec3,
}
