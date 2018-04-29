use ScoreBoard;
use amethyst::ecs::prelude::{Entity, ReadExpect, System, Write};
use amethyst::shrev::{EventChannel, ReaderId};
use amethyst::ui::{UiEvent, UiEventType};

pub struct CheatSystem {
    reader_id: Option<ReaderId<UiEvent>>,
}

impl CheatSystem {
    pub fn new() -> Self {
        CheatSystem { reader_id: None }
    }
}

impl<'s> System<'s> for CheatSystem {
    type SystemData = (
        Write<'s, EventChannel<UiEvent>>,
        Write<'s, ScoreBoard>,
        ReadExpect<'s, CheatButton>,
    );

    fn run(
        &mut self,
        (
            mut events,
            mut score_board,
            cheat_button,
        ): Self::SystemData,
    ) {
        if self.reader_id.is_none() {
            self.reader_id = Some(events.register_reader());
        }
        for e in events.read(self.reader_id.as_mut().unwrap()) {
            if e.target == cheat_button.button {
                match e.event_type {
                    UiEventType::ClickStop => {
                        score_board.score_right += 1;
                    }
                    _ => {}
                }
            }
        }
    }
}

pub struct CheatButton {
    pub button: Entity
}
