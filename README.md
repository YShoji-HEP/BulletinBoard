Bulletin Board
===========================
Object storage for [`ArrayObject`](https://github.com/YShoji-HEP/ArrayObject) for debugging and data taking purposes.

`BulletinBoard` is a part of [`dbgbb`](https://github.com/YShoji-HEP/dbgbb) project.

Highlights
----------
* Hybrid backend of memory and file, selected based on the size of the object and the allocated memory.
* Key is a combination of a name and a tag. Each key contains revisions of `ArrayObject`.
* If there is only one tag for a given name, tag can be omitted.
* If the revision is omitted, the most recent revision is returned.
* The data is temporary except for archives.

Example
-------
Install and run the server with the specified listen address.
```bash
cargo install bulletin-board-server
export BB_LISTEN_ADDR = "0.0.0.0:7578"
bulletin-board-server
```

Rust client:
```rust
use bulletin_board_client as bbclient;
use array_object::*;

fn main() {
    let data: ArrayObject = vec![1f32, 2., -3., 5.].into();
    bbclient::post("x".to_string(), "tag".to_string(), data.clone());
    let rcvd = bbclient::read("x".to_string());
    let restored = rcvd.unpack().unwrap();
    assert_eq!(data, restored);
}
```

Mathematica client:
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

Q&A
--------------
#### Why temporary?
Since `BulletinBoard` was originally designed for debugging purposes, it is assumed that most of the data will be deleted at the end. Persistent options (`archive` and `dump`) have been added for a more extensive use such as data taking.
The advantages of not making it persistent first are (i) holding data in memory makes read/write speeds faster, (ii) metadata of the revisions is combined and becomes smaller and (iii) data can be more easily deleted before archiving.
#### Why not other object storages or databases?
Especially for debugging, storage may receive large amounts of small data and thus in-memory databases are ideal. However, it may also receive large data like a few hundred MiB, and such data should be stored in files. `BulletinBoard` uses a hybrid backend of memory and file to solve this problem. It also provides persistence options, which are useful for data taking.