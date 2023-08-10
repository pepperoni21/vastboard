use valence::{testing::ScenarioSingleClient, scoreboard::Objective, packet::packets::play::{ScoreboardObjectiveUpdateS2c, ScoreboardPlayerUpdateS2c}, prelude::Despawned};

use crate::{VastboardBundle, VastboardData, VastboardPlugin, VastboardTitle, VastboardLines};

#[test]
fn test_initialization() {
    let ScenarioSingleClient {
        mut app,
        client,
        mut helper,
        layer: _
    } = prepare();

    app.update();

    let vastboard_data = app.world.entity(client).get::<VastboardData>().unwrap();
    assert!(app.world.entity(vastboard_data.objective.unwrap()).contains::<Objective>());

    let packets = helper.collect_received();
    packets.assert_count::<ScoreboardObjectiveUpdateS2c>(1);
}

#[test]
fn test_title_update() {
    let ScenarioSingleClient {
        mut app,
        client,
        mut helper,
        layer: _
    } = prepare();

    app.update();

    helper.clear_received();

    let mut client_entity = app.world.entity_mut(client);
    let mut vastboard_title = client_entity.get_mut::<VastboardTitle>().unwrap();
    vastboard_title.update("New title".into());

    app.update();

    let packets = helper.collect_received();
    packets.assert_count::<ScoreboardObjectiveUpdateS2c>(1);
}

#[test]
fn test_lines_update() {
    let ScenarioSingleClient {
        mut app,
        client,
        mut helper,
        layer: _
    } = prepare();

    app.update();

    helper.clear_received();

    let mut client_entity = app.world.entity_mut(client);
    let mut vastboard_lines = client_entity.get_mut::<VastboardLines>().unwrap();
    vastboard_lines.update(vec![
        "New line".to_string()
    ]);

    app.update();

    let packets = helper.collect_received();
    packets.assert_count::<ScoreboardPlayerUpdateS2c>(3);
}

#[test]
fn test_vastboard_removal() {
    let ScenarioSingleClient {
        mut app,
        client,
        mut helper,
        layer: _
    } = prepare();

    app.update();

    helper.clear_received();

    let mut client_entity = app.world.entity_mut(client);

    let objective_ent = client_entity.get::<VastboardData>().unwrap().objective.unwrap();

    client_entity.remove::<(VastboardTitle, VastboardLines)>();

    app.update();

    assert!(app.world.get_entity(objective_ent).is_none());

    let packets = helper.collect_received();
    packets.assert_count::<ScoreboardObjectiveUpdateS2c>(1);
}

#[test]
fn test_objective_removal() {
    let ScenarioSingleClient {
        mut app,
        client,
        mut helper,
        layer: _
    } = prepare();

    app.update();

    helper.clear_received();

    let client_entity = app.world.entity(client);

    let objective_ent = client_entity.get::<VastboardData>().unwrap().objective.unwrap();
    app.world.entity_mut(objective_ent).insert(Despawned);

    app.update();

    let client_entity = app.world.entity_mut(client);
    assert!(client_entity.get::<VastboardData>().is_none());

    let packets = helper.collect_received();
    packets.assert_count::<ScoreboardObjectiveUpdateS2c>(1);
}

fn prepare() -> ScenarioSingleClient {
    let mut s = ScenarioSingleClient::new();

    s.app.add_plugins(VastboardPlugin);

    s.app.update();

    s.app.world.entity_mut(s.client).insert(VastboardBundle::new(s.client, "Test".into(), vec![
        "Test line".to_string()
    ]));

    s
}