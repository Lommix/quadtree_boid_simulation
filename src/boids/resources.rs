use bevy::prelude::*;
use crate::quadtree::{tree::QuadTree, coord::Coord, region::Region};
use super::components::*;


#[derive(Resource)]
pub struct BoidUniverse {
    pub graph: QuadTree<Body>,
    pub speration: f32,
    pub cohesion: f32,
    pub alignment: f32,
    pub vision: f32,
    pub speed: f32,
    pub show_graph: bool,
    pub boid_count : u32,
}

impl BoidUniverse {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        let _min = Coord::from_f32(min.x, min.y);
        let _max = Coord::from_f32(max.x, max.y);
        Self {
            graph: QuadTree::new(Region::new(_min, _max)),
            speration: 0.0,
            cohesion: 0.0,
            speed: 1.0,
            vision : 1.0,
            alignment: 0.0,
            boid_count : 0,
            show_graph: true,
        }
    }
}
