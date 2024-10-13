Mathematica client for BulletinBoard
====================================
For more details, see [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard).

Example
-------
To compile, run
```bash
cargo build -r
```
If it does not compile, see [`wolfram-library-link`](https://crates.io/crates/wolfram-library-link). Notice that `Mathematica` or `Wolfram Engine` has to be installed before the compilation.
Then, copy `target/release/libbulletin_board_mathematica.dylib` to the same directory as `bulletin-board.wl`.

To post and read the bulletins, 
```
<< "bulletin-board.wl";
BBLoadFunctions["127.0.0.1:7578"];
BBPost["test",{1,2,3}];
BBRead["test"]
```

Functions
----------
* BBLoadFunctions[address]
Load functions of BulletinBoard client. The address is either "ADDRESS:PORT" or "SOCKETPATH". This has to be executed first.
* BBPost[varName, varTag(optional), data]
Post the data to the server.
* BBRead[varName, varTag(optional), revisions(optional)]
Read the bulletin.
* BBStatus[]
Show the status of the server.
* BBLog[]
Show the log of the server.
* BBViewBoard[]
List the bulletins.
* BBGetInfo[varName, varTag(optional)]
See the details of the bulletin.
* BBClearRevisions[varName, varTag, revisions]
Clear the specified revisions.
* BBRemove[varName, varTag]
Remove all revisions of the specified bulletin.
* BBArchive[varName, varTag, archive]
Save the bulletin to an archive and make the data persistent.
* BBLoad[archiveName]
Load the archived data. (The archive name is added to the tag)
* BBListArchive[]
List the archives.
* BBRenameArchive[archiveFrom, archiveTo]
Rename an archive.
* BBDeleteArchive[archiveName]
Delete an archive.
* BBDump[archiveName]
Save all the bulletins to an archive.
* BBRestore[archiveName]
Restore the archived data. (The data is restored to memory/file without modification of the tag)
* BBReset[]
Reset the BulletinBoard server.

Crate Features
--------------
* `unix`
Use the UNIX socket instead of TCP. Only for UNIX-like OS.
