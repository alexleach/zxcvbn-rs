# zxcvbn-rs Python bindings

This package provides opt-in Python bindings for `zxcvbn-rs`.
The `zxcvbn` function returns a dictionary-style payload similar to `zxcvbn-python`.

## Local development

```bash
cd bindings/python
python3 -m pip install -U maturin
maturin develop
python3 -c "from zxcvbn_rs import zxcvbn; print(zxcvbn('correcthorsebatterystaple'))"
```

## Build wheels

```bash
cd bindings/python
maturin build --release
```
