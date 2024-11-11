Rust client for BulletinBoard
=============================
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

Caution
-------
* Clients do not check whether the operation is successful or not to improve performance. Check the log of the server for the errors.
* The data is not encrypted. Please do not send any confidential data over the network.
* This crate is under development and is subject to change in specification. (Compatibility across `BulletinBoard` and `dbgbb` is ensured for the most minor version numbers.)
* The included tests will access the server and potentially erase existing data.

Example
-------
Before using `bulletin-board-client`, you must set up a [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard) server and set the server address in the environmental variable. It is convenient to set it in `.cargo/config.toml` of your Rust project:
```rust
[env]
BB_ADDR = "ADDRESS:PORT" // or "PATH" for Unix socket
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
    bbclient::archive("acv", "x", Some("tag")).unwrap();
    bbclient::reset_server().unwrap(); // Delete all temporary data.

    bbclient::load("acv").unwrap();
    dbg!(bbclient::view_board().unwrap());
}
```

See the docs for the details of functions.

Environment Variables
---------------------
|Variable|Default|Description|
|-|-|-|
|BB_ADDR|"127.0.0.1:7578" or "/tmp/bb.sock"|Address of the bulletin board server. It is either [IP address]:[port] or [hostname]:[port]. If you use a Unix socket, the address should be the path to an uncreated socket. The address can be modified later by calling `set_addr(...)`.|


Crate Features
--------------
|Feature|Description|
|-|-|
|`ndarray_15`|Enable ndarray support. The compatible version is 0.15.x.|
|`ndarray_16`|Enable ndarray support. The compatible version is 0.16.x.|
|`nalgebra`|Enable nalgebra support. Confirmed to work with version 0.33.0.|