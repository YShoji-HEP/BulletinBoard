# Mathematica client for BulletinBoard

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-mathematica?style=flat-square)](https://crates.io/crates/bulletin-board-mathematica)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

## Caution

* Clients do not check whether the operation is successful or not to improve performance. Check the log of the server for the errors.
* The data is not encrypted. Please do not send any confidential data over the network.
* This crate is under development and is subject to change in specification. (Compatibility across `BulletinBoard` and `dbgbb` is ensured for the most minor version numbers.)

## Install

The package can be installed via [Paclet repository](https://resources.wolframcloud.com/PacletRepository/resources/Yshojihep/BulletinBoardClient/) as
```bash
PacletInstall["Yshojihep/BulletinBoardClient"]
```
For the first run, you need to compile the library. Run
```bash
<< "Yshojihep`BulletinBoardClient`";
```
and follow the instruction.

## Example

To post and read the bulletins, 
```bash
<< "Yshojihep`BulletinBoardClient`";
BBSetAddr["192.168.0.3:7578"];

BBPost["test",{1,2,3}];
BBRead["test"]
```
## Compilation

Instead of installing via Paclet, you can compile the source yourself.

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
Then, copy `target/release/libbulletin_board_mathematica.*` to `BulletinBoardClient/LibraryResources/`.

## Functions

|Function|Description|
|-|-|
|BBSetAddr[address]|Set the address of the server. The address is either "ADDRESS:PORT" or "SOCKETPATH". If this function is not called, the default address is "127.0.0.1:7578".|
|BBSetTimeout[timeout]|Set timeout for TCP connections in msec. If it is executed without the argument, timeout is disabled (default).|
|BBPost[title, tag(optional), data]|Post the data to the server. `title` and `tag` are Text. `data` can be Integer, Real, Complex, Text, or List. For List, the types of the elements should be the same and has to have the same number of elements for nested Lists. If tag is not set, the default value "Mathematica" is used.|
|BBRead[title, tag(optional), revisions(optional)]|Read the bulletin. `revisions` can be Integer or List of Integer.|
|BBRelabel[titleFrom, tagFrom, titleTo, tagTo]|Relabel a bulletin. The last three arguments can be ommited by setting them as "".|
|BBClientVersion[]|Show the client version|
|BBServerVersion[]|Show the server version.|
|BBStatus[]|Show the status of the server.|
|BBLog[]|Show the log of the server.|
|BBViewBoard[]|List the bulletins.|
|BBGetInfo[title, tag(optional)]|See the details of the bulletin.|
|BBClearRevisions[title, tag(optional), revisions]|Clear the specified revisions.|
|BBRemove[title, tag(optional)]|Remove all revisions of the specified bulletin.|
|BBArchive[archiveName, title, tag(optinoal)]|Save the bulletin to an archive and make the data persistent.|
|BBLoad[archiveName]|Load the archived data. (The archive name is added to the tag)|
|BBListArchive[]|List the archives.|
|BBRenameArchive[archiveFrom, archiveTo]|Rename an archive. This is executed when `BBReset` is called.|
|BBDeleteArchive[archiveName]|Delete an archive. This is executed when `BBReset` is called.|
|BBDump[archiveName]|Save all the bulletins to an archive.|
|BBRestore[archiveName]|Reset the server and restore the archived data. (The data is restored to memory/file without modification of the tag)|
|BBClearLog[]|Clear the log of the server.|
|BBResetServer[]|Reset the BulletinBoard server.|
|BBTerminateServer[]|Terminate the BulletinBoard server.|
