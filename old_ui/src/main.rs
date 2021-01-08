use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(start.system())
        .add_system(oscillator.system())
        .run()
}

fn start(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let transparent = materials.add(Color::NONE.into());
    let dark_gray = materials.add(Color::DARK_GRAY.into());
    let gray = materials.add(Color::GRAY.into());
    let black = materials.add(Color::BLACK.into());

    commands.spawn(CameraUiBundle::default());
    commands
        .spawn(NodeBundle {
            material: black.clone(),
            style: Style {
                size: Size {
                    width: Val::Percent(50.),
                    height: Val::Percent(100.),
                },
                position: Rect {
                    left: Val::Percent(50.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    material: transparent.clone(),
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        flex_direction: FlexDirection::ColumnReverse,
                        padding: Rect {
                            top: Val::Px(0.),
                            ..Rect::all(Val::Px(5.))
                        },
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for i in 0..8 {
                        parent
                            .spawn(NodeBundle {
                                material: gray.clone(),
                                style: Style {
                                    padding: Rect::all(Val::Px(3.)),
                                    margin: Rect {
                                        top: Val::Px(5.),
                                        ..Default::default()
                                    },
                                    flex_shrink: 1.,
                                    flex_direction: FlexDirection::Row,
                                    ..Default::default()
                                },
                                ..Default::default()
                            })
                            .with_children(|parent| {
                                const J_MAX: i32 = 2;
                                for j in 0..J_MAX {
                                    parent
                                        .spawn(NodeBundle {
                                            material: dark_gray.clone(),
                                            style: Style {
                                                padding: Rect::all(Val::Px(2.)),
                                                margin: Rect {
                                                    right: Val::Px(5.),
                                                    ..Default::default()
                                                },
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                text: Text {
                                                    value: format!(
                                                        "Row {}, Column {}, Box {}",
                                                        i + 1,
                                                        j + 1,
                                                        i * J_MAX + j + 1
                                                    ),
                                                    font: asset_server
                                                        .load("fonts/FiraSans-Bold.ttf"),
                                                    style: TextStyle {
                                                        font_size: 30.,
                                                        color: Color::WHITE,
                                                        ..Default::default()
                                                    },
                                                },
                                                ..Default::default()
                                            });
                                        });
                                }
                            });
                    }

                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 1.,
                            margin: Rect {
                                top: Val::Px(5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: materials.add(Color::CRIMSON.into()),
                        ..Default::default()
                    });
                    parent.spawn(NodeBundle {
                        style: Style {
                            flex_grow: 2.,
                            margin: Rect {
                                top: Val::Px(5.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: materials.add(Color::DARK_GREEN.into()),
                        ..Default::default()
                    });
                    parent
                        .spawn(NodeBundle {
                            style: Style {
                                flex_grow: 3.,
                                margin: Rect {
                                    top: Val::Px(5.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            material: materials.add(Color::MIDNIGHT_BLUE.into()),
                            ..Default::default()
                        })
                        .with(Oscillator {
                            min: 100.,
                            max: 300.,
                            size: 100.,
                            direction: 1.,
                        });
                });
        });

    commands
        .spawn(NodeBundle {
            material: transparent.clone(),
            style: Style {
                position_type: PositionType::Absolute,
                margin: Rect {
                    right: Val::Percent(50.),
                    top: Val::Px(-150.0),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(300.),
                    width: Val::Percent(50.),
                    ..Default::default()
                },
                position: Rect {
                    top: Val::Percent(30.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    material: black.clone(),
                    style: Style {
                        margin: Rect {
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                            ..Default::default()
                        },
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        padding: Rect::all(Val::Px(5.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn(NodeBundle {
                        material: materials.add(Color::NAVY.into()),
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                            aspect_ratio: Some(2.5),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        });
    
    let mut _p = None;
    commands
        .spawn(NodeBundle {
            material: transparent.clone(),
            style: Style {
                position_type: PositionType::Absolute,
                margin: Rect {
                    right: Val::Percent(50.),
                    bottom: Val::Px(-150.0),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(300.),
                    width: Val::Percent(50.),
                    ..Default::default()
                },
                position: Rect {
                    bottom: Val::Percent(30.),
                    ..Default::default()
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    material: black.clone(),
                    style: Style {
                        margin: Rect {
                            left: Val::Px(5.),
                            right: Val::Px(5.),
                            ..Default::default()
                        },
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        padding: Rect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                });

            _p = Some(parent.current_entity().unwrap());
        });

    let parent = _p.unwrap();
    commands
        .spawn(NodeBundle {
            material: materials.add(Color::SALMON.into()),
            ..Default::default()
        })
        .with(Oscillator {
            size: 50.,
            direction: 1.,
            min: 50.,
            max: 100.,
        });
    let child1 = commands.current_entity().unwrap();

    commands.spawn(NodeBundle {
        material: gray.clone(),
        style: Style {
            flex_grow: 1.,
            margin: Rect {
                left: Val::Px(5.),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    });
    let child2 = commands.current_entity().unwrap();

    commands.push_children(parent, &[child1, child2]);
    let mut parent = child2;

    for i in (1..=20).rev() {
        commands.spawn(NodeBundle {
            material: materials.add(
                Color::rgb_linear((i % 2) as f32, ((i + 1) % 2) as f32, (i % 3) as f32 / 2.).into(),
            ),
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Auto),
                margin: Rect {
                    left: Val::Px(i as f32),
                    ..Rect::all(Val::Px(5.))
                },
                ..Default::default()
            },
            ..Default::default()
        });

        let child = commands.current_entity().unwrap();
        commands.push_children(parent, &[child]);
        parent = child;
    }
}

struct Oscillator {
    size: f32,
    direction: f32,
    min: f32,
    max: f32,
}

fn oscillator(mut targets: Query<(&mut Style, &mut Oscillator)>, time: Res<Time>) {
    println!("FRAME!");
    for (mut t, mut oscillator) in targets.iter_mut() {
        oscillator.size += time.delta_seconds() * 20. * oscillator.direction;
        if oscillator.size < oscillator.min {
            oscillator.direction = 1.;
        } else if oscillator.size > oscillator.max {
            oscillator.direction = -1.;
        }
        t.flex_basis = Val::Px(oscillator.size);
    }
}
