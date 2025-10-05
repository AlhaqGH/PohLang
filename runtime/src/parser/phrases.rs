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
