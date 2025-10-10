#[derive(Debug, Clone)]
pub enum Expr {
    Str(String),
    Num(f64),
    Bool(bool),
    Null,
    Ident(String),
    Plus(Box<Expr>, Box<Expr>),
    Minus(Box<Expr>, Box<Expr>),
    Times(Box<Expr>, Box<Expr>),
    DividedBy(Box<Expr>, Box<Expr>),
    Call { name: String, args: Vec<Expr> },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Cmp(CmpOp, Box<Expr>, Box<Expr>),
    ListLit(Vec<Expr>),
    DictLit(Vec<(String, Expr)>),
    Index(Box<Expr>, Box<Expr>), // collection[index]
    // Phrasal built-in expressions
    TotalOf(Box<Expr>),             // total of list
    SmallestIn(Box<Expr>),          // smallest in list
    LargestIn(Box<Expr>),           // largest in list
    AbsoluteValueOf(Box<Expr>),     // absolute value of number
    Round(Box<Expr>),               // round number
    RoundDown(Box<Expr>),           // round down number
    RoundUp(Box<Expr>),             // round up number
    MakeUppercase(Box<Expr>),       // make uppercase string
    MakeLowercase(Box<Expr>),       // make lowercase string
    TrimSpaces(Box<Expr>),          // trim spaces from string
    FirstIn(Box<Expr>),             // first in list/string
    LastIn(Box<Expr>),              // last in list/string
    ReverseOf(Box<Expr>),           // reverse of list/string
    CountOf(Box<Expr>),             // count of list/string/dict
    JoinWith(Box<Expr>, Box<Expr>), // join list with separator
    SplitBy(Box<Expr>, Box<Expr>),  // split string by separator
    // Additional collection operations
    Contains(Box<Expr>, Box<Expr>), // contains item in collection
    Remove(Box<Expr>, Box<Expr>),   // remove item from list
    Append(Box<Expr>, Box<Expr>),   // append item to list
    InsertAt(Box<Expr>, Box<Expr>, Box<Expr>), // insert item at index in list
    // File I/O operations
    ReadFile(Box<Expr>),                       // read file at path
    WriteFile(Box<Expr>, Box<Expr>),           // write content to file at path
    AppendFile(Box<Expr>, Box<Expr>),          // append content to file at path
    FileExists(Box<Expr>),                     // file exists at path
    DeleteFile(Box<Expr>),                     // delete file at path
    CreateDir(Box<Expr>),                      // create directory at path
    ListDir(Box<Expr>),                        // list files in directory at path
    ReadLines(Box<Expr>),                      // read lines from file at path
    CopyFile(Box<Expr>, Box<Expr>),            // copy file from source to dest
    MoveFile(Box<Expr>, Box<Expr>),            // move file from source to dest
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Write(Expr),
    AskFor {
        var_name: String,
    },
    IfInline {
        cond: Expr,
        then_write: Expr,
        otherwise_write: Option<Expr>,
    },
    IfBlock {
        cond: Expr,
        then_body: Program,
        otherwise_body: Option<Program>,
    },
    FuncInline {
        name: String,
        params: Vec<Param>,
        body: Expr,
    },
    FuncBlock {
        name: String,
        params: Vec<Param>,
        body: Program,
    },
    WhileBlock {
        cond: Expr,
        body: Program,
    },
    RepeatBlock {
        count: Expr,
        body: Program,
    },
    ImportLocal {
        path: String,
    },
    ImportSystem {
        name: String,
        alias: Option<String>,
        exposing: Vec<String>,
    },
    Use {
        name: String,
        args: Vec<Expr>,
    },
    Set {
        name: String,
        value: Expr,
    },
    Return(Option<Expr>),
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: Option<Expr>,
}

pub type Program = Vec<Stmt>;

#[derive(Debug, Clone)]
pub enum CmpOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}
