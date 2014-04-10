use std::io::timer;
use std::io::stdio;

struct Shell {
    cmd_prompt: ~str,
}

impl Shell {
    fn new(prompt_str: &str) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
        }
    }

    fn start(&mut self) {
        let mut stdin = stdio::stdin();
        loop {
            stdio::print(self.cmd_prompt);
            stdio::flush();

            let line = stdin.read_line().unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(' ', 1).nth(0).expect("no program");

            match program {
                ""      =>  { continue; }
                "\u274c"  =>  { return; }
                // _       =>  { self.line(cmd_line); }
                _ => { println!("You just ran {}", cmd_line) }
            }
        }
    }
}

fn main() {
    Shell::new("\U0001f41a \U0001f4b2 ").start();
}
