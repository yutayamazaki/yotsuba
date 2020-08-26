from .yotsuba import ja
from yotsuba.yotsuba import (
    clean_emails, clean_html_tags, clean_url, get_stopwords,
    get_stopwords_by_frequency, pad_sequence, pad_sequences,
    remove_stopwords
)


def say_hello() -> None:
    print('You say goodbye and I say hello.')
