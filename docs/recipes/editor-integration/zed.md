# Integrating Mago with Zed

Zed is a powerful code editor that supports external formatters for various languages.
You can easily configure Mago as the formatter for PHP files within Zed's settings.

Steps:

1. Open Zed settings:
   Open Zed and navigate to the settings (Zed > Preferences or Ctrl+,).

2. Configure the PHP formatter:
   In the settings, locate the languages entry. If it's not there, add it.
   Within the languages object, add the following configuration for PHP:
   ```json
   {
     // ...
     "languages": {
         "PHP": {
             "language_servers":,
             "formatter": {
                 "external": {
                     "command": "mago",
                     "arguments": ["fmt", "--stdin-input"]
                 }
             }
         }
     }
   }
   ```

_That's it!_ Zed will now use Mago to format your PHP files whenever you trigger the formatting action (usually by pressing Ctrl+Shift+F or through the command palette).

> Note: Make sure that the `mago` executable is in your system's `PATH` so that Zed can find it.
