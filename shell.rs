use std::io::stdio;
use std::io::Process;
use std::io::process;

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
                _       =>  { self.cmd(cmd_line.clone()); }
                // _ => { println!("You just ran {}", cmd_line) }
            }
        }
    }

    fn cmd(&mut self, cmd_line: ~str) {
        let mut arg_vec = ~[];
        let mut cmd = ~"";
        for (index, s) in cmd_line.split_str(" ").enumerate() {
            if index == 0 { cmd = s.to_owned(); } else { arg_vec.push(s.to_owned()); }
        }
        self.run_cmd(cmd, arg_vec);
    }

    fn run_cmd(&mut self, cmd: ~str, argv: ~[~str]) {
        if self.cmd_exists(cmd) {
            println!("Executing {}", cmd);
            let p = Process::new(cmd, argv);
            let status = p.unwrap().wait();
            match status {
                process::ExitSignal(st) => println!("Exited with signal {}", st),
                process::ExitStatus(st) => println!("Exited with status {}", st)
            }
        }
        else {
            println!("Command {} not found.", cmd)
        }
    }

    fn cmd_exists(&mut self, cmd: &str) -> bool {
        let p = Process::new("which", [cmd.to_owned()]);
        let status = p.unwrap().wait();
        match status {
            process::ExitSignal(st) => st == 0,
            process::ExitStatus(st) => st == 0
        }
    }
}

fn main() {
    Shell::new("\U0001f41a  ").start();
}
