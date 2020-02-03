## 3.2.-20
### Windows, Etana
1. Put Black-Scholes as submodule.
2. Recorded type annotations of BS-py using `monkeytype run bench.py`.
3. Added type annotations to BS-py using `monkeytype apply black_scholes_dp` and `monkeytype apply black_scholes_ndp`.
    * +: Added concrete runtime types.
    * -: Concrete types do not accurately reflect full capability of function: looser type constraints.
    * ->: Could also leave this step out to improve generality of transform.
4. Ran Python benchmark to observe possible changes in results
    * euro_vanilla_call 0.1252935778150575
    * euro_vanilla_put 0.1256573810358579
    * euro_vanilla: 0.12484568569469023
    * black_scholes_call_div 0.12585119058427818
    * black_scholes_put_div 0.12568964770662194
    * euro_vanilla_dividend: 0.1258356858204046
5. It seems that results have changed, but backtracking revealed that this change is unrelated to recent modifications.
    * Benched functions run twice as fast. Reason is unknown.

### Arch Linux, pylon
1. Ran `__init__.py` through `time`. Contents: run each BS-function once. Result: `0.33s user, 0.366 total`.

### Windows, Etana
1. Apply `pyrs` for syntax conversion using `python pyrs/pyrs.py black-scholes-py`.
2. Copy "black_scholes_*.rs" to `src/`. Rename `black-scholes-pyrs/__init__.rs` as `main.rs`.
3. Project does not compile, manual edits are required.

#### Started making manual edits to the project
1. Added `mod black_scholes*`. Might be automatable by pyrs if Python syntax for locals is used.
