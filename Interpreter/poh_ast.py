from __future__ import annotations
from dataclasses import dataclass
from typing import Any, List, Optional


# Program
@dataclass
class Program:
    statements: List["Stmt"]


# Statements
class Stmt: ...


@dataclass
class WriteStmt(Stmt):
    expr: "Expr"
    line: int


@dataclass
class AskStmt(Stmt):
    name: str
    kind: str  # "text" | "number" | "decimal"
    line: int


@dataclass
class SetStmt(Stmt):
    name: str
    expr: "Expr"
    line: int


@dataclass
class IncStmt(Stmt):
    name: str
    amount: "Expr"
    line: int


@dataclass
class DecStmt(Stmt):
    name: str
    amount: "Expr"
    line: int


@dataclass
class IfStmt(Stmt):
    condition: "Expr"
    then_body: List[Stmt]
    else_body: Optional[List[Stmt]]
    line: int


@dataclass
class WhileStmt(Stmt):
    condition: "Expr"
    body: List[Stmt]
    line: int


@dataclass
class RepeatStmt(Stmt):
    count: "Expr"
    body: List[Stmt]
    line: int


@dataclass
class BlockStmt(Stmt):
    body: List[Stmt]
    line: int


@dataclass
class FunctionDefStmt(Stmt):
    name: str
    params: List[str]
    body: List[Stmt]
    line: int


@dataclass
class ReturnStmt(Stmt):
    value: Optional["Expr"]
    line: int


@dataclass
class UseStmt(Stmt):
    name: str
    args: List["Expr"]
    line: int


@dataclass
class ImportStmt(Stmt):
    path: str
    line: int


# Expressions
class Expr: ...


@dataclass
class LiteralExpr(Expr):
    value: Any


@dataclass
class IdentifierExpr(Expr):
    name: str


@dataclass
class BinaryExpr(Expr):
    left: Expr
    op: str
    right: Expr


@dataclass
class UnaryExpr(Expr):
    op: str
    expr: Expr


@dataclass
class CallExpr(Expr):
    name: str
    args: List[Expr]


@dataclass
class PredicateExpr(Expr):
    name: str  # 'even' | 'odd' | 'positive' | 'negative'
    value: Expr


@dataclass
class StopStmt(Stmt):
    line: int


@dataclass
class SkipStmt(Stmt):
    line: int


# Collections

@dataclass
class ListLiteralExpr(Expr):
    items: list[Expr]


@dataclass
class DictLiteralExpr(Expr):
    items: list[tuple[Expr, Expr]]  # key, value (allow Expr keys for flexibility)


@dataclass
class AtExpr(Expr):
    container: Expr
    key: Expr  # index or dict key


@dataclass
class KeysOfExpr(Expr):
    container: Expr


@dataclass
class ValuesOfExpr(Expr):
    container: Expr


@dataclass
class AddToListStmt(Stmt):
    value: Expr
    target: Expr
    line: int


@dataclass
class RemoveFromListStmt(Stmt):
    value: Expr
    target: Expr
    line: int


@dataclass
class AddToDictStmt(Stmt):
    key: Expr
    value: Expr
    target: Expr
    line: int


@dataclass
class RemoveFromDictStmt(Stmt):
    key: Expr
    target: Expr
    line: int

@dataclass
class DebugStmt(Stmt):
    enabled: bool
    line: int

# Random / Collection predicates

@dataclass
class RandomIntBetweenExpr(Expr):
    low: Expr
    high: Expr


@dataclass
class RandomFloatBetweenExpr(Expr):
    low: Expr
    high: Expr


@dataclass
class RandomFromExpr(Expr):
    collection: Expr


@dataclass
class ContainsExpr(Expr):
    collection: Expr
    needle: Expr


@dataclass
class AllPredicateExpr(Expr):
    collection: Expr
    predicate: str  # 'even' | 'odd' | 'positive' | 'negative'


@dataclass
class AnyPredicateExpr(Expr):
    collection: Expr
    predicate: str  # 'even' | 'odd' | 'positive' | 'negative'
