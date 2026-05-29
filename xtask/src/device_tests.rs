use anyhow::{Context, Result};
use std::process;

use crate::{args::UploadArgs, helpers};

pub fn device_tests(args: UploadArgs) -> Result<()> {
    let project_dir = helpers::workspace_dir()?.join(&args.project);
    let binary =
        helpers::artifacts_dir(args.model, args.emulator)?.join(format!("{}.elf", &args.project));

    let binary = binary
        .canonicalize()
        .with_context(|| format!("Failed to locate `{}` for upload", binary.display()))?;

    let mut cmd = process::Command::new("uv");
    cmd.args([
        "run",
        "pytest",
        &format!("--extapp={}", binary.display()),
        "--ui=test",
        "--verbose",
    ])
    .env("TREZOR_TRANSLATIONS_DIR", project_dir.join("translations"))
    .current_dir(&project_dir);

    println!("xtask: Running device tests");
    println!("\x1b[1;90m{}\x1b[0m", helpers::command_args_to_string(&cmd));

    let status = cmd.status().context("Failed to spawn `pytest`")?;

    match status.code() {
        Some(0) | Some(1) => {
            // 0 = all tests passed, 1 = some tests failed (pytest convention)
            // Continue as normal
        }
        Some(code) => {
            // pytest exited with an unexpected code
            anyhow::bail!("pytest exited with unexpected code: {}", code);
        }
        None => {
            // pytest did not exit normally (e.g., killed by signal)
            anyhow::bail!("pytest did not exit normally (terminated by signal or unknown error)");
        }
    }

    let mut cmd = process::Command::new("uv");
    cmd.args([
        "run",
        project_dir
            .join("tests")
            .join("show_results.py")
            .to_str()
            .unwrap(),
    ])
    .current_dir(&project_dir);

    println!("xtask: Showing device test results");
    println!("\x1b[1;90m{}\x1b[0m", helpers::command_args_to_string(&cmd));

    let _ = cmd.spawn().context("Failed to spawn `show_results.py`")?;
    Ok(())
}
