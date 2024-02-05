meta <- Sys.info()

output_path <- paste0(
    tolower(meta['sysname']),
    "_",
    tolower(meta['machine']),
    "_",
    "testbuild",
    switch(
        tolower(meta['sysname']), 
        'windows' = '.zip', 
        'darwin' = '.tgz', 
        'linux-gnu' = '.tar.gz'
    )
)

please_no_output <- rextendr::document(quiet = TRUE)

please_no_output <- devtools::build(quiet = TRUE, binary = TRUE, path = output_path)

cat("::set-env name=R_BUILD_ARTIFACT_PATH::", output_path, "\n")