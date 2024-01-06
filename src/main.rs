use ecs::World;
use env_logger;

struct Res;

fn main() {
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::TRACE)
    .init();

    let mut world = World::default();

    world.add_resource(Res);
}
