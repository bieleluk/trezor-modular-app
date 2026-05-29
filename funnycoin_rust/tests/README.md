# Integration & Functional Tests

This directory contains Python tests for the Modular App, using `pytest`.  
Tests are executed via the Rust `xtask` system.

---

## Structure

- **fixtures.py**: Common pytest fixtures for device setup, emulator connection, and language configuration.
- **results.py**: Helpers for parsing and validating results from the device/emulator.
- **funnycoin.py**: Abstraction layer for sending and receiving custom protobuf messages.
- **generated/**: Auto-generated Python modules from custom protobuf definitions.
- **test_*.py**: Individual test modules for various app features.

---

## Running Tests

Tests are run with:

```sh
cargo xtask device-tests --model <model> --language <lang>
```

---

## Test Execution Modes

- **Emulator**: Tests can be run using the Trezor emulator.
- **Hardware**: (TODO) Direct execution on physical hardware is planned but not yet implemented.

---

## Test Environment Requirements

- **Emulator**: Start the Trezor emulator with animations disabled and the correct language set.
- **Language Matching**: The Modular App language and the Core App language must match.  
  This is required for correct rendering and comparison of special characters.

---

## Protobufs

There are **two types of protobuf messages** used in the tests:

- **Trezor protobufs**: Used for communication with the Trezor device/emulator (standard messages).
- **Custom protobufs**: Application-specific messages, serialized and sent inside a generic Trezor message.

Custom message definitions are compiled using `pb2py` and stored in `tests/generated/`.  
This allows Python code to easily construct and parse custom messages.

---

## funnycoin.py

The `funnycoin.py` module encapsulates all logic for working with custom protobuf messages:

- Imports the generated custom message classes from `tests/generated/`.
- Handles serialization of custom messages into the generic Trezor message format.
- Provides a simple API for sending requests and parsing responses, so test code can work directly with custom messages without dealing with low-level serialization details.

**Example usage:**

```python
# filepath: tests/test_public_key.py
from funnycoin import send_custom_message, FunnycoinGetPublicKey

def test_get_public_key(client):
    req = FunnycoinGetPublicKey(address_n=[44 | 0x80000000, 0x80000000])
    resp = send_custom_message(client, req)
    assert resp.public_key is not None
```

---

## Fixtures

The `fixtures.py` file provides reusable pytest fixtures, for example:

```python
import pytest
from trezorlib.client import TrezorClient
from trezorlib.transport import enumerate_devices

@pytest.fixture(scope="session")
def client():
    devices = enumerate_devices()
    assert devices, "No Trezor devices found"
    with TrezorClient(devices[0]) as client:
        yield client

@pytest.fixture
def set_language(client, request):
    lang = request.config.getoption("--language")
    # Set language on device/emulator if needed
    # ...
    yield
```

You can add or update fixtures as needed for your test setup.

---

## Results Helpers

The `results.py` file contains helpers for parsing and validating device responses, for example:

```python
def assert_public_key_response(response, expected_pubkey):
    assert response.public_key == expected_pubkey, "Public key mismatch"
```

---

## Trezorlib

- Tests use [`trezorlib`](https://github.com/trezor/trezor-firmware/tree/master/python/trezorlib) to communicate with the emulator or device.
- Make sure `trezorlib` and all dependencies are installed in your Python environment.

---

## Example

```sh
# Start emulator with correct language and animations disabled
./emu.sh --language <lang> --animations false

# Run tests
cargo xtask device-tests --model t3w1 --language en
```

---

## Notes

- If languages do not match, tests may fail due to differences in special character rendering.
- Ensure protobufs are up-to-date and generated for both Rust and Python code.
