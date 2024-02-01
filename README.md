# Dranik, a framework for creating robots

## Example

`main.rs`:

```rust
dranik::load_config!();
dranik::main!();
```

`main.py`:

```python
from dranik.api import DranikRuntime

def main(r: DranikRuntime):
    r.log("Hello, world!")
```
