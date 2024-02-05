# testbuild 🔨

Just trying to figure out how to serve cross-platform binaries for combined R/Rust packages.

CRAN has some fairly strict rules about hosting non-R binaries, not trying to deal with that right now.

## Install

manual download? small R script that downloads and installs it?

## Verify

If this works, you should be able to run this from wherever you run R:

```{r}
testbuild::verify_install()
```

This should print out "Package build was successful" if it successfully interfaced with the pre-compiled rust code.
