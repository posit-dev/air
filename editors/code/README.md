# Air extension for Visual Studio Code

A Visual Studio Code extension for [Air](https://github.com/posit-dev/air), an R formatter and language server, written in Rust.

> Air is currently in beta. Expect breaking changes both in the API and in formatting results. We also recommend that you use a version control system like git so you can easily see the changes that Air makes.

Once installed, Air will automatically be activated when you open an R file. To configure your settings to allow Air to format R code on save, enable the `editor.formatOnSave` action in your `settings.json`.

```json
{
    "[r]": {
        "editor.formatOnSave": true
    }
}
```

Click [here](https://posit-dev.github.io/air/editor-vscode.html) to learn about all of Air's features.

Click [here](https://posit-dev.github.io/air/configuration.html) to learn about how Air can be configured.
