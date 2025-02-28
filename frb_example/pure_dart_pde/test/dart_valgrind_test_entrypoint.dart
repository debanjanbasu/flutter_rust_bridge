/// NOTE: This file is auto-generated by frb_internal. Please do not manually modify it.

import 'dart:io';

import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test_core/src/direct_run.dart';
import 'package:test_core/src/runner/reporter/expanded.dart';
import 'package:test_core/src/util/print_sink.dart';

import 'api/array_test.dart' as array_test;
import 'api/async_misc_test.dart' as async_misc_test;
import 'api/async_spawn_test.dart' as async_spawn_test;
import 'api/attribute_test.dart' as attribute_test;
import 'api/chrono_type_test.dart' as chrono_type_test;
import 'api/comment_test.dart' as comment_test;
import 'api/constructor_test.dart' as constructor_test;
import 'api/customization_test.dart' as customization_test;
import 'api/dart_code_test.dart' as dart_code_test;
import 'api/dart_fn_test.dart' as dart_fn_test;
import 'api/dart_opaque_sync_test.dart' as dart_opaque_sync_test;
import 'api/dart_opaque_test.dart' as dart_opaque_test;
import 'api/enumeration_test.dart' as enumeration_test;
import 'api/event_listener_test.dart' as event_listener_test;
import 'api/exception_test.dart' as exception_test;
import 'api/external_impl_test.dart' as external_impl_test;
import 'api/external_type_in_crate_test.dart' as external_type_in_crate_test;
import 'api/inside_macro_test.dart' as inside_macro_test;
import 'api/map_and_set_test.dart' as map_and_set_test;
import 'api/method_test.dart' as method_test;
import 'api/mirror_test.dart' as mirror_test;
import 'api/misc_example_test.dart' as misc_example_test;
import 'api/misc_no_twin_example_a_test.dart' as misc_no_twin_example_a_test;
import 'api/misc_no_twin_example_b_test.dart' as misc_no_twin_example_b_test;
import 'api/misc_type_test.dart' as misc_type_test;
import 'api/newtype_pattern_test.dart' as newtype_pattern_test;
import 'api/optional_primitive_misc_test.dart' as optional_primitive_misc_test;
import 'api/optional_test.dart' as optional_test;
import 'api/primitive_list_misc_test.dart' as primitive_list_misc_test;
import 'api/primitive_misc_test.dart' as primitive_misc_test;
import 'api/pseudo_manual/array_twin_rust_async_test.dart'
    as array_twin_rust_async_test;
import 'api/pseudo_manual/array_twin_sync_test.dart' as array_twin_sync_test;
import 'api/pseudo_manual/attribute_twin_rust_async_test.dart'
    as attribute_twin_rust_async_test;
import 'api/pseudo_manual/attribute_twin_sync_test.dart'
    as attribute_twin_sync_test;
import 'api/pseudo_manual/basic_list_test.dart' as basic_list_test;
import 'api/pseudo_manual/basic_list_twin_rust_async_test.dart'
    as basic_list_twin_rust_async_test;
import 'api/pseudo_manual/basic_list_twin_sync_test.dart'
    as basic_list_twin_sync_test;
import 'api/pseudo_manual/basic_map_test.dart' as basic_map_test;
import 'api/pseudo_manual/basic_map_twin_rust_async_test.dart'
    as basic_map_twin_rust_async_test;
import 'api/pseudo_manual/basic_map_twin_sync_test.dart'
    as basic_map_twin_sync_test;
import 'api/pseudo_manual/basic_optional_test.dart' as basic_optional_test;
import 'api/pseudo_manual/basic_optional_twin_rust_async_test.dart'
    as basic_optional_twin_rust_async_test;
import 'api/pseudo_manual/basic_optional_twin_sync_test.dart'
    as basic_optional_twin_sync_test;
import 'api/pseudo_manual/basic_test.dart' as basic_test;
import 'api/pseudo_manual/basic_twin_rust_async_test.dart'
    as basic_twin_rust_async_test;
import 'api/pseudo_manual/basic_twin_sync_test.dart' as basic_twin_sync_test;
import 'api/pseudo_manual/chrono_type_twin_rust_async_test.dart'
    as chrono_type_twin_rust_async_test;
import 'api/pseudo_manual/chrono_type_twin_sync_test.dart'
    as chrono_type_twin_sync_test;
import 'api/pseudo_manual/comment_twin_rust_async_test.dart'
    as comment_twin_rust_async_test;
import 'api/pseudo_manual/comment_twin_sync_test.dart'
    as comment_twin_sync_test;
import 'api/pseudo_manual/dart_fn_twin_rust_async_test.dart'
    as dart_fn_twin_rust_async_test;
import 'api/pseudo_manual/dart_opaque_twin_rust_async_test.dart'
    as dart_opaque_twin_rust_async_test;
import 'api/pseudo_manual/dart_opaque_twin_sync_test.dart'
    as dart_opaque_twin_sync_test;
import 'api/pseudo_manual/enumeration_twin_rust_async_test.dart'
    as enumeration_twin_rust_async_test;
import 'api/pseudo_manual/enumeration_twin_sync_test.dart'
    as enumeration_twin_sync_test;
import 'api/pseudo_manual/event_listener_twin_rust_async_test.dart'
    as event_listener_twin_rust_async_test;
import 'api/pseudo_manual/exception_twin_rust_async_test.dart'
    as exception_twin_rust_async_test;
import 'api/pseudo_manual/exception_twin_sync_test.dart'
    as exception_twin_sync_test;
import 'api/pseudo_manual/external_type_in_crate_twin_rust_async_test.dart'
    as external_type_in_crate_twin_rust_async_test;
import 'api/pseudo_manual/external_type_in_crate_twin_sync_test.dart'
    as external_type_in_crate_twin_sync_test;
import 'api/pseudo_manual/map_and_set_twin_rust_async_test.dart'
    as map_and_set_twin_rust_async_test;
import 'api/pseudo_manual/map_and_set_twin_sync_test.dart'
    as map_and_set_twin_sync_test;
import 'api/pseudo_manual/method_twin_rust_async_test.dart'
    as method_twin_rust_async_test;
import 'api/pseudo_manual/method_twin_sync_test.dart' as method_twin_sync_test;
import 'api/pseudo_manual/mirror_twin_rust_async_test.dart'
    as mirror_twin_rust_async_test;
import 'api/pseudo_manual/mirror_twin_sync_test.dart' as mirror_twin_sync_test;
import 'api/pseudo_manual/misc_example_twin_rust_async_test.dart'
    as misc_example_twin_rust_async_test;
import 'api/pseudo_manual/misc_example_twin_sync_test.dart'
    as misc_example_twin_sync_test;
import 'api/pseudo_manual/misc_type_twin_rust_async_test.dart'
    as misc_type_twin_rust_async_test;
import 'api/pseudo_manual/misc_type_twin_sync_test.dart'
    as misc_type_twin_sync_test;
import 'api/pseudo_manual/newtype_pattern_twin_rust_async_test.dart'
    as newtype_pattern_twin_rust_async_test;
import 'api/pseudo_manual/newtype_pattern_twin_sync_test.dart'
    as newtype_pattern_twin_sync_test;
import 'api/pseudo_manual/optional_primitive_misc_twin_rust_async_test.dart'
    as optional_primitive_misc_twin_rust_async_test;
import 'api/pseudo_manual/optional_primitive_misc_twin_sync_test.dart'
    as optional_primitive_misc_twin_sync_test;
import 'api/pseudo_manual/optional_twin_rust_async_test.dart'
    as optional_twin_rust_async_test;
import 'api/pseudo_manual/optional_twin_sync_test.dart'
    as optional_twin_sync_test;
import 'api/pseudo_manual/primitive_list_misc_twin_rust_async_test.dart'
    as primitive_list_misc_twin_rust_async_test;
import 'api/pseudo_manual/primitive_list_misc_twin_sync_test.dart'
    as primitive_list_misc_twin_sync_test;
import 'api/pseudo_manual/primitive_misc_twin_rust_async_test.dart'
    as primitive_misc_twin_rust_async_test;
import 'api/pseudo_manual/primitive_misc_twin_sync_test.dart'
    as primitive_misc_twin_sync_test;
import 'api/pseudo_manual/raw_string_twin_rust_async_test.dart'
    as raw_string_twin_rust_async_test;
import 'api/pseudo_manual/raw_string_twin_sync_test.dart'
    as raw_string_twin_sync_test;
import 'api/pseudo_manual/rust_auto_opaque_twin_rust_async_test.dart'
    as rust_auto_opaque_twin_rust_async_test;
import 'api/pseudo_manual/rust_auto_opaque_twin_sync_test.dart'
    as rust_auto_opaque_twin_sync_test;
import 'api/pseudo_manual/rust_opaque_twin_rust_async_test.dart'
    as rust_opaque_twin_rust_async_test;
import 'api/pseudo_manual/rust_opaque_twin_sync_test.dart'
    as rust_opaque_twin_sync_test;
import 'api/pseudo_manual/simple_twin_rust_async_test.dart'
    as simple_twin_rust_async_test;
import 'api/pseudo_manual/simple_twin_sync_test.dart' as simple_twin_sync_test;
import 'api/pseudo_manual/stream_twin_rust_async_test.dart'
    as stream_twin_rust_async_test;
import 'api/pseudo_manual/structure_twin_rust_async_test.dart'
    as structure_twin_rust_async_test;
import 'api/pseudo_manual/structure_twin_sync_test.dart'
    as structure_twin_sync_test;
import 'api/pseudo_manual/tuple_twin_rust_async_test.dart'
    as tuple_twin_rust_async_test;
import 'api/pseudo_manual/tuple_twin_sync_test.dart' as tuple_twin_sync_test;
import 'api/pseudo_manual/type_alias_twin_rust_async_test.dart'
    as type_alias_twin_rust_async_test;
import 'api/pseudo_manual/type_alias_twin_sync_test.dart'
    as type_alias_twin_sync_test;
import 'api/pseudo_manual/uuid_type_twin_rust_async_test.dart'
    as uuid_type_twin_rust_async_test;
import 'api/pseudo_manual/uuid_type_twin_sync_test.dart'
    as uuid_type_twin_sync_test;
import 'api/raw_string_test.dart' as raw_string_test;
import 'api/rust_auto_opaque_test.dart' as rust_auto_opaque_test;
import 'api/rust_opaque_sync_test.dart' as rust_opaque_sync_test;
import 'api/rust_opaque_test.dart' as rust_opaque_test;
import 'api/simple_test.dart' as simple_test;
import 'api/stream_misc_test.dart' as stream_misc_test;
import 'api/stream_test.dart' as stream_test;
import 'api/structure_test.dart' as structure_test;
import 'api/tuple_test.dart' as tuple_test;
import 'api/type_alias_test.dart' as type_alias_test;
import 'api/uuid_type_test.dart' as uuid_type_test;

Future<void> main() async {
  await RustLib.init();

  final success = await directRunTests(
    () async => callFileEntrypoints(),
    reporterFactory: (engine) => ExpandedReporter.watch(
      engine,
      PrintSink(),
      color: true,
      printPlatform: false,
      printPath: false,
    ),
  );

  exit(success ? 0 : 1);
}

Future<void> callFileEntrypoints() async {
  await array_test.main(skipRustLibInit: true);
  await async_misc_test.main(skipRustLibInit: true);
  await async_spawn_test.main(skipRustLibInit: true);
  await attribute_test.main(skipRustLibInit: true);
  await chrono_type_test.main(skipRustLibInit: true);
  await comment_test.main(skipRustLibInit: true);
  await constructor_test.main(skipRustLibInit: true);
  await customization_test.main(skipRustLibInit: true);
  await dart_code_test.main(skipRustLibInit: true);
  await dart_fn_test.main(skipRustLibInit: true);
  await dart_opaque_sync_test.main(skipRustLibInit: true);
  await dart_opaque_test.main(skipRustLibInit: true);
  await enumeration_test.main(skipRustLibInit: true);
  await event_listener_test.main(skipRustLibInit: true);
  await exception_test.main(skipRustLibInit: true);
  await external_impl_test.main(skipRustLibInit: true);
  await external_type_in_crate_test.main(skipRustLibInit: true);
  await inside_macro_test.main(skipRustLibInit: true);
  await map_and_set_test.main(skipRustLibInit: true);
  await method_test.main(skipRustLibInit: true);
  await mirror_test.main(skipRustLibInit: true);
  await misc_example_test.main(skipRustLibInit: true);
  await misc_no_twin_example_a_test.main(skipRustLibInit: true);
  await misc_no_twin_example_b_test.main(skipRustLibInit: true);
  await misc_type_test.main(skipRustLibInit: true);
  await newtype_pattern_test.main(skipRustLibInit: true);
  await optional_primitive_misc_test.main(skipRustLibInit: true);
  await optional_test.main(skipRustLibInit: true);
  await primitive_list_misc_test.main(skipRustLibInit: true);
  await primitive_misc_test.main(skipRustLibInit: true);
  await array_twin_rust_async_test.main(skipRustLibInit: true);
  await array_twin_sync_test.main(skipRustLibInit: true);
  await attribute_twin_rust_async_test.main(skipRustLibInit: true);
  await attribute_twin_sync_test.main(skipRustLibInit: true);
  await basic_list_test.main(skipRustLibInit: true);
  await basic_list_twin_rust_async_test.main(skipRustLibInit: true);
  await basic_list_twin_sync_test.main(skipRustLibInit: true);
  await basic_map_test.main(skipRustLibInit: true);
  await basic_map_twin_rust_async_test.main(skipRustLibInit: true);
  await basic_map_twin_sync_test.main(skipRustLibInit: true);
  await basic_optional_test.main(skipRustLibInit: true);
  await basic_optional_twin_rust_async_test.main(skipRustLibInit: true);
  await basic_optional_twin_sync_test.main(skipRustLibInit: true);
  await basic_test.main(skipRustLibInit: true);
  await basic_twin_rust_async_test.main(skipRustLibInit: true);
  await basic_twin_sync_test.main(skipRustLibInit: true);
  await chrono_type_twin_rust_async_test.main(skipRustLibInit: true);
  await chrono_type_twin_sync_test.main(skipRustLibInit: true);
  await comment_twin_rust_async_test.main(skipRustLibInit: true);
  await comment_twin_sync_test.main(skipRustLibInit: true);
  await dart_fn_twin_rust_async_test.main(skipRustLibInit: true);
  await dart_opaque_twin_rust_async_test.main(skipRustLibInit: true);
  await dart_opaque_twin_sync_test.main(skipRustLibInit: true);
  await enumeration_twin_rust_async_test.main(skipRustLibInit: true);
  await enumeration_twin_sync_test.main(skipRustLibInit: true);
  await event_listener_twin_rust_async_test.main(skipRustLibInit: true);
  await exception_twin_rust_async_test.main(skipRustLibInit: true);
  await exception_twin_sync_test.main(skipRustLibInit: true);
  await external_type_in_crate_twin_rust_async_test.main(skipRustLibInit: true);
  await external_type_in_crate_twin_sync_test.main(skipRustLibInit: true);
  await map_and_set_twin_rust_async_test.main(skipRustLibInit: true);
  await map_and_set_twin_sync_test.main(skipRustLibInit: true);
  await method_twin_rust_async_test.main(skipRustLibInit: true);
  await method_twin_sync_test.main(skipRustLibInit: true);
  await mirror_twin_rust_async_test.main(skipRustLibInit: true);
  await mirror_twin_sync_test.main(skipRustLibInit: true);
  await misc_example_twin_rust_async_test.main(skipRustLibInit: true);
  await misc_example_twin_sync_test.main(skipRustLibInit: true);
  await misc_type_twin_rust_async_test.main(skipRustLibInit: true);
  await misc_type_twin_sync_test.main(skipRustLibInit: true);
  await newtype_pattern_twin_rust_async_test.main(skipRustLibInit: true);
  await newtype_pattern_twin_sync_test.main(skipRustLibInit: true);
  await optional_primitive_misc_twin_rust_async_test.main(
      skipRustLibInit: true);
  await optional_primitive_misc_twin_sync_test.main(skipRustLibInit: true);
  await optional_twin_rust_async_test.main(skipRustLibInit: true);
  await optional_twin_sync_test.main(skipRustLibInit: true);
  await primitive_list_misc_twin_rust_async_test.main(skipRustLibInit: true);
  await primitive_list_misc_twin_sync_test.main(skipRustLibInit: true);
  await primitive_misc_twin_rust_async_test.main(skipRustLibInit: true);
  await primitive_misc_twin_sync_test.main(skipRustLibInit: true);
  await raw_string_twin_rust_async_test.main(skipRustLibInit: true);
  await raw_string_twin_sync_test.main(skipRustLibInit: true);
  await rust_auto_opaque_twin_rust_async_test.main(skipRustLibInit: true);
  await rust_auto_opaque_twin_sync_test.main(skipRustLibInit: true);
  await rust_opaque_twin_rust_async_test.main(skipRustLibInit: true);
  await rust_opaque_twin_sync_test.main(skipRustLibInit: true);
  await simple_twin_rust_async_test.main(skipRustLibInit: true);
  await simple_twin_sync_test.main(skipRustLibInit: true);
  await stream_twin_rust_async_test.main(skipRustLibInit: true);
  await structure_twin_rust_async_test.main(skipRustLibInit: true);
  await structure_twin_sync_test.main(skipRustLibInit: true);
  await tuple_twin_rust_async_test.main(skipRustLibInit: true);
  await tuple_twin_sync_test.main(skipRustLibInit: true);
  await type_alias_twin_rust_async_test.main(skipRustLibInit: true);
  await type_alias_twin_sync_test.main(skipRustLibInit: true);
  await uuid_type_twin_rust_async_test.main(skipRustLibInit: true);
  await uuid_type_twin_sync_test.main(skipRustLibInit: true);
  await raw_string_test.main(skipRustLibInit: true);
  await rust_auto_opaque_test.main(skipRustLibInit: true);
  await rust_opaque_sync_test.main(skipRustLibInit: true);
  await rust_opaque_test.main(skipRustLibInit: true);
  await simple_test.main(skipRustLibInit: true);
  await stream_misc_test.main(skipRustLibInit: true);
  await stream_test.main(skipRustLibInit: true);
  await structure_test.main(skipRustLibInit: true);
  await tuple_test.main(skipRustLibInit: true);
  await type_alias_test.main(skipRustLibInit: true);
  await uuid_type_test.main(skipRustLibInit: true);
}
