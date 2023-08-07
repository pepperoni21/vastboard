# Vastboard
### A simple Valence plugin for making scoreboards easier.

# Installation
### Add Vastboard to your project
```toml
[dependencies]
vastboard = { git = "https://github.com/pepperoni21/vastboard" }
```
### Add Vastboard plugin to your app
```rust
use vastboard::VastboardPlugin;

---

App::new()
    .add_plugins((DefaultPlugins, VastboardPlugin))
```

# Usage
### Create a scoreboard
```rust
create_vastboard(
    &mut commands, // Bevy Commands
    client_entity, // Client bevy entity
    "Title".into(), // Title of scoreboard (Text)
    vec![
        "Line 1".to_string(), // Lines
        "Line 2".to_string(), // Lines
        "Line 3".to_string(), // Lines
        "Line 4".to_string(), // Lines
        "Line 5".to_string(), // Lines
        "Line 6".to_string(), // Lines
    ]
);
```
### Update the title/lines
You can update the title by updating the `VastboardTitle` component of the client entity.

In this example, we update the title to "Sneaking" when the player starts sneaking, and "Not sneaking" when the player stops sneaking:
```rust
pub fn example_system(
    mut sneaking: EventReader<SneakEvent>,
    mut clients: Query<(&mut VastboardTitle, &mut VastboardLines)>
) {
    for event in sneaking.iter() {
        if let Ok((mut title, mut lines)) = clients.get_mut(event.client) {

            title.update(match event.state {
                SneakState::Start => "Sneaking",
                SneakState::Stop => "Not sneaking"
            }.bold());

            lines.update(vec![
                match event.state {
                    SneakState::Start => "Sneaking".to_string(),
                    SneakState::Stop => "Not sneaking".to_string()
                }
            ]);
        }
    }
}
```

### Remove a scoreboard
```rust
remove_vastboard(
    &mut commands, // Bevy Commands
    client_entity // Client bevy entity
);
```