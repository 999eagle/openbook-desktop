# openbook-desktop

This is a still very WIP desktop app for openbook. As the app itself is written using flutter, `openbook-desktop` uses the flutter engine library to run the app on desktop.

## Known issues

These issues are specific to the desktop version and not necessarily to the app itself. If you find more issues specific to this desktop version, please report them on [GitLab](https://gitlab.com/999eagle/openbook-desktop).
 
* Scrolling doesn't work with the scroll wheel yet, you have to click and drag like a touchscreen
* Window size is reset on startup
* List icons don't load in timeline filter

## Dependencies

* [Flutter SDK](https://flutter.dev/docs/development/tools/sdk/releases)
* [Rust](https://www.rust-lang.org/tools/install)

## Running

I'm developing on and primarily for Linux, but running on Windows and MacOS probably works as well.

* Make sure you have checked out `openbook-app` as submodule
* Add these lines in `openbook-app/lib/main.dart`:

	```dart
	// top of the file
	import 'package:flutter/foundation.dart'
        show debugDefaultTargetPlatformOverride;
	    
	// in void main()
    debugDefaultTargetPlatformOverride = TargetPlatform.android;
	```

* Create the `openbook-app/.env.json` file according to the README of the original repository
* Run `flutter build bundle` inside the `openbook-app` directory
* Run `cargo run`

## Building

The `build-all.sh` script builds the entire app for Linux and Windows in release mode. Make sure you've edited `openbook-app/lib/main.dart` as specified in Running before executing the script.
