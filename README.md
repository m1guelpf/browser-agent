# A browser AI agent, using GPT-4

This project provides a bridge between GPT-4 and a headless Chromium browser, allowing you to automate actions simply by describing them to the program. It takes the form of a Rust CLI, but also exports most of the internals as a library for others to use.

## Installation

`browser-agent` is built using Rust, so you'll need to install the Rust toolchain. You can do this by following the instructions at [rustup.rs](https://rustup.rs/).

Once you have Rust installed, you can install `browser-agent` by running:

```bash
cargo install browser-agent
```

You should also place your OpenAI API key in the `OPENAI_API_KEY` environment variable. This key should have access to the `gpt-4` model.

You can copy the contents of the `example.env` file to a `.env` file in the root of the project, and fill in the `OPENAI_API_KEY` variable. The `.env` file is ignored by git, so you don't have to worry about accidentally committing your API key. Note though, `.env.example` is not ignored, so you should not change that file.

## Usage

```
Usage: browser-agent [OPTIONS] <GOAL>

Arguments:
  <GOAL>  The goal for the agent to achieve

Options:
      --visual                Whether to show the browser window. Warning: this makes the agent more unreliable
  -v...                       Set the verbosity level, can be used multiple times
      --include-page-content  Whether to include text from the page in the prompt
  -h, --help                  Print help
  -V, --version               Print version
```

## Aknowledgements

This project was inspired and builds on top of [Nat Friedman](https://github.com/nat)'s [natbot](https://github.com/nat/natbot) experiment.

## License

This project is licensed under the MIT license. See [LICENSE](LICENSE) for more details.
