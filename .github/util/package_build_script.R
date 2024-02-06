meta <- Sys.info()


# desc_path <- "DESCRIPTION"
# version_info <- readLines(desc_path)[3]
# extracted_version <- sub("^[^0-9]*", "", version_info)

build_dir <- "build"
dir.create(build_dir)

output_path <- paste0(
    tolower(meta['sysname']),
    "_",
    tolower(meta['machine']),
    "_",
    "testbuild",
    # "_",
    # extracted_version,
    switch(
        tolower(meta['sysname']), 
        'windows' = '.zip', 
        'darwin' = '.tgz', 
        'linux-gnu' = '.tar.gz'
    )
)

please_no_output <- rextendr::document(quiet = TRUE)

please_no_output <- devtools::build(
    quiet = TRUE, 
    binary = TRUE, 
    path = paste(
        build_dir, 
        output_path, 
        sep = "/"
    )
)