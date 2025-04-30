# Timelock

![GitHub License](https://img.shields.io/github/license/joflucki/timelock?color=red)
![Rust Badge](https://img.shields.io/badge/built%20with-Rust-00894f?logo=rust)
![GitHub last commit](https://img.shields.io/github/last-commit/joflucki/timelock?color=purple)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/joflucki/timelock/test.yml?label=tests)

Timelock is a secure, time-locked file sharing application built with Rust. It allows users to encrypt files and share them with recipients who can only decrypt them after a specified future date. This project is developed as part of a cryptography course.

## Features

* **End-to-end Encryption:** Files are encrypted before being sent and decrypted only by the intended recipient.
* **Time-Based Access Control:** Recipients can only decrypt the files after a predefined unlock date and time.
* **Secure Key Management:** [Briefly mention how keys are handled, e.g., key derivation, exchange mechanism, etc. If it's part of the project scope later, you can add a placeholder and elaborate later.]
* **Cross-Platform Compatibility:** Built with Rust for potential cross-platform deployment.
* **Modular Design:** Organized into client, server, and shared library crates.

## Project Structure

The repository is structured as a Rust workspace containing the following crates:

* `client`: Contains the command-line interface (CLI) or graphical user interface (GUI) for users to encrypt, send, and receive time-locked files.
* `server`: Implements the backend server responsible for storing and managing the encrypted files and associated metadata.
* `shared`: Houses common code and data structures used by both the client and the server.