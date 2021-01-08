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
            anchor_layout: AnchorLayout {
                anchors: Anchors {
                    left: 0.5,
                    right: 1.,
                    bottom: 0.,
                    top: 1.,
                },
                constraint: Constraint::Independent {
                    x: AxisConstraint::DoubleMargin(0., 0.),
                    y: AxisConstraint::DoubleMargin(0., 0.),
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    material: transparent.clone(),
                    anchor_layout: AnchorLayout {
                        anchors: Anchors::FULL,
                        constraint: Constraint::Independent {
                            x: AxisConstraint::DoubleMargin(5., 5.),
                            y: AxisConstraint::DoubleMargin(5., 5.),
                        },
                        children_spread: Some(SpreadConstraint {
                            margin: 5.,
                            direction: Direction::Down,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    for i in 0..8 {
                        parent
                            .spawn(NodeBundle {
                                material: gray.clone(),
                                anchor_layout: AnchorLayout {
                                    anchors: Anchors::FULL,
                                    child_constraint: Some(ChildConstraint {
                                        max_size: ConstraintSize::FromContent,
                                        min_size: ConstraintSize::FromContent,
                                        ..Default::default()
                                    }),
                                    padding: Rect::all(3.),
                                    children_spread: Some(SpreadConstraint {
                                        direction: Direction::Right,
                                        margin: 3.,
                                        ..Default::default()
                                    }),
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
                                            anchor_layout: AnchorLayout {
                                                anchors: Anchors::FULL,
                                                constraint: Constraint::Independent {
                                                    x: AxisConstraint::FromContentSize(
                                                        Alignment::Offset(0.),
                                                    ),
                                                    y: AxisConstraint::FromContentSize(
                                                        Alignment::Offset(0.),
                                                    ),
                                                },
                                                padding: Rect::all(2.),
                                                child_constraint: Some(ChildConstraint {
                                                    max_size: ConstraintSize::FromContent,
                                                    min_size: ConstraintSize::FromContent,
                                                    ..Default::default()
                                                }),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        })
                                        .with_children(|parent| {
                                            parent.spawn(TextBundle {
                                                anchor_layout: AnchorLayout {
                                                    anchors: Anchors::FULL,
                                                    constraint: Constraint::Independent {
                                                        x: AxisConstraint::FromContentSize(
                                                            Alignment::Offset(0.),
                                                        ),
                                                        y: AxisConstraint::FromContentSize(
                                                            Alignment::Offset(0.),
                                                        ),
                                                    },
                                                    ..Default::default()
                                                },
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
                        anchor_layout: AnchorLayout {
                            anchors: Anchors::FULL,
                            child_constraint: Some(ChildConstraint {
                                weight: 1.,
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        material: materials.add(Color::CRIMSON.into()),
                        ..Default::default()
                    });
                    parent.spawn(NodeBundle {
                        anchor_layout: AnchorLayout {
                            anchors: Anchors::FULL,
                            child_constraint: Some(ChildConstraint {
                                weight: 2.,
                                ..Default::default()
                            }),
                            ..Default::default()
                        },
                        material: materials.add(Color::DARK_GREEN.into()),
                        ..Default::default()
                    });
                    parent
                        .spawn(NodeBundle {
                            anchor_layout: AnchorLayout {
                                anchors: Anchors::FULL,
                                child_constraint: Some(ChildConstraint {
                                    weight: 3.,
                                    ..Default::default()
                                }),
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
            material: black.clone(),
            anchor_layout: AnchorLayout {
                anchors: Anchors {
                    left: 0.,
                    right: 0.5,
                    top: 0.7,
                    bottom: 0.7,
                },
                constraint: Constraint::Independent {
                    x: AxisConstraint::DoubleMargin(5., 5.),
                    y: AxisConstraint::Centered(300.),
                },
                padding: Rect::all(5.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(NodeBundle {
                material: materials.add(Color::NAVY.into()),
                anchor_layout: AnchorLayout {
                    anchors: Anchors::FULL,
                    constraint: Constraint::MaxAspect(Aspect::Value(2.5)),
                    ..Default::default()
                },
                ..Default::default()
            });
        });

    commands.spawn(NodeBundle {
        material: black.clone(),
        anchor_layout: AnchorLayout {
            anchors: Anchors {
                left: 0.,
                right: 0.5,
                top: 0.3,
                bottom: 0.3,
            },
            constraint: Constraint::Independent {
                x: AxisConstraint::DoubleMargin(5., 5.),
                y: AxisConstraint::Centered(300.),
            },
            children_spread: Some(SpreadConstraint {
                margin: 5.,
                direction: Direction::Right,
                ..Default::default()
            }),
            padding: Rect::all(5.),
            ..Default::default()
        },
        ..Default::default()
    });

    let parent = commands.current_entity().unwrap();
    commands
        .spawn(NodeBundle {
            material: materials.add(Color::SALMON.into()),
            anchor_layout: AnchorLayout {
                child_constraint: Some(ChildConstraint::default()),
                ..Default::default()
            },
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
        anchor_layout: AnchorLayout {
            child_constraint: Some(ChildConstraint::default()),
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
            anchor_layout: AnchorLayout {
                anchors: Anchors::FULL,
                constraint: Constraint::Independent {
                    x: AxisConstraint::DoubleMargin(i as f32, 5.),
                    y: AxisConstraint::DoubleMargin(5., 5.),
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

fn oscillator(mut targets: Query<(&mut AnchorLayout, &mut Oscillator)>, time: Res<Time>) {
    for (mut t, mut oscillator) in targets.iter_mut() {
        oscillator.size += time.delta_seconds() * 20. * oscillator.direction;
        if oscillator.size < oscillator.min {
            oscillator.direction = 1.;
        } else if oscillator.size > oscillator.max {
            oscillator.direction = -1.;
        }
        t.child_constraint.as_mut().unwrap().min_size = ConstraintSize::Pixels(oscillator.size);
        t.child_constraint.as_mut().unwrap().max_size = ConstraintSize::Pixels(oscillator.size);
    }
}
