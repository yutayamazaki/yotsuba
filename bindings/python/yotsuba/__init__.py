from .yotsuba import ja
from yotsuba.yotsuba import (
    get_stopwords, get_stopwords_by_frequency, pad_sequence, pad_sequences,
    remove_stopwords
)


def say_hello() -> None:
    print('You say goodbye and I say hello.')
