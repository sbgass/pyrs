# pyrs

pip installable rust library using pyO3 for bindings. Library functions written in rust can be packaged, distributed, and called from python. To get started, clone the repo and install with:

```
pip install -e pyrs/
```

The tests can be called with:
```
pytest pyrs/tests/
```

A CLI is also exposed: 
```
$ pyrs 1 2
3
```