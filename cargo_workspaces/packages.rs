// Installing binaries from crates.io
// cargo install <crate-name>

// You can install either a binary or a library crate from crates.io.
// To install a binary crate from crates.io, you can use the cargo install command and specify the name of the crate you want to install.

// All binaries installed using cargo install are stored in the ~/.cargo/bin directory.
// make sure that directory is in your PATH environment variable so you can run the installed binaries from the command line.
// You can add the following line to your shell configuration file to add the ~/.cargo/bin directory to your PATH:

// export PATH="$HOME/.cargo/bin:$PATH"
// to check existing path, use echo $PATH

// to install ripgrep, use the following command:
// cargo install ripgrep

// to verify the installation, use the following command:
// rg --version

// Extending Cargo with custom commands
// You can extend Cargo with custom commands by creating a binary crate that implements the cargo subcommand you want to add.

// eg. cargo-count
// cargo-count is a custom command that counts the number of lines, words, and characters in the specified files.
// To install cargo-count, use the following command:
// cargo install cargo-count