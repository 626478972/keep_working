use orbtk::prelude::*;

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;

fn main() {
    Application::from_name("keep-working")
        .window(move |ctx| {
            Window::new()
                .title("keep-working")
                .position((100.0, 100.0))
                .size(372.0, 768.0)
                .resizeable(true)
                .child(MainView::new().title("Keep Working").build(ctx))
                .build(ctx)
        })
        .run();
}
