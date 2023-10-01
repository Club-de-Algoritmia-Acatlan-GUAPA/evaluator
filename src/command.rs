use crate::consts::NSJAIL_DIR;
use std::process::{Child, Command, Output, Stdio};
#[derive(Default, Debug)]
pub struct JailedCommand {
    mounted_directories: Vec<[String; 2]>,
    time_limit: usize,
    input_file: String,
    args: Vec<String>,
    _user: String,
    _group: String,
    current_dir: String,
    config_file: Option<String>,
    stdout: Option<Stdio>,
    stdin: Option<Stdio>,
}

impl JailedCommand {
    pub fn new(inner_command: String) -> Self {
        Self {
            args: vec![inner_command],
            _user: "2000".to_string(),
            _group: "2000".to_string(),
            time_limit: 1,
            ..Default::default()
        }
    }

    pub fn arg(mut self, arg: &str) -> JailedCommand {
        self.args.push(arg.to_string());
        self
    }

    /// Mount a directory  into the jail filesystem
    pub fn mount(mut self, src: &str, dst: &str) -> JailedCommand {
        self.mounted_directories
            .push([src.to_string(), dst.to_string()]);
        self
    }

    /// Set time limit in seconds for execution time
    pub fn time_limit(mut self, time_limit: usize) -> JailedCommand {
        self.time_limit = time_limit;
        self
    }
    // Set name for input file
    pub fn input_file(mut self, input_file: &str) -> JailedCommand {
        self.input_file = input_file.to_string();
        self
    }

    pub fn current_dir(mut self, cur_dir: &str) -> JailedCommand {
        self.current_dir = cur_dir.to_string();
        self
    }

    pub fn config_file(mut self, dir: &str) -> JailedCommand {
        self.config_file = Some(dir.to_string());
        self
    }

    pub fn stdout<T: Into<Stdio>>(mut self, cfg: T) -> JailedCommand {
        self.stdout = Some(cfg.into());
        self
    }
    pub fn stdin<T: Into<Stdio>>(mut self, cfg: T) -> JailedCommand {
        self.stdin = Some(cfg.into());
        self
    }

    pub fn spawn(self) -> std::io::Result<Child> {
        let args = self.generate_nsjail_args();

        let mut c = Command::new(NSJAIL_DIR);

        c.args(args).stderr(Stdio::piped());

        c.stdin(self.stdin.unwrap_or(Stdio::piped()))
            .stdout(self.stdout.unwrap_or(Stdio::piped()))
            .spawn()
    }

    pub fn output(self) -> std::io::Result<Output> {
        let args = self.generate_nsjail_args();

        let mut c = Command::new(NSJAIL_DIR);

        c.current_dir(&self.current_dir)
            .args(args)
            .stdout(self.stdout.unwrap_or(Stdio::piped()))
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
    }

    #[inline]
    pub fn generate_nsjail_args(&self) -> Vec<String> {
        let mounts: Vec<String> = self
            .mounted_directories
            .iter()
            .map(|[src, dst]| ["-R".to_string(), format!("{src}:{dst}")])
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect();

        // TODO make this optional, we already have a config file
        let mut nsjail_args = vec![
            //self.mode.to_string(),
            //"--user".to_string(),
            //self.user.to_string(),
            //"--group".to_string(),
            //self.group.to_string(),
            //"--time_limit".to_string(),
            //self.time_limit.to_string(),
            //// this options are not optional
            //"--disable_proc".to_string(),
            //"--keep_caps".to_string(),
            //"--really_quiet".to_string(),
        ];

        if let Some(dir) = &self.config_file {
            nsjail_args.push("--config".to_string());
            nsjail_args.push(dir.to_string());
        }

        let exec_command = ["--", "/bin/sh", "-c", &self.args.join(" ").to_string()]
            .iter()
            .map(|s| s.to_string())
            .collect();

        [nsjail_args, mounts, exec_command].concat()
    }
}

#[cfg(test)]
mod tests {
    use crate::command::JailedCommand;
    #[test]
    pub fn test_command() -> () {
        let process = "python3".to_string();
        let _ = JailedCommand::new(process)
            .arg("Python3 ")
            .arg("Python3 ")
            .mount("/bin/python3/", "/src/bin/python3")
            .time_limit(2);
    }
    #[test]
    pub fn test_generate_nsjail_commands() -> () {
        let c = JailedCommand::new("/usr/bin/python3".to_string());
        let m = c
            .arg("/code/script.py")
            .arg("<")
            .arg("/code/b.in")
            .mount("/bin/python3/", "/src/bin/python3")
            .mount("/usr/lib/", "/usr/libi")
            .mount("/dev/null", "/tmp/null")
            .time_limit(2);
        println!("{:?}", m.generate_nsjail_args());
    }
}
