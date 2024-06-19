use crate::v2::V2;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Inertia {
    pub velocity: V2,
    pub force: V2,
    pub pos: V2,
    pub mass: i32,
    pub elasticity: f64, // 0..1
    pub collision_stats: usize,
}

fn inverse_mass(mass: i32) -> f64 {
    if mass == 0 {
        return 0.0;
    }
    return 1.0 / (mass as f64);
}

impl Inertia {
    pub fn collide(inertia1: &Inertia, inertia2: &Inertia) -> (Inertia, Inertia) {
        let rel_velocity = inertia1.velocity.minus(inertia2.velocity);
        let normal = inertia1.pos.minus(inertia2.pos);
        // coefficient of restitution
        let e = inertia1.elasticity.min(inertia2.elasticity);

        // let collision_vel = rel_velocity.dot(normal);
        let collision_vel: f64 = rel_velocity.dot(normal) as f64 * -(1.0 + e);

        // for simplicity the rest here treats them as circles, not boxes:
        let distance = normal.magnitude();

        let normal_direction = if distance == 0.0 {
            // the two are perfectly aligned on top of each other
            V2 { x: 1.0, y: 0.0 }
        } else {
            normal.cdiv(distance)
        };

        let im1 = inverse_mass(inertia1.mass);
        let im2 = inverse_mass(inertia2.mass);

        let penetration = 1.0 - distance; // 1.0 = "radius"
        let slop = 0.02;
        let pos_correct = normal_direction
            .cmul((penetration - slop) / (im1 + im2))
            .cmul(0.4);

        let impulse = collision_vel / (im1 + im2);

        /*  log!("rel_velocity: {rel_velocity:?}");
        log!("norm: {normal:?}");
        log!("collision_vel: {collision_vel:?}"); */

        (
            Inertia {
                pos: inertia1.pos.plus(pos_correct.cmul(im1)),
                velocity: inertia1.velocity.plus(normal_direction.cmul(impulse * im1)),
                ..*inertia1
            },
            Inertia {
                pos: inertia2.pos.minus(pos_correct.cmul(im2)),
                velocity: inertia2
                    .velocity
                    .minus(normal_direction.cmul(impulse * im2)),
                ..*inertia2
            },
        )
    }

    pub fn is_collision(inertia1: &Inertia, inertia2: &Inertia) -> bool {
        // collision between infinite masses?!
        if (inertia1.mass == 0) && (inertia2.mass == 0) {
            return false;
        }

        let normal = inertia1.pos.minus(inertia2.pos);
        let radius = 1.0; // they're actually boxes but ok
        if normal.magnitude_sqr() > radius * radius {
            return false;
        }

        let rel_velocity = inertia1.velocity.minus(inertia2.velocity);

        // if the dot product is negative, the two objects are colliding,
        let dot = rel_velocity.dot(normal);

        //log!("checking collision: dot: {dot:?}\n1: {inertia1:?}\n2: {inertia2:?}");

        if dot >= 0.0 {
            // moving away from each other
            return false;
        }
        if dot * dot < 0.00001 {
            // negligible velocity (floating point error)
            return false;
        }

        return true;
    }
}
