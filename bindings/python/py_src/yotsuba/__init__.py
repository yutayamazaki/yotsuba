from typing import List, Optional

from .yotsuba import ja
from yotsuba.yotsuba import (
    clean_emails, clean_html_tags, clean_number, clean_url, get_stopwords,
    get_stopwords_by_frequency, pad_sequences,
    remove_stopwords
)
from yotsuba import yotsuba


def say_hello() -> None:
    print('You say goodbye and I say hello.')


def pad_sequence(
    sequence: List[int], maxlen: int, value: Optional[int],
    padding: str = 'post'
):
    """Pad sequence with given value like
       keras.preprocessing,sequence.pad_sequence.
    Args:
        sequence (List[int]): Sequence to pad like [0, 3, 1, ...].
        maxlen (int): Maximum length of sequence to pad with.
        value (Optional[int]): A value to pad, default=1.
        padding (str): 'pre' or 'post', default is 'post'.
    Returns:
        List[int]: Padded sequence which has maxlen length.
    """
    return yotsuba.pad_sequence(sequence, maxlen, value, padding)
