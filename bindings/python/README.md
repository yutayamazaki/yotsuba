# 🍀 yotsuba

## Install yotsuba.

- Python

```shell
git clone https://github.com/yutayamazaki/yotsuba.git
cd yotsuba/bindings/python
python setup.py install
```

## API documents.

`yotsuba.pad_sequences(sequences: List[List[int]], maxlen: int, value: int = 1, padding: str = 'post') -> List[List[int]]`

Pad sequences by given value.

### Parameters

- **sequences (List[List[int]])** - Sequences to pad.
- **maxlen (int)** - Max limit value to pad.
- **value (int)** - A value to pad.
- **padding (str)** - 'pre' or 'post', default is 'post'.

