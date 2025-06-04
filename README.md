# `project_with_bindings`

### Setup

Project tree structure:

```sh
.
├── Cargo.lock
├── Cargo.toml
├── LICENSE
├── README.md
├── py
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   └── target
├── r
│   ├── DESCRIPTION
│   ├── LICENSE.md
│   ├── NAMESPACE
│   ├── R
│   ├── configure
│   ├── configure.win
│   ├── man
│   ├── src
│   └── tools
├── rust
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   └── target
└── target
    ├── CACHEDIR.TAG
    └── debug

```

```ignore
.Rproj.user
target
rust/target
py/target
r/src/rust/target
```

```r
rextendr::use_extendr(path = "", crate_name = "projectRbindings")
```

### Issue

Calling

```r
remotes::install_github("CGMossa/proj_with_bindings", ref = "main", subdir = "r")
```

results in

```sh
Updating crates.io index
error: failed to get `project_with_bindings` as a dependency of package `projectRbindings v0.1.0 (/private/var/folders/_x/bq8vb1b156sgl363l71by61h0000gn/T/RtmpNU4dE7/R.INSTALL98f31ac83ca7/projectRbindings/src/rust)`

Caused by:
  failed to load source for dependency `project_with_bindings`

Caused by:
  Unable to update /private/var/folders/_x/bq8vb1b156sgl363l71by61h0000gn/T/RtmpNU4dE7/R.INSTALL98f31ac83ca7/rust

Caused by:
  failed to read `/private/var/folders/_x/bq8vb1b156sgl363l71by61h0000gn/T/RtmpNU4dE7/R.INSTALL98f31ac83ca7/rust/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
make: *** [rust/target/release/libprojectRbindings.a] Error 101
ERROR: compilation failed for package ‘projectRbindings’
* removing ‘/Users/elea/Library/R/arm64/4.5/library/projectRbindings’
Warning message:
In i.p(...) :
  installation of package ‘/var/folders/_x/bq8vb1b156sgl363l71by61h0000gn/T//RtmpY6WvXn/file904e49772f9c/projectRbindings_0.0.0.9000.tar.gz’ had non-zero exit status
```
