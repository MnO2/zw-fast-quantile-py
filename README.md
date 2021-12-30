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

summary = zw_fast_quantile_py.IntQuantileSummary(0.01)
summary.update(1,2,3,4,5,6,7,8,9,10)
summary.query(0.0)
```

## Benchmark

We calculate the median of `[1, 1000]`, the `zw_fast_quantile_py` is slightly slower than `statistics.quantiles` due to the serilization cost between python and extension.

### zw_fast_quantile_py
```
502
4.4792000000000096e-05
```

### statistics.quantiles
```
499.5
3.883300000000027e-05
```
