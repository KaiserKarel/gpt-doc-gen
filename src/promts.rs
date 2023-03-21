pub const RS_PROMPT: &str = "Document and format this Rust code according to Rust standards. Make sure to link to different crate items using [`Item`](path::to::Item) syntax. Ensure you use proper punctiation. For all public functions, add example code showing the usage of that function. Ensure every struct, trait, and function has a doc comment. For private items, do not provide examples. Make sure to add # SAFETY sections for unsafe code describing the invariants, and # Errors sections describing when the function returns an error for functions returning Results. Add a # Panics section for functions that panic, describing when they panic. Only add comments, no new code.";