use std::io;

// https://hoverbear.org/2016/10/12/rust-state-machine-pattern/

enum State {
    Start,
    First,
    Second,
    Third,
    Fourth,
    End
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
    
    fn update_state(&mut self, choice: char) {
        // https://doc.rust-lang.org/rust-by-example/flow_control/match.html
        match self.state {
            State::Start {..} => {
                match choice {
                    's' => self.state = State::First,
                    _ => (),
                }           
            }

            State::First {..} => {
                match choice {
                    's' => self.state = State::Second,
                    _ => (),
                }
            }

            State::Second {..} => {
                match choice {
                    'e' => self.state = State::Third,
                    'w' => self.state = State::Fourth,
                    _ => (),
                }
            }

            State::Third {..} => {
                match choice {
                    's' => self.state = State::End,
                    'u' => self.state = State::Second,
                    _ => (),
                }
            }

            State::Fourth {..} => {
                match choice {
                    'd' => self.state = State::First,
                    _ => (),
                }
            }

            State::End {..} => {
                self.state = State::End
            }
        };
    }

    fn print_state(&mut self) {

        match self.state {
            State::Start {..} => println!("You are in the Start State.\n\nThe \
                First State lies to the (s)outh."),

            State::First {..} => println!("As your eyes adjust to the light, \
                you see that you are now in the First State.\n\nThe Second \
                State lies to the (s)outh."),

            State::Second {..} => println!("You know, in somewhere deep within \
                yourself: this is the Second State.\n\nA branch in the path \
                lies before you. A signpost reads \"Third State, (e)ast. \
                Fourth State, (w)est."),

            State::Third {..} => println!("The air is thick and old here. You \
                are being watched.\n\nWith a sense of horror, you realize--no, \
                it's not possible--The Second State lies (u)p. . . above you.\
                \n\nOh, also, there's a door marked \"End State\" \
                To the south."),

            State::Fourth {..} => println!("You have never known silence so \
                intense and complete, but--for the life of you--you cannot \
                hear your heartbeat, and your attempts to check your pulse are \
                fruitless.\n\nYour foot sinks into a horribly soft spot in the \
                ground, revealing a small trapdoor. Below you--it cannot be--\
                (d)own underneath you, lies the First State."),

            State::End {..} => println!("You try walking walking forward \
                through the darkness, but it only seems to get darker. What \
                place is this?\n\n(It's the End State)"),    
        }
    }


}

fn get_input() -> char {
    let mut temp = String::new();

    io::stdin().read_line(&mut temp)
        .expect("failed to read line");

    match temp.to_ascii_lowercase().as_str().trim() {
        "n" | "north" => return 'n',
        "s" | "south" => return 's',
        "e" | "east" => return 'e',
        "w" | "west" => return 'w',
        "u" | "up" => return 'u',
        "d" | "down" => return 'd',
        _ => return 'x',
    }
}

fn main() {
    let mut engine = Engine::new();
    let mut input = 'x';

    loop{
        print!("{}[2J", 27 as char);
        engine.print_state();
        input = get_input();
        engine.update_state(input);
    }
}