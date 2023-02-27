# wmata
wmata is a high level async Rust interface to the [Washington Metropolitan Area Transit Authority API](https://developer.wmata.com).

## Contents
- [Requirements](#requirements)
- [Installation](#installation)
- [Usage](#usage)
    - [Getting Started](#getting-started)
    - [Design](#design)
    - [Using `MetroRail`](#using-MetroRail)
    - [Using `MetroBus`](#using-MetroBus)
- [Testing](#testing)
- [Dependencies](#dependencies)
- [Contact](#contact)
- [License](#license)
- [Deploying](#deploying)
   - [Kubernetes](#Kubernetes)
   - [ECS](#ECS)
   - [Terraform](#Terraform)


## Requirements
- Rust 1.39

## Installation

### Cargo
```toml
wmata = "7.1.0"
```

## Usage

### Getting Started
```rust
use wmata::{MetroRail, Station};

let client = MetroRail::new(api_key);

let trains = client.next_trains(Station::A01).await?;
```

### Design
wmata breaks the WMATA API into two components: `MetroRail` and `MetroBus`.

#### `MetroRail`
Provides access to all MetroRail related endpoints.

##### Using `MetroRail`
```rust
use wmata::{MetroRail, Station};

let client = MetroRail::new(api_key);

let trains = client.next_trains(Station::A01).await?;
```

#### `MetroBus`
Provides access to all MetroBus related endpoints.


##### Using `MetroBus`
```rust
use wmata::MetroBus;

let client = MetroBus::new(api_key);

let routes = client.routes().await?;
```

## Testing
Note that tests must currently be run with `--test-threads 1` in order to pass, due to using live data.

## Dependencies
- serde
- serde_json
- reqwest
- chrono
- await_trait
- tokio_test

## Contact
Feel free to email questions and comments to [emma@emma.sh](mailto:emma@emma.sh)

## License

wmata is released under the MIT license. [See LICENSE](https://github.com/emma-k-alexandra/wmata/blob/master/LICENSE) for details.



###### I forked this repo to add examples for commonly used dynamic container API systems; or apps. The engineer of this is a respected colleague/friend, and a very talented technologist.

## Deploying


### Kubernetes


### ECS


### Terraform

