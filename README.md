# Bulletin Board

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

Object storage for [`ArrayObject`](https://github.com/YShoji-HEP/ArrayObject) for debugging and data taking purposes.

`BulletinBoard` is a part of [`dbgbb`](https://github.com/YShoji-HEP/dbgbb) project.

<img src="bulletin-board-gui/snapshot.png" width="500">

## Highlights

* Hybrid backend of memory and file, selected based on the size of the object and the allocated memory.
* Key is a combination of a title and a tag. Each key contains revisions of `ArrayObject`.
* Simple access to data. For example, revision can be omitted. Then, the most recent revision is returned. The tag can also be omitted if no other tags are present.
* The commands `archive` and `dump` make data persistent. (Data does not persist by default.)
* Unix sockets can be used with Unix-like operating systems, which makes the communication speed quite fast.
* GUI application is available. You can download the binary files from [Release](https://github.com/YShoji-HEP/BulletinBoard/releases).

## Caution

* Clients do not check whether the operation is successful or not to improve performance. Check the log of the server for the errors.
* The data is not encrypted. Please do not send any confidential data over the network.
* This crate is under development and is subject to change in specification. (Compatibility across `BulletinBoard` and `dbgbb` is ensured for the most minor version numbers.)
* The included tests will access the server and potentially erase existing data.

## Crates.io

|Crate|Version|
|-|-|
|BulletinBoard Server|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-server?style=flat-square)](https://crates.io/crates/bulletin-board-server)|
|Rust Client|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-client?style=flat-square)](https://crates.io/crates/bulletin-board-client)|
|Mathematica Client|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)|
|Python Client|[![Crates.io](https://img.shields.io/crates/v/bulletin-board-python?style=flat-square)](https://crates.io/crates/bulletin-board-python)|

## Server

There are several options for running the `BulletinBoard` server.

**Cargo**

Install and run the server with the specified listen address.
```bash
cargo install bulletin-board-server
export BB_LISTEN_ADDR = "0.0.0.0:7578"
bulletin-board-server
```

**Docker**

The docker image is available and you can run the server by
```
docker run -p 7578:7578 -v /path/to/vol:/data yshojihep/bulletin-board:latest
```
For details, see [DockerHub](https://hub.docker.com/r/yshojihep/bulletin-board).

**GUI**

The GUI app includes a built-in server. The binary file can be downloaded from [Release](https://github.com/YShoji-HEP/BulletinBoard/releases).
## Client

Currently available clients are listed below.

**Rust client** (see [`bulletin-board-client`](bulletin-board-client/README.md))
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

**Mathematica client** (see [`bulletin-board-mathematica`](bulletin-board-mathematica/README.md))
```python
<< "bulletin-board.wl";
BBSetAddr["192.168.0.3:7578"];
BBPost["test",{1,2,3}];
BBRead["test"]
```

**Python client** (see [`bulletin-board-python`](bulletin-board-python/README.md))
```python
import bulletin_board_client as bbclient
bbclient.set_addr("192.168.0.3:7578")

bbclient.post("test", "tag", [1,2,3])
bbclient.read("test")
```

## ToDo

- [x] Support for Python.
- [x] Windows support.
- [x] GUI app (server + client + palettes).
- [x] Distribute at PyPI and Mathematica paclet repositories.
- [ ] Support for other languages. [Julia, Go, C++, Fortran, ...]
- [ ] `Watch` function to signal external programs.

## Q&A

#### Why not persistent by default?
Since `BulletinBoard` was originally designed for debugging purposes, it is assumed that most of the data will be deleted at the end. Persistent options (`archive` and `dump`) have been added for a more extensive use such as data taking.
The advantages of not making it persistent by default are (i) holding data in memory makes read/write speeds faster, (ii) metadata of the archive becomes smaller and (iii) data can be more easily deleted before archiving.
#### Why not other object storages or databases?
Especially for debugging, storage may receive large amounts of small data and thus in-memory databases are ideal. However, it may also receive large data like a few hundred MiB, and such data should be stored in files. `BulletinBoard` uses a hybrid backend of memory and file to solve this problem.
Also, the `BulletinBoard` will not return a response if it is not needed. Thus, it can handle very frequent data flows.