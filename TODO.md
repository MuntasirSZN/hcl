# TODO

## High Priority (Future Work)

- [x] Add support for elvish and nushell in format
- [x] Add --write flag for caching completions to ~/.hcl (cli.rs) (used with -c just caches in ~/.hcl the output)
- [x] Finalize --write design: user adds `source "$(hcl -c fzf --format zsh --write)"` (or similar) in rc files; hcl never auto-appends to rc files
- [x] Add more snapshot regression tests for generators and parser
- [x] Reach 90%+ coverage in cargo llvm-cov
- [x] Full h2o compatibility (100%) (https://github.com/yamaton/h2o.git)
- [x] Document --write usage pattern for rc files (no auto-append; user-managed source lines)
- [x] Caching mechanism with TTL (cache parsed help for N hours)
- [x] Performance optimization for 100+ MB help text files (added benchmarks, optimized hot paths - see below)
- [x] Parallel parsing for multiple commands using rayon
- [ ] Future roff parser (separate crate)
- [ ] Async framework (tokio)
- [ ] Asyncification
- [ ] `ecow`, `scc` etc crates (more crates for perf)
- [ ] Future convert one completion format to another

## Medium Priority

- [x] Add -f short flag support for --file (if original supported)
- [x] Property-based testing with proptest
- [x] Stress tests with massive help text files (benchmarks added for 1MB and 10MB inputs)
- [x] Performance benchmarking suite with criterion (using divan)

## Nice to Have

- [ ] Plugin system for custom parsers
- [ ] Interactive mode for testing completions
- [ ] Completion validation checker
- [x] Shell-specific optimizations (bash/zsh completions improved)

## Research Items

- [x] Verify if original h2o supported -f shorthand
- [x] Check bioinformatics tool parsing edge cases (covered by dedicated parser test)
- [x] Evaluate roff parser complexity vs. benefit (decision: keep future roff parser as separate crate; defer implementation)

## Performance Optimizations Completed

Benchmarks added for massive inputs (1MB, 10MB files). Key improvements:

| Benchmark | Before | After | Improvement |
|-----------|--------|-------|-------------|
| parse_blockwise_small | 10.5µs | 2.5µs | **76% faster** |
| parse_blockwise_medium | 73µs | 34µs | **53% faster** |
| parse_blockwise_massive | 20ms | 15ms | **25% faster** |
| preprocess_blockwise_small | 11µs | 0.76µs | **93% faster** |
| preprocess_blockwise_medium | 48µs | 8.6µs | **82% faster** |
| postprocess_remove_bullets | 20µs | 6µs | **70% faster** |
| postprocess_remove_bullets_massive | 2.2ms | 0.58ms | **73% faster** |

Optimizations applied:
- Added `memchr` crate for fast byte searching
- Pre-allocated vectors with capacity hints
- Reduced string allocations in hot paths
- Smart parallelization threshold (only use rayon for >4 blocks)
- Fast-path checks to skip non-option lines early
- Optimized bullet removal with byte-level matching
