use crate::v2::V2;

#[derive(Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Inertia {
    pub velocity: V2,
    pub force: V2,
    pub pos: V2,
    pub mass: i32,
    pub elasticity: f64, // 0..1
    pub collision_stats: usize,
}

fn fixup_mass(mass: i32) -> i32 {
    if mass < 0 {
        panic!("Mass cannot be negative: {mass}");
    }
    if mass == 0 {
        return 10_000_000; // large mass
    }
    return mass;
}

fn inverse_mass(mass: f64) -> f64 {
    if mass.abs() < 0.000001 {
        return 0.0;
    }
    return 1.0 / (mass as f64);
}

impl Inertia {
    pub fn collide(inertia1: &Inertia, inertia2: &Inertia) -> (Inertia, Inertia) {
        let m1 = fixup_mass(inertia1.mass) as f64;
        let m2 = fixup_mass(inertia2.mass) as f64;

        let v1 = inertia1.velocity;
        let v2 = inertia2.velocity;

        let x1 = inertia1.pos;
        let x2 = inertia2.pos;

        let distance = x2.minus(x1).magnitude();
        let normal = x2.minus(x1).cdiv(distance);
        let v_rel = v2.minus(v1).dot(normal);
        if v_rel > 0.0 {
            // objects are moving away from each other
            return (*inertia1, *inertia2);
        }
        let e = inertia1.elasticity.min(inertia2.elasticity);
        let j = (m1 * m2) / (m1 + m2) * (1.0 + e) * (v_rel);

        let u1 = normal.cmul(j / m1).plus(v1);
        let u2 = normal.cmul(-j / m2).plus(v2);

        let im1 = inverse_mass(m1);
        let im2 = inverse_mass(m2);

        let penetration = 1.0 - distance; // 1.0 = "radius"
        let slop = 0.02;
        let pos_correct = if penetration > slop {
            normal.cmul((penetration - slop) / (im1 + im2)).cmul(0.1)
        } else {
            V2::zero()
        };

        let uf1 = if inertia1.mass == 0 { v1 } else { u1 };
        let uf2 = if inertia2.mass == 0 { v2 } else { u2 };

        let p1 = if inertia1.mass == 0 {
            x1
        } else {
            x1.minus(pos_correct.cmul(im1))
        };
        let p2 = if inertia2.mass == 0 {
            x2
        } else {
            x2.plus(pos_correct.cmul(im2))
        };

        let inertia1_new = Inertia {
            pos: p1,
            velocity: uf1,
            ..*inertia1
        };
        let inertia2_new = Inertia {
            pos: p2,
            velocity: uf2,
            ..*inertia2
        };
        (inertia1_new, inertia2_new)
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
