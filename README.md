# Hyper Template

Template repo for a backend app.  Do a global find-and-replace on `hyper-template`, filling in your app's name.  Change the Author info to yourself in these three files:

- LICENSE
- Cargo.toml
- Package.json

TODO - a separate template that's this plus a blog option, using the build script.
TODO - tests, unit and E2E

## Requirements

- [Rust 2018](https://www.rust-lang.org/) - stable toolchain
- [Node/NPM](https://nodejs.org/en/)
- [Docker](https://www.docker.com/)

## Usage

### NPM Scripts

- `dev`: start dev server on port 3000, watching for source changes
- `prod`: build and start production Docker container on port 8080 - must stop container via docker
- `run`: run local image
- `lint`: run linters
- `test`: run tests
- `test:watch`: run tests, watching for changes

### Executable Options

Options set in `src/config.toml` override these defaults, but options passed at the command line override `config.toml`.

## Dependencies

### Crates

- [askama](https://github.com/djc/askama) - Templates
- [hyper](https://hyper.rs/) - HTTP
- [lazy_static](https://github.com/rust-lang-nursery/lazy-static.rs) - Runtime-evaluated statics
- [log](https://github.com/rust-lang/log) - Logging macros
- [pretty_env_logger](https://github.com/seanmonstar/pretty-env-logger) - Pretty log output
- [structopt](https://github.com/TeXitoi/structopt) - CLI

### Style

- [TailwindCSS](https://tailwindcss.com/)
- [Postcss](https://postcss.org/)
- [Autoprefixer](https://github.com/postcss/autoprefixer)
- [Purgecss](https://purgecss.com/)
- [Cssnano](https://cssnano.co/)
- [Stylelint](https://stylelint.io/)
