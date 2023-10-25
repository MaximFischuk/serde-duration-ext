# Serde support for Duration and chrono::Duration

## Installation

Add the following to your Cargo.toml:

```toml
[dependencies]
serde_duration_ext = "0.1.0"
```

Also you can enable the `chrono` feature to support chrono::Duration

```toml
[dependencies]
serde_duration_ext = { version = "0.1.0", features = ["chrono"] }
```

## Usage

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo {
    #[serde(with = "serde_duration_ext")]
    duration: std::time::Duration,
}

fn main() {
    let foo = Foo {
        duration: std::time::Duration::from_secs(123),
    };
    let json = serde_json::to_string(&foo).unwrap();
    assert_eq!(json, r#"{"duration":"123s"}"#);

    let foo2: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(foo, foo2);
}
```

You can also use `chrono::Duration` if you enable the `chrono` feature.

```rust
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo {
    #[serde(with = "serde_duration_ext::chrono")]
    duration: chrono::Duration,
}

fn main() {
    let foo = Foo {
        duration: chrono::Duration::seconds(123),
    };
    let json = serde_json::to_string(&foo).unwrap();
    assert_eq!(json, r#"{"duration":"123s"}"#);

    let foo2: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(foo, foo2);
}
```

Library also provides useful types such as `DurationUnit` and `TimeUnit`

```rust
use serde::{Serialize, Deserialize};
use serde_json;

use serde_duration_ext::DurationUnit;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Foo {
    duration: DurationUnit,
}

fn main() {
    let foo = Foo {
        duration: DurationUnit::from_secs(123),
    };
    let json = serde_json::to_string(&foo).unwrap();
    assert_eq!(json, r#"{"duration":"123s"}"#);

    let foo2: Foo = serde_json::from_str(&json).unwrap();
    assert_eq!(foo, foo2);
}
```
