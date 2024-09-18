mod modules {
    pub mod lesson2 {
        pub mod guessing_game;
    }
    pub mod lesson3 {
        pub mod learning;
    }
}

fn main() {
    if false {
        modules::lesson2::guessing_game::guessing_game();
    }

    modules::lesson3::learning::learnings();
}
