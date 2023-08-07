use valence::{prelude::*, DefaultPlugins};
use vastboard::*;

const SPAWN_Y: i32 = 64;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VastboardPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, init_clients)
        .run();
}

fn setup(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -25..25 {
        for x in -25..25 {
            layer
                .chunk
                .set_block([x, SPAWN_Y, z], BlockState::GRASS_BLOCK);
        }
    }

    commands.spawn(layer);
}

fn init_clients(
    mut commands: Commands,
    mut clients: Query<
        (
            Entity,
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, With<ChunkLayer>>
) {
    let layer = layers.single();

    for(
        entity,
        _,
        mut layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
    ) in &mut clients {
        layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);
        pos.set([0.5, SPAWN_Y as f64 + 1.0, 0.5]);
        *game_mode = GameMode::Creative;

        create_vastboard(&mut commands, entity, "Test".into(), vec![
            "Test".to_string(),
            "Test 2".to_string(),
            "Test 3".to_string(),
            "Test 4".to_string(),
            "Test 5".to_string(),
            "Test 6".to_string(),
        ]);
    }
}