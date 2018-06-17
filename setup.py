from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

setup(
    name='hello-rust',
    version='0.1.0',
    rust_extensions=[
        RustExtension('rusty.lib', 'rusty/Cargo.toml', binding=Binding.PyO3)
    ],
    packages=find_packages('src'),
    package_dir={'': 'src'},
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False)
