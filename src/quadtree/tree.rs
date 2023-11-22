use bevy::utils::{HashMap, HashSet};

use super::{
    node::QuadNode,
    region::Region,
    slot_map::{SlotId, SlotMap},
};

#[derive(Debug)]
pub struct QuadTree<T> {
    region_store: SlotMap<Region>,
    value_store: SlotMap<T>,
    root: Box<QuadNode>,
}

impl<T> QuadTree<T> {
    pub fn new(region: Region) -> Self {
        Self {
            region_store: SlotMap::new(),
            value_store: SlotMap::new(),
            root: Box::new(QuadNode::new(region, 0)),
        }
    }

    pub fn value_count(&self) -> usize {
        self.value_store.len()
    }

    pub fn clear(&mut self) {
        self.region_store.clear();
        self.value_store.clear();
        self.root.clear();
    }

    pub fn query(&self, region: &Region, exclude: &Vec<SlotId>) -> Vec<&T> {
        let _distinct_result: HashMap<SlotId, SlotId> = HashMap::new();

        let set: HashSet<_> = self
            .root
            .query(region, &self.region_store, exclude)
            .drain(..)
            .collect();

        set.iter().map(|id| self.value_store.get(id).unwrap()).collect()
    }

    pub fn size(&self) -> &Region {
        self.root.size()
    }

    pub fn get_regions(&self) -> Vec<&Region> {
        self.root.get_regions()
    }

    pub fn insert(&mut self, region: Region, values: T) -> SlotId {
        let region_id = self.region_store.insert(region);
        let value_id = self.value_store.insert(values);

        assert!(region_id == value_id);

        self.root.insert(&value_id, &self.region_store);
        value_id
    }
}
