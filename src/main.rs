use bevy::{prelude::*, utils::HashMap};
use leafwing_input_manager::prelude::*;
use strum::EnumIter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_startup_system(setup)
        .add_system(skill_mapping)
        .add_system(fireball)
        .add_system(mov)
        .run();
}

fn setup(mut commands: Commands) {
    use KeyCode::*;
    use PlayerAction::*;
    use Skill::*;

    let mut input_map = InputMap::default();
    input_map.insert(Skill1, A);
    input_map.insert(Skill2, O);
    input_map.insert(OpenInventory, I);

    let mut skill_mapping = SkillSlotMapping::default();
    skill_mapping.insert(Skill1, Fireball);

    commands
        .spawn_bundle(InputManagerBundle {
            input_map,
            ..InputManagerBundle::default()
        })
        .insert_bundle(InputManagerBundle::<Skill>::default())
        .insert(skill_mapping);
}

fn skill_mapping(
    mut query: Query<(
        &ActionState<PlayerAction>,
        &mut ActionState<Skill>,
        &SkillSlotMapping,
    )>,
) {
    for (actions, mut skills, mapping) in query.iter_mut() {
        PlayerAction::iter()
            .filter_map(|action| mapping.get(&action).map(|skill| (action, skill)))
            .for_each(|(action, skill)| {
                if actions.state(action) != skills.state(*skill) {
                    skills.set_state(*skill, actions.state(action))
                }
            });
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
enum PlayerAction {
    Skill1,
    Skill2,
    OpenInventory,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, EnumIter)]
enum Skill {
    Fireball,
    Move,
}

#[derive(Component, Default)]
struct SkillSlotMapping(HashMap<PlayerAction, Skill>);

impl SkillSlotMapping {
    pub fn get(&self, action: &PlayerAction) -> Option<&Skill> {
        self.0.get(action)
    }

    pub fn insert(&mut self, action: PlayerAction, skill: Skill) -> Option<Skill> {
        self.0.insert(action, skill)
    }

    pub fn remove(&mut self, action: &PlayerAction) -> Option<Skill> {
        self.0.remove(action)
    }
}

fn fireball(query: Query<&ActionState<Skill>>) {
    use Skill::*;

    for skills in query.iter() {
        if skills.just_pressed(Fireball) {
            println!("Fireball!");
        } else if skills.just_released(Fireball) {
            println!("No fireball :c");
        }
    }
}

fn mov(query: Query<&ActionState<Skill>>) {
    use Skill::*;

    for skills in query.iter() {
        if skills.just_pressed(Move) {
            println!("Move!");
        } else if skills.just_released(Move) {
            println!("No move :c");
        }
    }
}
