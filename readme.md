# Wercker build status

## About
This is a simple tool to make getting the status of the last running build easy.

## installing

for now, just clone the repo and do:

```shell
cargo build --release
```

you will need to have installed rust first, you can easily do this using rustup:

```shell
curl https://sh.rustup.rs -sSf | sh && rustup install stable
```

The binary will be in the `./target/release` folder which you can copy and move to more usable location.

## Config
To configure the tool create a config file `config.toml` and then pass the
file to the tool like this (file relative to current directory):

```shell
wercker_build_status config
wercker_build_status config.toml
wercker_build_status ./config.toml
```
an example config file can be found in this repo's root dir.

## envs

The tool can also pull info from the environment variables as well, just prefix
the config options with `WERCKER_` and capitalize the key name.

## output

The output of the tool is like this:

```
status|result
```
examples being:

+ `running`
+ `passed`
+ `failed`
