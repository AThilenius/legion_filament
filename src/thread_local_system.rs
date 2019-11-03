use legion::prelude::*;

pub trait ThreadLocalSystem {
    fn new(world: &mut World) -> Self;
    fn run(&mut self, world: &mut World);
}
