<h1 align="center">Kanzen</h1>
<div align="center">
 <strong>
 Powerful, invite-only anime discord bot
 </strong>
<br />
<br />
</div>
</h1>


Kanzen is an invite-only discord bot built for anime communities. Currently, Kanzen is in heavily development. I advise against running your own instance.

### Building + Running

If you do decide to run this yourself, keep in mind, I won't provide support. However, I've provided some basic instructions below.

1. Ensure you have Rust + Cargo installed. You can install it [here](https://www.rust-lang.org/tools/install)
2. Clone the repository:
    `git clone https://github.com/justanotherbyte/kanzen`
3. Create a `config.toml` and fill it with the appropriate data. See [src/config.rs](src/config.rs)
4. Build the project with `cargo build`. Optionally add the `--release` flag for an optimized build
5. `cargo run`. If you built in `release` mode, ensure you add the `--release` flag.

### Testing

If you're happy to test the bot in your own server, I'm open to you sending me a DM @ `mooon#2472`. Keep in mind, whether you can add it is at my own discretion.

