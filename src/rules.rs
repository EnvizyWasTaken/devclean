pub const JUNK_DIRS: &[&str] = &[
    "node_modules",
    "target",
    "__pycache__",
];

pub const IGNORE_DIRS: &[&str] = &[
    ".cache",
    ".cargo",
    ".var",
    ".npm",
    ".vscode",

    // exclusions for root scanning
    "proc",
    "sys",
    "dev",
    "run",
    "tmp",

];