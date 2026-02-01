use avian3d::prelude::*;
use bevy::{
    input::common_conditions::input_just_pressed,
    prelude::*,
    window::{CursorGrabMode, CursorOptions},
};
use bevy_ahoy::prelude::*;
use bevy_enhanced_input::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            EnhancedInputPlugin,
            AhoyPlugins::default(),
        ))
        .add_input_context::<PlayerInput>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                capture_cursor.run_if(input_just_pressed(MouseButton::Left)),
                release_cursor.run_if(input_just_pressed(KeyCode::Escape)),
            ),
        )
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Spawn the player character controller
    let player = commands
        .spawn((
            CharacterController::default(),
            Collider::cylinder(0.35, 0.9),  // Capsule-like shape
            Transform::from_xyz(0.0, 5.0, 0.0),
            // Visible mesh for the character
            Mesh3d(meshes.add(Capsule3d::new(0.35, 1.8))),
            MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
            // Configure inputs
            PlayerInput,
            actions!(PlayerInput[
                (
                    Action::<Movement>::new(),
                    DeadZone::default(),
                    bindings![
                        Cardinal::wasd_keys(),
                        Axial::left_stick()
                    ]
                ),
                (
                    Action::<Jump>::new(),
                    bindings![KeyCode::Space, GamepadButton::South],
                ),
                (
                    Action::<Crouch>::new(),
                    bindings![KeyCode::ControlLeft, GamepadButton::LeftTrigger2],
                ),
                (
                    Action::<RotateCamera>::new(),
                    Bindings::spawn((
                        Spawn((Binding::mouse_motion(), Scale::splat(0.1))),
                        Axial::right_stick().with((Scale::splat(4.0), DeadZone::default())),
                    ))
                ),
            ]),
        ))
        .id();

    // Spawn the camera
    commands.spawn((
        Camera3d::default(),
        CharacterControllerCameraOf::new(player),
    ));

    // Spawn ground plane
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::cuboid(50.0, 1.0, 50.0),
        Mesh3d(meshes.add(Cuboid::new(100.0, 2.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.5, 0.5))),
    ));

    // Spawn some stairs
    for i in 0..5 {
        let y = (i + 1) as f32 * 0.5;
        let z = (i + 1) as f32 * 2.0 - 10.0;
        commands.spawn((
            Transform::from_xyz(5.0, y, z),
            RigidBody::Static,
            Collider::cuboid(2.0, y, 1.0),
            Mesh3d(meshes.add(Cuboid::new(4.0, y * 2.0, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.7, 0.7, 0.8))),
        ));
    }

    // Spawn a ramp
    commands.spawn((
        Transform::from_xyz(-8.0, 1.0, 0.0).with_rotation(Quat::from_rotation_z(0.3)),
        RigidBody::Static,
        Collider::cuboid(4.0, 0.2, 3.0),
        Mesh3d(meshes.add(Cuboid::new(8.0, 0.4, 6.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.6, 0.8, 0.6))),
    ));

    // Spawn some pushable boxes
    for i in 0..3 {
        let x = (i as f32 - 1.0) * 3.0;
        commands.spawn((
            Transform::from_xyz(x, 3.0, 8.0),
            RigidBody::Dynamic,
            Collider::cuboid(1.0, 1.0, 1.0),
            Mesh3d(meshes.add(Cuboid::new(2.0, 2.0, 2.0))),
            MeshMaterial3d(materials.add(Color::srgb(0.9, 0.6, 0.2))),
        ));
    }

    // Lighting
    commands.spawn((
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        DirectionalLight {
            illuminance: 3000.0,
            shadows_enabled: true,
            ..default()
        },
    ));

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::srgb(1.0, 1.0, 1.0),
        brightness: 0.1,
    });
}

#[derive(Component, Default)]
pub(crate) struct PlayerInput;

fn capture_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.grab_mode = CursorGrabMode::Locked;
    cursor.visible = false;
}

fn release_cursor(mut cursor: Single<&mut CursorOptions>) {
    cursor.visible = true;
    cursor.grab_mode = CursorGrabMode::None;
}