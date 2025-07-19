pub mod command_page;
pub mod page_add;
pub mod page_check;
pub mod page_move;
pub mod page_playlist;
pub mod page_remove;

pub use command_page::{CommandPage, PageType};
pub use page_add::PageAdd;
pub use page_check::PageCheck;
pub use page_move::PageMove;
pub use page_playlist::PagePlaylist;
pub use page_remove::PageRemove;
