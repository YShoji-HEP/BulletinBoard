# Common source for Bulletin Board

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![Crates.io](https://img.shields.io/crates/v/bulletin-board-common?style=flat-square)](https://crates.io/crates/bulletin-board-common)
[![Crates.io](https://img.shields.io/crates/d/bulletin-board-common?style=flat-square)](https://crates.io/crates/bulletin-board-common)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

`BulletinBoard` is an object strage for `ArrayObject` for debugging and data taking purposes.
See [`BulletinBoard`](https://github.com/YShoji-HEP/BulletinBoard) for details.
This crate includes a common source for `bulletin-board-server` and `bulletin-board-client`.

## Caution

* Clients do not check whether the operation is successful or not to improve performance. Check the log of the server for the errors.
* The data is not encrypted. Please do not send any confidential data over the network.
* This crate is under development and is subject to change in specification. (Compatibility across `BulletinBoard` and `dbgbb` is ensured for the most minor version numbers.)