use amethyst::ecs::prelude::{System, Write, WriteExpect};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiButton, UiEvent, UiEventType};

pub struct StartButtonSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl StartButtonSystem {
    pub fn new() -> Self {
        StartButtonSystem { reader_id: None }
    }
}

impl<'s> System<'s> for StartButtonSystem {
    type SystemData = (
        Write<'s, EventChannel<UiEvent>>,
        WriteExpect<'s, StartButton>,
    );

    fn run(&mut self, (mut events, mut start_button): Self::SystemData) {
        let image = start_button.button.image;
        if self.reader_id.is_none() {
            self.reader_id = Some(events.register_reader());
        }
        for e in events.read(self.reader_id.as_mut().unwrap()) {
            if e.target == image {
                match e.event_type {
                    UiEventType::ClickStop => start_button.is_clicked = true,
                    _ => {}
                }
            }
        }
    }
}

pub struct StartButton {
    pub button: UiButton,
    pub is_clicked: bool,
}
