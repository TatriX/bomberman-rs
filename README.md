<p align="center">
  <a href="https://amethyst.rs">
    <img
        alt="Amethyst"
        src="https://amethyst.rs/brand/logo-standard.svg"
        width="60"
    />
  </a>
</p>
<h1 align="center">
  Bomberman!
</h1>

## Quickstart

- Clone the repository

```bash
git clone https://github.com/TatriX/bomberman-rs.git bomberman
cd bomberman
```

- Build and run the project

```bash
cargo run
```

#### For Mac Users

This starter uses vulkan as a renderer by default. You'll want to change the backend to use `metal`, which can be done by opening the `Cargo.toml` file and changing

```toml
[features]
default = ["vulkan"]
```

to

```toml
[features]
default = ["metal"]
```

#### For Linux Users

You might need to install some dependencies. Please refer to [this section](https://github.com/amethyst/amethyst#dependencies) of the README for more details.
