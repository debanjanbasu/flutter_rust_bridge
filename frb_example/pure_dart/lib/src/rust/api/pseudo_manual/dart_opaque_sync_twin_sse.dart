// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.31.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Object syncLoopbackTwinSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.syncLoopbackTwinSse(opaque: opaque, hint: hint);

Object? syncOptionLoopbackTwinSse({Object? opaque, dynamic hint}) =>
    RustLib.instance.api.syncOptionLoopbackTwinSse(opaque: opaque, hint: hint);

String syncAcceptDartOpaqueTwinSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .syncAcceptDartOpaqueTwinSse(opaque: opaque, hint: hint);

/// [DartWrapObject] can be safely retrieved on a dart thread.
String unwrapDartOpaqueTwinSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.unwrapDartOpaqueTwinSse(opaque: opaque, hint: hint);

Object? syncOptionDartOpaqueTwinSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .syncOptionDartOpaqueTwinSse(opaque: opaque, hint: hint);
