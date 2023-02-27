# Benchmarks
Now it's comparing aiohttp and reqsnaked using local httpbin image

To run image use
```
docker run -p 80:80 -p 443:443 simonkowallik/httpbin:nginx
```

To run benchmarks:
```
.venv/bin/maturin develop -r && .venv/bin/pytest benchmarks --benchmark-min-rounds=250 --benchmark-max-time=10
```
```
======================================================================== test session starts =========================================================================
platform darwin -- Python 3.8.2, pytest-7.2.1, pluggy-1.0.0
benchmark: 4.0.0 (defaults: timer=time.perf_counter disable_gc=False min_rounds=250 min_time=0.000005 max_time=10 calibration_precision=10 warmup=False warmup_iterations=100000)
rootdir: /Users/an/Desktop/reqsnaked
plugins: asyncio-0.20.3, anyio-3.6.2, benchmark-4.0.0
asyncio: mode=strict
collected 2 items

benchmarks/test_libs.py ..                                                                                                                                     [100%]


------------------------------------------------------------------------------------- benchmark: 2 tests ------------------------------------------------------------------------------------
Name (time in ms)                 Min                Max               Mean            StdDev             Median               IQR            Outliers      OPS            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_reqsnaked_x10_linear     25.9375 (1.0)      40.9148 (1.0)      30.5369 (1.0)      2.7515 (1.0)      30.1445 (1.0)      2.3587 (1.0)         69;20  32.7472 (1.0)         310           1
test_aiohttp_x10_linear       28.2000 (1.09)     63.7135 (1.56)     35.0434 (1.15)     4.6091 (1.68)     33.9884 (1.13)     4.2655 (1.81)        36;12  28.5360 (0.87)        250           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean
========================================================================= 2 passed in 19.78s =========================================================================
```
***
In results, reqsnaked ~15% faster