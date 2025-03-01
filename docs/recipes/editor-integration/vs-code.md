# Integrating Mago with VS Code

The Mago formatter can be seamlessly integrated with Visual Studio Code (VS Code) to provide automatic code formatting within your editor.
This integration leverages the ["Custom Local Formatters"](https://marketplace.visualstudio.com/items?itemName=jkillian.custom-local-formatters) extension, which allows you to specify custom commands for formatting files.

Steps:

1. Install the [Custom Local Formatters](https://marketplace.visualstudio.com/items?itemName=jkillian.custom-local-formatters) extension:
   Open VS Code and navigate to the Extensions view (Ctrl+Shift+X).
   Search for "Custom Local Formatters" by jkillian.
   Click the "Install" button to install the extension.

2. Configure the extension:
   Open the VS Code settings (File > Preferences > Settings or Ctrl+,).
   Search for "Custom Local Formatters: Formatters".
   Click the "Edit in `settings.json`" link to open the `settings.json` file.

3. Add Mago as a formatter:
   In the `settings.json` file, add the following configuration within the `customLocalFormatters.formatters` array:
   ```json
   {
     "command": "mago fmt --stdin-input",
     "languages": ["php"]
   }
   ```
   Save the `settings.json` file.

_That's it!_ Now, whenever you save a PHP file in VS Code, the Custom Local Formatters extension will automatically
run the `mago fmt --stdin-input` command to format your code.

> Note: Make sure that the `mago` executable is in your system's `PATH` so that VS Code can find it.
