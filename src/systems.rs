use crate::components::*;
use crate::remove_vastboard;
use valence::registry::Commands;
use valence::prelude::*;
use valence::scoreboard::*;

pub fn create_scoreboard(
    mut commands: Commands,
    server: Res<Server>,
    mut clients: Query<(&mut VastboardData, &VastboardTitle, &VastboardLines, &UniqueId, &mut VisibleEntityLayers), Added<VastboardData>>
) {
    for (mut data, title, scores, uuid, mut visible_entity_layers) in clients.iter_mut() {
        let scoreboard_layer = commands.spawn(EntityLayer::new(&server)).id();

        let objective_entity_id = commands.spawn(ObjectiveBundle {
            name: Objective::new(format!("vastboard-{}", uuid.0)[..16].to_string()),
            display: ObjectiveDisplay(title.0.clone()),
            scores: scores.to_objective_scores(),
            layer: EntityLayerId(scoreboard_layer),
            ..Default::default()
        }).id();

        data.objective = Some(objective_entity_id);

        visible_entity_layers.0.insert(scoreboard_layer);
    }
}

pub fn update_title(
    mut clients: Query<(&mut VastboardTitle, &VastboardData), Changed<VastboardTitle>>,
    mut objectives_displays: Query<&mut ObjectiveDisplay>
) {
    for (title, data) in clients.iter_mut() {
        let objective_entity_id = match data.objective {
            Some(entity_id) => entity_id,
            None => continue
        };

        if let Ok(mut display) = objectives_displays.get_mut(objective_entity_id) {
            display.0 = title.0.clone();
        }
    }
}

pub fn update_scores(
    mut clients: Query<(&mut VastboardLines, &VastboardData), Changed<VastboardLines>>,
    mut objectives_scores: Query<&mut ObjectiveScores>
) {
    for (scores, data) in clients.iter_mut() {
        let objective_entity_id = match data.objective {
            Some(entity_id) => entity_id,
            None => continue
        };

        if let Ok(mut objective_scores) = objectives_scores.get_mut(objective_entity_id) {
            *objective_scores = scores.to_objective_scores();
        }
    }
}

pub fn remove_despawned_objectives_vastboards(
    mut commands: Commands,
    objectives: Query<(Entity, &Objective), With<Despawned>>,
    vastboards: Query<(Entity, &VastboardData)>
) {
    for (objective_ent, objective) in objectives.iter() {
        let objective_name = objective.name();
        if objective_name.starts_with("vastboard-") {
            for (vastboard_ent, vastboard) in vastboards.iter() {
                if let Some(objective_entity_id) = vastboard.objective {
                    if objective_entity_id == objective_ent {
                        let mut entity_commands = commands.entity(vastboard_ent);
                        entity_commands.remove::<(VastboardData, VastboardTitle, VastboardLines)>();
                    }
                }
            }
        }
    }
}

pub fn remove_vastboards(
    mut commands: Commands,
    mut removed_titles: RemovedComponents<VastboardTitle>,
    mut removed_scores: RemovedComponents<VastboardLines>,
    vastboards: Query<(Entity, &VastboardData)>
) {
    let mut treated_entities = Vec::new();

    for removed_title in removed_titles.iter() {
        let (client_ent, vastboard_data) = match vastboards.get(removed_title) {
            Ok(vastboard) => vastboard,
            Err(_) => continue
        };

        let objective_ent = match vastboard_data.objective {
            Some(objective_ent) => objective_ent,
            None => continue
        };

        let mut entity_commands = commands.entity(client_ent);
        entity_commands.remove::<(VastboardData, VastboardLines)>();

        let mut objective_commands = commands.entity(objective_ent);
        objective_commands.insert(Despawned);

        treated_entities.push(client_ent);
    }

    for removed_score in removed_scores.iter() {
        let (client_ent, vastboard_data) = match vastboards.get(removed_score) {
            Ok(vastboard) => vastboard,
            Err(_) => continue
        };

        if treated_entities.contains(&client_ent) {
            continue;
        }

        let objective_ent = match vastboard_data.objective {
            Some(objective_ent) => objective_ent,
            None => continue
        };

        let mut entity_commands = commands.entity(client_ent);
        entity_commands.remove::<(VastboardData, VastboardTitle)>();

        let mut objective_commands = commands.entity(objective_ent);
        objective_commands.insert(Despawned);
    }
}

pub fn remove_disconnected_clients_vastboards(
    mut commands: Commands,
    mut disconnected_clients: RemovedComponents<Client>
) {
    for client_ent in disconnected_clients.iter() {
        remove_vastboard(&mut commands, client_ent);
    }
}