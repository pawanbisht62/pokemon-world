# Pokemon World
This repository pertians to get basic information of pokemon based on the name(input param).

It contains two endpints:
* Basic Pokemon Information 
* Translated Pokemon Description

### Endpoint Details
#### Basic Pokemon Information
This endpoint takes pokemon name is an input and returns name, standard description, habitat & is_legendary status.

* Request Verb: GET
* Path Param: Pokemon name

##### Example:
* CURL:
```
curl --location --request GET '127.0.0.1:8080/pokemon/mewtwo'
```
* Response:
```
{
    "name": "ditto",
    "description": "Capable of copying\nan enemy's genetic\ncode to instantly\ftransform itself\ninto a duplicate\nof the enemy.",
    "habitat": "urban",
    "isLegendary": false
}
```

#### Translated Pokemon Description
This endpoint takes pokemon name is an input and returns name, translated description, habitat & is_legendary status.

* Request Verb: GET
* Path Param: Pokemon name

##### Example:
* CURL:
```
curl --location --request GET '127.0.0.1:8080/pokemon/translated/mewtwo'
```
* Response:
```
{
    "name": "ditto",
    "description": "Capable of copying\nan enemy's genetic\ncode to instantly\ftransform itself\ninto a duplicate\nof the enemy.",
    "habitat": "urban",
    "isLegendary": false
}
```

## Setting up your environment

### Rustup.rs

Building this project requires [rustup](https://rustup.rs/), version 1.8.0 or more recent.
If you have an older version, run `rustup self update`.

To install on Windows, download and run [`rustup-init.exe`](https://win.rustup.rs/)
then follow the onscreen instructions.

To install on other systems, run:

```
curl https://sh.rustup.rs -sSf | sh
```

This will also download the current stable version of Rust, which this project won’t use.
To skip that step, run instead:

```
curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain none
```

## Running Application

```
git clone https://github.com/pawanbisht62/pokemon-world.git
cd pokemon-world
cargo run
```

### Running application will look like this:
```
Compiling pokemon-world v0.1.0 (/home/knoldus/programming/pokemon-world)
    Finished dev [unoptimized + debuginfo] target(s) in 5.29s
     Running `target/debug/pokemon-world`
[2021-12-19T10:40:55Z INFO  pokemon_world] Application Started...
[2021-12-19T10:40:55Z INFO  actix_server::builder] Starting 8 workers
[2021-12-19T10:40:55Z INFO  actix_server::server] Actix runtime found; starting in Actix runtime

```

### Testing
This repository contains unit and integration test cases of all the available functionality. To execute the test cases hit this command:
```
cargo test
```

* Note:
All the test case won't be able to executed successfully as the translated endpoint of pokeapi accepts only 5 requests in an hour.
