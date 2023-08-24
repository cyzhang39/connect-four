mod frontend;
mod scripts;

use frontend::ui::{Connect4};

fn main() {
    yew::start_app::<Connect4>();
}
    