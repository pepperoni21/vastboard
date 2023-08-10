pub mod components;
pub use components::*;

mod systems;
use systems::*;

use valence::registry::Commands;
use valence::prelude::*;

#[cfg(test)]
mod tests;

pub struct VastboardPlugin;

impl Plugin for VastboardPlugin {

    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            create_scoreboard,
            update_title,
            update_scores,
            remove_despawned_objectives_vastboards,
            remove_vastboards,
            remove_disconnected_clients_vastboards
        ));
    }

}

pub fn create_vastboard(commands: &mut Commands, client: Entity, title: Text, lines: Vec<String>) {
    let bundle = VastboardBundle::new(client, title, lines);

    commands.get_entity(client).unwrap().insert(bundle);
}

pub fn remove_vastboard(commands: &mut Commands, client: Entity) {
    let mut entity_commands = commands.get_entity(client).unwrap();
    entity_commands.remove::<(VastboardTitle, VastboardLines)>();
}