# zw_fast_quantile_py

[![CI](https://github.com/MnO2/zw-fast-quantile-py/actions/workflows/CI.yml/badge.svg)](https://github.com/MnO2/zw-fast-quantile-py/actions/workflows/CI.yml)

[zw-fast-quantile](https://github.com/MnO2/zw-fast-quantile) python binding

## Installation

```bash
pip install zw_fast_quantile_py
```

## Usage

```python
import zw_fast_quantile_py

summary = zw_fast_quantile_py.QuantileSummary(0.01)

for i in range(0, 10):
    summary.update(i)

summary.query(0.0)
```
