#![feature(test)]

extern crate test;
use test::Bencher;

extern crate specs;

extern crate ecs_bench;

use specs::{
    Component, DenseVecStorage, Entity, Join, ReadStorage, RunNow, System, VecStorage, World,
    WriteStorage,
};

use ecs_bench::pos_vel::{Acceleration, Position, Velocity, N_POS, N_POS_PER_ACC, N_POS_PER_VEL};

struct PosComp(Position);
impl Component for PosComp {
    type Storage = DenseVecStorage<PosComp>;
}

struct VelComp(Velocity);
impl Component for VelComp {
    type Storage = DenseVecStorage<VelComp>;
}

struct AccelComp(Acceleration);
impl Component for AccelComp {
    type Storage = DenseVecStorage<AccelComp>;
}

struct VelSys;
impl<'a> System<'a> for VelSys {
    type SystemData = (
        ReadStorage<'a, AccelComp>,
        WriteStorage<'a, VelComp>,
        WriteStorage<'a, PosComp>,
    );
    fn run(&mut self, (acc, mut vel, mut pos): Self::SystemData) {
        (&mut pos, &mut vel, &acc).join().for_each(|(p, v, a)| {
            p.0.x += v.0.x;
            p.0.y += v.0.y;
            v.0.x += a.0.x;
            v.0.y += a.0.y;
        });
    }
}

fn build() -> (World, VelSys) {
    let mut w = World::new();
    w.register::<PosComp>();
    w.register::<VelComp>();
    w.register::<AccelComp>();

    // setup entities
    {
        let mut ents: Vec<Entity> = w.create_iter().take(N_POS).collect();

        let mut positions = w.write::<PosComp>();
        let mut velocities = w.write::<VelComp>();
        let mut accelerations = w.write::<AccelComp>();

        for i in 0..N_POS {
            let l = ents.len();
            let e = ents.swap_remove(rand::random::<usize>() % l);
            positions.insert(e, PosComp(Position { x: 0.0, y: 0.0 }));
            if i % N_POS_PER_VEL == 0 {
                velocities.insert(e, VelComp(Velocity { x: 0.0, y: 0.0 }));
            }
            if i % N_POS_PER_ACC == 0 {
                accelerations.insert(e, AccelComp(Acceleration { x: 0.0, y: 0.0 }));
            }
        }
    }

    let sys = VelSys;
    (w, sys)
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let (world, mut sys) = build();

    b.iter(|| {
        sys.run_now(&world.res);
    });
}
