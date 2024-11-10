Bulletin Board Server
=====================
[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-server?style=flat-square)](https://crates.io/crates/bulletin-board-server)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-server?style=flat-square)](https://crates.io/crates/bulletin-board-server)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

Object storage for [`ArrayObject`](https://github.com/YShoji-HEP/ArrayObject) for debugging and data taking purposes. See [GitHub page](https://github.com/YShoji-HEP/BulletinBoard).

## Highlights
* Hybrid backend of memory and file, selected based on the size of the object and the allocated memory.
* Key is a combination of a title and a tag. Each key contains revisions of `ArrayObject`.
* Simple access to data. For example, revision can be omitted. Then, the most recent revision is returned. The tag can also be omitted if no other tags are present.
* The commands `archive` and `dump` make data persistent. (Data does not persist by default.)
* Docker image of the server is available.
* Unix sockets can be used with Unix-like operating systems, which makes the communication speed quite fast.

## Docker
```
docker run -p 7578:7578 -v /path/to/vol:/data yshojihep/bulletin-board
```
Or use `docker-compose.yml`
```
version: "3"
services:
  vscode:
    ports:
      - "7578:7578"
    container_name: bulletin-board
    restart: always
    image: yshojihep/bulletin-board:latest
    volumes:
      - /path/to/vol:/data
```

## Kubernetes
```
---
apiVersion: v1
kind: Pod
metadata:
  name: bulletin-board
spec:
  containers:
    - name: app
      image: yshojihep/bulletin-board:latest
      ports:
        - containerPort: 7578
      volumeMounts:
        - name: bulletin-board-data
          mountPath: /data
```