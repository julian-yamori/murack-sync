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
use murack_core_data_file::{DapRepositoryImpl, FileLibraryRepositoryImpl};
use murack_core_domain::{
    check::CheckUsecaseImpl, dap::DapPlaylistUsecaseImpl, folder::FolderUsecaseImpl,
    sync::SyncUsecaseImpl, track::TrackUsecaseImpl,
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
        let file_library_repository = self.file_library_repository();
        let sync_usecase = self.sync_usecase();
        CommandAdd::new(
            args,
            &self.config,
            &self.cui,
            file_library_repository,
            sync_usecase,
        )
    }

    pub fn command_check(&self, args: CommandCheckArgs) -> TypeCommandCheck {
        let file_library_repository1 = self.file_library_repository();
        let file_library_repository2 = self.file_library_repository();
        let file_library_repository3 = self.file_library_repository();
        let file_library_repository4 = self.file_library_repository();
        let track_usecase = self.track_usecase();
        let sync_usecase = self.sync_usecase();
        let check_usecase1 = self.check_usecase();
        let check_usecase2 = self.check_usecase();
        let check_usecase3 = self.check_usecase();
        let db_track_sync_repository1 = self.db_registry.db_track_sync_repository();
        let db_track_sync_repository2 = self.db_registry.db_track_sync_repository();

        CommandCheck::new(
            args,
            &self.config,
            ResolveExistanceImpl::new(
                &self.config,
                &self.cui,
                file_library_repository1,
                track_usecase,
                sync_usecase,
                db_track_sync_repository1,
            ),
            ResolveDataMatchImpl::new(
                &self.config,
                &self.cui,
                file_library_repository2,
                check_usecase1,
                self.db_registry.db_artwork_repository(),
                db_track_sync_repository2,
            ),
            ResolveDapImpl::new(
                &self.config,
                &self.cui,
                file_library_repository3,
                check_usecase2,
            ),
            &self.cui,
            file_library_repository4,
            check_usecase3,
            self.db_registry.db_track_repository(),
        )
    }

    pub fn command_move(&self, args: CommandMoveArgs) -> TypeCommandMove {
        let file_library_repository = self.file_library_repository();
        let track_usecase = self.track_usecase();
        CommandMove::new(
            args,
            &self.config,
            file_library_repository,
            self.db_registry.db_track_repository(),
            self.db_registry.db_folder_repository(),
            track_usecase,
        )
    }

    pub fn command_remove(&self, args: CommandRemoveArgs) -> TypeCommandRemove {
        let track_usecase = self.track_usecase();
        CommandRemove::new(args, &self.config, &self.cui, track_usecase)
    }

    pub fn command_playlist(&self) -> TypeCommandPlaylist {
        let dap_playlist_usecase = self.dap_playlist_usecase();
        CommandPlaylist::new(&self.config, &self.cui, dap_playlist_usecase)
    }

    // -----------------------------
    // Domain Services

    fn check_usecase(&self) -> TypeCheckUsecase {
        CheckUsecaseImpl::new(
            self.db_registry.db_track_sync_repository(),
            self.file_library_repository(),
        )
    }

    fn dap_playlist_usecase(&self) -> TypeDapPlaylistUsecase {
        DapPlaylistUsecaseImpl::new(
            self.dap_repository(),
            self.db_registry.db_playlist_repository(),
            self.db_registry.track_finder(),
        )
    }

    fn folder_usecase(&self) -> TypeFolderUsecase {
        FolderUsecaseImpl::new(
            self.db_registry.db_folder_repository(),
            self.db_registry.db_track_repository(),
        )
    }

    fn track_usecase(&self) -> TypeTrackUsecase {
        TrackUsecaseImpl::new(
            self.file_library_repository(),
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

    // -----------------------------
    // Repositories

    fn file_library_repository(&self) -> FileLibraryRepositoryImpl {
        FileLibraryRepositoryImpl {}
    }

    fn dap_repository(&self) -> DapRepositoryImpl {
        DapRepositoryImpl {}
    }
}

pub type TypeCommandAdd<'config, 'cui> =
    CommandAdd<'config, 'cui, EguiCui, FileLibraryRepositoryImpl, TypeSyncUsecase>;
pub type TypeCommandCheck<'config, 'cui> = CommandCheck<
    'config,
    'cui,
    EguiCui,
    ResolveExistanceImpl<
        'config,
        'cui,
        EguiCui,
        FileLibraryRepositoryImpl,
        TypeTrackUsecase,
        TypeSyncUsecase,
        TypeDbTrackSyncRepository,
    >,
    ResolveDataMatchImpl<
        'config,
        'cui,
        EguiCui,
        FileLibraryRepositoryImpl,
        TypeCheckUsecase,
        TypeDbArtworkRepository,
        TypeDbTrackSyncRepository,
    >,
    ResolveDapImpl<'config, 'cui, EguiCui, FileLibraryRepositoryImpl, TypeCheckUsecase>,
    FileLibraryRepositoryImpl,
    TypeCheckUsecase,
    TypeDbTrackRepository,
>;
pub type TypeCommandMove<'config> = CommandMove<
    'config,
    FileLibraryRepositoryImpl,
    TypeDbTrackRepository,
    TypeDbFolderRepository,
    TypeTrackUsecase,
>;
pub type TypeCommandRemove<'config, 'cui> = CommandRemove<'config, 'cui, EguiCui, TypeTrackUsecase>;
pub type TypeCommandPlaylist<'config, 'cui> =
    CommandPlaylist<'config, 'cui, EguiCui, TypeDapPlaylistUsecase>;

type TypeCheckUsecase = CheckUsecaseImpl<TypeDbTrackSyncRepository, FileLibraryRepositoryImpl>;
type TypeDapPlaylistUsecase =
    DapPlaylistUsecaseImpl<DapRepositoryImpl, TypeDbPlaylistRepository, TypeTrackFinder>;
type TypeFolderUsecase = FolderUsecaseImpl<TypeDbFolderRepository, TypeDbTrackRepository>;
type TypeTrackUsecase = TrackUsecaseImpl<
    FileLibraryRepositoryImpl,
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
