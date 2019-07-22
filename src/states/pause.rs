use amethyst::prelude::*;
use amethyst::input::get_key;
use log::info;

pub struct PauseState;

impl SimpleState for PauseState {
    fn on_start(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Pause")
    }

    fn on_stop(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        info!("Unpause")
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
                return Trans::Pop;
            }
        }
        Trans::None
    }
}
