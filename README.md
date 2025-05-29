# Timelock - Securing files for the future ⌛

![GitHub License](https://img.shields.io/github/license/joflucki/timelock)
![Rust Badge](https://img.shields.io/badge/built%20with-Rust-f74c00?logo=rust)
![GitHub last commit](https://img.shields.io/github/last-commit/joflucki/timelock?color=purple)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/joflucki/timelock/test.yml?label=tests&color=blue)

Timelock is a secure, time-locked file sharing application built with Rust. It allows users to encrypt files and share them with recipients who can only decrypt them after a specified future date.

## Project Structure

The repository is structured as a Rust workspace containing the following crates:

* `client`: Contains the CLI for users to encrypt, send, and receive time-locked files.
* `server`: Implements the backend server responsible for storing and managing the encrypted files and associated metadata.
* `shared`: Houses common code and data structures used by both the client and the server.

## Quickstart

Download both the Timelock client and server executables, available in the [latest release](https://github.com/joflucki/timelock/releases/).

Rename the downloaded files for ease-of-use:

```sh
mv timelock-client-v0.1.0-linux-x86_64 tl
mv timelock-server-v0.1.0-linux-x86_64 timelock-server
```

Add execution permissions if necessary:
```sh
chmod +x tl
chmod +x timelock-server
```

Start the server:
```sh
./timelock-server
```
This will start the server with the default TLS certificate on local port 8443.

Then, you can use the CLI to interact with the local server:
```
./tl signup --user jonas
./tl logout
./tl login --user jonas
```