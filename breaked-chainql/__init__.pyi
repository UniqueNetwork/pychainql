from typing import Optional

def enable_logs(): ...

def disable_logs(): ...

class JsonnetObject:
    def __getitem__(self, key: any) -> any:
        ...

    def manifest_json(self, minified: bool = True) -> str:
        ...

class JsonnetArray:
    def __getitem__(self, key: any) -> any:
        ...

class JsonnetFunc:
    def __call__(self, *args) -> any:
        ...

class ChainOpts:
    """Selection of optional flags for chain data processing"""

    omit_empty: bool
    """Whether or not to ignore trie prefixes with no keys"""

    include_defaults: bool
    """Should default values be included in output"""

    def __init__(self, omit_empty: bool = True, include_defaults: bool = True):
        ...

class Chain:
    def __init__(url: str, opts: Optional[ChainOpts]):
        ...
    
    def latest(self) -> JsonnetObject:
        ...

    def block(self, block: int) -> JsonnetObject:
        ...

def dump(meta: JsonnetObject | bytes, data: dict[bytes, bytes], opts: Optional[ChainOpts]) -> JsonnetObject:
    ...