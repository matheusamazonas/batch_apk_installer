# Batch APK Installer
A command-line tool for batch installation of Android Packages (APKs).

# Features
- Support for multiple device platforms based on the device's name and model.
- Multiple APK support based on package identifier.
- Conditional installation based on APK file name and device's platform.

# Compiling
Simply run `cargo build --release` and use the generated binary.

# Usage
Batch APK Installer is accessible via the command line with the following syntax:

```
<batch_apk_installer> <version> [options...]
```

Where:
  - `<batch_apk_installer>` is the name of the binary.
  - `<version>` is the version in the semantic versioning format (e.g., 2.1 and 4.1.2), 

And the following options are available:
- `-u`: whether the packages should be uninstalled from the devices before being installed. 
- `-h`: displays the help text.

For example:

```
bai 5.1 -u
```

On the command above, the Batch APK Installer binary is called `bai`. The command requests installation of version 5.1, preceded by uninstallation.

# Configuration and first run
Batch APK Installer loads installation requirements from a config file named `config.toml`, located at the user's configuration directory, dependent on the OS:
- macOS: `$HOME/Library/Application Support/Batch APK Installer`.
- Linux: `$XDG_CONFIG_HOME/Batch APK Installer` or `$HOME/.config/Batch APK Installer`.
- Windows: `{FOLDERID_RoamingAppData}/Batch APK Installer`.

The first time Batch APK Installer runs, no configuration file exists yet. In those cases, it will output a message about it and create a template config file, like the one below:

```toml
directory = "/Users/user_name/Desktop"  
platforms = [ "quest", "pico" ]  
  
[[packages]]  
id = "com.company.product.app"  
platforms = [ "pico", "quest" ]  
match_file_name = false  
  
[[packages]]  
id = "com.company.product.pico_only_app"  
platforms = [ "pico" ]  
match_file_name = true  
  
[[packages]]  
id = "com.company.product.quest_only_app"  
platforms = [ "pico" ]  
match_file_name = true
```

The file contains two parts: global and package configuration. 

## Global configuration
Global configuration contains two fields:
- `directory`: the folder where APKs should be placed, aggregated in subfolders named after different application versions. Taking the config file as an example, if we run `bai 5.1`, the tool will look for APKs inside `/Users/user_name/Desktop/5.1`.
- `platforms`: which platforms can be used with this config file. Platforms act like filters on devices, separating them into different buckets. The filtering is based off the device's name and model name, comparing it (case insensitive) to the platform name. Taking the config file above as an example, 
	- Devices named `Pico_Neo_3` and `PICOA7H10` will be filtered as `pico` platform.
	- Devices named `Quest_2` and `Quest_3S` will be filtered as `quest` platform.

## Package configuration
The package configuration follows the global configuration. Each package must be preceded with a `[[packages]]` tag and contain the following fields:
- `id`: the APK's application identifier.
- `platforms`: a list of platforms that the package should be installed on. Values in this list must match values in the global configuration's `platforms` field, otherwise they will be ignored. Taking the config file above as an example, `com.company.product.pico_only_app` will only be installed on Pico devices and `com.company.product.quest_only_app` on Quest devices.
- `match_file_name`: whether the APK's file name should match (by containing) the `platform` field in order to be installed. This flag is useful whenever you have multiple builds of the same application for different platforms. For example, if you have an app that targets both Pico and Quest devices, there might be two separate APKs, one for each platform. It's probably a good idea to avoid installing the Pico application on Quest devices and vice versa. To accomplish that, define the package twice in the config file, once for each platform, and both with `match_file_name` set to `true`.
  ```toml
  [[packages]]  
  id = "com.company.product.my_app"  
  platforms = [ "pico" ]  
  match_file_name = true

  [[packages]]  
  id = "com.company.product.my_app"  
  platforms = [ "quest" ]  
  match_file_name = true
  ```
  
  Finally, ensure that the APK files contain the name of their respective platform. For example:
  - `my_app_quest.apk`.
  - `my_app_pico.apk`.

## Example
Let's take a look at an example setup and use of Batch APK Installer. Take the following config file with 3 platforms and 2 packages:
```toml
directory = "/Users/matheus/Desktop"
platforms = [ "quest", "pico", "sm_g" ]

[[packages]]
id = "com.my_company.my_vr_game"
platforms = [ "pico", "quest" ]
match_file_name = false

[[packages]]
id = "com.lazysquirrellabs.airhockey"
platforms = [ "sm_g" ]
match_file_name = false
```
It contains the following packages:
- `com.my_company.my_vr_game`, which will be installed on both Pico and Quest devices, but only whenever the APK's file name matches the device's platform. This ensures that the Pico APK is only installed on Pico devices and that the Quest APK is only installed on Quest devices.
- `com.lazysquirrellabs.airhockey`, which will be installed on Samsung Galaxy devices (`sm_g`), regardless of the APK name.

With the following APKs in `/Users/matheus/Desktop/5.1`:
- `game_quest.apk`, Quest APK for `com.my_company.my_vr_game`.
- `game_pico.apk`, Pico APK for `com.my_company.my_vr_game`.
- `airhockey.apk`, APK for `com.lazysquirrellabs.airhockey`.

Finally, if the following devices are connected:
- Pico Neo 3.
- Quest 2.
- Samsung Galaxy S8.

An example run of `batch_apk_installer 5.1 -u` could output:

 ```
Found device: [quest] Quest 2 (1WMHHA63PR1501).
Found device: [pico] Pico Neo 3 (PA7L50MGF8290021W).
Found device: [sm_g] Galaxy S8 (ce031713396bc92803).
Running 3 installation(s) on 3 device(s)...
Uninstallation of com.my_company.my_vr_game on Pico Neo 3 completed successfully.
Uninstallation of com.my_company.my_vr_game on Quest 2 completed successfully.
Uninstallation of com.lazysquirrellabs.airhockey on Galaxy S8 completed successfully.
Installation of com.lazysquirrellabs.airhockey on Galaxy S8 completed successfully.
Installation of com.my_company.my_vr_game on Pico Neo 3 completed successfully.
Installation of com.my_company.my_vr_game on Quest 2 completed successfully.
 ```

# System requirements
Batch APK Installer supports macOS, Linux and Windows. The following command-line tools must be installed in order to successfully use the tool:
- [Android Debug Bridge](https://developer.android.com/tools/adb) (ADB).
- [Android Asset Packaging Tool](https://developer.android.com/tools/aapt2) (AAPT2).

# Dependencies
Batch APK Installer has the following dependencies:
- `dirs`: for OS-agnostic fetching of the user's config path.
- `futures`, `tokio` and `tokio-stream`: for asynchronous programming.
- `serde` and `toml`: for (de)serialization of config files.
- `regex`: for parsing version strings.

# License
Batch APK Installer is distributed under the terms of the MIT license. For more information, check the [LICENSE](LICENSE) file in this repository.

