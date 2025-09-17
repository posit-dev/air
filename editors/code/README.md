# Air extension for Visual Studio Code

A Visual Studio Code extension for [Air](https://github.com/posit-dev/air), an R formatter and language server, written in Rust.

Once installed, Air will automatically be activated when you open an R file. To configure your settings to allow Air to format R code on save, enable the `editor.formatOnSave` action in your `settings.json`.

``` json
{
    "[r]": {
        "editor.formatOnSave": true
    }
}
```

Click [here](https://posit-dev.github.io/air/editor-vscode.html) to learn about all of Air's features.

Click [here](https://posit-dev.github.io/air/configuration.html) to learn about how Air can be configured.
