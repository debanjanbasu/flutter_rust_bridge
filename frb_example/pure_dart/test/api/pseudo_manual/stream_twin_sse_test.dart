// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `stream_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"forbiddenDuplicatorModes": ["sync", "sync sse"]}

import 'dart:async';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/stream_twin_sse.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

import '../../test_utils.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart call funcStreamSinkArgPositionTwinSse', () async {
    // We only care about whether the codegen can understand StreamSink
    // as non-first argument in Rust, thus we do not test the return values.
    // ignore: unawaited_futures
    funcStreamSinkArgPositionTwinSse(a: 100, b: 200);
  });

  test('call funcStreamReturnErrorTwinSse', () async {
    await expectLater(
      () async {
        await for (final _ in await funcStreamReturnErrorTwinSse()) {}
      },
      throwsA(isA<AnyhowException>()
          .having((x) => x.message, 'message', startsWith('deliberate error'))),
    );
  });

  // TODO implement in web
  test('call funcStreamReturnPanicTwinSse', skip: kIsWeb, () async {
    await expectRustPanic(
      () async {
        await for (final _ in await funcStreamReturnPanicTwinSse()) {}
      },
      'TwinSse',
      messageOnNative: 'deliberate panic',
    );
  });

  Future<void> testHandleStream(
      FutureOr<Stream<LogTwinSse>> Function(
              {dynamic hint, required int key, required int max})
          handleStreamFunction) async {
    final max = 5;
    final key = 8;
    final stream = await handleStreamFunction(key: key, max: max);
    var cnt = 0;
    await for (final value in stream) {
      print("output from handle_stream_x's stream: $value");
      expect(value.key, key);
      cnt++;
    }
    expect(cnt, max);
  }

  test('dart call handle_stream_sink_at_1', () {
    testHandleStream(handleStreamSinkAt1TwinSse);
  });

  test('dart call handle_stream_sink_at_2', () {
    testHandleStream(handleStreamSinkAt2TwinSse);
  });

  test('dart call handle_stream_sink_at_3', () {
    testHandleStream(handleStreamSinkAt3TwinSse);
  });

  test('stream_sink_fixed_sized_primitive_array_twin_normal', () async {
    final output =
        await (await streamSinkFixedSizedPrimitiveArrayTwinSse()).toList();
    expect(output, [
      orderedEquals([1, 2]),
      orderedEquals([3, 4]),
    ]);
  });

  test('stream_sink_inside_vec_twin_normal', () async {
    final sinks = [RustStreamSink<int>(), RustStreamSink<int>()];
    await streamSinkInsideVecTwinSse(arg: sinks);
    expect(await sinks[0].stream.toList(), [100, 200]);
    expect(await sinks[1].stream.toList(), [100, 200]);
  });

  test('stream_sink_inside_struct_twin_normal', () async {
    final arg =
        MyStructContainingStreamSinkTwinSse(a: 1000, b: RustStreamSink<int>());
    await streamSinkInsideStructTwinSse(arg: arg);
    expect(await arg.b.stream.toList(), [1000]);
  });
}
