// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, prefer_single_quotes

import 'dart:convert';
import 'dart:typed_data';

import 'dart:convert';
import 'dart:typed_data';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'dart:ffi' as ffi;

abstract class MirrorXCore {
  Future<bool> initSdk({required String configDbPath, dynamic hint});

  Future<String?> readDeviceId({dynamic hint});

  Future<String?> readDevicePassword({dynamic hint});

  Future<void> saveDevicePassword(
      {required String devicePassword, dynamic hint});

  Future<String> generateRandomDevicePassword({dynamic hint});

  Future<void> deviceGoesOnline({dynamic hint});

  Future<bool> desktopConnectTo({required String askDeviceId, dynamic hint});
}

class MirrorXCoreImpl extends FlutterRustBridgeBase<MirrorXCoreWire>
    implements MirrorXCore {
  factory MirrorXCoreImpl(ffi.DynamicLibrary dylib) =>
      MirrorXCoreImpl.raw(MirrorXCoreWire(dylib));

  MirrorXCoreImpl.raw(MirrorXCoreWire inner) : super(inner);

  Future<bool> initSdk({required String configDbPath, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_init_sdk(port_, _api2wire_String(configDbPath)),
        parseSuccessData: _wire2api_bool,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "init_sdk",
          argNames: ["configDbPath"],
        ),
        argValues: [configDbPath],
        hint: hint,
      ));

  Future<String?> readDeviceId({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_read_device_id(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "read_device_id",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<String?> readDevicePassword({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_read_device_password(port_),
        parseSuccessData: _wire2api_opt_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "read_device_password",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> saveDevicePassword(
          {required String devicePassword, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_save_device_password(
            port_, _api2wire_String(devicePassword)),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "save_device_password",
          argNames: ["devicePassword"],
        ),
        argValues: [devicePassword],
        hint: hint,
      ));

  Future<String> generateRandomDevicePassword({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_generate_random_device_password(port_),
        parseSuccessData: _wire2api_String,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "generate_random_device_password",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<void> deviceGoesOnline({dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) => inner.wire_device_goes_online(port_),
        parseSuccessData: _wire2api_unit,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "device_goes_online",
          argNames: [],
        ),
        argValues: [],
        hint: hint,
      ));

  Future<bool> desktopConnectTo({required String askDeviceId, dynamic hint}) =>
      executeNormal(FlutterRustBridgeTask(
        callFfi: (port_) =>
            inner.wire_desktop_connect_to(port_, _api2wire_String(askDeviceId)),
        parseSuccessData: _wire2api_bool,
        constMeta: const FlutterRustBridgeTaskConstMeta(
          debugName: "desktop_connect_to",
          argNames: ["askDeviceId"],
        ),
        argValues: [askDeviceId],
        hint: hint,
      ));

  // Section: api2wire
  ffi.Pointer<wire_uint_8_list> _api2wire_String(String raw) {
    return _api2wire_uint_8_list(utf8.encoder.convert(raw));
  }

  int _api2wire_u8(int raw) {
    return raw;
  }

  ffi.Pointer<wire_uint_8_list> _api2wire_uint_8_list(Uint8List raw) {
    final ans = inner.new_uint_8_list(raw.length);
    ans.ref.ptr.asTypedList(raw.length).setAll(0, raw);
    return ans;
  }

  // Section: api_fill_to_wire

}

// Section: wire2api
String _wire2api_String(dynamic raw) {
  return raw as String;
}

bool _wire2api_bool(dynamic raw) {
  return raw as bool;
}

String? _wire2api_opt_String(dynamic raw) {
  return raw == null ? null : _wire2api_String(raw);
}

int _wire2api_u8(dynamic raw) {
  return raw as int;
}

Uint8List _wire2api_uint_8_list(dynamic raw) {
  return raw as Uint8List;
}

void _wire2api_unit(dynamic raw) {
  return;
}

// ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides, constant_identifier_names

// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.

/// generated by flutter_rust_bridge
class MirrorXCoreWire implements FlutterRustBridgeWireBase {
  /// Holds the symbol lookup function.
  final ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
      _lookup;

  /// The symbols are looked up in [dynamicLibrary].
  MirrorXCoreWire(ffi.DynamicLibrary dynamicLibrary)
      : _lookup = dynamicLibrary.lookup;

  /// The symbols are looked up with [lookup].
  MirrorXCoreWire.fromLookup(
      ffi.Pointer<T> Function<T extends ffi.NativeType>(String symbolName)
          lookup)
      : _lookup = lookup;

  void wire_init_sdk(
    int port_,
    ffi.Pointer<wire_uint_8_list> config_db_path,
  ) {
    return _wire_init_sdk(
      port_,
      config_db_path,
    );
  }

  late final _wire_init_sdkPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(
              ffi.Int64, ffi.Pointer<wire_uint_8_list>)>>('wire_init_sdk');
  late final _wire_init_sdk = _wire_init_sdkPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_read_device_id(
    int port_,
  ) {
    return _wire_read_device_id(
      port_,
    );
  }

  late final _wire_read_device_idPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_read_device_id');
  late final _wire_read_device_id =
      _wire_read_device_idPtr.asFunction<void Function(int)>();

  void wire_read_device_password(
    int port_,
  ) {
    return _wire_read_device_password(
      port_,
    );
  }

  late final _wire_read_device_passwordPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_read_device_password');
  late final _wire_read_device_password =
      _wire_read_device_passwordPtr.asFunction<void Function(int)>();

  void wire_save_device_password(
    int port_,
    ffi.Pointer<wire_uint_8_list> device_password,
  ) {
    return _wire_save_device_password(
      port_,
      device_password,
    );
  }

  late final _wire_save_device_passwordPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_save_device_password');
  late final _wire_save_device_password = _wire_save_device_passwordPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  void wire_generate_random_device_password(
    int port_,
  ) {
    return _wire_generate_random_device_password(
      port_,
    );
  }

  late final _wire_generate_random_device_passwordPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_generate_random_device_password');
  late final _wire_generate_random_device_password =
      _wire_generate_random_device_passwordPtr.asFunction<void Function(int)>();

  void wire_device_goes_online(
    int port_,
  ) {
    return _wire_device_goes_online(
      port_,
    );
  }

  late final _wire_device_goes_onlinePtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Int64)>>(
          'wire_device_goes_online');
  late final _wire_device_goes_online =
      _wire_device_goes_onlinePtr.asFunction<void Function(int)>();

  void wire_desktop_connect_to(
    int port_,
    ffi.Pointer<wire_uint_8_list> ask_device_id,
  ) {
    return _wire_desktop_connect_to(
      port_,
      ask_device_id,
    );
  }

  late final _wire_desktop_connect_toPtr = _lookup<
      ffi.NativeFunction<
          ffi.Void Function(ffi.Int64,
              ffi.Pointer<wire_uint_8_list>)>>('wire_desktop_connect_to');
  late final _wire_desktop_connect_to = _wire_desktop_connect_toPtr
      .asFunction<void Function(int, ffi.Pointer<wire_uint_8_list>)>();

  ffi.Pointer<wire_uint_8_list> new_uint_8_list(
    int len,
  ) {
    return _new_uint_8_list(
      len,
    );
  }

  late final _new_uint_8_listPtr = _lookup<
      ffi.NativeFunction<
          ffi.Pointer<wire_uint_8_list> Function(
              ffi.Int32)>>('new_uint_8_list');
  late final _new_uint_8_list = _new_uint_8_listPtr
      .asFunction<ffi.Pointer<wire_uint_8_list> Function(int)>();

  void free_WireSyncReturnStruct(
    WireSyncReturnStruct val,
  ) {
    return _free_WireSyncReturnStruct(
      val,
    );
  }

  late final _free_WireSyncReturnStructPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(WireSyncReturnStruct)>>(
          'free_WireSyncReturnStruct');
  late final _free_WireSyncReturnStruct = _free_WireSyncReturnStructPtr
      .asFunction<void Function(WireSyncReturnStruct)>();

  void store_dart_post_cobject(
    DartPostCObjectFnType ptr,
  ) {
    return _store_dart_post_cobject(
      ptr,
    );
  }

  late final _store_dart_post_cobjectPtr =
      _lookup<ffi.NativeFunction<ffi.Void Function(DartPostCObjectFnType)>>(
          'store_dart_post_cobject');
  late final _store_dart_post_cobject = _store_dart_post_cobjectPtr
      .asFunction<void Function(DartPostCObjectFnType)>();
}

class wire_uint_8_list extends ffi.Struct {
  external ffi.Pointer<ffi.Uint8> ptr;

  @ffi.Int32()
  external int len;
}

typedef DartPostCObjectFnType = ffi.Pointer<
    ffi.NativeFunction<ffi.Uint8 Function(DartPort, ffi.Pointer<ffi.Void>)>>;
typedef DartPort = ffi.Int64;
