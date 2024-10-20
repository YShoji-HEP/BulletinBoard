Bulletin Board
===========================
[![Sponsors](https://img.shields.io/badge/offer-Coffee-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

Object storage for [`ArrayObject`](https://github.com/YShoji-HEP/ArrayObject) for debugging and data taking purposes.

`BulletinBoard` is a part of [`dbgbb`](https://github.com/YShoji-HEP/dbgbb) project.

Highlights
----------
* Hybrid backend of memory and file, selected based on the size of the object and the allocated memory.
* Key is a combination of a title and a tag. Each key contains revisions of `ArrayObject`.
* The tag can be omitted if no other tags are present.
* If the revision is omitted, the most recent revision is returned.
* The data does not persist by default. Use `archive` or `dump` command to make it persistent.
* Docker image of the server is available.

Crates.io
-----------------
|Crate|Version|
|-|-|
|BulletinBoard Server|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-server?style=flat-square)](https://crates.io/crates/bulletin-board-server)|
|Rust Client|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)|
|Mathematica Client|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)|

Docker
------

The docker image is found [here](https://hub.docker.com/r/yshojihep/bulletin-board). (currently only available for arm64)
 
Example
-------
Install and run the server with the specified listen address.
```bash
cargo install bulletin-board-server
export BB_LISTEN_ADDR = "0.0.0.0:7578"
bulletin-board-server
```

Rust client: (see [`bulletin-board-client`](bulletin-board-client/README.md))
```rust
use bulletin_board_client as bbclient;
use array_object::*;

fn main() {
    let data: ArrayObject = vec![1f32, 2., -3., 5.].into();
    bbclient::post("x", "tag", data.clone());
    let rcvd = bbclient::read("x");
    let restored = rcvd.unpack().unwrap();
    assert_eq!(data, restored);
}
```

Mathematica client: (see [`bulletin-board-mathematica`](bulletin-board-mathematica/README.md))
```
<< "bulletin-board.wl";
BBLoadFunctions["127.0.0.1:7578"];
BBPost["test",{1,2,3}];
BBRead["test"]
```

ToDo
----
- [ ] Support for other languages. [Mathematica, Python, Julia, Go, C++, Fortran, ...]
- [ ] Windows support. 
- [ ] More informative logs.

Q&A
--------------
#### Why not persistent by default?
Since `BulletinBoard` was originally designed for debugging purposes, it is assumed that most of the data will be deleted at the end. Persistent options (`archive` and `dump`) have been added for a more extensive use such as data taking.
The advantages of not making it persistent by default are (i) holding data in memory makes read/write speeds faster, (ii) metadata of the archive becomes smaller and (iii) data can be more easily deleted before archiving.
#### Why not other object storages or databases?
Especially for debugging, storage may receive large amounts of small data and thus in-memory databases are ideal. However, it may also receive large data like a few hundred MiB, and such data should be stored in files. `BulletinBoard` uses a hybrid backend of memory and file to solve this problem. It also provides persistence options, which are useful for data taking.