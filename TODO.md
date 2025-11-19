# TODO

## High Priority (Future Work)

- [ ] Add support for elvish and nushell in format
- [ ] Add --write flag for caching completions to rc files (cli.rs) (used with -c just caches in ~/.hcl the output)
- [ ] more test cases with snapshot regression testing (90% whole coverage, full cargo llvm-cov must say 90%+ coverage)
- [ ] Full h2o compatibility (100%) (https://github.com/yamaton/h2o.git)
- [ ] Implement --write functionality to auto-append to .bashrc/.zshrc/.config/fish/config.fish
- [ ] Caching mechanism with TTL (cache parsed help for N hours)
- [ ] Performance optimization for 100+ MB help text files
- [ ] Parallel parsing for multiple commands using rayon
- [ ] Future roff parser (separate crate)
- [ ] Async framework (tokio)
- [ ] Asyncification
- [ ] `ecow`, `scc`, `memchr` etc etc crates (more crates for perf)
- [ ] Future convert one completion format to another

## Medium Priority

- [ ] Add -f short flag support for --file (if original supported)
- [ ] Property-based testing with proptest
- [ ] Stress tests with massive help text files
- [ ] Performance benchmarking suite with criterion

## Nice to Have

- [ ] Plugin system for custom parsers
- [ ] Interactive mode for testing completions
- [ ] Completion validation checker
- [ ] Shell-specific optimizations

## Research Items

- [ ] Verify if original h2o supported -f shorthand
- [ ] Check bioinformatics tool parsing edge cases
- [ ] Evaluate roff parser complexity vs. benefit
