# Bulletin Board GUI

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/YShojiHEP)

[!["Github Sponsors"](https://img.shields.io/badge/GitHub-Sponsors-red?style=flat-square)](https://github.com/sponsors/YShoji-HEP)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue?style=flat-square)](https://github.com/YShoji-HEP/BulletinBoard/blob/main/LICENSE.txt)

GUI application for `BulletinBoard`. 

<img src="snapshot.png" width="500">

## Highlights
* Includes a built-in server.
* Copy the title and the tag by clicking the panel.
* Various operations of the server via GUI.

## Compilation (Rinf)

The pre-compiled binaries are available in [Release](https://github.com/YShoji-HEP/BulletinBoard/releases). If you want to build them yourself, follow the instruction below.

This project leverages Flutter for GUI and Rust for the backend logic,
utilizing the capabilities of the
[Rinf](https://pub.dev/packages/rinf) framework.

To run and build this app, you need to have
[Flutter SDK](https://docs.flutter.dev/get-started/install)
and [Rust toolchain](https://www.rust-lang.org/tools/install)
installed on your system.
You can check that your system is ready with the commands below.
Note that all the Flutter subcomponents should be installed.

```bash
rustc --version
flutter doctor
```

You also need to have the CLI tool for Rinf ready.

```bash
cargo install rinf
```

Messages sent between Dart and Rust are implemented using Protobuf.
If you have newly cloned the project repository
or made changes to the `.proto` files in the `./messages` directory,
run the following command:

```bash
rinf message
```

Now you can run and build this app just like any other Flutter projects.

```bash
flutter run
```

For detailed instructions on writing Rust and Flutter together,
please refer to Rinf's [documentation](https://rinf.cunarist.com).

