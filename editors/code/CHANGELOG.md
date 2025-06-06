# Changelog

- The CLI version and the extension versions are only kept in sync with their major component (the first number).

- Regarding minor versions (the second number), odd numbers indicate a pre-release of the extension. Even numbers are for public-facing releases.


## Development version


## 0.14.0

- [Air 0.7.0](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.


## 0.12.0

- [Air 0.6.0](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.

- Fixed an issue that could cause Positron's Test Explorer to infinitely loop through opening and closing the same document (#320).

- New `Air: Format Workspace Folder` command to format an entire project (similar to running `air format {folder}` at the command line). Combined with `usethis::use_air()`, this is the easiest way to transition an existing project to use Air (#312)!


## 0.10.0

- [Air 0.5.0](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.

- The extension now activates automatically when an `air.toml`, `DESCRIPTION`, or `.Rproj` file is detected at the workspace root. Previously, the extension only activated after an R file was opened or when an R file was detected at the workspace root level (but not recursively within any subfolder) (#285).


## 0.8.0

- [Air 0.4.1](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.


## 0.6.0

- [Air 0.4.0](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.

- New `air.executablePath` configuration option for specifying a fixed path to an air executable. You must also set `air.executableStrategy` to `"path"` for this to have any affect. This is mostly useful for debug builds of air (#243).

- `air.executableLocation` has been renamed to `air.executableStrategy`.


## 0.4.0

- [Air 0.3.0](https://github.com/posit-dev/air/blob/main/CHANGELOG.md) is now bundled with the extension.

- The extension is now available on Linux (#71).

- The extension is now available on ARM Windows (#170).

- The extension now works properly for Intel macOS (#194).


## 0.2.0

- Initial release
