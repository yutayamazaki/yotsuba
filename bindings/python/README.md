# 🍀 yotsuba

## Install yotsuba.

- Python

```shell
git clone https://github.com/yutayamazaki/yotsuba.git
cd yotsuba/bindings/python
python setup.py install
```

## API documents.

### yotsuba

#### `yotsuba.pad_sequences(sequences: List[List[int]], maxlen: int, value: int = 1, padding: str = 'post') -> List[List[int]]`

Pad sequences with given value like `keras.preprocessing.sequence.pad_sequences`.

##### Parameters

- `sequences (List[List[int]])` - Sequences to padded.
- `maxlen (int)` - Maximum sequence length to pad.
- `value (int)` - A value used to pad sequences.
- `padding (str)` - 'pre' or 'post', default is 'post'.

##### Returns
- `List[List[int]]` - An list of integers with shape (len(sequences), maxlen).

##### Raises

- ValueError - If `padding` is not 'pre' or 'post'.
