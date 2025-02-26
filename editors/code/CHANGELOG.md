# Changelog

- The CLI version and the extension versions are only kept in sync with their major component (the first number).

- Regarding minor versions (the second number), odd numbers indicate a pre-release of the extension. Even numbers are for public-facing releases.


## Development version


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
