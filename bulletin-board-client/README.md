Rust client for BulletinBoard
=============================
[![Sponsors](https://img.shields.io/badge/offer-Coffee-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

Example
-------
Before using `bulletin-board-client`, you must set up a [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard) server and set the server address in the environmental variable. It is convenient to set it in `.cargo/config.toml` of your Rust project:
```rust
[env]
BB_ADDR = "ADDRESS:PORT"
```

To post and read the bulletins, 
```rust
use bulletin_board_client as bbclient;
use bbclient::*;

fn main() {
    let data: ArrayObject = vec![1f32, 2., -3., 5.].try_into().unwrap();
    bbclient::post("x", "tag", data.clone()).unwrap();

    let recv = bbclient::read("x", None, vec![]).unwrap().pop().unwrap();
    let restored = recv.try_into().unwrap();
    assert_eq!(data, restored);
}
```

To make the data persistent,
```rust
use bulletin_board_client as bbclient;

fn main() {
    bbclient::archive("x", "tag", "acv").unwrap();
    bbclient::reset().unwrap(); // Delete all temporary data.

    bbclient::load("acv").unwrap();
    dbg!(bbclient::view_board().unwrap());
}
```
Environment Variables
---------------------
|Variable|Default|Description|
|-|-|-|
|BB_ADDR|"127.0.0.1:7578" or "/tmp/bb.sock"|Address of the bulletin board server. It is either [IP address]:[port] or [hostname]:[port]. When UNIX socket is used, the address should be the path to the uncreated socket.|


Crate Features
--------------
|Feature|Description|
|-|-|
|`unix`|Use the UNIX socket instead of TCP. Only for UNIX-like OS.|
|`ndarray_15`|Enable ndarray support. The compatible version is 0.15.x.|
|`ndarray_16`|Enable ndarray support. The compatible version is 0.16.x.|
|`nalgebra`|Enable nalgebra support. Confirmed to work with version 0.33.0.|