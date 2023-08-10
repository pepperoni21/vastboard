use valence::{prelude::*, DefaultPlugins, client::Ping};
use vastboard::*;

const SPAWN_Y: i32 = 64;

const FOOTER: &'static str = "play.myserver.com";
const UPDATE_INTERVAL: i8 = 1;
const HIGHLIGHT_COOLDOWN: i8 = 10;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, VastboardPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (init_clients, update_vastboards))
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

    commands.insert_resource(VastboardUpdateTimer::default());
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

        create_vastboard(&mut commands, entity, "Test".into(), vec![]);
    }
}

fn update_vastboards(
    mut timer: ResMut<VastboardUpdateTimer>,
    mut clients: Query<(&Username, &Ping, &mut VastboardLines)>
) {
    if clients.is_empty() {
        return;
    }
    
    if !timer.should_update() {
        timer.tick();
        return;
    }

    timer.reset();



    let footer = if timer.should_highlight() {
        let char = FOOTER.chars().nth(timer.highlighted_char).unwrap();
        let split = FOOTER.split_at(timer.highlighted_char);
        let split = (split.0, &split.1[1..]);
        let footer = format!("§e{}§6{}§e{}", split.0, char, split.1);

        if timer.highlighted_char >= FOOTER.len() - 1 {
            timer.reset_highlight();
        } else {
            timer.highlighted_char += 1;
        }

        footer
    } else {
        timer.tick_highlight();
        FOOTER.to_string()
    };

    for (Username(username), Ping(ping), mut lines) in clients.iter_mut() {
        lines.update(vec![
            "§c".to_string(),
            format!("Name: §e{username}"),
            "§b".to_string(),
            format!("Ping: §e{ping}ms"),
            "§a".to_string(),
            format!("Coins: §e250"),
            "§6".to_string(),
            format!("Rank: §eVIP"),
            "§d".to_string(),
            format!("§e{footer}"),
        ]);
    }
}

#[derive(Resource)]
pub struct VastboardUpdateTimer {
    value: i8,
    highlighted_char: usize,
    highlight_cooldown: i8,
}

impl VastboardUpdateTimer {
    pub fn tick(&mut self) {
        self.value -= 1;
    }

    pub fn should_update(&self) -> bool {
        self.value <= 0
    }

    pub fn reset(&mut self) {
        self.value = UPDATE_INTERVAL;
    }

    pub fn tick_highlight(&mut self) {
        self.highlight_cooldown -= 1;
    }

    pub fn should_highlight(&self) -> bool {
        self.highlight_cooldown <= 0
    }

    pub fn reset_highlight(&mut self) {
        self.highlight_cooldown = HIGHLIGHT_COOLDOWN;
        self.highlighted_char = 0;
    }
}

impl Default for VastboardUpdateTimer {
    fn default() -> Self {
        Self {
            value: 0,
            highlighted_char: 0,
            highlight_cooldown: 0,
        }
    }
}