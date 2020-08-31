# 🍀 yotsuba

![Rust](https://github.com/yutayamazaki/yotsuba/workflows/Rust/badge.svg)
![Python](https://github.com/yutayamazaki/yotsuba/workflows/python/badge.svg)
[![PyPI Version](https://img.shields.io/pypi/v/yotsuba-python.svg)](https://pypi.org/project/yotsuba-python/)
[![MIT License](http://img.shields.io/badge/license-MIT-blue.svg?style=flat)](LICENSE)
![GitHub Starts](https://img.shields.io/github/stars/yutayamazaki/yotsuba.svg?style=social)
![GitHub Forks](https://img.shields.io/github/forks/yutayamazaki/yotsuba.svg?style=social)

[Documentation](https://yutayamazaki.github.io/yotsuba-docs/)

## Install yotsuba.

- Python

```shell
# Via pip
pip install yotsuba-python

# Build from source
git clone https://github.com/yutayamazaki/yotsuba.git
cd yotsuba/bindings/python
python setup.py install
```

## Contributing.

### Python binding

- Check code format and apply type checking.

```shell
cd bindings/python
flake8 py_src/yotsuba tests
mypy py_src/yotsuba tests
```

- Run unit-tests.

```shell
cd bindings/python
python -m unittest discover tests
```
