Install [Rust](https://www.rust-lang.org/en-US/)


```
cd rusty
cargo build --release
cd ..
cp rusty/targets/release/rustly.so rusty.pyd
pip install .
python -m hello_rusty.py
```
