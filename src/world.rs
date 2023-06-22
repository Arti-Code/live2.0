use nalgebra::{Unit, Complex, Isometry2};
use rapier2d::{prelude::*, na::Vector2}; 
use macroquad::prelude::*;
use std::f32::consts::PI;
//use crate::element::*;
use std::time::Duration;
use std::thread::sleep;

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
    event_handler: (),
}

impl World {
    pub fn new() -> Self {
        Self {
            rigid_bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            gravity: Vector2::new(0.0, -1.0),
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
            physics_hooks: (),
            event_handler: (),
        }
    }
    
    /* pub fn example(&mut self) {
        let ground_collider = ColliderBuilder::cuboid(100.0, 0.1).restitution(0.7).build();
        self.colliders.insert(ground_collider);
        let ball = RigidBodyBuilder::dynamic()
          .translation(vector![0.0, 10.0])
          .build();
        let ball_collider = ColliderBuilder::ball(0.5).restitution(0.9).build();
        let ball_body_handle = self.rigid_bodies.insert(ball);
        self.colliders.insert_with_parent(ball_collider, ball_body_handle, &mut self.rigid_bodies);
        for _ in 0..200 {
            self.step_physics();
            let ball_body = &self.rigid_bodies[ball_body_handle];
            println!("Ball altitude: {}",ball_body.translation().y);
            sleep(Duration::from_secs_f32(0.3));
        }
    } */
    
    pub fn add_circle_body(&mut self, position: &Vec2, radius: f32) -> RigidBodyHandle {
        let iso = Isometry::new(Vector2::new(position.x, position.y), 0.0);
        let ball = RigidBodyBuilder::dynamic()
            .position(iso);
        let collider = ColliderBuilder::ball(radius).build();
        let rb_handle = self.rigid_bodies.insert(ball);
        let coll_handle = self.colliders.insert_with_parent(collider, rb_handle, &mut self.rigid_bodies);
        return rb_handle;
    }

    pub fn run(&mut self) {
        self.step_physics();
        self.draw();
        sleep(Duration::from_secs_f32(0.3));
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
            None,
            &self.physics_hooks,
            &self.event_handler,
        );
        //sleep(Duration::from_secs_f32(0.3));
    }

    pub fn draw(&self) {
        for (_, rb) in self.rigid_bodies.iter() {
            let isometry = rb.position();
            if let Some(collider_handle) = rb.colliders().first() {
                if let Some(shape) = self.colliders.get(*collider_handle) {
                    let pos = Vec2::new(isometry.translation.x, isometry.translation.y);
                    match shape.shape().shape_type() {
                        ShapeType::Ball => {
                            let r = shape.shape().as_ball().unwrap().radius;
                            draw_circle(pos.x, pos.y, r, GREEN);
                            println!("CIRCLE: x:{} y:{}", pos.x, pos.y)
                        },
                        ShapeType::Cuboid => {
                            let hx = shape.shape().as_cuboid().unwrap().half_extents.data.0[0][0];
                            let hy = shape.shape().as_cuboid().unwrap().half_extents.data.0[0][1];
                            draw_rectangle(pos.x-hx, pos.y-hy, 2.0*hx, 2.0*hy, YELLOW);
                        },
                        _ => {},
                    }
                    
                    
                }
            }
        }
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