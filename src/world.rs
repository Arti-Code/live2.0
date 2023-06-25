use crossbeam::channel::{Receiver, Sender};
use nalgebra::{Unit, Complex, Isometry2};
use rapier2d::{prelude::*, na::Vector2}; 
use macroquad::prelude::*;
use std::f32::consts::PI;
//use crate::element::*;
use std::time::Duration;
use std::thread::sleep;
use crossbeam::*;
use macroquad::rand::*;
use crate::consts::*;

pub struct World {
    pub rigid_bodies: RigidBodySet,
    pub colliders: ColliderSet,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    physics_pipeline: PhysicsPipeline,
    island_manager: IslandManager,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    impulse_joint_set: ImpulseJointSet,
    multibody_joint_set: MultibodyJointSet,
    ccd_solver: CCDSolver,
    physics_hooks: (),
    event_handler: ChannelEventCollector,
    //collision_send: Sender<CollisionEvent>,
    collision_recv: Receiver<CollisionEvent>,
}

impl World {
    pub fn new() -> Self {
        let (collision_send, collision_recv) = crossbeam::channel::unbounded();
        let collision_send2 = collision_send.clone();
        //let collision_recv2 = collision_recv.clone();
        let (contact_force_send, contact_force_recv) = crossbeam::channel::unbounded();
        let contact_force_send2 = contact_force_send.clone();
        let event_handler = ChannelEventCollector::new(collision_send, contact_force_send);
        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, 0.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            //collision_send: collision_send,
            event_handler: event_handler,
            //event_handler: ChannelEventCollector::new(collision_send2, contact_force_send2),
            collision_recv: collision_recv,
        }
    }
    
    pub fn build(&mut self) {
        println!("build");
        let cx = WORLD_W/2.0;
        let cy = WORLD_H/2.0;
        let edges1 = RigidBodyBuilder::fixed()
            //.position(Isometry::new(Vector2::new(cx, cy), 0.0))
            .build();
        let edges2 = RigidBodyBuilder::fixed().position(Isometry::new(Vector2::new(cx, cy), 0.0))
            .build();
        let edges3 = RigidBodyBuilder::fixed().position(Isometry::new(Vector2::new(cx, cy), 0.0))
            .build();
        let edges4 = RigidBodyBuilder::fixed().position(Isometry::new(Vector2::new(cx, cy), 0.0))
            .build();
        let edge_left = ColliderBuilder::cuboid(100.0, WORLD_H)
            .position(Isometry::new(Vector2::new(0.0, cy), 0.0))
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_FIXED)
            .build();
        let edge_right = ColliderBuilder::cuboid(100.0, WORLD_H)
            .position(Isometry::new(Vector2::new(WORLD_W, cy), 0.0))
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_FIXED)
            .build();
        let edge_top = ColliderBuilder::cuboid(WORLD_W, 100.0)
            .position(Isometry::new(Vector2::new(cx, 0.0), 0.0))
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_FIXED)
            .build();
        let edge_down = ColliderBuilder::cuboid(WORLD_W, 100.0)
            .position(Isometry::new(Vector2::new(cx, WORLD_H), 0.0))
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_FIXED)
            .build();
        let edge_handle1 = self.rigid_bodies.insert(edges1);
        //let edge_handle2 = self.rigid_bodies.insert(edges2);
        //let edge_handle3 = self.rigid_bodies.insert(edges3);
        //let edge_handle4 = self.rigid_bodies.insert(edges4);
        
        _ = self.colliders.insert_with_parent(edge_left, edge_handle1, &mut self.rigid_bodies);
        _ = self.colliders.insert_with_parent(edge_right, edge_handle1, &mut self.rigid_bodies);
        _ = self.colliders.insert_with_parent(edge_top, edge_handle1, &mut self.rigid_bodies);
        _ = self.colliders.insert_with_parent(edge_down, edge_handle1, &mut self.rigid_bodies);
    }

    pub fn add_circle_body(&mut self, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        //let iso = Isometry::new(Vector2::new(0.0, 0.0), 0.0);
        let ball = RigidBodyBuilder::dynamic()
            //.linvel(Vector2::new(gen_range(-1.0, 1.0), gen_range(-1.0, 1.0))*20.0)
            .position(iso).linear_damping(0.0)
            .can_sleep(false);
        let mut collider = ColliderBuilder::ball(radius)
            .active_collision_types(ActiveCollisionTypes::default() | ActiveCollisionTypes::DYNAMIC_DYNAMIC | ActiveCollisionTypes::DYNAMIC_FIXED)
            //.active_events(ActiveEvents::COLLISION_EVENTS)
            .restitution(1.0).friction(0.0)
            .restitution_combine_rule(CoefficientCombineRule::Max).friction_combine_rule(CoefficientCombineRule::Min)
            .build();
        //collider.set_active_events(ActiveEvents::COLLISION_EVENTS);
        let rb_handle = self.rigid_bodies.insert(ball);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        return rb_handle;
    }
    
    fn reciv_events(&self) {
        while let Ok(collision_event) = self.collision_recv.try_recv() {
            println!("COLLISION!");
        }
    }

    pub fn remove_physics_object(&mut self, body_handle: RigidBodyHandle) {
        _ = self.rigid_bodies.remove(body_handle, &mut self.island_manager, &mut self.colliders, &mut self.impulse_joint_set, &mut self.multibody_joint_set, true);
    }

    pub fn get_physics_obj_num(&self) -> usize {
        let body_num = self.rigid_bodies.len();
        return body_num;
    }

    pub fn step_physics(&mut self) {
        for (rbh, rb) in self.rigid_bodies.iter_mut() {
            let mut pos = Vec2::new((rb.position().translation.x-WORLD_W/2.0).to_owned(), (rb.position().translation.y-WORLD_H/2.0).to_owned());
            let r2 = pos.distance(Vec2::new(WORLD_W/2.0, WORLD_H/2.0));
            let gvec2 = pos.normalize();
            let f = GRAV * (gvec2/r2);
            rb.reset_forces(true);
            rb.add_force(-Vector2::new(f.x, f.y), true);
        }
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
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
        self.reciv_events();
    }

    fn iso_to_vec2_rot(&self, isometry: &Isometry<Real>) -> (Vec2, f32) {
        let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
        let rot = isometry.rotation.angle()+PI;
        return (pos, rot);
    }

    pub fn get_physics_data(&self, handle: RigidBodyHandle) -> PhysicsData {
        let rb = self.rigid_bodies.get(handle).expect("handle to non-existent rigid body");
        let iso = rb.position();
        let (pos, rot) = self.iso_to_vec2_rot(iso);
        let data = PhysicsData {position: pos, rotation: rot};
        return data;
    }
}


pub struct PhysicsData {
    pub position: Vec2,
    pub rotation: f32,
}