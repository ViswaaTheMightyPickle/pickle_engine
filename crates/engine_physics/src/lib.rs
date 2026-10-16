use engine_core::{BodyState, ImportedObject, ObjectId, Transform};

// Adapter contract: Rapier internals must stay private to this crate.
// The initial methods are placeholders, not a working physics engine.
pub struct PhysicsWorld {
    objects: Vec<BodyState>,
    next_id: u64,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            next_id: 1,
        }
    }

    pub fn spawn(&mut self, _asset: &ImportedObject, at: Transform) -> ObjectId {
        let id = ObjectId(self.next_id);
        self.next_id += 1;
        self.objects.push(BodyState { id, transform: at });
        // TODO (Luis): create Rapier body and mask-based collider.
        id
    }

    pub fn step(&mut self, _dt_seconds: f32) {
        // TODO (Luis): step Rapier, then update object transforms.
    }

    pub fn erase(&mut self, _id: ObjectId, _local_x: u32, _local_y: u32) -> Result<(), String> {
        // TODO (Luis): update structural occupancy and collider.
        Err("erase is not implemented yet".to_string())
    }

    pub fn body_states(&self) -> Vec<BodyState> {
        self.objects.clone()
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}
