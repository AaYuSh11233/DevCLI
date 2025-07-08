use std::fs;
use std::process::Command;
use sysinfo::{System};

pub fn init_project(project_name: &str) {
    println!("🔷 Initializing project: {}", project_name);
    fs::create_dir_all(format!("{}/src", project_name)).unwrap();
    fs::write(format!("{}/README.md", project_name), "# Project\n").unwrap();
    println!("✅ Project {} initialized!", project_name);
}

// Git
pub fn git_init() { run("git", &["init"]); }
pub fn git_status() { run("git", &["status"]); }
pub fn git_add(files: &[String]) {
    let mut args: Vec<&'static str> = vec!["add"];
    for f in files { args.push(f); }
    run("git", &args);
}
pub fn git_commit(message: &str) { run("git", &["commit", "-m", message]); }
pub fn git_push() { run("git", &["push"]); }
pub fn git_fetch() { run("git", &["fetch"]); }
pub fn git_merge(branch: &str) { run("git", &["merge", branch]); }
pub fn git_branch() { run("git", &["branch", "-a"]); }
pub fn git_remote() { run("git", &["remote", "-v"]); }

// Node.js
pub fn npm_install(package: &str) { run("npm", &["install", package]); }
pub fn npm_remove(package: &str) { run("npm", &["uninstall", package]); }
pub fn npx(command: &str) { run("npx", &[command]); }

// Rust
pub fn cargo_build() { run("cargo", &["build"]); }
pub fn cargo_run() { run("cargo", &["run"]); }
pub fn cargo_check() { run("cargo", &["check"]); }
pub fn cargo_fmt() { run("cargo", &["fmt"]); }

// Python
pub fn py_venv_create() { run("python3", &["-m", "venv", "venv"]); }
pub fn pip_install(package: &str) { run("pip", &["install", package]); }

// Sysinfo
pub fn sysinfo() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpus = sys.cpus();
    let avg_cpu = cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32;

    println!("🔷 CPU Usage (avg): {:.2}%", avg_cpu);
    println!(
        "🔷 Memory: {} MB used / {} MB total",
        sys.used_memory() / 1024,
        sys.total_memory() / 1024
    );
}

// Helper
fn run(cmd: &str, args: &[&str]) {
    println!("🔷 Running: {} {:?}", cmd, args);
    Command::new(cmd).args(args).status().unwrap();
}
