- # Verso

A web browser that plays old world blues to build new world hope.

<img src="https://github.com/user-attachments/assets/ca124b2b-c54c-4796-b2cb-0819228495b6" width="600" alt="The Verso logo with a sitting cat, written in ASCII art, rendered in orange on black as on an old CRT monitor" />

Verso is a web browser built on top of the [Servo](https://servo.org/) web engine. It was archived Oct 8th, 2025, but this fork seeks to salvage components of Verso for a learning project. 

I am using AI and seeing how close I can get to a vision for a different kind of browser UI. This is not intended to be a functional browser or a proper successor to Verso; I'm trying to familiarize myself with Servo, the choices made in creating Verso, and to prepare for a future when Servo is more complete and embeddable.

You can see summaries of the planned design in /design_docs. Though I am using AI to understand the codebase and make changes, I have years of notes regarding this force directed node graph browser UI, and I merely fed those notes into a model to generate much of that documentation, which, trust me, is me doing you a favor.

Happy to stop using any component or repo for this project if the original author would prefer it. I do not pretend to be a professional developer, and I welcome criticism and advice regarding the idea.

# Usage

## Getting Started

### Windows

- Install [scoop](https://scoop.sh/) and then install other tools:

```sh
scoop install git python llvm cmake curl
pip install mako
```

> You can also use chocolatey to install if you prefer it.

- Build & run:

```sh
cargo run
```

### MacOS

- Install [Xcode](https://developer.apple.com/xcode/)
- Install [Homebrew](https://brew.sh/) and then install other tools:

```sh
brew install cmake pkg-config harfbuzz python@3 # Install required dependencies CMake, pkg-config, HarfBuzz, and Python 3.
pip3 install mako # Install the Mako templating engine
curl https://sh.rustup.rs -sSf | sh # Install Rust and Cargo
```

- Build & run:

```sh
cargo run
```

### Linux

#### Flatpak

For unified environment setup and package experience, we choose Flatpak to build the project from the start.
Please follow the [Flatpak Setup](https://flatpak.org/setup/) page to install Flatpak based on your distribution.

- Install flatpak runtimes and extensions:

```sh
flatpak install flathub org.freedesktop.Platform//24.08
flatpak install flathub org.freedesktop.Sdk//24.08
flatpak install flathub org.freedesktop.Sdk.Extension.rust-stable//24.08
flatpak install flathub org.freedesktop.Sdk.Extension.llvm18//24.08
```

- Generate manifests and build:
// TODO Exporting to a repository instead

```sh
python3 ./flatpak-cargo-generator.py ./Cargo.lock -o cargo-sources.json
flatpak-builder --user --install --force-clean target org.versotile.verso.yml
flatpak run org.versotile.verso
```

#### Nix

We also support building Verso in nix shell. But we don't bundle it in nix at the moment.

- For NixOS:

```sh
nix-shell shell.nix --run 'cargo r'
```

- For non-NixOS distributions:

```sh
nix-shell shell.nix --run 'nixGL cargo r'
```

If you prefer to build the project without any sandbox, please follow the instructions in [Servo book](https://book.servo.org/hacking/setting-up-your-environment.html#tools-for-linux) to bootstrap.
But please understand we don't triage any build issue without flatpak or nix setup.
