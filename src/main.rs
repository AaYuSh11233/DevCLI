mod ui;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "devcli", version = "1.3.0", about = "Developer CLI Toolkit")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { project_name: String },

    // Git
    GitInit,
    GitStatus,
    GitAdd { files: Vec<String> },
    GitCommit { message: String },
    GitPush,
    GitFetch,
    GitMerge { branch: String },
    GitBranch,
    GitRemote,

    // Node.js
    NpmInstall { package: String },
    NpmRemove { package: String },
    Npx { command: String },

    // Rust
    CargoBuild,
    CargoRun,
    CargoCheck,
    CargoFmt,

    // Python
    PyVenvCreate,
    PipInstall { package: String },

    Sysinfo,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { project_name } => ui::init_project(project_name),

        // Git
        Commands::GitInit => ui::git_init(),
        Commands::GitStatus => ui::git_status(),
        Commands::GitAdd { files } => ui::git_add(files),
        Commands::GitCommit { message } => ui::git_commit(message),
        Commands::GitPush => ui::git_push(),
        Commands::GitFetch => ui::git_fetch(),
        Commands::GitMerge { branch } => ui::git_merge(branch),
        Commands::GitBranch => ui::git_branch(),
        Commands::GitRemote => ui::git_remote(),

        // Node.js
        Commands::NpmInstall { package } => ui::npm_install(package),
        Commands::NpmRemove { package } => ui::npm_remove(package),
        Commands::Npx { command } => ui::npx(command),

        // Rust
        Commands::CargoBuild => ui::cargo_build(),
        Commands::CargoRun => ui::cargo_run(),
        Commands::CargoCheck => ui::cargo_check(),
        Commands::CargoFmt => ui::cargo_fmt(),

        // Python
        Commands::PyVenvCreate => ui::py_venv_create(),
        Commands::PipInstall { package } => ui::pip_install(package),

        Commands::Sysinfo => ui::sysinfo(),
    }
}
