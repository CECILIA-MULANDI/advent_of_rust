// 1. Define the states as zero-sized types
pub struct Empty;
pub struct Ready;
pub struct Flying;

// 2. Define the Sleigh struct as a generic over the state
pub struct Sleigh<State> {
    state: State,
}

// 3. Implement Sleigh methods for each state

// Empty State
impl Sleigh<Empty> {
    // Create a new Sleigh in the Empty state
    pub fn new() -> Self {
        Self { state: Empty }
    }

    // Transition from Empty to Ready
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: Ready }
    }
}

// Ready State
impl Sleigh<Ready> {
    // Transition from Ready to Flying
    pub fn take_off(self) -> Sleigh<Flying> {
        Sleigh { state: Flying }
    }

    // Transition from Ready to Empty
    pub fn unload(self) -> Sleigh<Empty> {
        Sleigh { state: Empty }
    }
}

// Flying State
impl Sleigh<Flying> {
    // Transition from Flying to Ready
    pub fn land(self) -> Sleigh<Ready> {
        Sleigh { state: Ready }
    }
}
