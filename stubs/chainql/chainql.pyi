# Copyright 2024 Valery Klachkov
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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