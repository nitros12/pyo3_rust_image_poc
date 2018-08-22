import sys

from setuptools import setup

try:
    from setuptools_rust import RustExtension
except ImportError:
    import subprocess

    errno = subprocess.call([sys.executable, "-m", "pip", "install", "setuptools-rust"])
    if errno:
        print("Please install setuptools-rust package")
        raise SystemExit(errno)
    else:
        from setuptools_rust import RustExtension


setup_requires = ["setuptools-rust>=0.10.1", "wheel"]
install_requires = []

setup(
    name="image-meme",
    version="0.1.0",
    packages=["image_meme"],
    rust_extensions=[RustExtension("image_meme.image_meme", "Cargo.toml")],
    install_requires=install_requires,
    setup_requires=setup_requires,
    include_package_data=True,
    zip_safe=False,
)
