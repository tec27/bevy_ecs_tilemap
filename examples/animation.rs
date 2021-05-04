use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

mod helpers;

#[derive(Default)]
struct Animated {
    last_update: f64,
}

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(1042.0, 1024.0, 1000.0 - 0.1),
        ..OrthographicCameraBundle::new_2d()
    });

    let texture_handle = asset_server.load("tiles.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let mut map = Map::new(UVec2::new(4, 4), UVec2::new(32, 32), Vec2::new(16.0, 16.0), Vec2::new(96.0, 256.0), 0);
    let map_entity = commands.spawn().id();
    map.build(&mut commands, &mut meshes, material_handle, map_entity, true);
    commands.entity(map_entity).insert_bundle(MapBundle {
        map,
        ..Default::default()
    });

    let texture_handle = asset_server.load("flower_sheet.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    let mut map = Map::new(UVec2::new(4, 4), UVec2::new(16, 16), Vec2::new(32.0, 32.0), Vec2::new(32.0, 448.0), 1);
    let map_entity = commands.spawn().id();
    map.build(&mut commands, &mut meshes, material_handle, map_entity, true);
    for (_, entity) in map.get_all_tiles() {
        if let Some(entity) = entity {
            commands.entity(*entity).insert(Animated::default());
        }
    }
    commands.entity(map_entity).insert_bundle(MapBundle {
        map,
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()
    });
}

fn animate(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&UVec2, &mut Tile, &mut Animated)>,
    chunk_query: Query<&Chunk>,
    map_query: Query<&Map>,
) {
    let current_time = time.seconds_since_startup();
    for (position, mut tile, mut animated) in query.iter_mut() {
        if (current_time - animated.last_update) > 0.05 {
            tile.texture_index += 1;
            if tile.texture_index > 13 {
                tile.texture_index = 0;
            }
            animated.last_update = current_time;

            // TODO: Provide a better API for this.
            // We should be able to do something like:
            // `map.notify(commands, position, layer)`;
            // Have a hierarchy like: map entity > layer entities > chunk entities > tile entities
            if let Ok(chunk) = chunk_query.get(tile.chunk) {
                if let Ok(map) = map_query.get(chunk.map_entity) {
                    map.notify(&mut commands, *position);
                }
            }
        }
    }
}

fn main() {
    env_logger::Builder::from_default_env()
    .filter_level(log::LevelFilter::Info)
    .init();

    App::build()
        .insert_resource(WindowDescriptor {
            width: 1270.0,
            height: 720.0,
            title: String::from("Animated Map Example"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(TileMapPlugin)
        .add_startup_system(startup.system())
        .add_system(helpers::camera::movement.system())
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        .add_system(animate.system())
        .run();
}
