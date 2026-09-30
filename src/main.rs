use bevy::prelude::*;

pub struct HelloPlugin;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(HelloPlugin)
        .run();
}

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(5.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (change_name, greet_people).chain());
    }
}

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Chen".to_string())));
    commands.spawn((Person, Name("Anastasya".to_string())));
    commands.spawn((Person, Name("YangYang".to_string())));
}

fn change_name(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Chen" {
            name.0 = "Chency".to_string();
        }
    }
}

fn greet_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello, {}", name.0);
        }
    }
}
