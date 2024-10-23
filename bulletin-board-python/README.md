Python client for BulletinBoard
====================================
[![Sponsors](https://img.shields.io/badge/offer-Coffee-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-python?style=flat-square)](https://crates.io/crates/bulletin-board-python)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-python?style=flat-square)](https://crates.io/crates/bulletin-board-python)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

Example
-------
This crate depends on python packages of `numpy` and `maturin`.

First, you need to clone the repository:
```bash
cargo clone bulletin-board-python
# OR
git clone https://github.com/YShoji-HEP/BulletinBoard.git
```
Then, go to `bulletin-board-python` directory and run
```bash
maturin develop -r
```

To post and read the bulletins,
```python
import bulletin_board_client as bbclient
bbclient.set_addr("127.0.0.1:7578")

bbclient.post("test", "tag", [1,2,3])
bbclient.read("test")
```

Functions
----------
|Function|Description|
|-|-|
|set_addr(address)|Set the address of the server if . The address is either "ADDRESS:PORT" or "SOCKETPATH". This has to be executed first if you want to use different address from the default value of "127.0.0.1:7578" or "/tmp/bb.sock".|
|post(title, tag, data)|Post the data to the server. `title` and `tag` are str. `data` can be int, float, complex, str, list or numpy.array. Here, list must be able to be comverted to numpy.array.|
|read(title, tag=None, revisions=None)|Read the bulletin. `revisions` is a list of int.|
|status()|Show the status of the server.|
|log()|Show the log of the server.|
|view_board()|List the bulletins.|
|get_info(title, tag=None)|See the details of the bulletin.|
|clear_revisions(title, tag, revisions)|Clear the specified revisions.|
|remove(title, tag)|Remove all revisions of the specified bulletin.|
|archive(title, tag, archive_name)|Save the bulletin to an archive and make the data persistent.|
|load(archive_name)|Load the archived data. (The archive name is added to the tag)|
|list_archive()|List the archives.|
|rename_archive(archive_from, archive_to)|Rename an archive. This is executed when `reset` is called.|
|delete_archive(archive_name)|Delete an archive. This is executed when `reset` is called.|
|dump(archive_name)|Save all the bulletins to an archive.|
|restore(archive_name)|Restore the archived data. (The data is restored to memory/file without modification of the tag)|
|reset()|Reset the BulletinBoard server.|

Crate Features
--------------
|Feature|Description|
|-|-|
|`unix`|Use the UNIX socket instead of TCP. Only for UNIX-like OS.|
