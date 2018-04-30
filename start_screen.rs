use pong::Pong;
use amethyst::assets::Loader;
use amethyst::ecs::prelude::World;
use amethyst::prelude::*;
use amethyst::ui::{Anchor, Anchored, TtfFormat, UiText, UiTransform, UiButtonBuilder,
                    UiButtonResources};

use systems::StartButton;

pub struct StartScreen;

impl State for StartScreen {
    fn on_start(&mut self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
            );
        let button_builder = {
        // Until we can borrow immutably whilst also borrowing mutably, we need to restrict this
        // lifetime
        UiButtonBuilder::new("start_btn", "Start", UiButtonResources::from_world(&world))
            .with_uitext(UiText::new(
                font,
                "Start".to_string(),
                [0.2, 0.2, 1.0, 1.0],
                20.,
            ))
            .with_transform(UiTransform::new(
                "btn_transform".to_string(),
                0.0,
                32.0,
                -1.0,
                128.0,
                64.0,
                9,
            ))
            .with_anchored(Anchored::new(Anchor::Middle))
        };
        let button = button_builder.build_from_world(world);
        world.add_resource(StartButton { button, is_clicked: false })
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let start_button = world.read_resource::<StartButton>();
        if start_button.is_clicked {
            return Trans::Switch(Box::new(Pong));
        }
        Trans::None
    }
}
