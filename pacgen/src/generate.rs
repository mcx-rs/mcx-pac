mod linker_script;
mod pac;
mod peripheral;

pub use linker_script::generate_linker_script;
pub use pac::generate_pac;
pub use peripheral::generate_peripheral;
