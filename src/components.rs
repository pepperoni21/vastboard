use valence::{prelude::*, scoreboard::ObjectiveScores};

#[derive(Bundle)]
pub struct VastboardBundle {
    pub data: VastboardData,
    pub title: VastboardTitle,
    pub scores: VastboardScores
}

impl VastboardBundle {

    pub fn new(client: Entity, title: Text, lines: Vec<String>) -> Self {
        VastboardBundle {
            data: VastboardData {
                objective: None,
                client
            },
            title: VastboardTitle(title),
            scores: VastboardScores(lines)
        }
    }

}

#[derive(Component)]
pub struct VastboardData {
    pub objective: Option<Entity>,
    pub client: Entity
}

#[derive(Component)]
pub struct VastboardTitle(pub Text);

impl VastboardTitle {
    pub fn update(&mut self, title: Text) {
        self.0 = title;
    }
}

#[derive(Component)]
pub struct VastboardScores(pub Vec<String>);

impl VastboardScores {
    pub fn update(&mut self, scores: Vec<String>) {
        self.0 = scores;
    }

    pub fn to_objective_scores(&self) -> ObjectiveScores {
        let mut objective_scores = ObjectiveScores::new();
        let mut index = self.0.len() as i32;
        for score in self.0.iter() {
            objective_scores.insert(score, index);
            index -= 1;
        }
        objective_scores
    }
}