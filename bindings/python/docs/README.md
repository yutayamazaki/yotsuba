# Yotsuba Documentations

```shell
# Install yotsuba python binding to convert docstring to html automatically
python setup.py install

# Install sphinx related dependencies
cd docs
pip install -r requirements.txt

# Run sphinx server in port 8000
sphinx-autobuild . build

# Build html documents
make html
```

## Build docs and host them on GitHub Pages.

```shell
make html

```
