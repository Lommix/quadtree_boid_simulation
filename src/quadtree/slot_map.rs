

#[derive(Debug, Clone, Hash, Eq)]
pub struct SlotId {
    index: u64,
}

impl PartialEq for SlotId {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl SlotId {
    pub fn new(index: u64) -> Self {
        Self { index }
    }
}

#[derive(Debug)]
pub struct SlotMap<T> {
    data: Vec<Option<T>>,
    id_stack: Vec<SlotId>,
}

impl<T> SlotMap<T> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            id_stack: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> SlotId {
        match self.id_stack.pop() {
            Some(id) => {
                let cell = &mut self.data[id.index as usize];
                *cell = Some(value);
                id
            }
            None => {
                let id = SlotId::new(self.data.len() as u64);
                self.data.push(Some(value));
                id
            }
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.id_stack.clear();
    }

    pub fn get(&self, id: &SlotId) -> Option<&T> {
        self.data[id.index as usize].as_ref()
    }

    pub fn get_mut(&mut self, id: &SlotId) -> Option<&mut T> {
        self.data[id.index as usize].as_mut()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter().flatten()
    }

    pub fn update(&mut self, id: &SlotId, value: T) {
        self.data[id.index as usize] = Some(value);
    }

    pub fn remove(&mut self, id: SlotId) -> Option<T> {
        let idx = id.index as usize;
        self.id_stack.push(id);
        self.data[idx].take()
    }
}
