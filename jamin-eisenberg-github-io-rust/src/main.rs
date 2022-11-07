use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CirclesPlugin)
        .add_plugin(ShapePlugin)
        .run();
}

pub struct CirclesPlugin;

impl Plugin for CirclesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_system)
            .add_system(greet_people);
    }
}

fn add_circles(mut commands: Commands) {
    commands
        .spawn()
        .insert(Circle)
        .insert(Radius(100))
        .insert(Frequency(1));
    commands
        .spawn()
        .insert(Circle)
        .insert(Radius(50))
        .insert(Frequency(-2));
}

fn greet_people(query: Query<&Circle, (With<Radius>, With<Frequency>)>) {
    for name in query.iter() {
        println!("hello {:?}!", name);
    }
}

#[derive(Component, Debug)]
struct RotatingVector;

#[derive(Component)]
struct Frequency(i32);

#[derive(Component)]
struct Vector {
    x: i32,
    y: i32,
}

fn setup_system(mut commands: Commands) {
    commands
        .spawn()
        .insert(RotatingVector)
        .insert(Vector { x: 100, y: 0 })
        .insert(Frequency(1));
    // shapes::Circle {
    //     radius: 0.0,
    //     center: Default::default()
    // }
    // let shape = shapes::RegularPolygon {
    //     sides: 6,
    //     feature: shapes::RegularPolygonFeature::Radius(200.0),
    //     ..shapes::RegularPolygon::default()
    // };
    //
    // commands.spawn_bundle(Camera2dBundle::default());
    // commands.spawn_bundle(GeometryBuilder::build_as(
    //     &shape,
    //     DrawMode::Outlined {
    //         fill_mode: FillMode::color(Color::CYAN),
    //         outline_mode: StrokeMode::new(Color::BLACK, 10.0),
    //     },
    //     Transform::default(),
    // ));
}
