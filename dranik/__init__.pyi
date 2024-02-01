"""
# Dranik

If you are reading this, you are probably looking for `dranik.api`

Example:
```python
from dranik.api import DranikRuntime

def main(r: DranikRuntime) -> None:
    r.log("Hello, world!")
```
"""

import dranik.api as api

VERSION = "0.1-dev"
"""
Version of the Dranik API
Format: "MAJOR.PATCH"
"""

__all__ = ["VERSION", "api"]
