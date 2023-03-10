# rust-chatgpt

Rust CLI to interact with ChatGPT

## Run the CLI

Currently you need to use `cargo run` in order to kick off the CLI.

### Todos

- [ ] Save API key in .env and use it without prompting if it's already there.
- [ ] Prevent empty prompts from being passed to ChatGPT.
- [ ] Allow saving the conversation to a file when done.
- [ ] Setup a way where this can be installed as a binary with an alias on your machine.
- [ ] Cleanup the structure of the project. Everything is currently in main. Lets break it into single purpose functions to make it easier to change.
