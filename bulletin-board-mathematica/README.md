Mathematica client for BulletinBoard
====================================
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

Example
-------
First, you need to clone the repository:
```bash
cargo clone bulletin-board-mathematica
# OR
git clone https://github.com/YShoji-HEP/BulletinBoard.git
```
Then, go to `bulletin-board-mathematica` directory and run
```bash
cargo build -r
```
If it does not compile, see [`wolfram-library-link`](https://crates.io/crates/wolfram-library-link). Notice that `Mathematica` or `Wolfram Engine` has to be installed before the compilation.
Then, copy `target/release/libbulletin_board_mathematica.dylib` to the same directory as `bulletin-board.wl`, which can be downloaded from [here](https://github.com/YShoji-HEP/BulletinBoard/blob/main/bulletin-board-mathematica/bulletin-board.wl).

To post and read the bulletins, 
```
<< "bulletin-board.wl";
BBLoadFunctions["127.0.0.1:7578"];
BBPost["test",{1,2,3}];
BBRead["test"]
```

Functions
----------
|Function|Description|
|-|-|
|BBLoadFunctions[address]|Load functions of BulletinBoard client. The address is either "ADDRESS:PORT" or "SOCKETPATH". This has to be executed first.|
|BBPost[varName, varTag(optional), data]|Post the data to the server. `varName` and `varTag` are Text. `data` can be Integer, Real, Complex, Text, or List. For List, the types of the elements should be the same and has to have the same number of elements for nested Lists.|
|BBRead[varName, varTag(optional), revisions(optional)]|Read the bulletin. `revisions` can be Integer or List of Integer.|
|BBStatus[]|Show the status of the server.|
|BBLog[]|Show the log of the server.|
|BBViewBoard[]|List the bulletins.|
|BBGetInfo[varName, varTag(optional)]|See the details of the bulletin.|
|BBClearRevisions[varName, varTag, revisions]|Clear the specified revisions.|
|BBRemove[varName, varTag]|Remove all revisions of the specified bulletin.|
|BBArchive[varName, varTag, archiveName]|Save the bulletin to an archive and make the data persistent.|
|BBLoad[archiveName]|Load the archived data. (The archive name is added to the tag)|
|BBListArchive[]|List the archives.|
|BBRenameArchive[archiveFrom, archiveTo]|Rename an archive.|
|BBDeleteArchive[archiveName]|Delete an archive.|
|BBDump[archiveName]|Save all the bulletins to an archive.|
|BBRestore[archiveName]|Restore the archived data. (The data is restored to memory/file without modification of the tag)|
|BBReset[]|Reset the BulletinBoard server.|

Crate Features
--------------
|Feature|Description|
|-|-|
|`unix`|Use the UNIX socket instead of TCP. Only for UNIX-like OS.|
