mod builder;
mod menu;
pub fn init() -> State {
    State::Menu(menu::Menu::new())
}

pub async fn update(mut state: State) -> Option<State> {
    if let Some(exit_code) = match &mut state {
        State::Menu(menu) => menu.update().await,
        State::Builder(builder) => builder.update().await,
    } {
        return match exit_code {
            0 => None,
            1 => Some(State::Menu(menu::Menu::new())),
            2 => Some(State::Builder(builder::Builder::new())),
            _ => {
                panic!();
            }
        };
    }
    Some(state)
}

pub enum State {
    Menu(menu::Menu),
    Builder(builder::Builder),
}
