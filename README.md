# openbook-desktop

This is a still very WIP desktop app for openbook. As the app itself is written using flutter, `openbook-desktop` uses the flutter engine library to run the app on desktop.

## Dependencies

* Flutter SDK
* Rust

## Running

For now, this probably won't run on any platform other than Linux. If you are on Linux, these steps should get it running:

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
