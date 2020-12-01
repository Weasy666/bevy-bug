use bevy::prelude::*;


fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Bevy Bug".to_string(),
            width: 1920,
            height: 1080,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .init_resource::<FontMaterials>()
        .add_startup_system(setup.system())
        .add_startup_system(grid_drawer.system())
        .run();
}

struct FontMaterials {
    names: Handle<Font>,
}

impl FromResources for FontMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let asset_server = resources.get_mut::<AssetServer>().expect(&format!("Could not find 'AssetServer'"));
        let font = asset_server.load("fonts/unispace.ttf");
        FontMaterials {
            names: font,
        }
    }
}

pub struct System;
pub struct MapGrid;

fn grid_drawer(
    mut commands: Commands,
    fonts: Res<FontMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>) {

    commands
        .spawn(
            SpriteComponents::default()
        )
        .with(MapGrid)
        .with_children(|parent| {
            parent
                .spawn(SpriteComponents::default())
                .with(System {})
                // This component creates a 'called `Option::unwrap()` on a `None` value', ..\.cargo\registry\src\github.com-1ecc6299db9ec823\bevy_ui-0.3.0\src\flex\mod.rs:151:64' error
                .spawn(NodeComponents {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::ColumnReverse,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                // One of the following 2 variants should work
                // Test Variant 1
                // .with_children(|parent| {
                //     parent.spawn(TextComponents {
                //         text: Text {
                //             value: name.to_string(),
                //             font: fonts.names.clone(),
                //             style: TextStyle {
                //                 font_size: 10.0,
                //                 color: Color::rgb(0.9, 0.9, 0.9),
                //                 ..Default::default()
                //             },
                //         },
                //         ..Default::default()
                //     });
                // })
                // Test Variant 2
                // .spawn(TextComponents {
                //     text: Text {
                //         value: name.to_string(),
                //         font: fonts.names.clone(),
                //         style: TextStyle {
                //             font_size: 10.0,
                //             color: Color::rgb(0.9, 0.9, 0.9),
                //             ..Default::default()
                //         },
                //     },
                //     ..Default::default()
                // });
                ;
        });
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default());
}
