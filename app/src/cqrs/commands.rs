#[derive(PartialEq, Clone)]
pub enum CounterCommand {
    Create {
        name: String,
        value: i32,
    },
    Increment {
        id: String,
        amount: i32,
    },
    Decrement {
        id: String,
        amount: i32,
    },
    Update {
        id: String,
        name: String,
        value: i32,
    },
    Delete {
        id: String,
    },
}
