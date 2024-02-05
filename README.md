# testbuild 🔨

Just trying to figure out how to serve cross-platform binaries for combined R/Rust packages.

## Background

Compiled code from languages like Rust or C is specific to a CPU architecture and operating system. That is, it wouldn't be easy for me to build a combined R/Rust package for a windows or x86 machine from my personal aarch64 macbook. Rather than sending end users on a deranged sidequest to replicate my exact toolchain (and keep up with future changes), I'd like to provide pre-built binaries for common platforms.

CRAN has some fairly strict rules about hosting non-R binaries, not going to deal with that until I've had more time to read their policies & update my workflows to comply.

This is my attempt at a one-day workaround using github actions as a build-and-release system.

## Install

manual download? small R script that downloads and installs it?

## Verify

If this works, you should be able to run this from wherever you run R:

```{r}
testbuild::verify_install()
```

This should print out "Package build was successful" if it successfully interfaced with the pre-compiled rust code.
