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

### `nasu.json` Reference

`nasu.json` is the default configuration file for Nasu. This file is
parsed at startup by Nasu to initialize `Workers`.

The `nasu.json` file is composed by an array of `Task` objects as shown
below:

```json
// nasu.json
[
  {
    "id": "HTTPBIN POST Request",
    "type": "http",
    "task": {
      "interval": "* */10 * * * *"
    },
    "params": {
      "url": "http://httpbin.org/post",
      "method": "POST",
      "headers": {
        "authorization": "Bearer <Token>",
        "content-type": "application/json"
      }
    }
  }
]
```

Each of these `Task` must contain the following properties:

Property | Description | Required | Possible Values
--- | --- | --- | ---
`id` | The id of the service. Used as reference for the user | Yes | N/A
`type` | Type of service to perform check on | Yes | `http`
`task` | Task configuration | Yes | N/A
`task.interval` | Cron defintion to specify when to perform the test | Yes | N/A
`params` | Params for the `Worker` used on perform. [Refer to Worker Params](#worker-params) | Yes | N/A

### Worker Params

`Worker` params may vary based on the `type` of worker in question.
Properties defined below belong to an object specified in the `Task`
object, inside of the `params` property.

#### HTTP Worker Params

- Type: `http`
- Property: `params`

Property | Description | Required | Possible Values
--- | --- | --- | ---
`url` | URL to perform the HTTP Request | Yes | N/A
`method` | HTTP Method to perform the request with | Yes | `GET`, `PATCH`, `POST`, `PUT`, `DELETE`
`headers` | HTTP Headers to provide to the request | No | N/A

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
