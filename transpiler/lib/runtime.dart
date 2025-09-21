/// PohLang runtime helpers (v0.1)
/// Lightweight utilities used by transpiled Dart code.

import 'dart:io';
import 'dart:math';
import 'package:path/path.dart' as p;

class PohRuntime {
  static String? inputText(String name) {
    stdout.write('Enter $name: ');
    return stdin.readLineSync();
  }

  static int? inputInt(String name) {
    final s = inputText(name);
    if (s == null) return null;
    return int.tryParse(s.trim());
  }

  static double? inputDouble(String name) {
    final s = inputText(name);
    if (s == null) return null;
    return double.tryParse(s.trim());
  }

  static num randomBetween(num a, num b) {
    // Inclusive for ints, approximate for doubles
    if (a is int && b is int) {
      final min = a <= b ? a : b;
      final max = a <= b ? b : a;
      return min + Random().nextInt(max - min + 1);
    }
    final min = a <= b ? a : b;
    final max = a <= b ? b : a;
    return min + Random().nextDouble() * (max - min);
  }

  // ---- Files ----
  static String readFile(String path) {
    try {
      return File(path).readAsStringSync();
    } catch (e) {
      stderr.writeln('readFile error: $e');
      return '';
    }
  }

  static bool writeFile(String path, String content, {bool append = false}) {
    try {
      final file = File(path);
      if (append && file.existsSync()) {
        file.writeAsStringSync(content, mode: FileMode.append);
      } else if (append) {
        file.createSync(recursive: true);
        file.writeAsStringSync(content, mode: FileMode.append);
      } else {
        file.createSync(recursive: true);
        file.writeAsStringSync(content);
      }
      return true;
    } catch (e) {
      stderr.writeln('writeFile error: $e');
      return false;
    }
  }

  static bool deleteFile(String path) {
    try {
      final f = File(path);
      if (f.existsSync()) {
        f.deleteSync();
        return true;
      }
      return false;
    } catch (e) {
      stderr.writeln('deleteFile error: $e');
      return false;
    }
  }

  // ---- Directories ----
  static List<String> listFiles(String dirPath) {
    try {
      final dir = Directory(dirPath);
      if (!dir.existsSync()) return <String>[];
      return dir
          .listSync(followLinks: false)
          .whereType<File>()
          .map((f) => p.basename(f.path))
          .toList();
    } catch (e) {
      stderr.writeln('listFiles error: $e');
      return <String>[];
    }
  }

  static bool changeDirectory(String path) {
    try {
      final dir = Directory(path);
      if (!dir.existsSync()) return false;
      Directory.current = dir.path;
      return true;
    } catch (e) {
      stderr.writeln('changeDirectory error: $e');
      return false;
    }
  }

  static bool createDirectory(String path) {
    try {
      Directory(path).createSync(recursive: true);
      return true;
    } catch (e) {
      stderr.writeln('createDirectory error: $e');
      return false;
    }
  }

  static bool deleteDirectory(String path) {
    try {
      final dir = Directory(path);
      if (dir.existsSync()) {
        dir.deleteSync(recursive: true);
        return true;
      }
      return false;
    } catch (e) {
      stderr.writeln('deleteDirectory error: $e');
      return false;
    }
  }

  // ---- Processes ----
  static Future<int> runProgramWait(String command) async {
    try {
      final result = await Process.run(_platformShell(), _shellArgs(command));
      stdout.write(result.stdout);
      stderr.write(result.stderr);
      return result.exitCode;
    } catch (e) {
      stderr.writeln('runProgramWait error: $e');
      return -1;
    }
  }

  static Future<Process?> runProgramBackground(String command) async {
    try {
      final process =
          await Process.start(_platformShell(), _shellArgs(command));
      return process;
    } catch (e) {
      stderr.writeln('runProgramBackground error: $e');
      return null;
    }
  }

  static String _platformShell() {
    if (Platform.isWindows) return 'cmd.exe';
    return '/bin/sh';
  }

  static List<String> _shellArgs(String cmd) {
    if (Platform.isWindows) return ['/c', cmd];
    return ['-c', cmd];
  }

  // ================= Collections Runtime =================
  static dynamic listLiteral(List<dynamic> items, {bool mutable = false, bool legacy = false}) {
    if (mutable || legacy) {
      return _PohList(items, legacy: legacy);
    }
    return List<dynamic>.unmodifiable(items);
  }

  static dynamic mapLiteral(Map<dynamic, dynamic> entries, {bool mutable = false, bool legacy = false}) {
    if (mutable || legacy) {
      return _PohMap(Map<dynamic, dynamic>.from(entries), legacy: legacy);
    }
    return Map<dynamic, dynamic>.unmodifiable(entries);
  }

  static dynamic indexAt(dynamic container, dynamic index) {
    if (container is _PohList) {
      final i = _toIndex(index) - 1;
      if (i < 0 || i >= container._items.length) return null;
      return container._items[i];
    }
    if (container is List) {
      final i = _toIndex(index) - 1;
      if (i < 0 || i >= container.length) return null;
      return container[i];
    }
    if (container is _PohMap) {
      return container._map[index];
    }
    if (container is Map) {
      return container[index];
    }
    return null;
  }

  static List<dynamic> keysOf(dynamic m) {
    if (m is _PohMap) return m._map.keys.toList(growable: false);
    if (m is Map) return m.keys.toList(growable: false);
    return <dynamic>[];
  }

  static List<dynamic> valuesOf(dynamic m) {
    if (m is _PohMap) return m._map.values.toList(growable: false);
    if (m is Map) return m.values.toList(growable: false);
    return <dynamic>[];
  }

  static void listAdd(dynamic target, dynamic value) {
    if (target is _PohList) {
      if (target.legacy) _legacyWarn('list');
      target._items.add(value);
      return;
    }
    _mutationError('list');
  }

  static void collectionRemove(dynamic target, dynamic valueOrKey) {
    if (target is _PohList) {
      if (target.legacy) _legacyWarn('list');
      target._items.remove(valueOrKey);
      return;
    }
    if (target is _PohMap) {
      if (target.legacy) _legacyWarn('dictionary');
      target._map.remove(valueOrKey);
      return;
    }
    _mutationError('collection');
  }

  static void mapAdd(dynamic target, dynamic key, dynamic value) {
    if (target is _PohMap) {
      if (target.legacy) _legacyWarn('dictionary');
      target._map[key] = value;
      return;
    }
    _mutationError('dictionary');
  }

  static int _toIndex(dynamic n) {
    if (n is int) return n;
    if (n is double) return n.floor();
    if (n is String) {
      final v = int.tryParse(n.trim());
      if (v != null) return v;
    }
    return 0;
  }

  static Never _mutationError(String kind) {
    throw StateError('Cannot mutate $kind; use "Make a mutable ..." to create a mutable one');
  }

  static void _legacyWarn(String what) {
    stderr.writeln('Deprecated: mutating legacy $what literal; use "Make a mutable $what ..."');
  }
}

class _PohList {
  final List<dynamic> _items;
  final bool legacy;
  _PohList(List<dynamic> items, {this.legacy = false}) : _items = List<dynamic>.from(items);
}

class _PohMap {
  final Map<dynamic, dynamic> _map;
  final bool legacy;
  _PohMap(Map<dynamic, dynamic> map, {this.legacy = false}) : _map = Map<dynamic, dynamic>.from(map);
}
