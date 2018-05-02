use amethyst::assets::Loader;
use amethyst::ecs::prelude::{Entity, World};
use amethyst::prelude::*;
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::*;
use pong::Pong;

pub struct StartScreen {
    button_image: Option<Entity>,
    reader_id: Option<ReaderId<UiEvent>>,
}

impl StartScreen {
    pub fn new() -> StartScreen {
        StartScreen {
            button_image: None,
            reader_id: None,
        }
    }
}

impl State for StartScreen {
    fn on_start(&mut self, world: &mut World) {
        let font = world.read_resource::<Loader>().load(
            "font/square.ttf",
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );
        let button = UiButtonBuilder::new("start_btn", "Start", UiButtonResources::from_world(&world))
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
            .build_from_world(world);
        self.button_image = Some(button.image);
        let mut ui_events = world.write_resource::<EventChannel<UiEvent>>();
        self.reader_id = Some(ui_events.register_reader());
    }

    fn update(&mut self, world: &mut World) -> Trans {
        let mut start_button_clicked = false;
        let button_image = self.button_image.unwrap();
        let reader_id = self.reader_id.as_mut().unwrap();
        {
            let ui_events = world.read_resource::<EventChannel<UiEvent>>();
            for e in ui_events.read(reader_id) {
                if e.target == button_image {
                    match e.event_type {
                        UiEventType::Click => {
                            start_button_clicked = true;
                        }
                        _ => {}
                    }
                }
            }
        }
        if start_button_clicked {
            world.delete_all();
            Trans::Switch(Box::new(Pong))
        } else {
            Trans::None
        }
    }
}
