use legion::prelude::*;

pub trait ThreadLocalSystem {
    fn new(world: &mut World, resources: &mut Resources) -> Self;
    fn run(&mut self, world: &mut World, resources: &mut Resources);
}
