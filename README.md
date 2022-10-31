# Overpass Turbo Crate

**Contact:** <revol-xut@protonmail.com>

This crate implements the overpass-turbo export schema. overpass-turbo 
is a domain specific language for querying openstreetmap features. The 
website can be found [here](https://overpass-turbo.eu/).

## Documentation

Soon TM lol

## Usage

```rust
use overpass_turbo::{OverpassTurbo, simplified};

fn main() {
    let mut file = OverpassTurbo::from_file("./tramdata.json").unwrap();
    let mut simplified = simplified::SimplifiedOverpassTurbo::from_struct(file);

    simplified.prune_nodes();
    simplified.prune_ways();

    // only prints relations
    for element in simplified {
        print!("{:?}", element);
    }
}
```

