use super::{
    region::Region,
    slot_map::{SlotId, SlotMap},
    MAX_CELL_SIZE, MAX_DEPTH,
};

#[derive(Debug)]
enum NodeType {
    Leaf,
    Parent(Box<[QuadNode; 4]>),
}

#[derive(Debug)]
pub struct QuadNode {
    region: Region,
    values: Vec<SlotId>,
    node_type: NodeType,
    depth: usize,
}

impl QuadNode {
    pub fn new(region: Region, depth: usize) -> Self {
        Self {
            values: Vec::new(),
            region,
            node_type: NodeType::Leaf,
            depth,
        }
    }

    pub fn size(&self) -> &Region {
        &self.region
    }

    pub fn clear(&mut self) {
        self.values.clear();
        self.node_type = NodeType::Leaf
    }

    pub fn query(
        &self,
        region: &Region,
        region_store: &SlotMap<Region>,
        exclude: &Vec<SlotId>,
    ) -> Vec<&SlotId> {
        match &self.node_type {
            NodeType::Leaf => self
                .values
                .iter()
                .filter(|id| {
                    region_store.get(id).unwrap().intersects(region) && !exclude.contains(id)
                })
                .collect(),
            NodeType::Parent(children) => children
                .iter()
                .filter(|child| child.region.intersects(region))
                .flat_map(|child| child.query(region, region_store, exclude))
                .collect(),
        }
    }

    pub fn get_regions(&self) -> Vec<&Region> {
        match &self.node_type {
            NodeType::Leaf => vec![&self.region],
            NodeType::Parent(children) => children
                .iter()
                .flat_map(|child| child.get_regions())
                .collect(),
        }
    }

    pub fn remove(&mut self, value: &SlotId, region_store: &SlotMap<Region>) {
        match &mut self.node_type {
            NodeType::Leaf => self.values.retain(|id| id != value),
            NodeType::Parent(children) => {
                children
                    .iter_mut()
                    .filter(|child| child.region.intersects(region_store.get(value).unwrap()))
                    .for_each(|child| child.remove(value, region_store));
                if self.value_count_rec() < MAX_CELL_SIZE {
                    let values = self.drain_values_rec();
                    self.node_type = NodeType::Leaf;
                    self.values.extend(values);
                }
            }
        }
    }

    pub fn value_count_rec(&self) -> usize {
        match &self.node_type {
            NodeType::Leaf => self.values.len(),
            NodeType::Parent(children) => children
                .iter()
                .fold(0, |acc, child| acc + child.value_count_rec()),
        }
    }

    pub fn drain_values_rec(&mut self) -> Vec<SlotId> {
        match &mut self.node_type {
            NodeType::Leaf => self.values.drain(..).collect(),
            NodeType::Parent(children) => children
                .iter_mut()
                .flat_map(|c| c.drain_values_rec())
                .collect(),
        }
    }

    pub fn insert(&mut self, value: &SlotId, region_store: &SlotMap<Region>) {
        match &mut self.node_type {
            NodeType::Leaf => {
                self.values.push(value.clone());

                if self.values.len() > MAX_CELL_SIZE && self.depth < MAX_DEPTH {
                    let _divide = self.region.quad_divide();
                    self.node_type = NodeType::Parent(Box::new(
                        self.region
                            .quad_divide()
                            .map(|reg| QuadNode::new(reg, self.depth + 1)),
                    ));
                    let ids: Vec<SlotId> = self.values.drain(..).collect();
                    for id in ids {
                        self.insert(&id, region_store);
                    }
                }
            }
            NodeType::Parent(children) => children
                .iter_mut()
                .filter(|child| child.region.intersects(region_store.get(value).unwrap()))
                .for_each(|child| child.insert(value, region_store)),
        }
    }
}
