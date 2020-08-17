from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='yotsuba',
    version='0.1',
    rust_extensions=[
        RustExtension('yotsuba', 'Cargo.toml', binding=Binding.PyO3)
    ],
    zip_safe=False
)
