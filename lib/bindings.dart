// AUTO GENERATED FILE, DO NOT EDIT.
//
// Generated by `package:ffigen`.
import 'dart:ffi' as ffi;

/// Dart bindings to call mylib functions
class LGJRustBindings {
  /// Holds the Dynamic library.
  final ffi.DynamicLibrary _dylib;

  /// The symbols are looked up in [dynamicLibrary].
  LGJRustBindings(ffi.DynamicLibrary dynamicLibrary) : _dylib = dynamicLibrary;

  /// network module for `lgj.wlwsx.client.flutter`
  ffi.Pointer<ffi.Int8> rust_greeting(
    ffi.Pointer<ffi.Int8> to,
  ) {
    _rust_greeting ??= _dylib
        .lookupFunction<_c_rust_greeting, _dart_rust_greeting>('rust_greeting');
    return _rust_greeting(
      to,
    );
  }

  _dart_rust_greeting _rust_greeting;

  void rust_cstr_free(
    ffi.Pointer<ffi.Int8> s,
  ) {
    _rust_cstr_free ??=
        _dylib.lookupFunction<_c_rust_cstr_free, _dart_rust_cstr_free>(
            'rust_cstr_free');
    return _rust_cstr_free(
      s,
    );
  }

  _dart_rust_cstr_free _rust_cstr_free;

  ffi.Pointer<ffi.Int8> net_test() {
    _net_test ??=
        _dylib.lookupFunction<_c_net_test, _dart_net_test>('net_test');
    return _net_test();
  }

  _dart_net_test _net_test;
}

typedef _c_rust_greeting = ffi.Pointer<ffi.Int8> Function(
  ffi.Pointer<ffi.Int8> to,
);

typedef _dart_rust_greeting = ffi.Pointer<ffi.Int8> Function(
  ffi.Pointer<ffi.Int8> to,
);

typedef _c_rust_cstr_free = ffi.Void Function(
  ffi.Pointer<ffi.Int8> s,
);


typedef _dart_rust_cstr_free = void Function(
  ffi.Pointer<ffi.Int8> s,
);

typedef _c_net_test = ffi.Pointer<ffi.Int8> Function();

typedef _dart_net_test = ffi.Pointer<ffi.Int8> Function();
