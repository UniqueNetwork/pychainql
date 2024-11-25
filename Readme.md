# PyChainql

Python bindings to [chainql](https://github.com/uniquenetwork/chainql), Substrate blockchains querier.

## Use

PyChainql requires Python 3.10+.

You can either download binaries from the [latest release](https://github.com/vklachkov/pychainql/releases/latest) or install it with pip:

```
pip install chainql
```

Stubs can be found [here](stubs), examples [here](examples).

## Build project

Build requirements:
- Rust 1.80+
- Python 3.10+
- pyenv
- maturin

First, create and activate a virtualenv environment:

```
$ pyenv virtualenv 3.11 pychainql
$ pyenv activate pychainql
```

Then install maturin:

```
$ pip install maturin
```

And build the project:

```
$ maturin build --release
```

The built wheel will be located in the `target/wheels` directory.

## License

Project is licensed under [Apache License, Version 2.0](License).