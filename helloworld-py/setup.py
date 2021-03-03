from setuptools import setup, find_packages

setup(
    name='helloworld',
    version='1.0',
    package_dir={'': 'src'},
    packages=find_packages('src'),
    zip_safe=False,
    install_requires=['Flask']
)
