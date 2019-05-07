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
    last_state: State,
    state: State
}

impl Engine {
    fn new() -> Self {
        Engine {
            last_state: State::Start,
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
                self.last_state = State::First; 
            }

            State::Second {..} => {
                match choice {
                    'e' => self.state = State::Third,
                    'w' => self.state = State::Fourth,
                    _ => (),
                }

                self.last_state = State::Second; 
            }

            State::Third {..} => {
                match choice {
                    's' => self.state = State::End,
                    'u' => self.state = State::Second,
                    _ => (),
                }

                self.last_state = State::Third; 
            }

            State::Fourth {..} => {
                match choice {
                    'd' => self.state = State::First,
                    _ => (),
                }

                self.last_state = State::Fourth; 
            }

            State::End {..} => {
                self.state = State::End;
                self.last_state = State::End; 
            }
        };
    }

    fn print_state(&mut self) {

        match self.state {
            State::Start {..} => println!("You are in the Start State of this \
                Finite State Machine.\n\nThe First State lies to the (s)outh."),

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

    fn print_transition(&mut self)  {
        // Only certain state changes will have transitions
        match self.last_state {
            State::Start {..} => {
                match self.state {
                    State::First {..} => println!("The path out of the cave \
                        leads upwards, and you feel a breeze from the exit.\n"),

                    _ => (),
                }
            }

            State::Second {..} => {
                match self.state {
                    State::Third {..} => println!("You walk down the winding \
                        road to the east.\n"),

                    State::Fourth {..} => println!("You start up the hill \
                        looming to the west.\n"),

                    _ => (),
                }
            }

            State::Third {..} => {
                match self.state {
                    State::Second {..} => println!("A rope ladder drops from \
                        somewhere in the sky, but outside of your vision. You \
                        climb.\n"),

                    State::End {..} => println!("With a start, you realize \
                        you have been walking forwards toward the End State for \
                        some time now.\n"),

                    _ => (),
                }
            }

            State::Fourth {..} => {
                match self.state {
                    State::First {..} => println!("You feel exhausted as you \
                        are lowered into the ground. You awake standing in a \
                        vast field with the sun directly overhead\n"),

                    _ => (),
                }
            }

            _ => ()
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
        engine.print_transition();
        engine.print_state();
        input = get_input();
        engine.update_state(input);    
    }
}