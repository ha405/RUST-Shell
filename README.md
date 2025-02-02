# Basic Shell Implementation in Rust

This project provides a basic implementation of a shell in Rust, demonstrating fundamental shell functionalities using low-level system calls. It is designed to mimic a command-line interface, allowing users to interact with the operating system by executing commands. This project showcases Rust's ability to interact with the operating system via system calls and demonstrates basic file navigation and control.

## Features

This shell implementation provides the following core features:

*   **Navigation:**
    *   `pwd`: Print the current working directory.
    *   `cd`: Change the current directory.
    *   `pushd`: Push the current directory onto a stack and change to the specified directory.
    *   `popd`: Pop the top directory from the stack and change to that directory.
    *   `dirs`: Display the stack of directories used by `pushd`.

*   **Basic Commands:**
    *   `echo`: Print the provided arguments to the standard output.
    *   `clear`: Clear the terminal screen.
    *   `ls`: List the contents of the current directory.

*   **File Management:**
    *   `mkdir`: Create a new directory.
    *   `rmdir`: Remove an empty directory.

*   **Command Execution:**
    *   Supports basic command execution using system calls.

*   **Exit:**
    *   `exit`: Terminate the shell.

## How It Works

The shell operates through a loop that reads user input, parses the command, and executes the corresponding action. It utilizes several key concepts:

*   **Current Directory Tracking:** The current working directory is managed through system calls, ensuring proper navigation.
*   **Directory Stack:** A stack (implemented using `VecDeque`) maintains the history of directories when using the `pushd` and `popd` commands.
*   **Command Parsing:** The user input is tokenized, and the first token is used to determine the command to execute. The remainder of the tokens are arguments to the command.
*   **System Calls:** Direct system calls are used for changing directories (`chdir`), creating directories (`mkdir`), removing directories (`rmdir`), and executing programs (`fork`, `execvp`).
*   **Process Management:** The shell forks processes to execute commands, then uses `waitpid` to await the termination of the child process.
*   **Error Handling:** Basic error handling is included, such as printing messages if commands fail.

## Technical Implementation

The shell uses several key Rust libraries and system call interfaces:

*   `std::collections::VecDeque`: For managing the stack of directories.
*   `std::env`: For environment variables.
*   `std::ffi::CString`: For conversion to C-style strings for system calls.
*   `std::io`: For input and output operations (reading user input, writing to the console).
*   `libc`: Rust bindings for C standard library functions.
*   **Low-Level System Calls:** Uses system calls like `chdir`, `execvp`, `fork`, `getcwd`, `mkdir`, `rmdir`, and `waitpid`.

## Challenges

This project required careful management of:

*   **Low-Level System Calls:** Using `libc` requires unsafe blocks, which means the programmer has to be careful not to use those functions incorrectly.
*   **Process Management:** Correct use of `fork` to run other executables as child processes.
*   **C String Conversion:** When working with low-level system calls, arguments must be converted to C-style strings.

## Setup and Usage

### Prerequisites

*   Ensure that Rust and Cargo are installed on your system:
    *   [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
*   Docker (optional): If you want to containerize the application, ensure Docker is installed:
    *   [https://docs.docker.com/get-docker/](https://docs.docker.com/get-docker/)
*   This project is designed to run on **Unix-like operating systems**. It has been tested on a Linux environment and will not work natively on Windows without modification.

### Running the Shell with Docker

To containerize the application and run it in a Linux environment, follow these steps:

#### 1. Build the Docker Image

Run the following command to build the Docker image:
```bash
docker build -t rust-shell-app .
```

This command builds the image using the `Dockerfile` provided in the project. The `Dockerfile` uses a multi-stage build process:
- **Builder Stage**: Compiles the Rust application using the `rust:1.83` image.
- **Runtime Stage**: Copies the compiled binary into a minimal Debian-based image (`debian:bullseye-slim`) to reduce the final image size.

#### 2. Run the Container

Once the image is built, run the container interactively:
```bash
docker run -it --rm rust-shell-app
```

This starts the shell inside the container, allowing you to interact with it as if it were running locally.

#### 3. Debugging Inside the Container (Optional)

If you need to debug or inspect the container, you can start an interactive shell session:
```bash
docker run -it --rm rust-shell-app /bin/bash
```

From there, you can manually run the binary:
```bash
/usr/local/bin/RUST-Shell
```

#### 4. Notes on Docker

- The runtime image (`debian:bullseye-slim`) includes a newer version of `glibc` to avoid compatibility issues with the Rust binary.
- If you encounter issues related to `glibc` versions, consider switching to a newer runtime image (e.g., `debian:bookworm-slim`) or statically linking the binary using the `musl` target.
- To avoid rebuilding the Docker image every time you make changes to the code, you can mount your source code into the container at runtime:
  ```bash
  docker run -it --rm -v "$(pwd):/usr/src/RUST-Shell" rust-shell-app cargo run --release
  ```

## Future Improvements

* Add support for more shell commands (e.g., `cp`, `mv`, `rm`).
* Improve error handling and user feedback.
* Support for scripting and batch execution of commands.
* Cross-platform compatibility (e.g., Windows support using alternative system calls).
