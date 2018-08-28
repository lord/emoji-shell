use std::env;
use std::io::stdin;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

struct Shell {
    cmd_prompt: String,
    cwd: PathBuf
}

impl Shell {
    fn new(prompt_str: &str, cwd: PathBuf) -> Shell {
        Shell {
            cmd_prompt: prompt_str.to_owned(),
            cwd: cwd
        }
    }

    fn start(&mut self) {
        let stdin = stdin();
        loop {
            println!("{}", self.cmd_prompt);

            let mut line = String::new();
            stdin.read_line(&mut line).unwrap();
            let cmd_line = line.trim().to_owned();
            let program = cmd_line.splitn(1, ' ').nth(0).expect("no program");

            match program {
                ""      =>  { continue; }
                "\u{274c}"  =>  { return; }
                _       =>  { self.cmd(cmd_line.clone()); }
                // _ => { println!("You just ran {}", cmd_line) }
            }
        }
    }

    fn cmd(&mut self, cmd_line: String) {
        let mut arg_vec = Vec::new();
        let mut cmd = String::new();
        for (index, raw_s) in cmd_line.split("\u{1F345}").enumerate() {
            let s = raw_s.trim();
            if s == "" { continue; }
            if index == 0 { cmd = s.to_owned(); } else { arg_vec.push(s.to_owned()); }
        }
        self.run_cmd(cmd, arg_vec);
    }

    fn run_cmd(&mut self, cmd: String, argv: Vec<String>) {
        match &*cmd {
            "\u{1F697}" => self.cd(argv),
            "\u{1F4CD}" => self.pwd(),
            "\u{1F50D}" => self.execute_program("ls".to_string(), argv),
            "\u{1F431}" => self.execute_program("cat".to_string(), argv),
            "cat" => self.reject("cat"),
            "ls" => self.reject("ls"),
            "pwd" => self.reject("pwd"),
            _ => self.execute_program(cmd, argv)
        }
    }

    fn reject(&mut self, attempted_command: &str) {
        println!("Please do not use {}, it is offensive", attempted_command);
    }

    fn execute_program(&mut self, cmd: String, argv: Vec<String>) {
        if self.cmd_exists(&cmd) {
            println!("\u{1F3C3}  {}", cmd);

            let status = Command::new(cmd.clone())
                .args(argv)
                .stdin(Stdio::inherit())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .current_dir(&self.cwd)
                .status()
                .expect("Command not found.");

            println!("\u{1F6B7}  {} \u{27A1}  {}", cmd, status);
        }
        else {
            println!("Command {} not found.", cmd);
        }
    }

    fn cd(&mut self, path: Vec<String>) {
        if path.len() == 0 {
            println!("Please specify a path to \u{1F697}  to.");
            return;
        }
        let p = Path::new(&path[0]);
        if !p.is_dir() {
            println!("\u{1F697}  could not find that, \u{1F62D}  ")
        }
        self.cwd = p.to_path_buf();
    }

    fn pwd(&mut self) {
        println!("\u{27A1} {} \u{2B05}  \u{1F4CD}", self.cwd.display());
    }

    fn cmd_exists(&mut self, cmd: &str) -> bool {
        let p = Command::new("which")
            .arg(cmd.to_owned())
            .output()
            .expect("failed to execute process");
        p.status.success()
    }
}

fn main() {
    println!("\x1bc");
    let path = env::current_dir().unwrap();
    Shell::new("\u{1f41a}  ", path).start();
}
