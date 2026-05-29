# Trezor Modular App

A minimalistic example of a Trezor modular app.

## Prerequisites

- **Trezor App SDK crate** – update the local path in `Cargo.toml`:

  ```toml
  [dependencies]
  trezor-app-sdk = { path = "/path/to/trezor-app-sdk" }
  ```

- **Modular Xtask** – add the build alias in `.cargo/config.toml`:

  ```toml
  [alias]
  xtask = "run --manifest-path /path/to/modular-xtask/Cargo.toml --"
  ```

    > **Note:** Both `trezor-api` and `modular-xtask` are located in the
  > `bieleluk/modular-ethereum-wip` branch of the [trezor-firmware](https://github.com/trezor/trezor-firmware) repo.
  > They will be published to [crates.io](https://crates.io) in the future.

- **Nix shell** – enter the dev environment:

  ```bash
  nix-shell
  ```

- **Python environment (uv)** – set up the virtual environment:

  ```bash
  # First time only
  uv venv .venv

  # Activate
  source .venv/bin/activate
  ```

## Configuration

| Option     | Description         | Example        |
|------------|---------------------|----------------|
| `model`    | Target Trezor model | `t2t1`, `t3t1` |
| `lang`     | Firmware language   | `en`, `cs`     |
| `debug`    | Enable debug build  |                |
| `emulator` | Build for emulator  |                |

See all available options:

```bash
cargo xtask --help
```

## Commands

### Build

- Emulator debug build with english language for T3W1 model.

```bash
cargo xtask build -m t3w1 --lang en -d -e
```

### Other Cargo Commands

Available commands: `clippy`, `check`, `clean`, `fmt`

```bash
cargo xtask <command>
```

### Unit Tests

```bash
cargo xtask unit-tests --lang en -m t3t1
```

### Device Tests

> **Note:** Currently only emulator is supported; firmware target will follow soon.
> The `trezor-firmware` dependency must be checked out at the `bieleluk/modular-ethereum-wip` branch.

#### Prerequisites

A running emulator with disabled animations matching the target model, either:

- built directly from the `trezor-firmware` repo, or
- launched via a pre-built binary using `trezor-user-env` (see [Appendix](#trezor-user-env))

#### Run

```bash
cargo xtask device-tests -m t3w1 -e -t tests/test_getpublickey.py::test_getpublickey
```

> The model and emulator/firmware flags must match the existing build, otherwise the artifact will not be found.

#### Show UI Results

Results are shown automatically at the end of each test run. To show them manually:

```bash
uv run ./tests/show_results.py
```

#### Python Style

To check or fix the Python style (linter, imports, ...) of test files:

```bash
# Check
cargo xtask py-style-check

# Fix
cargo xtask py-style
```

## Appendix

### Trezor User Env

[trezor-user-env](https://github.com/trezor/trezor-user-env) allows running the emulator without building the firmware monorepo locally.

**Prerequisites:** Docker

**Start the environment:**

```bash
./run.sh --no-regtest
```

**Launch the emulator:**

1. Open the web UI and go to **Emulator**
2. Select the correct model and version
3. Click **Start Emulator**

> **Note:** The implementation is not yet in `main`. Use a custom firmware URL:
>
> **Custom firmware → from URL:**
>
> ```
> https://data.trezor.io/dev/firmware/releases/emulators-new/T3W1/trezor-emu-core-T3W1-v0.0.0
> ```
>
> If repeated starts trigger an error, delete the contents of:
>
> ```
> src/binaries/firmware/bin/user_downloaded
> ```

**UI test screenshots:**

Because the emulator runs in Docker, it cannot access local UI test screenshots — all UI tests will report failure by default. To fix this, mount a volume in `docker/compose.yml`:

```yaml
- /path/to/modular-app/tests/ui_tests:/path/to/modular-app/tests/ui_tests
```
