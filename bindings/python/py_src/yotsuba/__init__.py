from typing import List, Optional

from .yotsuba import ja
from yotsuba.yotsuba import (
    clean_url, get_stopwords,
    get_stopwords_by_frequency, remove_stopwords
)
from yotsuba import yotsuba


def clean_emails(text: str, replace: str = '') -> str:
    """Replace emails with given string.

    Args:
        text: A text want to clean emails.
        replace: Token to replace emails, default is ''.
    Returns:
        Cleaned text.
    Examples:
        >>> import yotsuba
        >>> text: str = 'Hello aaa@example.com!'
        >>> print(yotsuba.clean_emails(text))
        Hello !
        >>> print(yotsuba.clean_emails(text, '<URL>'))
        Hello <URL>!
    """
    return yotsuba.clean_emails(text, replace)


def clean_html_tags(text: str, replace: str = '') -> str:
    """Replace html tags and replace it with given token.

    Args:
        text: A text want to clean html tags.
        replace: Token to replace html tags, default is ''.
    Returns:
        Cleaned text.
    Examples:
        >>> import yotsuba
        >>> text: str = 'aaa<a>bbb</a>ccc'
        >>> print(yotsuba.clean_html_tags(text))
        aaabbbccc
        >>> print(yotsuba.clean_html_tags(text, '<HTML>'))
        aaa<HTML>bbb<HTML>ccc
    """
    return yotsuba.clean_html_tags(text, replace)


def clean_number(text: str, replace: str = '0') -> str:
    """Replace sequence of numbers with given string.

    Args:
        text: A text want to clean number.
        replace: Token to replace number, default is '0'.
    Returns:
        Cleaned text.
    Examples:
        >>> import yotsuba
        >>> text: str = 'I was born in 2020.'
        >>> print(yotsuba.clean_number(text))
        I was born in 0.
    """
    return yotsuba.clean_number(text, replace)


def pad_sequence(
    sequence: List[int], maxlen: int, value: Optional[int] = None,
    padding: str = 'post'
) -> List[int]:
    """
    Pad sequence with given value like
    keras.preprocessing,sequence.pad_sequence.

    Args:
        sequence: Sequence to pad like [0, 3, 1, ...].
        maxlen: Maximum length of sequence to pad with.
        value: A value to pad, default=0.
        padding: 'pre' or 'post', default is 'post'.
    Returns:
        Padded sequence which has maxlen length.
    Examples:
        >>> import yotsuba
        >>> print(yotsuba.pad_sequence([0, 1, 2], 5))
        [0, 1, 2, 0, 0]
        >>> print(yotsuba.pad_sequence([0, 1, 2], 5, -1))
        [0, 1, 2, -1, -1]
    """
    return yotsuba.pad_sequence(sequence, maxlen, value, padding)


def pad_sequences(
    sequences: List[List[int]], maxlen: Optional[int] = None,
    value: Optional[int] = None, padding: str = 'post'
) -> List[List[int]]:
    """
    Pad sequences with given value like
    keras.preprocessing,sequence.pad_sequence.

    Args:
        sequences: A list of list to pad like [[0, 1], ...].
        maxlen:
            Maximum length of sequence to pad with. If None was passed,
            pad with maximum size of sequence.
        value: A value to pad, default=0.
        padding: 'pre' or 'post', default is 'post'.
    Returns:
        Padded sequences which has maxlen length.
    Examples:
        >>> import yotsuba
        >>> print(yotsuba.pad_sequences([[0, 1, 2], [0, 1]], 5))
        [[0, 1, 2, 0, 0], [0, 1, 0, 0, 0]]
        >>> print(yotsuba.pad_sequences([[0, 1, 2], [0, 1]], 5, padding='pre'))
        [[0, 0, 0, 1, 2], [0, 0, 0, 0, 1]]
    """
    return yotsuba.pad_sequences(sequences, maxlen, value, padding)
