use std::io::stdio;
use std::io::Process;
use std::io::process;
use std::path::posix::Path;

struct Shell {
    cmd_prompt: ~str,
    cwd: Path
}

impl Shell {
    fn new(prompt_str: &str, cwd: Path) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
            cwd: cwd
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
        match cmd.as_slice() {
            &"\U0001F697" => self.cd(argv),
            &"\U0001F4CD" => self.pwd(),
            _ => self.execute_program(cmd, argv)
        }
    }

    fn execute_program(&mut self, cmd: ~str, argv: ~[~str]) {
        if self.cmd_exists(cmd) {
            println!("\U0001F3C3 {}", cmd);
            let config = process::ProcessConfig {
                program: cmd,
                args: argv,
                cwd: Some(&self.cwd),
                stdin: process::InheritFd(0),
                stdout: process::InheritFd(1),
                stderr: process::InheritFd(2),
                .. process::ProcessConfig::new()
            };

            let p = Process::configure(config);
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

    fn cd(&mut self, path: ~[~str]) {
        if path.len() == 0 {
            println!("Please specify a path to \U0001F697  to.");
            return;
        }
        let p = Path::new(path[0]);
        if !p.is_dir() {
            println!("\U0001F697  could not find that, \U0001F62D  ")
        }
        self.cwd = p;
    }

    fn pwd(&mut self) {
        println!("\u27A1 {} \u2B05  \U0001F4CD", self.cwd.as_str().unwrap());
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
    Shell::new("\U0001f41a  ", std::os::getcwd()).start();
}
