use crossbeam::channel::{Receiver, Sender};
use crossbeam::*;
use macroquad::prelude::*;
use nalgebra::{Complex, Isometry2, Unit};
use rapier2d::{na::Vector2, prelude::*};
use std::collections::{HashSet, HashMap};
use std::f32::consts::PI;
use std::thread::sleep;
use std::time::Duration;



pub struct World {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    query_pipeline: QueryPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: ChannelEventCollector,
    collision_recv: Receiver<CollisionEvent>,
    pub detections: HashMap<RigidBodyHandle, (RigidBodyHandle, f32)>,
}

impl World {
    pub fn new() -> Self {
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            query_pipeline: QueryPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: event_handler,
            collision_recv: collision_recv,
            detections: HashMap::new(),
        }
    }

    fn update_intersections(&mut self) {
        self.query_pipeline.update(&self.rigid_bodies, &self.colliders);
    }

    pub fn check_intersections(&self) {
        
    }

    pub fn add_circle_body(&mut self, key: u64, position: &Vec2, radius: f32, detection_range: Option<f32>) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let ball = RigidBodyBuilder::dynamic().position(iso).user_data(key as u128).build();
        let mut collider = ColliderBuilder::ball(radius)
            .active_collision_types(ActiveCollisionTypes::default())
            .active_events(ActiveEvents::COLLISION_EVENTS).build();
        let rb_handle = self.rigid_bodies.insert(ball);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        if detection_range.is_some() {
            let detector  = ColliderBuilder::ball(detection_range.unwrap()).sensor(true).build();
            _ = self.colliders.insert_with_parent(detector, rb_handle, &mut self.rigid_bodies);
        }
        return rb_handle;
    }

    fn is_collider_exist(&self, collider_handle: ColliderHandle) -> bool {
        let collider = self.colliders.get(collider_handle);
        match collider {
            Some(some_collider) => {
                return true;
            },
            None => {
                return false;
            }
        }
    }

    fn is_collider_sensor(&self, collider_handle: ColliderHandle) -> bool {
        let collider = self.colliders.get(collider_handle).unwrap();
        return collider.is_sensor();
    }

    fn get_object_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let collider = self.colliders.get(collider_handle);
        match collider {
            None => {
                return None;
            },
            Some(collider) => {
                return collider.parent();
            }
        }
    }

    fn reciv_events(&mut self) {
        while let Ok(collision_event) = self.collision_recv.try_recv() {
            match collision_event {
                CollisionEvent::Stopped(collider1_hand, collider2_hand, CollisionEventFlags::REMOVED) => {
                    println!("REMOVED");
                    let mut agent_hand: RigidBodyHandle;
                    if self.is_collider_exist(collider1_hand) {
                        if self.is_collider_sensor(collider1_hand) {
                            agent_hand = self.get_body_handle_from_collider(collider1_hand).unwrap();
                            self.detections.remove_entry(&agent_hand);
                        }
                    }
                    if self.is_collider_exist(collider2_hand) {
                        if self.is_collider_sensor(collider2_hand) {
                            agent_hand = self.get_body_handle_from_collider(collider2_hand).unwrap();
                            self.detections.remove_entry(&agent_hand);
                        }   
                    }
                },
                CollisionEvent::Started(c1, c2, CollisionEventFlags::SENSOR) => {
                    println!("DETECTION");
                    let mut agent_hand: RigidBodyHandle; 
                    let mut target_hand: RigidBodyHandle; 
                    if self.is_collider_sensor(c1) {
                        agent_hand = self.get_object_handle_from_collider(c1).unwrap();
                        target_hand = self.get_object_handle_from_collider(c2).unwrap();
                    } else if self.is_collider_sensor(c2) {
                        agent_hand = self.get_object_handle_from_collider(c2).unwrap();
                        target_hand = self.get_object_handle_from_collider(c1).unwrap();
                    } else {
                        return;
                    }
                    let target = self.rigid_bodies.get(target_hand).unwrap();
                    let agent = self.rigid_bodies.get(agent_hand).unwrap();
                    let pos1 = self.matric_to_vec2(agent.position().translation);
                    let pos2 = self.matric_to_vec2(target.position().translation);
                    let new_dist = pos1.distance(pos2);
                    match self.detections.get(&agent_hand) {
                        Some((target, dist)) => {
                            //let (actual_target, actual_dist) = self.detections.get(&agent_hand).unwrap();
                            let distance = *dist;
                            let new_distance = new_dist;
                            if new_distance < distance {
                                _ = self.detections.remove_entry(&agent_hand);
                                self.detections.insert(agent_hand, (target_hand, new_distance));
                            }
                        },
                        None => {
                            self.detections.insert(agent_hand, (target_hand, new_dist));
                        }
                    }
                },
                CollisionEvent::Stopped(c1, c2, CollisionEventFlags::SENSOR) => {
                    println!("CONTACT LOST");
                    let mut agent_hand: RigidBodyHandle; 
                    if self.is_collider_sensor(c1) {
                        agent_hand = self.get_object_handle_from_collider(c1).unwrap();
                        self.detections.remove_entry(&agent_hand);
                    } else if self.is_collider_sensor(c2) {
                        agent_hand = self.get_object_handle_from_collider(c2).unwrap();
                        self.detections.remove_entry(&agent_hand);
                    }
                },
                _ => {}
                //CollisionEvent::Stopped(c1, c2, _) => {
                    //println!("SEPARATION");
                //}
            }
        }
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(
            body_handle,
            &mut self.island_manager,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            true,
        );
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

    pub fn step_physics(&mut self) {
        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_bodies,
            &mut self.colliders,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &self.physics_hooks,
            &self.event_handler,
        );
        self.reciv_events();
    }

    fn iso_to_vec2_rot(&self, isometry: &Isometry<Real>) -> (Vec2, f32) {
        let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
        let rot = isometry.rotation.angle() + PI;
        return (pos, rot);
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicsData {
        let rb = self
            .rigid_bodies
            .get(handle)
            .expect("handle to non-existent rigid body");
        let iso = rb.position();
        let (pos, rot) = self.iso_to_vec2_rot(iso);
        let data = PhysicsData {
            position: pos,
            rotation: rot,
        };
        return data;
    }

    pub fn get_object_position(&self, handle: RigidBodyHandle) -> Option<Vec2> {
        let rb = self.rigid_bodies.get(handle);
        match rb {
            Some(body) => {
                let pos = Vec2::new(body.position().translation.x, body.position().translation.y);
                return Some(pos);
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_contacts2(&self, agent_body_handle: RigidBodyHandle) -> Option<RigidBodyHandle> {
        let target = self.detections.get(&agent_body_handle);
        match target {
            Some((tg, dst)) => {
                return Some(*tg);
            },
            None => {
                return None;
            }
        }
    }

    fn matric_to_vec2(&self, translation: Translation<Real>) -> Vec2 {
        let v = Vec2::new(translation.x, translation.y);
        return v;
    }

    fn get_body_handle_from_collider(&self, collider_handle: ColliderHandle) -> Option<RigidBodyHandle> {
        let mut collider: &Collider; 
        match self.colliders.get(collider_handle) {
            Some(col) => {
                collider = col;
            },
            None => {
                return None;
            }
        };
        match collider.parent() {
            Some(rbh) => {
                return Some(rbh);
            },
            None => {
                return None;
            }
        }
    }

    pub fn get_contacts(&self, agent_body_handle: RigidBodyHandle) -> Option<(f32, RigidBodyHandle)> {
        let mut contacts_norm: Vec<Vec2> = vec![];
        let rb = self.rigid_bodies.get(agent_body_handle).unwrap();
        let pos1 = Vec2::new(rb.position().translation.x, rb.position().translation.y);
        let mut dist: f32 = f32::INFINITY;
        let mut target: RigidBodyHandle = RigidBodyHandle::invalid();
        for c in rb.colliders() {
            let collider = self.colliders.get(*c).unwrap();
            if !collider.is_sensor() {
                continue;
            }
            let filter = QueryFilter { 
                flags: QueryFilterFlags::ONLY_KINEMATIC, 
                groups: None, 
                exclude_collider: Some(*c), 
                exclude_rigid_body: Some(agent_body_handle), 
                ..Default::default()
            };
            self.query_pipeline.intersections_with_shape(&self.rigid_bodies, &self.colliders, rb.position(), collider.shape(), filter, |collided| {
                println!("***");
                let col2 = self.colliders.get(collided).unwrap();
                let rb2_handle = col2.parent().unwrap();
                let rb2 = self.rigid_bodies.get(rb2_handle).unwrap();
                let pos2 = Vec2::new(rb2.position().translation.x, rb2.position().translation.y);
                let new_dist = pos2.distance(pos1);
                if new_dist < dist {
                    dist = new_dist;
                    target = rb2_handle;
                }
                return true;
            });
            //collider.shape().
            //self.query_pipeline.cast_shape(&self.rigid_bodies, &self.colliders, rb.position(), shape_vel, shape, max_toi, stop_at_penetration, filter)
        }
        if dist < f32::INFINITY {
            return  Some((dist, target));
        } else {
            return None;
        }
    }
}

pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
}
