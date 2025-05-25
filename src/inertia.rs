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

fn fixup_mass(mass: i32) -> i32 {
    if mass < 0 {
        panic!("Mass cannot be negative: {mass}");
    }
    if mass == 0 {
        return 10_000_000; // large mass
    }
    return mass;
}

fn inverse_mass(mass: i32) -> f64 {
    if mass == 0 {
        return 0.0;
    }
    return 1.0 / (mass as f64);
}

impl Inertia {
    pub fn collide(inertia1: &Inertia, inertia2: &Inertia) -> (Inertia, Inertia) {
        //before: Inertia { velocity: V2 { x: -0.010527879132272405, y: 0.3453112992445176 }, force: V2 { x: 0.0, y: 0.1 }, pos: V2 { x: 26.110962326548314, y: 121.58459575217232 }, mass: 1, elasticity: 0.2, collision_stats: 9 }
        //   Inertia { velocity: V2 { x: 0.0, y: 0.0 }, force: V2 { x: 0.0, y: 0.0 }, pos: V2 { x: 27.0, y: 122.0 }, mass: 0, elasticity: 1.0, collision_stats: 3 }

        let rel_velocity = inertia1.velocity.minus(inertia2.velocity);
        let normal = inertia1.pos.minus(inertia2.pos); // Vector from 2 to 1

        let distance = normal.magnitude();

        let normal_direction = if distance == 0.0 {
            // This is a fallback; in real scenarios, you might want a more robust way
            // to handle perfect overlap or avoid it entirely.
            // For now, let's assume it's rare and doesn't cause the primary issue.
            V2 { x: 1.0, y: 0.0 }
        } else {
            normal.cdiv(distance) // This is the unit normal from 2 to 1
        };

        // Calculate the relative velocity along the normal (projected onto the normal)
        // This value is positive if objects are separating, negative if approaching
        let separating_velocity = rel_velocity.dot(normal_direction);

        let e = 0.0; // Coefficient of restitution
        let effective_collision_vel = f64::min(0.0, separating_velocity);

        let impulse_magnitude = effective_collision_vel * -(1.0 - e);

        let m1 = fixup_mass(inertia1.mass);
        let m2 = fixup_mass(inertia2.mass);

        let im1 = inverse_mass(m1);
        let im2 = inverse_mass(m2);

        let impulse = impulse_magnitude / (im1 + im2); // This is the scalar magnitude of the impulse
        let pos_correct = V2::zero();
        let inertia1_new = Inertia {
            pos: inertia1.pos.plus(pos_correct.cmul(im1)),
            velocity: inertia1.velocity.plus(normal_direction.cmul(impulse * im1)),
            ..*inertia1
        };
        let inertia2_new = Inertia {
            pos: inertia2.pos.minus(pos_correct.cmul(im2)),
            velocity: inertia2
                .velocity
                .minus(normal_direction.cmul(impulse * im2)),
            ..*inertia2
        };

        // check for conservation of momentum
        check_conservation(
            inertia1,
            inertia2,
            rel_velocity,
            normal,
            normal_direction,
            separating_velocity,
            impulse,
            inertia1_new,
            inertia2_new,
        );
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

fn check_conservation(
    inertia1: &Inertia,
    inertia2: &Inertia,
    rel_velocity: V2,
    normal: V2,
    normal_direction: V2,
    separating_velocity: f64,
    impulse: f64,
    inertia1_new: Inertia,
    inertia2_new: Inertia,
) {
    let momentum1 = inertia1.velocity.cmul(inertia1.mass as f64);
    let momentum2 = inertia2.velocity.cmul(inertia2.mass as f64);
    let momentum_total = momentum1.plus(momentum2);
    let momentum1_new = inertia1_new.velocity.cmul(inertia1_new.mass as f64);
    let momentum2_new = inertia2_new.velocity.cmul(inertia2_new.mass as f64);
    let momentum_total_new = momentum1_new.plus(momentum2_new);
    let epsilon = 0.01;
    if (momentum_total.x.abs() + epsilon) < momentum_total_new.x.abs()
        || (momentum_total.y.abs() + epsilon) < momentum_total_new.y.abs()
    {
        panic!(
            "Momentum not conserved: {momentum_total:?} != {momentum_total_new:?}
                 rel_velocity: {rel_velocity:?}, normal: {normal:?}, normal_direction: {normal_direction:?},
                 separating_velocity: {separating_velocity:?}, impulse: {impulse:?}
                 before: {inertia1:?}
                         {inertia2:?}
                 after: {inertia1_new:?}, {inertia2_new:?}
                 "
        );
    }
}
