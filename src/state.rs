mod builder;
mod menu;
pub fn init() -> State {
    State::Menu(menu::Menu::new())
}

pub async fn update(state: State, dt: f32) -> State {
    let mut state = state;
    if let Some(change) = match &mut state {
        State::Menu(menu) => menu.update().await,
        State::Builder(builder) => builder.update().await,
    } {
        state = match change {
            0=>State::Menu(menu::Menu::new()),
            1=>State::Builder(builder::Builder::new()),
            _=>{panic!();}

        }

    }
    state
}

pub enum State {
    Menu(menu::Menu),
    Builder(builder::Builder),
}
