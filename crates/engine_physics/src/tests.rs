use super::PhysicsWorld; 
use rapier2d::prelude::*;

#[test]
fn test_headless_collision() {
    let mut world = PhysicsWorld::new();

    // Added .into() to the vectors here as well
    let floor_collider = ColliderBuilder::cuboid(10.0, 0.1)
        .translation(vector![0.0, -1.0].into()) 
        .build();
    world.collider_set.insert(floor_collider);

    let falling_body = RigidBodyBuilder::dynamic()
        .translation(vector![0.0, 5.0].into()) 
        .build();
    let body_handle = world.rigid_body_set.insert(falling_body);
    
    let box_collider = ColliderBuilder::cuboid(1.0, 1.0).build();
    world.collider_set.insert_with_parent(
        box_collider, 
        body_handle, 
        &mut world.rigid_body_set
    );

    for _ in 0..180 {
        world.step(1.0 / 60.0);
    }

    let body = &world.rigid_body_set[body_handle];
    let final_y_position = body.translation().y;
    
    assert!(
        final_y_position > -1.0, 
        "The box clipped through the floor! Final position: {}", 
        final_y_position
    );
}
