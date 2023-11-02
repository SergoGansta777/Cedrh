mod buffer;
mod editor;
mod row;
mod terminal;

use editor::Editor;
pub use buffer::Buffer;
pub use editor::Position;
pub use row::Row;
pub use terminal::Terminal;

fn main() {
    Editor::default().run();
}
