// Centralized phrasal prefixes and helpers
// Note: Keep ASCII case-insensitive comparisons using eq_ignore_ascii_case.

pub const P_TOTAL_OF: &str = "total of ";
pub const P_SMALLEST_IN: &str = "smallest in ";
pub const P_LARGEST_IN: &str = "largest in ";
pub const P_ABS_OF: &str = "absolute value of ";
pub const P_ROUND: &str = "round ";
pub const P_ROUND_DOWN: &str = "round down ";
pub const P_ROUND_UP: &str = "round up ";
pub const P_MAKE_UPPER: &str = "make uppercase ";
pub const P_MAKE_LOWER: &str = "make lowercase ";
pub const P_TRIM_FROM: &str = "trim spaces from ";
pub const P_FIRST_IN: &str = "first in ";
pub const P_LAST_IN: &str = "last in ";
pub const P_REVERSE_OF: &str = "reverse of ";
// Aliases
pub const P_REVERSE_ALIAS: &str = "reverse ";
pub const P_CLEAN_ALIAS: &str = "clean spaces from ";

// New phrases
pub const P_COUNT_OF: &str = "count of ";
pub const P_JOIN: &str = "join "; // needs ' with '
pub const P_JOIN_WITH: &str = " with ";
pub const P_SPLIT: &str = "split "; // needs ' by '
pub const P_SPLIT_BY: &str = " by ";
// New aliases
pub const P_SIZE_OF: &str = "size of "; // -> count of
pub const P_SEPARATE: &str = "separate "; // needs ' by ' -> split

// Collection operation phrases
pub const P_CONTAINS: &str = "contains "; // needs ' in '
pub const P_CONTAINS_IN: &str = " in ";
pub const P_REMOVE: &str = "remove "; // needs ' from '
pub const P_REMOVE_FROM: &str = " from ";
pub const P_APPEND: &str = "append "; // needs ' to '
pub const P_APPEND_TO: &str = " to ";
pub const P_INSERT: &str = "insert "; // needs ' at ' and ' in '
pub const P_INSERT_AT: &str = " at ";
pub const P_INSERT_IN: &str = " in ";

// File I/O operation phrases
pub const P_READ_FILE: &str = "read file at ";
pub const P_WRITE_FILE: &str = "write "; // needs ' into file at '
pub const P_WRITE_TO_FILE: &str = " into file at ";
pub const P_APPEND_FILE: &str = "append "; // needs ' into file at '
pub const P_APPEND_TO_FILE: &str = " into file at ";
pub const P_FILE_EXISTS: &str = "file exists at ";
pub const P_DELETE_FILE: &str = "delete file at ";
pub const P_CREATE_DIR: &str = "create directory at ";
pub const P_LIST_DIR: &str = "list files in ";
pub const P_LIST_DIR_ALT: &str = "list files in directory at ";
pub const P_READ_LINES: &str = "read lines from file at ";
pub const P_READ_LINES_ALT: &str = "read lines from ";
pub const P_COPY_FILE: &str = "copy file from "; // needs ' to '
pub const P_COPY_TO: &str = " to ";
pub const P_MOVE_FILE: &str = "move file from "; // needs ' to '

// JSON operation phrases
pub const P_PARSE_JSON: &str = "parse json from ";
pub const P_TO_JSON: &str = "convert to json ";
pub const P_JSON_PRETTY: &str = "convert to pretty json ";
pub const P_JSON_GET: &str = "get "; // needs ' from json '
pub const P_JSON_FROM: &str = " from json ";
pub const P_JSON_SET: &str = "set "; // needs ' in json ' and ' to '
pub const P_JSON_IN: &str = " in json ";
pub const P_JSON_TO: &str = " to ";
pub const P_NEW_JSON_OBJECT: &str = "new json object";
pub const P_NEW_JSON_ARRAY: &str = "new json array";
pub const P_JSON_PUSH: &str = "push "; // needs ' to json '
pub const P_JSON_PUSH_TO: &str = " to json ";
pub const P_JSON_LENGTH: &str = "json length of ";

// Error handling phrases
pub const P_TRY: &str = "try this:";
pub const P_IF_ERROR: &str = "if error";
pub const P_OF_TYPE: &str = " of type ";
pub const P_AS: &str = " as ";
pub const P_FINALLY: &str = "finally:";
pub const P_END_TRY: &str = "end try";
pub const P_THROW: &str = "throw ";
pub const P_ERROR: &str = "error";
pub const P_ERROR_MESSAGE: &str = "error message of ";
pub const P_ERROR_TYPE: &str = "error type of ";
pub const P_WITH_MESSAGE: &str = " with message ";

// Web server phrases
pub const P_GET_PATH_PARAM: &str = "get path parameter ";

pub fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    if s.len() < prefix.len() {
        return None;
    }
    let (head, tail) = s.split_at(prefix.len());
    if head.eq_ignore_ascii_case(prefix) {
        Some(tail)
    } else {
        None
    }
}
