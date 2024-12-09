# Python client for BulletinBoard

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-python?style=flat-square)](https://crates.io/crates/bulletin-board-python)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-python?style=flat-square)](https://crates.io/crates/bulletin-board-python)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

## Caution

* Clients do not check whether the operation is successful or not to improve performance. Check the log of the server for the errors.
* The data is not encrypted. Please do not send any confidential data over the network.
* This crate is under development and is subject to change in specification. (Compatibility across `BulletinBoard` and `dbgbb` is ensured for the most minor version numbers.)

## Install

The package can be installed via [`pip`](https://pypi.org/project/bulletin-board-client/) as
```bash
pip install bulletin-board-client
```

## Example

To post and read the bulletins,
```python
import bulletin_board_client as bbclient
bbclient.set_addr("192.168.0.3:7578")

bbclient.post("test", "tag", [1,2,3])
bbclient.read("test")
```

## Compilation

Instead of installing via pip, you can compile the source yourself.

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

## Functions

|Function|Description|
|-|-|
|set_addr(address)|Set the address of the server. The address is either "ADDRESS:PORT" or "SOCKETPATH". If this function is not called, the default address is "127.0.0.1:7578".|
|set_timeout(timeout=None)|Set timeout for TCP connections in msec. If the argument is None, timeout is disabled (default).|
|post(title, tag(optional), data)|Post the data to the server. `title` and `tag` are str. `data` can be int, float, complex, str, list or numpy.array. Here, list must be able to be comverted to numpy.array. When the tag is ommitted, it becomes `Python`.|
|read(title, tag=None, revisions=None)|Read the bulletin. `revisions` is a list of int.|
|relabel(title_from, tag_from=None, title_to=None, tag_to=None)|Relabel a bulletin.|
|client_version()|Show the version of the client.|
|server_version()|Show the version of the server.|
|status()|Show the status of the server.|
|log()|Show the log of the server.|
|view_board()|List the bulletins.|
|get_info(title, tag=None)|See the details of the bulletin.|
|clear_revisions(title, tag(optional), revisions)|Clear the specified revisions.|
|remove(title, tag=None)|Remove all revisions of the specified bulletin.|
|archive(archive_name, title, tag=None)|Save the bulletin to an archive and make the data persistent.|
|load(archive_name)|Load the archived data. (The archive name is added to the tag)|
|list_archive()|List the archives.|
|rename_archive(archive_from, archive_to)|Rename an archive. This is executed when `reset` is called.|
|delete_archive(archive_name)|Delete an archive. This is executed when `reset` is called.|
|dump(archive_name)|Save all the bulletins to an archive.|
|restore(archive_name)|Reset the server and restore the archived data. (The data is restored to memory/file without modification of the tag)|
|clear_log()|Clear the log of the server.|
|reset_server()|Reset the BulletinBoard server.|
|terminate_server()|Terminate the BulletinBoard server.|

