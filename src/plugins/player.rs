use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct PlayerComponent;

#[derive(Component)]
struct PlayerMovement {
    speed: f32,
    // acceleration: f32,
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<PlayerComponent>)>,
    mut player_query: Query<(&mut Transform, &PlayerMovement), With<PlayerComponent>>
) {
    for (mut player_transform, player_movement) in player_query.iter_mut() {
        let camera = match camera_query.get_single() {
            Ok(camera) => camera,
            Err(e) => Err(format!("Error retrieving the camera: {}", e))
                .unwrap(),
        };
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::K) {
            direction += camera.forward();
        }
        if keys.pressed(KeyCode::J) {
            direction += camera.back();
        }
        if keys.pressed(KeyCode::H) {
            direction += camera.left();
        }
        if keys.pressed(KeyCode::L) {
            direction += camera.right();
        }
        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * time.delta_seconds() * player_movement.speed;
        player_transform.translation += movement;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::RED.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        PlayerMovement {
            speed: 5.0,
            // acceleration: 5.0,
        },
        PlayerComponent,
    );
    commands.spawn(player);
}
