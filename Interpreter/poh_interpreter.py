from __future__ import annotations
from typing import Any, Dict, List, Optional, Callable

from .poh_ast import *
from .poh_parser import parse_program, ParseError


class RuntimeErrorPoh(Exception):
    pass


class Environment:
    def __init__(self, parent: Optional['Environment']=None, frame_type: str = "block"):
        self.parent = parent
        self.frame_type = frame_type  # 'global' | 'function' | 'block'
        self.values: Dict[str, Any] = {}

    def get(self, name: str) -> Any:
        if name in self.values:
            return self.values[name]
        if self.parent:
            return self.parent.get(name)
        raise RuntimeErrorPoh(f"Undefined variable '{name}'")

    def set(self, name: str, value: Any) -> None:
        # Assign to nearest scope containing the name; otherwise define in nearest non-function ancestor.
        if name in self.values:
            self.values[name] = value
            return
        if self.parent and self.parent.has(name):
            # propagate down assignment to where it lives
            self.parent.set(name, value)
            return
        # escalate definition upward until reaching non-function if current is function inner block
        target: Environment = self
        while target.parent and target.frame_type == 'block' and target.parent.frame_type == 'function':
            target = target.parent
        target.values[name] = value

    def define(self, name: str, value: Any) -> None:
        self.values[name] = value

    def has(self, name: str) -> bool:
        if name in self.values:
            return True
        if self.parent:
            return self.parent.has(name)
        return False


class Function:
    def __init__(self, decl: FunctionDefStmt, closure: Environment, interpret: 'Interpreter'):
        self.decl = decl
        self.closure = closure
        self.interpret = interpret

    def call(self, args: List[Any]) -> Any:
        env = Environment(self.closure, frame_type='function')
        for name, val in zip(self.decl.params, args):
            env.define(name, val)
        try:
            return self.interpret._exec_block(self.decl.body, env)
        except _ReturnSignal as r:
            return r.value
        return None


class _ReturnSignal(Exception):
    def __init__(self, value: Any):
        self.value = value


class _BreakSignal(Exception):
    pass


class _ContinueSignal(Exception):
    pass


class Interpreter:
    def __init__(self, input_fn: Optional[Callable[[str], str]]=None, output_fn: Optional[Callable[[str], None]]=None):
        self.globals = Environment(frame_type='global')
        self.input_fn = input_fn or (lambda prompt: input(prompt))
        self.output_fn = output_fn or print
        self.functions: Dict[str, Function] = {}
        # Built-in standard library functions
        self._install_builtins()
        self.debug_enabled = False
        self.import_stack: list[str] = []
        self.loaded_files: set[str] = set()

    def run(self, src: str) -> None:
        lines = src.splitlines()
        program = parse_program(lines)
        self.execute(program)

    def run_file(self, path: str) -> None:
        import os, difflib
        full = os.path.abspath(path)
        if full in self.import_stack:
            chain = ' -> '.join(self.import_stack + [full])
            raise RuntimeErrorPoh(f"Circular import detected: {chain}")
        if full in self.loaded_files:
            return
        self.import_stack.append(full)
        try:
            try:
                with open(full, 'r', encoding='utf-8') as f:
                    src = f.read()
            except FileNotFoundError:
                # Build candidate list: scan current working directory (and examples/poh) for .poh files
                cwd = os.getcwd()
                candidates: list[str] = []
                for root, _, files in os.walk(cwd):
                    for fn in files:
                        if fn.lower().endswith('.poh'):
                            candidates.append(os.path.join(root, fn))
                short_requested = os.path.basename(full)
                scored = difflib.get_close_matches(short_requested, [os.path.basename(c) for c in candidates], n=1)
                if scored:
                    suggestion = scored[0]
                    raise RuntimeErrorPoh(f"I couldn't find the file '{path}'. Did you mean '{suggestion}'?")
                raise RuntimeErrorPoh(f"I couldn't find the file '{path}'.")
            cwd_before = os.getcwd()
            os.chdir(os.path.dirname(full) or cwd_before)
            try:
                self.run(src)
            finally:
                os.chdir(cwd_before)
            self.loaded_files.add(full)
        finally:
            self.import_stack.pop()

    def execute(self, program: Program) -> None:
        # first pass: collect function defs
        for st in program.statements:
            if isinstance(st, FunctionDefStmt):
                self.functions[st.name] = Function(st, self.globals, self)
        # second pass: run top-level statements except function defs
        for st in program.statements:
            if isinstance(st, FunctionDefStmt):
                continue
            self._exec_stmt(st, self.globals)

    def _exec_block(self, stmts: List[Stmt], env: Environment) -> Any:
        # Execute until a Return is thrown or end.
        for st in stmts:
            if isinstance(st, ReturnStmt):
                val = self._eval(st.value, env) if st.value is not None else None
                raise _ReturnSignal(val)
            self._exec_stmt(st, env)
        return None

    def _log(self, msg: str) -> None:
        if self.debug_enabled:
            self.output_fn(f"[debug] {msg}")

    def _exec_stmt(self, st: Stmt, env: Environment) -> None:
        # Debug: show statement type and line
        if self.debug_enabled:
            try:
                line = getattr(st, 'line', None)
                self._log(f"Executing {st.__class__.__name__} at line {line}")
            except Exception:
                pass
        if isinstance(st, WriteStmt):
            val = self._eval(st.expr, env)
            self.output_fn(str(val))
        elif isinstance(st, AskStmt):
            raw = self.input_fn(f"Enter {st.name}: ")
            if st.kind == 'number':
                try:
                    env.set(st.name, int(raw.strip()))
                except Exception:
                    self._log(f"Invalid number input for {st.name}, defaulting to 0")
                    env.set(st.name, 0)
            elif st.kind == 'decimal':
                try:
                    env.set(st.name, float(raw.strip()))
                except Exception:
                    self._log(f"Invalid decimal input for {st.name}, defaulting to 0.0")
                    env.set(st.name, 0.0)
            else:
                env.set(st.name, raw)
        elif isinstance(st, SetStmt):
            env.set(st.name, self._eval(st.expr, env))
        elif isinstance(st, IncStmt):
            env.set(st.name, (env.get(st.name) if env.has(st.name) else 0) + self._eval(st.amount, env))
        elif isinstance(st, DecStmt):
            env.set(st.name, (env.get(st.name) if env.has(st.name) else 0) - self._eval(st.amount, env))
        elif isinstance(st, IfStmt):
            if self._is_truthy(self._eval(st.condition, env)):
                try:
                    self._exec_block(st.then_body, Environment(env, frame_type='block'))
                except (_BreakSignal, _ContinueSignal):
                    pass
            else:
                if st.else_body is not None:
                    try:
                        self._exec_block(st.else_body, Environment(env, frame_type='block'))
                    except (_BreakSignal, _ContinueSignal):
                        pass
        elif isinstance(st, WhileStmt):
            while self._is_truthy(self._eval(st.condition, env)):
                try:
                    self._exec_block(st.body, Environment(env, frame_type='block'))
                except _BreakSignal:
                    break
                except _ContinueSignal:
                    continue
        elif isinstance(st, RepeatStmt):
            iter_val = self._eval(st.count, env)
            # If numeric, repeat N times; if list, iterate elements; if dict, iterate keys
            if isinstance(iter_val, int):
                iterator = range(iter_val)
            elif isinstance(iter_val, float):
                iterator = range(int(iter_val))
            elif isinstance(iter_val, list):
                iterator = iter_val
            elif isinstance(iter_val, dict):
                iterator = list(iter_val.keys())
            else:
                try:
                    iterator = range(int(iter_val))
                except Exception:
                    iterator = []
            for current in iterator:
                try:
                    loop_env = Environment(env, frame_type='block')
                    # Provide implicit 'it' for list/dict iteration
                    if isinstance(iter_val, (list, dict)):
                        loop_env.define('it', current)
                    self._exec_block(st.body, loop_env)
                except _BreakSignal:
                    break
                except _ContinueSignal:
                    continue
        elif isinstance(st, AddToListStmt):
            target = self._eval(st.target, env)
            value = self._eval(st.value, env)
            if isinstance(target, list):
                target.append(value)
            else:
                raise RuntimeErrorPoh("Add to expects a list")
        elif isinstance(st, RemoveFromListStmt):
            target = self._eval(st.target, env)
            value = self._eval(st.value, env)
            if isinstance(target, list):
                try:
                    target.remove(value)
                except ValueError:
                    pass
            else:
                raise RuntimeErrorPoh("Remove from expects a list")
        elif isinstance(st, AddToDictStmt):
            target = self._eval(st.target, env)
            key = self._eval(st.key, env)
            value = self._eval(st.value, env)
            if isinstance(target, dict):
                target[key] = value
            else:
                raise RuntimeErrorPoh("Add to expects a dictionary when using key: value")
        elif isinstance(st, RemoveFromDictStmt):
            target = self._eval(st.target, env)
            key = self._eval(st.key, env)
            if isinstance(target, dict):
                target.pop(key, None)
            else:
                raise RuntimeErrorPoh("Remove from expects a dictionary when key is provided")
        elif isinstance(st, StopStmt):
            raise _BreakSignal()
        elif isinstance(st, SkipStmt):
            raise _ContinueSignal()
        elif isinstance(st, DebugStmt):
            self.debug_enabled = st.enabled
            self._log(f"Debug {'enabled' if self.debug_enabled else 'disabled'} at line {st.line}")
        elif isinstance(st, UseStmt):
            if st.name not in self.functions:
                raise RuntimeErrorPoh(f"Line {st.line}: I can't find a function named '{st.name}'.")
            fn = self.functions[st.name]
            args = [self._eval(a, env) for a in st.args]
            fn.call(args)
        elif isinstance(st, ImportStmt):
            import os
            path = st.path
            if not os.path.isabs(path):
                path = os.path.normpath(os.path.join(os.getcwd(), path))
            try:
                self.run_file(path)
            except RuntimeErrorPoh as e:
                raise RuntimeErrorPoh(f"Line {st.line}: {e}")
        elif isinstance(st, ReturnStmt):
            raise _ReturnSignal(self._eval(st.value, env) if st.value is not None else None)
        else:
            # placeholders for Stop/Skip were mapped to Write None in parser; detect specifically
            pass

    def _eval(self, e: Expr, env: Environment) -> Any:
        if isinstance(e, LiteralExpr):
            return e.value
        if isinstance(e, IdentifierExpr):
            if e.name.lower() == 'true':
                return True
            if e.name.lower() == 'false':
                return False
            if e.name.lower() == 'nothing':
                return None
            return env.get(e.name)
        if isinstance(e, UnaryExpr):
            v = self._eval(e.expr, env)
            if e.op == '-':
                return -v
            if e.op == '+':
                return +v
            if e.op == '!':
                return not self._is_truthy(v)
        if isinstance(e, BinaryExpr):
            if e.op in ('+', '-', '*', '/'):
                l = self._eval(e.left, env)
                r = self._eval(e.right, env)
                if e.op == '+':
                    # Allow string concatenation
                    if isinstance(l, str) or isinstance(r, str):
                        return str(l) + str(r)
                    return l + r
                if e.op == '-':
                    return l - r
                if e.op == '*':
                    return l * r
                if e.op == '/':
                    try:
                        return l / r
                    except ZeroDivisionError:
                        raise RuntimeErrorPoh("Oops! You tried to divide by zero. That's not allowed.")
            if e.op in ('==', '!=', '>', '<', '>=', '<='):
                l = self._eval(e.left, env)
                r = self._eval(e.right, env)
                if e.op == '==':
                    return l == r
                if e.op == '!=':
                    return l != r
                if e.op == '>':
                    return l > r
                if e.op == '<':
                    return l < r
                if e.op == '>=':
                    return l >= r
                if e.op == '<=':
                    return l <= r
            if e.op in ('&&', '||'):
                l = self._is_truthy(self._eval(e.left, env))
                if e.op == '&&':
                    return l and self._is_truthy(self._eval(e.right, env))
                else:
                    return l or self._is_truthy(self._eval(e.right, env))
        if isinstance(e, PredicateExpr):
            v = self._eval(e.value, env)
            if not isinstance(v, (int, float)):
                raise RuntimeErrorPoh("Predicate expects a number")
            if e.name == 'even':
                return int(v) % 2 == 0
            if e.name == 'odd':
                return int(v) % 2 != 0
            if e.name == 'positive':
                return v > 0
            if e.name == 'negative':
                return v < 0
        if isinstance(e, CallExpr):
            if e.name not in self.functions:
                raise RuntimeErrorPoh(f"Undefined function '{e.name}'")
            fn = self.functions[e.name]
            args = [self._eval(a, env) for a in e.args]
            return fn.call(args)
        if isinstance(e, RandomIntBetweenExpr):
            import random
            lo = self._eval(e.low, env)
            hi = self._eval(e.high, env)
            return random.randint(int(lo), int(hi))
        if isinstance(e, RandomFloatBetweenExpr):
            import random
            lo = float(self._eval(e.low, env))
            hi = float(self._eval(e.high, env))
            return random.uniform(lo, hi)
        if isinstance(e, RandomFromExpr):
            import random
            col = self._eval(e.collection, env)
            if isinstance(col, (list, str)):
                if not col:
                    return None
                return random.choice(list(col))
            if isinstance(col, dict):
                keys = list(col.keys())
                if not keys:
                    return None
                return col[random.choice(keys)]
            raise RuntimeErrorPoh("random from expects a list, string, or dictionary")
        if isinstance(e, ContainsExpr):
            col = self._eval(e.collection, env)
            needle = self._eval(e.needle, env)
            if isinstance(col, dict):
                return needle in col
            try:
                return needle in col
            except TypeError:
                return False
        if isinstance(e, AllPredicateExpr):
            col = self._eval(e.collection, env)
            if not isinstance(col, (list, tuple)):
                raise RuntimeErrorPoh("all <collection> are <predicate> expects a list")
            return all(self._num_predicate(v, e.predicate) for v in col)
        if isinstance(e, AnyPredicateExpr):
            col = self._eval(e.collection, env)
            if not isinstance(col, (list, tuple)):
                raise RuntimeErrorPoh("any <collection> is <predicate> expects a list")
            return any(self._num_predicate(v, e.predicate) for v in col)
        if isinstance(e, ListLiteralExpr):
            return [self._eval(it, env) for it in e.items]
        if isinstance(e, DictLiteralExpr):
            return {self._eval(k, env): self._eval(v, env) for k, v in e.items}
        if isinstance(e, AtExpr):
            container = self._eval(e.container, env)
            key = self._eval(e.key, env)
            if isinstance(container, list):
                try:
                    return container[key]
                except IndexError:
                    raise RuntimeErrorPoh(f"Index {key} is out of range for the list.")
            if isinstance(container, dict):
                try:
                    return container[key]
                except KeyError:
                    raise RuntimeErrorPoh(f"Key {key} was not found in the dictionary.")
            raise RuntimeErrorPoh("'at' expects a list index or dictionary key")
        if isinstance(e, KeysOfExpr):
            container = self._eval(e.container, env)
            if isinstance(container, dict):
                return list(container.keys())
            raise RuntimeErrorPoh("keys of expects a dictionary")
        if isinstance(e, ValuesOfExpr):
            container = self._eval(e.container, env)
            if isinstance(container, dict):
                return list(container.values())
            raise RuntimeErrorPoh("values of expects a dictionary")
        return None

    def _is_truthy(self, v: Any) -> bool:
        return bool(v)

    def _install_builtins(self) -> None:
        # length(x): works for strings, lists, dicts
        def _length(args: List[Any]) -> Any:
            if len(args) != 1:
                raise RuntimeErrorPoh("length expects 1 argument")
            x = args[0]
            try:
                return len(x)
            except Exception:
                raise RuntimeErrorPoh("length expects a collection or string")

        # sum(list): numeric sum
        def _sum(args: List[Any]) -> Any:
            if len(args) != 1:
                raise RuntimeErrorPoh("sum expects 1 argument")
            x = args[0]
            if not isinstance(x, (list, tuple)):
                raise RuntimeErrorPoh("sum expects a list")
            total = 0
            for v in x:
                if not isinstance(v, (int, float)):
                    raise RuntimeErrorPoh("sum expects numeric values")
                total += v
            return total

        # min(list) / max(list): numeric min/max
        def _minf(args: List[Any]) -> Any:
            if len(args) != 1:
                raise RuntimeErrorPoh("min expects 1 argument")
            x = args[0]
            if not isinstance(x, (list, tuple)) or not x:
                raise RuntimeErrorPoh("min expects a non-empty list")
            for v in x:
                if not isinstance(v, (int, float)):
                    raise RuntimeErrorPoh("min expects numeric values")
            return min(x)

        def _maxf(args: List[Any]) -> Any:
            if len(args) != 1:
                raise RuntimeErrorPoh("max expects 1 argument")
            x = args[0]
            if not isinstance(x, (list, tuple)) or not x:
                raise RuntimeErrorPoh("max expects a non-empty list")
            for v in x:
                if not isinstance(v, (int, float)):
                    raise RuntimeErrorPoh("max expects numeric values")
            return max(x)

        class _Builtin(Function):
            def __init__(self, name: str, impl: Callable[[List[Any]], Any]):
                self.name = name
                self.impl = impl

            def call(self, args: List[Any]) -> Any:
                return self.impl(args)

        # New built-ins --------------------------------------------------
        def _range(args: List[Any]) -> Any:
            # range(n) or range(start, stop[, step])
            if not 1 <= len(args) <= 3:
                raise RuntimeErrorPoh("range expects 1 to 3 arguments")
            nums = [int(a) for a in args]
            if len(nums) == 1:
                return list(range(nums[0]))
            if len(nums) == 2:
                return list(range(nums[0], nums[1]))
            return list(range(nums[0], nums[1], nums[2]))

        def _join(args: List[Any]) -> Any:
            # join(list, sep) or join(list) default sep empty
            if not (1 <= len(args) <= 2):
                raise RuntimeErrorPoh("join expects list and optional separator")
            col = args[0]
            if not isinstance(col, (list, tuple)):
                raise RuntimeErrorPoh("join expects a list")
            sep = args[1] if len(args) == 2 else ''
            return str(sep).join(str(x) for x in col)

        def _split(args: List[Any]) -> Any:
            # split(text, sep) -> list
            if len(args) != 2:
                raise RuntimeErrorPoh("split expects text and separator")
            return str(args[0]).split(str(args[1]))

        def _now(args: List[Any]) -> Any:
            if args:
                raise RuntimeErrorPoh("now expects no arguments")
            import datetime
            return datetime.datetime.now().isoformat(timespec='seconds')

        self.functions['length'] = _Builtin('length', _length)
        self.functions['sum'] = _Builtin('sum', _sum)
        self.functions['min'] = _Builtin('min', _minf)
        self.functions['max'] = _Builtin('max', _maxf)
        self.functions['range'] = _Builtin('range', _range)
        self.functions['join'] = _Builtin('join', _join)
        self.functions['split'] = _Builtin('split', _split)
        self.functions['now'] = _Builtin('now', _now)

    def _num_predicate(self, v: Any, pred: str) -> bool:
        if not isinstance(v, (int, float)):
            return False
        if pred == 'even':
            return int(v) % 2 == 0
        if pred == 'odd':
            return int(v) % 2 != 0
        if pred == 'positive':
            return v > 0
        if pred == 'negative':
            return v < 0
        return False
