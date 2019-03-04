#![feature(test)]

extern crate test;
use test::Bencher;

extern crate alex;

extern crate ecs_bench;

use alex::*;

use ecs_bench::pos_vel::{Acceleration, Position, Velocity, N_POS, N_POS_PER_ACC, N_POS_PER_VEL};

#[derive(Debug)]
struct PosComp(Position);

#[derive(Debug)]
struct VelComp(Velocity);

#[derive(Debug)]
struct AccelComp(Acceleration);

struct VelSys;
impl VelSys {
    fn run(&mut self, w: &mut World) {
        w.join::<(&mut PosComp, &mut VelComp, &AccelComp)>()
            .for_each(|(p, v, a)| {
                p.0.x += v.0.x;
                p.0.y += v.0.y;
                v.0.x += a.0.x;
                v.0.y += a.0.y;
            });
    }
}

fn build() -> World {
    let mut w = World::new();

    // setup entities
    {
        let mut ents = Vec::new();

        for _ in 0..N_POS {
            ents.push(w.create_entity());
        }

        for i in 0..N_POS {
            let l = ents.len();
            let e = ents.swap_remove(rand::random::<usize>() % l);
            w.insert(e, PosComp(Position { x: 0.0, y: 0.0 }));
            if i % N_POS_PER_VEL == 0 {
                w.insert(e, VelComp(Velocity { x: 0.0, y: 0.0 }));
            }
            if i % N_POS_PER_ACC == 0 {
                w.insert(e, AccelComp(Acceleration { x: 0.0, y: 0.0 }));
            }
        }
    }

    w
}

#[bench]
fn bench_build(b: &mut Bencher) {
    b.iter(|| build());
}

#[bench]
fn bench_update(b: &mut Bencher) {
    let mut world = build();
    let mut sys = VelSys;

    b.iter(|| {
        sys.run(&mut world);
    });
}
