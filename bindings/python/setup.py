from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name='yotsuba',
    version='0.1',
    rust_extensions=[
        RustExtension(
            'yotsuba.yotsuba', path='Cargo.toml', binding=Binding.PyO3,
            debug=False
        )
    ],
    packages=[
        'yotsuba',
    ],
    zip_safe=False
)
