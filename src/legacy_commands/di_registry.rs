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
use murack_core_data_db::db_components::{
    DbComponents, TypeDbArtworkRepository, TypeDbFolderRepository, TypeDbPlaylistRepository,
    TypeDbPlaylistTrackRepository, TypeDbTrackRepository, TypeDbTrackSyncRepository,
    TypeDbTrackTagRepository, TypeTrackFinder,
};
use murack_core_domain::{
    folder::FolderUsecaseImpl, sync::SyncUsecaseImpl, track::TrackUsecaseImpl,
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
    db_registry: DbComponents,
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
            db_registry: DbComponents::new(),
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
        CommandAdd::new(args, &self.config, &self.cui, self.sync_usecase())
    }

    pub fn command_check(&self, args: CommandCheckArgs) -> TypeCommandCheck {
        CommandCheck::new(
            args,
            &self.config,
            ResolveExistanceImpl::new(
                &self.config,
                &self.cui,
                self.track_usecase(),
                self.sync_usecase(),
                self.db_registry.db_track_sync_repository(),
            ),
            ResolveDataMatchImpl::new(
                &self.config,
                &self.cui,
                self.db_registry.db_artwork_repository(),
                self.db_registry.db_track_sync_repository(),
            ),
            ResolveDapImpl::new(&self.config, &self.cui),
            &self.cui,
            self.db_registry.db_track_repository(),
            self.db_registry.db_track_sync_repository(),
        )
    }

    pub fn command_move(&self, args: CommandMoveArgs) -> TypeCommandMove {
        CommandMove::new(
            args,
            &self.config,
            self.db_registry.db_track_repository(),
            self.db_registry.db_folder_repository(),
            self.track_usecase(),
        )
    }

    pub fn command_remove(&self, args: CommandRemoveArgs) -> TypeCommandRemove {
        let track_usecase = self.track_usecase();
        CommandRemove::new(args, &self.config, &self.cui, track_usecase)
    }

    pub fn command_playlist(&self) -> TypeCommandPlaylist {
        CommandPlaylist {
            config: &self.config,
            cui: &self.cui,
            db_playlist_repository: self.db_registry.db_playlist_repository(),
            track_finder: self.db_registry.track_finder(),
        }
    }

    // -----------------------------
    // Domain Services

    fn folder_usecase(&self) -> TypeFolderUsecase {
        FolderUsecaseImpl::new(
            self.db_registry.db_folder_repository(),
            self.db_registry.db_track_repository(),
        )
    }

    fn track_usecase(&self) -> TypeTrackUsecase {
        TrackUsecaseImpl::new(
            self.db_registry.db_artwork_repository(),
            self.db_registry.db_folder_repository(),
            self.db_registry.db_playlist_repository(),
            self.db_registry.db_playlist_track_repository(),
            self.db_registry.db_track_repository(),
            self.db_registry.db_track_tag_repository(),
            self.folder_usecase(),
        )
    }

    fn sync_usecase(&self) -> TypeSyncUsecase {
        SyncUsecaseImpl::new(
            self.db_registry.db_folder_repository(),
            self.db_registry.db_playlist_repository(),
            self.db_registry.db_track_sync_repository(),
        )
    }
}

pub type TypeCommandAdd<'config, 'cui> = CommandAdd<'config, 'cui, EguiCui, TypeSyncUsecase>;
pub type TypeCommandCheck<'config, 'cui> = CommandCheck<
    'config,
    'cui,
    EguiCui,
    ResolveExistanceImpl<
        'config,
        'cui,
        EguiCui,
        TypeTrackUsecase,
        TypeSyncUsecase,
        TypeDbTrackSyncRepository,
    >,
    ResolveDataMatchImpl<
        'config,
        'cui,
        EguiCui,
        TypeDbArtworkRepository,
        TypeDbTrackSyncRepository,
    >,
    ResolveDapImpl<'config, 'cui, EguiCui>,
    TypeDbTrackRepository,
    TypeDbTrackSyncRepository,
>;
pub type TypeCommandMove<'config> =
    CommandMove<'config, TypeDbTrackRepository, TypeDbFolderRepository, TypeTrackUsecase>;
pub type TypeCommandRemove<'config, 'cui> = CommandRemove<'config, 'cui, EguiCui, TypeTrackUsecase>;
pub type TypeCommandPlaylist<'config, 'cui> =
    CommandPlaylist<'config, 'cui, EguiCui, TypeDbPlaylistRepository, TypeTrackFinder>;

type TypeFolderUsecase = FolderUsecaseImpl<TypeDbFolderRepository, TypeDbTrackRepository>;
type TypeTrackUsecase = TrackUsecaseImpl<
    TypeDbArtworkRepository,
    TypeDbFolderRepository,
    TypeDbPlaylistRepository,
    TypeDbPlaylistTrackRepository,
    TypeDbTrackRepository,
    TypeDbTrackTagRepository,
    TypeFolderUsecase,
>;
type TypeSyncUsecase =
    SyncUsecaseImpl<TypeDbFolderRepository, TypeDbPlaylistRepository, TypeDbTrackSyncRepository>;
