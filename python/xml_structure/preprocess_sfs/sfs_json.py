import json
from xml_structure._lowlevel import preprocess_sfs_json


def preprocess_json(source: str) -> bytes:
    return preprocess_sfs_json(source)
