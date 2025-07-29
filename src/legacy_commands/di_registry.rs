use std::sync::Arc;

use eframe::egui::mutex::Mutex;
use murack_core_app::{
    Config,
    command::{
        CommandAdd, CommandAddArgs, CommandCheck, CommandCheckArgs, CommandMove, CommandMoveArgs,
        CommandPlaylist, CommandRemove, CommandRemoveArgs, ResolveDapImpl, ResolveDataMatchImpl,
        ResolveExistanceImpl,
    },
};
use sqlx::PgPool;

use crate::legacy_commands::{
    console::Console,
    egui_cui::{CommandState, EguiCui},
};

/// DI の依存関係の解決
pub struct DIRegistry {
    cui: EguiCui,
    config: Arc<Config>,
    db_pool: Arc<PgPool>,
    #[allow(dead_code)]
    console: Arc<Mutex<Console>>,
}

impl DIRegistry {
    pub fn new(
        console: Arc<Mutex<Console>>,
        command_state: Arc<Mutex<CommandState>>,
        config: Arc<Config>,
        db_pool: Arc<PgPool>,
    ) -> Self {
        Self {
            cui: EguiCui::new(console.clone(), command_state),
            config,
            db_pool,
            console,
        }
    }

    pub fn db_pool(&self) -> Arc<PgPool> {
        self.db_pool.clone()
    }

    #[allow(dead_code)]
    pub fn console(&self) -> Arc<Mutex<Console>> {
        self.console.clone()
    }

    // -----------------------------
    // Commands

    pub fn command_add(&self, args: CommandAddArgs) -> TypeCommandAdd {
        CommandAdd::new(args, &self.config, &self.cui)
    }

    pub fn command_check(&self, args: CommandCheckArgs) -> TypeCommandCheck {
        CommandCheck::new(
            args,
            &self.config,
            ResolveExistanceImpl::new(&self.config, &self.cui),
            ResolveDataMatchImpl::new(&self.config, &self.cui),
            ResolveDapImpl::new(&self.config, &self.cui),
            &self.cui,
        )
    }

    pub fn command_move(&self, args: CommandMoveArgs) -> TypeCommandMove {
        CommandMove::new(args, &self.config)
    }

    pub fn command_remove(&self, args: CommandRemoveArgs) -> TypeCommandRemove {
        CommandRemove::new(args, &self.config, &self.cui)
    }

    pub fn command_playlist(&self) -> TypeCommandPlaylist {
        CommandPlaylist {
            config: &self.config,
            cui: &self.cui,
        }
    }
}

pub type TypeCommandAdd<'config, 'cui> = CommandAdd<'config, 'cui, EguiCui>;
pub type TypeCommandCheck<'config, 'cui> = CommandCheck<
    'config,
    'cui,
    EguiCui,
    ResolveExistanceImpl<'config, 'cui, EguiCui>,
    ResolveDataMatchImpl<'config, 'cui, EguiCui>,
    ResolveDapImpl<'config, 'cui, EguiCui>,
>;
pub type TypeCommandMove<'config> = CommandMove<'config>;
pub type TypeCommandRemove<'config, 'cui> = CommandRemove<'config, 'cui, EguiCui>;
pub type TypeCommandPlaylist<'config, 'cui> = CommandPlaylist<'config, 'cui, EguiCui>;
