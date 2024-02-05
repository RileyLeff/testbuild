# testbuild

Just trying to figure out how to serve cross-platform binaries for combined R/Rust packages.

CRAN has some fairly strict rules about hosting non-R binaries, not trying to deal with that right now.

If this works, you should be able to run this from R:

```{r}
testbuild::verify_install()
```

This should print out "Package build was successful" if it worked.