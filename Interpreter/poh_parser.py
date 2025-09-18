from __future__ import annotations
from typing import List, Tuple, Optional
import re
import difflib

from .poh_ast import (
    Program,
    Stmt,
    WriteStmt,
    AskStmt,
    SetStmt,
    IncStmt,
    DecStmt,
    IfStmt,
    WhileStmt,
    RepeatStmt,
    FunctionDefStmt,
    ReturnStmt,
    UseStmt,
    ImportStmt,
    StopStmt,
    SkipStmt,
    LiteralExpr,
    IdentifierExpr,
    BinaryExpr,
    UnaryExpr,
    CallExpr,
    PredicateExpr,
    RandomIntBetweenExpr,
    RandomFloatBetweenExpr,
    RandomFromExpr,
    ContainsExpr,
    AllPredicateExpr,
    AnyPredicateExpr,
    ListLiteralExpr,
    DictLiteralExpr,
    AtExpr,
    KeysOfExpr,
    ValuesOfExpr,
    AddToListStmt,
    RemoveFromListStmt,
    AddToDictStmt,
    RemoveFromDictStmt,
    DebugStmt,
)


class ParseError(Exception):
    def __init__(self, msg: str, line: Optional[int] = None):
        self.line = line
        super().__init__(msg if line is None else f"Line {line}: {msg}")


# Public API ---------------------------------------------------------------

def parse_program(lines: List[str]) -> Program:
    cleaned: List[Tuple[int, str]] = []
    for i, raw in enumerate(lines, 1):
        line = _strip_comment(raw.rstrip("\n"))
        if line.strip():
            cleaned.append((i, line))
    i = 0
    stmts: List[Stmt] = []
    while i < len(cleaned):
        stmt, i = _parse_stmt_or_block(cleaned, i)
        stmts.append(stmt)
    return Program(stmts)


# Statement Parsing -------------------------------------------------------

def _parse_stmt_or_block(cleaned: List[Tuple[int, str]], i: int) -> Tuple[Stmt, int]:
    ln, line = cleaned[i]
    s = line.strip()
    low = s.lower()

    # Block IF
    if low.startswith("if ") and " write " not in low:
        cond_src = s[3:].strip()
        i += 1
        then_body: List[Stmt] = []
        else_body: Optional[List[Stmt]] = None
        while i < len(cleaned):
            ln2, raw2 = cleaned[i]
            t = raw2.strip()
            l2 = t.lower()
            if l2 == "otherwise":
                else_body = []
                i += 1
                while i < len(cleaned):
                    ln3, raw3 = cleaned[i]
                    t3 = raw3.strip()
                    l3 = t3.lower()
                    if l3 in ("end", "end if"):
                        i += 1
                        return IfStmt(_parse_bool_expr(cond_src, ln), then_body, else_body, ln), i
                    if l3.startswith("end "):
                        raise ParseError(f"Mismatched '{t3}'; expected 'End' or 'End If'", ln3)
                    stmt3, i = _parse_stmt_or_block(cleaned, i)
                    else_body.append(stmt3)
                raise ParseError("If block not closed with End/End If", ln)
            if l2 in ("end", "end if"):
                i += 1
                return IfStmt(_parse_bool_expr(cond_src, ln), then_body, else_body, ln), i
            if l2.startswith("end "):
                raise ParseError(f"Mismatched '{t}'; expected 'End' or 'End If'", ln2)
            stmt2, i = _parse_stmt_or_block(cleaned, i)
            then_body.append(stmt2)
        raise ParseError("If block not closed with End/End If", ln)

    # Block WHILE
    if low.startswith("while ") and " write " not in low:
        cond_src = s[6:].strip()
        i += 1
        body: List[Stmt] = []
        while i < len(cleaned):
            ln2, raw2 = cleaned[i]
            t = raw2.strip()
            l2 = t.lower()
            if l2 in ("end", "end while"):
                i += 1
                return WhileStmt(_parse_bool_expr(cond_src, ln), body, ln), i
            if l2.startswith("end "):
                raise ParseError(f"Mismatched '{t}'; expected 'End' or 'End While'", ln2)
            stmt2, i = _parse_stmt_or_block(cleaned, i)
            body.append(stmt2)
        raise ParseError("While block not closed with End/End While", ln)

    # Block REPEAT
    if low.startswith("repeat ") and " write " not in low:
        count_src = s[7:].strip()
        i += 1
        body: List[Stmt] = []
        while i < len(cleaned):
            ln2, raw2 = cleaned[i]
            t = raw2.strip()
            l2 = t.lower()
            if l2 in ("end", "end repeat"):
                i += 1
                return RepeatStmt(_parse_expr(count_src, ln), body, ln), i
            if l2.startswith("end "):
                raise ParseError(f"Mismatched '{t}'; expected 'End' or 'End Repeat'", ln2)
            stmt2, i = _parse_stmt_or_block(cleaned, i)
            body.append(stmt2)
        raise ParseError("Repeat block not closed with End/End Repeat", ln)

    # Function block
    if low.startswith("make ") and " write " not in low:
        rest = s[5:].strip()
        name, params = _parse_func_sig(rest, ln)
        i += 1
        body: List[Stmt] = []
        while i < len(cleaned):
            ln2, raw2 = cleaned[i]
            t = raw2.strip()
            l2 = t.lower()
            if l2 == "end":
                i += 1
                return FunctionDefStmt(name, params, body, ln), i
            if l2.startswith("end "):
                raise ParseError(f"Mismatched '{t}'; expected 'End'", ln2)
            if l2.startswith("return"):
                expr_src = t[6:].strip()
                body.append(ReturnStmt(_parse_expr(expr_src, ln2) if expr_src else None, ln2))
                i += 1
                continue
            stmt2, i = _parse_stmt_or_block(cleaned, i)
            body.append(stmt2)
        raise ParseError("Function block not closed with End", ln)

    # Fallback single-line
    st = _parse_single_stmt(ln, s)
    return st, i + 1


def _parse_single_stmt(ln: int, s: str) -> Stmt:
    low = s.lower()

    # Write
    if low.startswith("write "):
        return WriteStmt(_parse_expr(s[6:].strip(), ln), ln)

    # Ask for
    if low.startswith("ask for "):
        body = s[8:].strip()
        parts = body.split()
        if len(parts) >= 2 and parts[-1].lower() in ("number", "decimal"):
            kind = "number" if parts[-1].lower() == "number" else "decimal"
            name = " ".join(parts[:-1])
            return AskStmt(name, kind, ln)
        return AskStmt(body, "text", ln)

    # Set
    if low.startswith("set "):
        rest = s[4:].strip()
        if " to " in rest.lower():
            name, expr_src = re.split(r"(?i)\s+to\s+", rest, maxsplit=1)
        else:
            parts = rest.split(" ", 1)
            if len(parts) != 2:
                raise ParseError("Malformed Set statement", ln)
            name, expr_src = parts
        return SetStmt(name.strip(), _parse_expr(expr_src.strip(), ln), ln)

    # Increase / Decrease
    if low.startswith("increase "):
        rest = s[9:].strip()
        if " by " in rest.lower():
            name, amt = re.split(r"(?i)\s+by\s+", rest, maxsplit=1)
        else:
            name, amt = rest, "1"
        return IncStmt(name.strip(), _parse_expr(amt.strip(), ln), ln)
    if low.startswith("decrease "):
        rest = s[9:].strip()
        if " by " in rest.lower():
            name, amt = re.split(r"(?i)\s+by\s+", rest, maxsplit=1)
        else:
            name, amt = rest, "1"
        return DecStmt(name.strip(), _parse_expr(amt.strip(), ln), ln)

    # Add / Remove (list or dict)
    if low.startswith("add ") and " to " in low:
        value_src, target_src = re.split(r"(?i)\s+to\s+", s[4:].strip(), maxsplit=1)
        if ":" in value_src:
            k, v = value_src.split(":", 1)
            return AddToDictStmt(_parse_expr(k.strip(), ln), _parse_expr(v.strip(), ln), _parse_expr(target_src.strip(), ln), ln)
        return AddToListStmt(_parse_expr(value_src.strip(), ln), _parse_expr(target_src.strip(), ln), ln)
    if low.startswith("remove ") and " from " in low:
        value_src, target_src = re.split(r"(?i)\s+from\s+", s[7:].strip(), maxsplit=1)
        if value_src.strip().startswith(('"', "'")):
            return RemoveFromDictStmt(_parse_expr(value_src.strip(), ln), _parse_expr(target_src.strip(), ln), ln)
        return RemoveFromListStmt(_parse_expr(value_src.strip(), ln), _parse_expr(target_src.strip(), ln), ln)

    # Inline If / While / Repeat / Make
    if low.startswith("if ") and " write " in low:
        # If cond Write expr [Otherwise Write expr]
        m = re.match(r"(?i)if (.*) write (.*)", s)
        if not m:
            raise ParseError("Malformed inline If", ln)
        cond_part = m.group(1)
        then_part = m.group(2)
        ow = re.search(r"(?i) otherwise write ", then_part)
        else_block = None
        if ow:
            then_expr_src = then_part[: ow.start()].strip()
            else_expr_src = then_part[ow.end():].strip()
            then_stmt = WriteStmt(_parse_expr(then_expr_src, ln), ln)
            else_block = [WriteStmt(_parse_expr(else_expr_src, ln), ln)]
        else:
            then_stmt = WriteStmt(_parse_expr(then_part.strip(), ln), ln)
        return IfStmt(_parse_bool_expr(cond_part, ln), [then_stmt], else_block, ln)

    if low.startswith("while ") and " write " in low:
        m = re.match(r"(?i)while (.*) write (.*)", s)
        if not m:
            raise ParseError("Malformed inline While", ln)
        cond_src = m.group(1)
        expr_src = m.group(2)
        return WhileStmt(_parse_bool_expr(cond_src, ln), [WriteStmt(_parse_expr(expr_src, ln), ln)], ln)

    if low.startswith("repeat ") and " write " in low:
        m = re.match(r"(?i)repeat (.*) write (.*)", s)
        if not m:
            raise ParseError("Malformed inline Repeat", ln)
        count_src = m.group(1)
        expr_src = m.group(2)
        return RepeatStmt(_parse_expr(count_src, ln), [WriteStmt(_parse_expr(expr_src, ln), ln)], ln)

    if low.startswith("make ") and " write " in low:
        rest = s[5:].strip()
        name, params = _parse_func_sig(rest, ln)
        m = re.search(r"(?i) write ", rest)
        if not m:
            raise ParseError("Malformed inline Make", ln)
        expr_src = rest[m.end():].strip()
        return FunctionDefStmt(name, params, [ReturnStmt(_parse_expr(expr_src, ln), ln)], ln)

    if low.startswith("use "):
        rest = s[4:].strip()
        name, args = _parse_call_sig(rest, ln)
        return UseStmt(name, [_parse_expr(a, ln) for a in args], ln)

    if low.startswith("import "):
        m = re.match(r"(?i)import\s+\"(.*)\"", s)
        if not m:
            raise ParseError("Import expects a quoted path", ln)
        return ImportStmt(m.group(1), ln)

    if low == "stop":
        return StopStmt(ln)
    if low == "skip":
        return SkipStmt(ln)

    if low in ("debug on", "debug off"):
        return DebugStmt(enabled=(low == "debug on"), line=ln)

    # Unknown statement suggestion
    KNOWN = [
        'write','ask','set','increase','decrease','if','while','repeat','make','use','import','stop','skip','debug'
    ]
    head = s.split()[0].lower() if s.split() else s.lower()
    suggestion = difflib.get_close_matches(head, KNOWN, n=1)
    if suggestion:
        raise ParseError(f"Unknown statement '{s}'. Did you mean '{suggestion[0]}'?", ln)
    raise ParseError(f"Unknown statement: {s}", ln)


# Helpers -----------------------------------------------------------------

def _parse_func_sig(src: str, ln: int) -> Tuple[str, List[str]]:
    if " with " in src.lower():
        name, tail = re.split(r"(?i)\swith\s", src, maxsplit=1)
        params = [p.strip() for p in tail.split(',') if p.strip()]
    else:
        name, params = src, []
    return name.strip(), params


def _parse_call_sig(src: str, ln: int) -> Tuple[str, List[str]]:
    if " with " in src.lower():
        name, tail = re.split(r"(?i)\swith\s", src, maxsplit=1)
        args = [p.strip() for p in tail.split(',') if p.strip()]
    else:
        name, args = src, []
    return name.strip(), args


# Boolean / Logical -------------------------------------------------------

def _parse_bool_expr(src: str, ln: int):
    s = src.strip()
    s = re.sub(r"(?i)\bis not\b", " != ", s)
    s = re.sub(r"(?i)\bis at least\b", " >= ", s)
    s = re.sub(r"(?i)\bis at most\b", " <= ", s)
    s = re.sub(r"(?i)\bis greater than\b", " > ", s)
    s = re.sub(r"(?i)\bis less than\b", " < ", s)
    pred_match = re.search(r"(?i)^(.*)\bis\s+(even|odd|positive|negative)\s*$", s)
    if pred_match:
        value_src = pred_match.group(1).strip()
        pred = pred_match.group(2).lower()
        return PredicateExpr(pred, _parse_expr(value_src, ln))
    s = re.sub(r"(?i)\bis\b", " == ", s)
    s = re.sub(r"(?i)\band\b", " && ", s)
    s = re.sub(r"(?i)\bor\b", " || ", s)
    s = re.sub(r"(?i)\bnot\b", " ! ", s)
    return _parse_logic_expr(s, ln)


# Expression Parsing ------------------------------------------------------

def _parse_expr(src: str, ln: int):
    # Random expressions
    s = src.strip()
    m = re.match(r"(?i)^random\s+decimal\s+between\s+(.*)\s+(?:and|to)\s+(.*)$", s)
    if m:
        return RandomFloatBetweenExpr(_parse_expr(m.group(1).strip(), ln), _parse_expr(m.group(2).strip(), ln))
    m = re.match(r"(?i)^random\s+between\s+(.*)\s+(?:and|to)\s+(.*)$", s)
    if m:
        return RandomIntBetweenExpr(_parse_expr(m.group(1).strip(), ln), _parse_expr(m.group(2).strip(), ln))
    m = re.match(r"(?i)^random\s+from\s+(.*)$", s)
    if m:
        return RandomFromExpr(_parse_expr(m.group(1).strip(), ln))
    m = re.match(r"(?i)^contains\s+(.*)\s+in\s+(.*)$", s)
    if m:
        return ContainsExpr(_parse_expr(m.group(2).strip(), ln), _parse_expr(m.group(1).strip(), ln))
    m = re.match(r"(?i)^all\s+(.*)\s+are\s+(even|odd|positive|negative)$", s)
    if m:
        return AllPredicateExpr(_parse_expr(m.group(1).strip(), ln), m.group(2).lower())
    m = re.match(r"(?i)^any\s+(.*)\s+is\s+(even|odd|positive|negative)$", s)
    if m:
        return AnyPredicateExpr(_parse_expr(m.group(1).strip(), ln), m.group(2).lower())
    # List literal: List contains a, b, c
    if re.match(r"(?i)^list\s+contains\b", s):
        tail = re.sub(r"(?i)^list\s+contains\s+", "", s)
        items = _split_top_level(tail)
        return ListLiteralExpr([_parse_expr(it.strip(), ln) for it in items if it.strip()])
    # Dict literal: Dictionary contains key: value, key: value
    if re.match(r"(?i)^dictionary\s+contains\b", s):
        tail = re.sub(r"(?i)^dictionary\s+contains\s+", "", s)
        pairs = _split_top_level(tail)
        kvs: list[tuple] = []
        for p in pairs:
            if ":" not in p:
                raise ParseError("dictionary entry must be 'key: value'", ln)
            k, v = p.split(":", 1)
            kvs.append((_parse_expr(k.strip(), ln), _parse_expr(v.strip(), ln)))
        return DictLiteralExpr(kvs)
    # keys/values of
    if re.match(r"(?i)^keys\s+of\s+", s):
        tail = re.sub(r"(?i)^keys\s+of\s+", "", s)
        return KeysOfExpr(_parse_expr(tail.strip(), ln))
    if re.match(r"(?i)^values\s+of\s+", s):
        tail = re.sub(r"(?i)^values\s+of\s+", "", s)
        return ValuesOfExpr(_parse_expr(tail.strip(), ln))

    # Arithmetic phrase normalization
    s = re.sub(r"(?i)\bdivided by\b", " / ", s)
    s = re.sub(r"(?i)\btimes\b", " * ", s)
    s = re.sub(r"(?i)\bplus\b", " + ", s)
    s = re.sub(r"(?i)\bminus\b", " - ", s)
    return _parse_add_sub(s, ln)


# Pratt-style precedence --------------------------------------------------

def _parse_add_sub(src: str, ln: int):
    tokens = _tokenize(src)
    expr, rest = _parse_mul_div(tokens, ln)
    while rest and rest[0][0] in ('+', '-'):
        op = rest[0][0]
        rhs, rest = _parse_mul_div(rest[1:], ln)
        expr = BinaryExpr(expr, op, rhs)
    return expr


def _parse_mul_div(tokens: List[Tuple[str, str]], ln: int):
    expr, rest = _parse_unary(tokens, ln)
    while rest and rest[0][0] in ('*', '/'):
        op = rest[0][0]
        rhs, rest = _parse_unary(rest[1:], ln)
        expr = BinaryExpr(expr, op, rhs)
    return expr, rest


def _parse_unary(tokens: List[Tuple[str, str]], ln: int):
    if tokens and tokens[0][0] in ('+', '-', '!'):
        op = tokens[0][0]
        expr, rest = _parse_primary(tokens[1:], ln)
        return UnaryExpr(op, expr), rest
    return _parse_primary(tokens, ln)


def _parse_primary(tokens: List[Tuple[str, str]], ln: int):
    if not tokens:
        raise ParseError("expected expression", ln)
    kind, val = tokens[0]
    if kind == 'NUMBER':
        if '.' in val:
            return LiteralExpr(float(val)), tokens[1:]
        return LiteralExpr(int(val)), tokens[1:]
    if kind == 'STRING':
        return LiteralExpr(_unescape(val[1:-1])), tokens[1:]
    if kind == 'IDENT':
        # possible call
        if len(tokens) > 1 and tokens[1][0] == '(':
            args: List = []
            rest = tokens[2:]
            if rest and rest[0][0] == ')':
                return CallExpr(val, []), rest[1:]
            while True:
                arg, rest = _parse_add_sub_tokens(rest, ln)
                args.append(arg)
                if not rest:
                    raise ParseError("expected ')' in call", ln)
                if rest[0][0] == ')':
                    rest = rest[1:]
                    expr: object = CallExpr(val, args)
                    break
                if rest[0][0] != ',':
                    raise ParseError("expected ',' between args", ln)
                rest = rest[1:]
        else:
            expr = IdentifierExpr(val)
            rest = tokens[1:]
        # postfix 'at'
        while rest and rest[0][0] == 'IDENT' and rest[0][1].lower() == 'at':
            rhs, rest2 = _parse_add_sub_tokens(rest[1:], ln)
            expr = AtExpr(expr, rhs)
            rest = rest2
        return expr, rest
    if kind == '(':
        expr, rest = _parse_add_sub_tokens(tokens[1:], ln)
        if not rest or rest[0][0] != ')':
            raise ParseError("expected ')' to close group", ln)
        rest = rest[1:]
        while rest and rest[0][0] == 'IDENT' and rest[0][1].lower() == 'at':
            rhs, rest2 = _parse_add_sub_tokens(rest[1:], ln)
            expr = AtExpr(expr, rhs)
            rest = rest2
        return expr, rest
    return IdentifierExpr(val), tokens[1:]


def _parse_add_sub_tokens(tokens: List[Tuple[str, str]], ln: int):
    expr, rest = _parse_mul_div(tokens, ln)
    while rest and rest[0][0] in ('+', '-'):
        op = rest[0][0]
        rhs, rest = _parse_mul_div(rest[1:], ln)
        expr = BinaryExpr(expr, op, rhs)
    return expr, rest


def _parse_logic_expr(src: str, ln: int):
    tokens = _tokenize(src)

    def parse_and(rest):
        left, rest = _parse_compare(rest, ln)
        while rest and rest[0][0] == '&&':
            right, rest = _parse_compare(rest[1:], ln)
            left = BinaryExpr(left, '&&', right)
        return left, rest

    def parse_or(rest):
        left, rest = parse_and(rest)
        while rest and rest[0][0] == '||':
            right, rest = parse_and(rest[1:])
            left = BinaryExpr(left, '||', right)
        return left, rest

    expr, rest = parse_or(tokens)
    return expr


def _parse_compare(tokens: List[Tuple[str, str]], ln: int):
    left, rest = _parse_add_sub_tokens(tokens, ln)
    if rest and rest[0][0] in ('==', '!=', '>=', '<=', '>', '<'):
        op = rest[0][0]
        right, rest = _parse_add_sub_tokens(rest[1:], ln)
        return BinaryExpr(left, op, right), rest
    return left, rest


# Tokenizer ---------------------------------------------------------------

def _tokenize(src: str) -> List[Tuple[str, str]]:
    spec = r"""
        (?P<SPACE>\s+)
      | (?P<NUMBER>\d+(?:\.\d+)?)
      | (?P<STRING>"([^"\\]|\\.)*")
      | (?P<OP>==|!=|>=|<=|&&|\|\||\+|\-|\*|/|\(|\)|,|>|<|!)
      | (?P<IDENT>[A-Za-z_][A-Za-z0-9_]*)
    """
    tokens: List[Tuple[str, str]] = []
    for m in re.finditer(spec, src, flags=re.VERBOSE):
        kind = m.lastgroup
        val = m.group()
        if kind == 'SPACE':
            continue
        if kind == 'OP':
            tokens.append((val, val))
        else:
            tokens.append((kind, val))
    return tokens


# Utilities ---------------------------------------------------------------

def _split_top_level(s: str) -> list[str]:
    items: list[str] = []
    buf: list[str] = []
    in_str = False
    depth = 0
    i = 0
    while i < len(s):
        ch = s[i]
        if ch == '"':
            in_str = not in_str
            buf.append(ch)
        elif ch == '(' and not in_str:
            depth += 1
            buf.append(ch)
        elif ch == ')' and not in_str and depth > 0:
            depth -= 1
            buf.append(ch)
        elif ch == ',' and not in_str and depth == 0:
            items.append(''.join(buf).strip())
            buf = []
        else:
            buf.append(ch)
        i += 1
    if buf:
        items.append(''.join(buf).strip())
    return items


def _strip_comment(line: str) -> str:
    out = []
    in_str = False
    for ch in line:
        if ch == '"':
            in_str = not in_str
            out.append(ch)
        elif ch == '#' and not in_str:
            break
        else:
            out.append(ch)
    return ''.join(out)


def _unescape(s: str) -> str:
    return s.replace('\\"', '"').replace('\\n', '\n')
