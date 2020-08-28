use specs::*;
use crate::alias::*;
use crate::components::*;
use crate::systems::ActiveCamera;
use crate::systems::ScreenDimensions;

pub struct PhysicsSystem {}

const MAX_ACCEPTABLE_VELOCITY: f32 = 5.0;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (Entities<'s>,
                       Read<'s, ActiveCamera>,
                       Read<'s, ScreenDimensions>,
                       WriteStorage<'s, Transform>,
                       ReadStorage<'s, Tile>,
                       WriteStorage<'s, Culled>,
                       WriteStorage<'s, Movement>);

    fn run(&mut self, (entities, active_camera, screen_size, mut transforms, tiles, mut culled_ents, mut movements) : Self::SystemData) {

        // Get Screen Bottom Left and Top Right
        let camera_transform = transforms.get(active_camera.entity.unwrap()).unwrap();
        let camera_size = (screen_size.x * (camera_transform.scale.x + 0.5), screen_size.y * (camera_transform.scale.y + 0.5) );
        
        let bot_left = Point2::new(camera_transform.position.x - (camera_size.0 / 2.0), 
                                   camera_transform.position.y - (camera_size.1 / 2.0));

        let top_right = Point2::new(camera_transform.position.x + (camera_size.0 / 2.0), 
                                    camera_transform.position.y + (camera_size.1 / 2.0));

        for (entity, transform, _tile) in (&entities, &transforms, &tiles).join() {
    
            if bot_left.x  < transform.position.x && bot_left.y  < transform.position.y &&
               top_right.x > transform.position.x && top_right.y > transform.position.y {
                culled_ents.remove(entity);
            } else {
                culled_ents.insert(entity, Culled {}).expect("Could not add 'Culled' Tag");
            }

        }

        // Calculate movement of all "movement" objects in world.
        for (_entity, transform, movement) in (&entities, &mut transforms, &mut movements).join() {
            movement.velocity += movement.acc; // Increment velocity by acceleration each frame.
            movement.acc *= 0.0; // Set acceleration to 0 each frame, prevents cumulative acceleration.
            // Prevents velocity of entity exceeding certain limit.
            clamp_magnitude(&mut movement.velocity, MAX_ACCEPTABLE_VELOCITY);
            transform.position += movement.velocity;
        }

    }
}

/// Takes mutable reference to Vector2 and limits its magnitude to specified value - preserves direction.
pub fn clamp_magnitude(vec: &mut Vector2, max_value: f32) {
    let magnitude: f32 = vec.magnitude().into();
    if magnitude > max_value {
        vec[0] = vec[0] * (max_value / magnitude);
        vec[1] = vec[1] * (max_value / magnitude);
    }
}