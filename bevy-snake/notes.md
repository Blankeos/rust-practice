Bevy uses ECS (Entity, Component, System).

```rust
// Component. - derive from component. Can be queried.
#[derive(Component)]
struct Position {
x: f32,
y: f32,
}

// System - regular functions. Can do
// "commands", "queries" (powerful)
fn print_position_system(query: Query<&Position>) {
for position in &query {
println!("position: {} {}", position.x, position.y);
}
}

App::new().add_systems()

// Entity - any data type.
struct Entity(u64);

// --- Other stuff ---

// Plugins. - Can abstract stuff in app
pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    build(&self, app: &mut App) {
        app.add_systems(...);
        app.add_systems(...);
    }
}

App::new().add_plugins(HelloPlugin);

// Resourecs. - global data stuff.
```
