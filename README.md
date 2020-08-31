# 🍀 yotsuba

[Documentation](https://yutayamazaki.github.io/yotsuba-docs/)

## Install yotsuba.

- Python

```shell
git clone https://github.com/yutayamazaki/yotsuba.git
cd yotsuba/bindings/python
python setup.py install

# via pip (under construction)
# pip install yotsuba-python
```

## Contributing.

### Python binding

- Check code format and apply type checking.

```shell
cd bindings/python
flake8 yotsuba tests
mypy yotsuba tests
```

- Run unit-tests.

```shell
cd bindings/python
python -m unittest discover tests
```
