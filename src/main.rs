use std::io::{stdin, stdout, Read, Write};

// https://hoverbear.org/2016/10/12/rust-state-machine-pattern/

enum State {
    Start,
    First,
    Second,
    Third,
    Fourth,
    Stop
}

struct Engine {
    state: State
}

impl Engine {
    fn new() -> Self {
        Engine {
            state: State::Start
        }
    }

    fn update_state(&mut self) {
        // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
        match self.state {
            State::Start {..} => {
                self.state = State::First;
                println!("Entering First State...");
            }
            State::First {..} => {
                self.state = State::Second;
                println!("Entering Second State...");
            }
            State::Second {..} => {
                self.state = State::Third;
                println!("Entering Third State...");
            }
            State::Third {..} => {
                self.state = State::Fourth;
                println!("Entering Fourth State...");
            }
            State::Fourth {..} => {
                self.state = State::Stop;
                println!("Entering Stop State...");
            }
            State::Stop {..} => {
                println!("Staying in Stop State until reset");
                pause();
                self.state = State::Start;
            }
        };
    }


}

// https://www.reddit.com/r/rust/comments/8tfyof/noob_question_pause/
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to reset").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {
    let mut engine = Engine::new();
    loop{  
        engine.update_state();
    }
    
}