use engine_core::{BodyState, ImportedObject, ObjectId, STEP_SECONDS, Transform};
use rapier2d::prelude::*;

// Adapter contract: Rapier internals must stay private to this crate.
// The initial methods are placeholders, not a working physics engine.
pub struct PhysicsWorld {
    rigid_body_set: RigidBodySet,
    collider_set: ColliderSet,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhaseBvh,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    soft_body_set: SoftBodySet,
    ccd_solver: CCDSolver,

    objects: Vec<BodyState>,
    next_id: u64,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let mut integration_parameters = IntegrationParameters::default();
        //timestep length 60fps, i assume ts is what you mean by 60hz
        integration_parameters.dt = STEP_SECONDS;
        Self {
            rigid_body_set: RigidBodySet::new(),
            collider_set: ColliderSet::new(),
            integration_parameters,
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhaseBvh::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            soft_body_set: SoftBodySet::new(),
            ccd_solver: CCDSolver::new(),

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
        let gravity = vector![0.0, -9.81];
        let physics_hooks = ();
        let event_handler = ();

        self.physics_pipeline.step(
            gravity.into(),
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.soft_body_set,
            &mut self.ccd_solver,
            &physics_hooks,
            &event_handler,
        );
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

#[cfg(test)]
mod tests;
