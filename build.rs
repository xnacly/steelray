use std::{
    env,
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(program).args(args).output().ok()?;
    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8(output.stdout).ok()?;
    Some(stdout.trim().to_owned())
}

fn env_or_unknown(name: &str) -> String {
    env::var(name).unwrap_or_else(|_| "unknown".to_owned())
}

fn main() {
    println!(
        "cargo:rustc-env=STEELRAY_BUILD_TARGET={}",
        env_or_unknown("TARGET")
    );
    println!(
        "cargo:rustc-env=STEELRAY_BUILD_HOST={}",
        env_or_unknown("HOST")
    );
    println!(
        "cargo:rustc-env=STEELRAY_BUILD_PROFILE={}",
        env_or_unknown("PROFILE")
    );

    let build_unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs().to_string())
        .unwrap_or_else(|_| "unknown".to_owned());
    println!("cargo:rustc-env=STEELRAY_BUILD_UNIX={build_unix}");

    let rustc = command_output("rustc", &["--version"]).unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=STEELRAY_RUSTC_VERSION={rustc}");

    let git_commit = command_output("git", &["rev-parse", "--short", "HEAD"])
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=STEELRAY_GIT_COMMIT={git_commit}");

    let git_dirty = Command::new("git")
        .args(["diff", "--quiet"])
        .status()
        .map(|status| if status.success() { "false" } else { "true" })
        .unwrap_or("unknown");
    println!("cargo:rustc-env=STEELRAY_GIT_DIRTY={git_dirty}");
}
