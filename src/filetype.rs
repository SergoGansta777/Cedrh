pub struct FileType {
    name: String,
    hl_opts: HighlightingOptions,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Default)]
pub struct HighlightingOptions {
    numbers: bool,
    strings: bool,
    characters: bool,
    comments: bool,
    multiline_comments: bool,
    primary_keywords: Vec<String>,
    secondary_keywords: Vec<String>,
    operators: Vec<String>,
    brackets: Vec<String>,
}

impl Default for FileType {
    fn default() -> Self {
        Self {
            name: String::from("No fyletype"),
            hl_opts: HighlightingOptions::default(),
        }
    }
}

impl HighlightingOptions {
    #[must_use]
    pub fn numbers(&self) -> bool {
        self.numbers
    }

    #[must_use]
    pub fn strings(&self) -> bool {
        self.strings
    }

    #[must_use]
    pub fn characters(&self) -> bool {
        self.characters
    }

    #[must_use]
    pub fn comments(&self) -> bool {
        self.comments
    }

    #[must_use]
    pub fn multiline_comments(&self) -> bool {
        self.multiline_comments
    }

    #[must_use]
    pub fn primary_keywords(&self) -> &Vec<String> {
        &self.primary_keywords
    }

    #[must_use]
    pub fn secondary_keywords(&self) -> &Vec<String> {
        &self.secondary_keywords
    }

    #[must_use]
    pub fn operators(&self) -> &Vec<String> {
        &self.operators
    }

    #[must_use]
    pub fn brackets(&self) -> &Vec<String> {
        &self.brackets
    }
}

impl FileType {
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn from(file_name: &str) -> Self {
        match file_name.rsplit('.').next() {
            Some("rs") => Self::get_rust_filetype(),
            Some("cpp" | "h") => Self::get_cpp_filetype(),
            Some("py") => Self::get_python_filetype(),
            Some("cs") => Self::get_csharp_filetype(),
            _ => Self::default(),
        }
    }

    #[allow(clippy::too_many_lines)]
    fn get_rust_filetype() -> FileType {
        FileType {
            name: String::from("Rust"),
            hl_opts: HighlightingOptions {
                numbers: true,
                strings: true,
                characters: true,
                comments: true,
                multiline_comments: true,
                primary_keywords: vec![
                    "as".to_owned(),
                    "break".to_owned(),
                    "const".to_owned(),
                    "continue".to_owned(),
                    "crate".to_owned(),
                    "else".to_owned(),
                    "enum".to_owned(),
                    "extern".to_owned(),
                    "false".to_owned(),
                    "fn".to_owned(),
                    "for".to_owned(),
                    "if".to_owned(),
                    "impl".to_owned(),
                    "in".to_owned(),
                    "let".to_owned(),
                    "loop".to_owned(),
                    "match".to_owned(),
                    "mod".to_owned(),
                    "move".to_owned(),
                    "mut".to_owned(),
                    "pub".to_owned(),
                    "ref".to_owned(),
                    "return".to_owned(),
                    "self".to_owned(),
                    "Self".to_owned(),
                    "static".to_owned(),
                    "struct".to_owned(),
                    "super".to_owned(),
                    "trait".to_owned(),
                    "true".to_owned(),
                    "type".to_owned(),
                    "unsafe".to_owned(),
                    "use".to_owned(),
                    "where".to_owned(),
                    "while".to_owned(),
                    "dyn".to_owned(),
                    "abstract".to_owned(),
                    "become".to_owned(),
                    "box".to_owned(),
                    "do".to_owned(),
                    "final".to_owned(),
                    "macro".to_owned(),
                    "override".to_owned(),
                    "priv".to_owned(),
                    "typeof".to_owned(),
                    "unsized".to_owned(),
                    "virtual".to_owned(),
                    "yield".to_owned(),
                    "async".to_owned(),
                    "await".to_owned(),
                    "try".to_owned(),
                    "Ok".to_owned(),
                    "Some".to_owned(),
                    "Err".to_owned(),
                ],
                secondary_keywords: vec![
                    "bool".to_owned(),
                    "char".to_owned(),
                    "i8".to_owned(),
                    "i16".to_owned(),
                    "i32".to_owned(),
                    "i64".to_owned(),
                    "isize".to_owned(),
                    "u8".to_owned(),
                    "u16".to_owned(),
                    "u32".to_owned(),
                    "u64".to_owned(),
                    "usize".to_owned(),
                    "f32".to_owned(),
                    "f64".to_owned(),
                    "str".to_owned(),
                    "String".to_owned(),
                    "Vec".to_owned(),
                    "Box".to_owned(),
                    "Option".to_owned(),
                    "Result".to_owned(),
                    "HashMap".to_owned(),
                    "HashSet".to_owned(),
                    "BTreeMap".to_owned(),
                    "BTreeSet".to_owned(),
                    "RefCell".to_owned(),
                    "Rc".to_owned(),
                    "Arc".to_owned(),
                    "Cell".to_owned(),
                    "Ref".to_owned(),
                    "Mutex".to_owned(),
                    "RwLock".to_owned(),
                ],
                operators: vec![
                    "+".to_owned(),
                    "-".to_owned(),
                    "*".to_owned(),
                    "/".to_owned(),
                    "%".to_owned(),
                    "&".to_owned(),
                    "|".to_owned(),
                    ":".to_owned(),
                    "!".to_owned(),
                    "^".to_owned(),
                    "~".to_owned(),
                    "<<".to_owned(),
                    ">>".to_owned(),
                    "&&".to_owned(),
                    "||".to_owned(),
                    "<".to_owned(),
                    ">".to_owned(),
                    "<=".to_owned(),
                    ">=".to_owned(),
                    "==".to_owned(),
                    "!=".to_owned(),
                    "=".to_owned(),
                    ".".to_owned(),
                    "+=".to_owned(),
                    "-=".to_owned(),
                    "*=".to_owned(),
                    "/=".to_owned(),
                    "%=".to_owned(),
                    "&=".to_owned(),
                    "|=".to_owned(),
                    "^=".to_owned(),
                    "<<=".to_owned(),
                    ">>=".to_owned(),
                    "&&=".to_owned(),
                    "||=".to_owned(),
                    "=>".to_owned(),
                    "::".to_owned(),
                ],
                brackets: vec![
                    "(".to_owned(),
                    ")".to_owned(),
                    "[".to_owned(),
                    "]".to_owned(),
                    "{".to_owned(),
                    "}".to_owned(),
                    "<".to_owned(),
                    ">".to_owned(),
                    "#".to_owned(),
                ],
            },
        }
    }

    #[allow(clippy::too_many_lines)]
    fn get_cpp_filetype() -> FileType {
        FileType {
            name: String::from("C++"),
            hl_opts: HighlightingOptions {
                numbers: true,
                strings: true,
                characters: true,
                comments: true,
                multiline_comments: true,
                primary_keywords: vec![
                    "alignas",
                    "alignof",
                    "and",
                    "and_eq",
                    "asm",
                    "atomic_cancel",
                    "atomic_commit",
                    "atomic_noexcept",
                    "auto",
                    "bitand",
                    "bitor",
                    "break",
                    "case",
                    "catch",
                    "compl",
                    "concept",
                    "const",
                    "consteval",
                    "constexpr",
                    "constinit",
                    "const_cast",
                    "continue",
                    "co_await",
                    "co_return",
                    "co_yield",
                    "decltype",
                    "default",
                    "delete",
                    "do",
                    "dynamic_cast",
                    "else",
                    "explicit",
                    "export",
                    "extern",
                    "false",
                    "for",
                    "friend",
                    "goto",
                    "if",
                    "inline",
                    "mutable",
                    "namespace",
                    "new",
                    "noexcept",
                    "not",
                    "not_eq",
                    "nullptr",
                    "operator",
                    "or",
                    "or_eq",
                    "private",
                    "protected",
                    "public",
                    "reflexpr",
                    "register",
                    "reinterpret_cast",
                    "requires",
                    "return",
                    "signed",
                    "sizeof",
                    "static",
                    "static_assert",
                    "static_cast",
                    "switch",
                    "synchronized",
                    "template",
                    "this",
                    "thread_local",
                    "throw",
                    "true",
                    "try",
                    "typedef",
                    "typeid",
                    "typename",
                    "union",
                    "unsigned",
                    "using",
                    "virtual",
                    "volatile",
                    "while",
                    "xor",
                    "xor_eq",
                    "class",
                    "struct",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                secondary_keywords: vec![
                    "bool",
                    "char",
                    "char8_t",
                    "char16_t",
                    "char32_t",
                    "double",
                    "enum",
                    "float",
                    "int",
                    "long",
                    "short",
                    "void",
                    "wchar_t",
                    "#include",
                    "template",
                    "map",
                    "set",
                    "stack",
                    "unordered_map",
                    "unordered_set",
                    "queue",
                    "priority_queue",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                operators: vec![
                    "+", "-", "*", "/", "%", "^", "&", "|", "~", "!", "=", "<", ">", "+=", "-=",
                    "*=", "/=", "%=", "^=", "&=", "|=", "<<", ">>", ">>=", "<<=", "==", "!=", "<=",
                    ">=", "&&", "||", "++", "--", ",", "->*", "->", ";", ":", "?", "::",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                brackets: ["{", "}", "(", ")", "[", "]"]
                    .iter()
                    .map(|st| (*st).to_owned())
                    .collect(),
            },
        }
    }

    fn get_python_filetype() -> FileType {
        FileType {
            name: String::from("Python"),
            hl_opts: HighlightingOptions {
                numbers: true,
                strings: true,
                characters: true,
                comments: true,
                multiline_comments: false, // Python uses triple quotes for multi-line strings, which can serve as multi-line comments
                primary_keywords: vec![
                    "False", "None", "True", "and", "as", "assert", "async", "await", "break",
                    "class", "continue", "def", "del", "elif", "else", "except", "finally", "for",
                    "from", "global", "if", "import", "in", "is", "lambda", "nonlocal", "not",
                    "or", "pass", "raise", "return", "try", "while", "with", "yield",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                secondary_keywords: [
                    "print", "len", "range", "int", "float", "str", "input", "open", "file", "os",
                    "sys",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                operators: vec![
                    "+", "-", "*", "**", "/", "//", "%", "@", "<<", ">>", "&", "|", "^", "~", "<",
                    ">", "<=", ">=", "==", "!=",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                brackets: ["(", ")", "[", "]", "{", "}"]
                    .iter()
                    .map(|st| (*st).to_owned())
                    .collect(),
            },
        }
    }

    #[allow(clippy::too_many_lines)]
    fn get_csharp_filetype() -> FileType {
        FileType {
            name: String::from("C#"),
            hl_opts: HighlightingOptions {
                numbers: true,
                strings: true,
                characters: true,
                comments: true,
                multiline_comments: true,
                primary_keywords: vec![
                    "abstract",
                    "as",
                    "base",
                    "bool",
                    "break",
                    "byte",
                    "case",
                    "catch",
                    "char",
                    "checked",
                    "class",
                    "const",
                    "continue",
                    "default",
                    "delegate",
                    "do",
                    "else",
                    "enum",
                    "event",
                    "explicit",
                    "extern",
                    "false",
                    "finally",
                    "fixed",
                    "for",
                    "foreach",
                    "goto",
                    "if",
                    "implicit",
                    "in",
                    "interface",
                    "internal",
                    "is",
                    "lock",
                    "long",
                    "namespace",
                    "new",
                    "null",
                    "object",
                    "operator",
                    "out",
                    "override",
                    "params",
                    "private",
                    "protected",
                    "public",
                    "readonly",
                    "ref",
                    "return",
                    "sealed",
                    "short",
                    "sizeof",
                    "stackalloc",
                    "static",
                    "string",
                    "struct",
                    "switch",
                    "this",
                    "throw",
                    "true",
                    "try",
                    "typeof",
                    "unchecked",
                    "unsafe",
                    "ushort",
                    "using",
                    "virtual",
                    "void",
                    "volatile",
                    "while",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                secondary_keywords: vec![
                    "Console",
                    "Math",
                    "Array",
                    "uint",
                    "ulong",
                    "decimal",
                    "float",
                    "List",
                    "sbyte",
                    "Dictionary",
                    "StringBuilder",
                    "System",
                    "double",
                    "int",
                    "IO",
                    "Linq",
                    "Text",
                    "Threading",
                    "Tasks",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                operators: vec![
                    "+", "-", "*", "/", "%", "&", "|", "^", "!", "~", "&&", "||", "++", "--", "==",
                    "!=", ">", "<", ">=", "<=", "<<", ">>", "=>", "+=", "-=", "*=", "/=", "%=",
                    "&=", "|=", "^=", "<<=", ">>=", "??", "?", ":", "::", ";", ".", ",",
                ]
                .iter()
                .map(|st| (*st).to_owned())
                .collect(),
                brackets: ["{", "}", "(", ")", "[", "]"]
                    .iter()
                    .map(|st| (*st).to_owned())
                    .collect(),
            },
        }
    }

    #[must_use]
    pub fn highlighting_options(&self) -> &HighlightingOptions {
        &self.hl_opts
    }
}
