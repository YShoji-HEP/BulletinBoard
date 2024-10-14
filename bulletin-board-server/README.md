Bulletin Board Server
=====================
Object storage for [`ArrayObject`](https://github.com/YShoji-HEP/ArrayObject) for debugging and data taking purposes.

`BulletinBoard` is a part of [`dbgbb`](https://github.com/YShoji-HEP/dbgbb) project.

Highlights
----------
* Hybrid backend of memory and file, selected based on the size of the object and the allocated memory.
* Key is a combination of a name and a tag. Each key contains revisions of `ArrayObject`.
* The tag can be omitted if no other tags are present.
* If the revision is omitted, the most recent revision is returned.
* The data does not persist by default. Use `archive` or `dump` command to make it persistent.

Example
-------
Install and run the server with the specified listen address.
```bash
cargo install bulletin-board-server
export BB_LISTEN_ADDR = "0.0.0.0:7578"
bulletin-board-server
```

Environment Variables
---------------------
* BB_LISTEN_ADDR = "127.0.0.1:7578" or "/tmp/bb.sock"

Address of the bulletin board server. It is either [IP address]:[port] or [hostname]:[port]. When UNIX socket is used, the address should be the path to the uncreated socket.

* BB_TMP_DIR = "./bb_tmp"

Directory for temporary data.

* BB_ACV_DIR = "./bb_acv"

Directory for archives.

* BB_TOT_MEM_LIMIT = "1GiB"

Total memory limit. If the memory exceeds the limit, all the bulletins are saved as files.

* BB_FILE_THRETHOLD = "1MiB"

Beyond this threthold, the bulletin is saved as a file.

* BB_LOG_FILE = "./bulletin-board.log"

Location of the log file.

Crate Features
--------------
* `unix`

Use the UNIX socket instead of TCP. Only for UNIX-like OS.

Q&A
--------------
#### Why not persistent by default?
Since `BulletinBoard` was originally designed for debugging purposes, it is assumed that most of the data will be deleted at the end. Persistent options (`archive` and `dump`) have been added for a more extensive use such as data taking.
The advantages of not making it persistent by default are (i) holding data in memory makes read/write speeds faster, (ii) metadata of the archive becomes smaller and (iii) data can be more easily deleted before archiving.
#### Why not other object storages or databases?
Especially for debugging, storage may receive large amounts of small data and thus in-memory databases are ideal. However, it may also receive large data like a few hundred MiB, and such data should be stored in files. `BulletinBoard` uses a hybrid backend of memory and file to solve this problem. It also provides persistence options, which are useful for data taking.