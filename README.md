<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://via.placeholder.com/120" height="120" width="120" />
  </div>
  <h1 align="center">ナース (Nāsu)</h1>
  <h4 align="center">
    🧑🏻‍⚕️ Command-line utility which poll on remote addresses in order to perform status checks periodically
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/nasu.svg)](https://crates.io/crates/nasu)
  [![Documentation](https://docs.rs/nasu/badge.svg)](https://docs.rs/nasu)
  ![Build](https://github.com/EstebanBorai/nasu/workflows/build/badge.svg)
  ![Lint](https://github.com/EstebanBorai/nasu/workflows/clippy/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/nasu/workflows/tests/badge.svg)

</div>

## Motivation

Nāsu (from Japanese ナース [Nāsu], which means nurse), is a command-line utility
to perform checks on remote addresses periodically.

## Terminology

### Service

Service to perform task against through the Worker

### Worker

Worker in responsible of performing the task

### Task

Defintion of steps to be performed on a Service by a Worker
## Release

In order to create a release you must push a Git tag as follows

```shell
git tag -a <version> -m <message>
```

**Example**

```shell
git tag -a  v0.1.0 -m "First release"
```

> Tags must follow SemVer conventions and must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```shell
git push origin main --follow-tags
```
