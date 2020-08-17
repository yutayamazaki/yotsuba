# yotsuba

## Build Rust package and use in Python.

```shell
# In root directory
cd bindings/python
make build-release
cp target/release/libyotsuba.so target/release/yotsuba.so
cd target/release
python -m unittest discover tests
```
