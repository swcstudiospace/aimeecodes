use std::sync::Arc;

use aimee_app::{
    AgentRepository, CommandInfra, DirectoryReaderInfra, EnvironmentInfra, FileDirectoryInfra,
    FileInfoInfra, FileReaderInfra, FileRemoverInfra, FileWriterInfra, HttpInfra, KVStore,
    McpServerInfra, Services, StrategyFactory, UserInfra, WalkerInfra,
};
use aimee_domain::{
    ChatRepository, ConversationRepository, FuzzySearchRepository, ProviderRepository,
    SkillRepository, SnapshotRepository, TextPatchRepository, ValidationRepository,
    WorkspaceIndexRepository,
};

use crate::AimeeProviderAuthService;
use crate::agent_registry::AimeeAgentRegistryService;
use crate::app_config::AimeeAppConfigService;
use crate::attachment::AimeeChatRequest;
use crate::auth::AimeeAuthService;
use crate::command::CommandLoaderService as AimeeCommandLoaderService;
use crate::conversation::AimeeConversationService;
use crate::discovery::AimeeDiscoveryService;
use crate::fd::FdDefault;
use crate::instructions::AimeeCustomInstructionsService;
use crate::mcp::{AimeeMcpManager, AimeeMcpService};
use crate::policy::AimeePolicyService;
use crate::provider_service::AimeeProviderService;
use crate::template::AimeeTemplateService;
use crate::tool_services::{
    AimeeFetch, AimeeFollowup, AimeeFsPatch, AimeeFsRead, AimeeFsRemove, AimeeFsSearch,
    AimeeFsUndo, AimeeFsWrite, AimeeImageRead, AimeePlanCreate, AimeeShell, AimeeSkillFetch,
};

type McpService<F> = AimeeMcpService<AimeeMcpManager<F>, F, <F as McpServerInfra>::Client>;
type AuthService<F> = AimeeAuthService<F>;

/// AimeeApp is the main application container that implements the App trait.
/// It provides access to all core services required by the application.
///
/// Type Parameters:
/// - F: The infrastructure implementation that provides core services like
///   environment, file reading, vector indexing, and embedding.
/// - R: The repository implementation that provides data persistence
#[derive(Clone)]
pub struct AimeeServices<
    F: HttpInfra
        + EnvironmentInfra
        + McpServerInfra
        + WalkerInfra
        + SnapshotRepository
        + ConversationRepository
        + KVStore
        + ChatRepository
        + ProviderRepository
        + WorkspaceIndexRepository
        + AgentRepository
        + SkillRepository
        + ValidationRepository,
> {
    chat_service: Arc<AimeeProviderService<F>>,
    config_service: Arc<AimeeAppConfigService<F>>,
    conversation_service: Arc<AimeeConversationService<F>>,
    template_service: Arc<AimeeTemplateService<F>>,
    attachment_service: Arc<AimeeChatRequest<F>>,
    discovery_service: Arc<AimeeDiscoveryService<F>>,
    mcp_manager: Arc<AimeeMcpManager<F>>,
    file_create_service: Arc<AimeeFsWrite<F>>,
    plan_create_service: Arc<AimeePlanCreate<F>>,
    file_read_service: Arc<AimeeFsRead<F>>,
    image_read_service: Arc<AimeeImageRead<F>>,
    file_search_service: Arc<AimeeFsSearch<F>>,
    file_remove_service: Arc<AimeeFsRemove<F>>,
    file_patch_service: Arc<AimeeFsPatch<F>>,
    file_undo_service: Arc<AimeeFsUndo<F>>,
    shell_service: Arc<AimeeShell<F>>,
    fetch_service: Arc<AimeeFetch>,
    followup_service: Arc<AimeeFollowup<F>>,
    mcp_service: Arc<McpService<F>>,
    custom_instructions_service: Arc<AimeeCustomInstructionsService<F>>,
    auth_service: Arc<AuthService<F>>,
    agent_registry_service: Arc<AimeeAgentRegistryService<F>>,
    command_loader_service: Arc<AimeeCommandLoaderService<F>>,
    policy_service: AimeePolicyService<F>,
    provider_auth_service: AimeeProviderAuthService<F>,
    workspace_service: Arc<crate::context_engine::AimeeWorkspaceService<F, FdDefault<F>>>,
    skill_service: Arc<AimeeSkillFetch<F>>,
    infra: Arc<F>,
}

impl<
    F: McpServerInfra
        + EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + FileWriterInfra
        + FileInfoInfra
        + FileReaderInfra
        + HttpInfra
        + WalkerInfra
        + DirectoryReaderInfra
        + CommandInfra
        + UserInfra
        + SnapshotRepository
        + ConversationRepository
        + ChatRepository
        + ProviderRepository
        + KVStore
        + WorkspaceIndexRepository
        + AgentRepository
        + SkillRepository
        + ValidationRepository,
> AimeeServices<F>
{
    pub fn new(infra: Arc<F>) -> Self {
        let mcp_manager = Arc::new(AimeeMcpManager::new(infra.clone()));
        let mcp_service = Arc::new(AimeeMcpService::new(mcp_manager.clone(), infra.clone()));
        let template_service = Arc::new(AimeeTemplateService::new(infra.clone()));
        let attachment_service = Arc::new(AimeeChatRequest::new(infra.clone()));
        let suggestion_service = Arc::new(AimeeDiscoveryService::new(infra.clone()));
        let conversation_service = Arc::new(AimeeConversationService::new(infra.clone()));
        let auth_service = Arc::new(AimeeAuthService::new(infra.clone()));
        let chat_service = Arc::new(AimeeProviderService::new(infra.clone()));
        let config_service = Arc::new(AimeeAppConfigService::new(infra.clone()));
        let file_create_service = Arc::new(AimeeFsWrite::new(infra.clone()));
        let plan_create_service = Arc::new(AimeePlanCreate::new(infra.clone()));
        let file_read_service = Arc::new(AimeeFsRead::new(infra.clone()));
        let image_read_service = Arc::new(AimeeImageRead::new(infra.clone()));
        let file_search_service = Arc::new(AimeeFsSearch::new(infra.clone()));
        let file_remove_service = Arc::new(AimeeFsRemove::new(infra.clone()));
        let file_patch_service = Arc::new(AimeeFsPatch::new(infra.clone()));
        let file_undo_service = Arc::new(AimeeFsUndo::new(infra.clone()));
        let shell_service = Arc::new(AimeeShell::new(infra.clone()));
        let fetch_service = Arc::new(AimeeFetch::new());
        let followup_service = Arc::new(AimeeFollowup::new(infra.clone()));
        let custom_instructions_service =
            Arc::new(AimeeCustomInstructionsService::new(infra.clone()));
        let agent_registry_service = Arc::new(AimeeAgentRegistryService::new(infra.clone()));
        let command_loader_service = Arc::new(AimeeCommandLoaderService::new(infra.clone()));
        let policy_service = AimeePolicyService::new(infra.clone());
        let provider_auth_service = AimeeProviderAuthService::new(infra.clone());
        let discovery = Arc::new(FdDefault::new(infra.clone()));
        let workspace_service = Arc::new(crate::context_engine::AimeeWorkspaceService::new(
            infra.clone(),
            discovery,
        ));
        let skill_service = Arc::new(AimeeSkillFetch::new(infra.clone()));

        Self {
            conversation_service,
            attachment_service,
            template_service,
            discovery_service: suggestion_service,
            mcp_manager,
            file_create_service,
            plan_create_service,
            file_read_service,
            image_read_service,
            file_search_service,
            file_remove_service,
            file_patch_service,
            file_undo_service,
            shell_service,
            fetch_service,
            followup_service,
            mcp_service,
            custom_instructions_service,
            auth_service,
            config_service,
            agent_registry_service,
            command_loader_service,
            policy_service,
            provider_auth_service,
            workspace_service,
            skill_service,
            chat_service,
            infra,
        }
    }
}

impl<
    F: FileReaderInfra
        + FileWriterInfra
        + CommandInfra
        + UserInfra
        + McpServerInfra
        + FileRemoverInfra
        + FileInfoInfra
        + FileDirectoryInfra
        + EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + DirectoryReaderInfra
        + HttpInfra
        + WalkerInfra
        + Clone
        + SnapshotRepository
        + ConversationRepository
        + KVStore
        + ChatRepository
        + ProviderRepository
        + AgentRepository
        + SkillRepository
        + StrategyFactory
        + WorkspaceIndexRepository
        + ValidationRepository
        + FuzzySearchRepository
        + TextPatchRepository
        + Clone
        + 'static,
> Services for AimeeServices<F>
{
    type AppConfigService = AimeeAppConfigService<F>;
    type ConversationService = AimeeConversationService<F>;
    type TemplateService = AimeeTemplateService<F>;
    type ProviderAuthService = AimeeProviderAuthService<F>;

    fn provider_auth_service(&self) -> &Self::ProviderAuthService {
        &self.provider_auth_service
    }
    type AttachmentService = AimeeChatRequest<F>;
    type CustomInstructionsService = AimeeCustomInstructionsService<F>;
    type FileDiscoveryService = AimeeDiscoveryService<F>;
    type McpConfigManager = AimeeMcpManager<F>;
    type FsWriteService = AimeeFsWrite<F>;
    type PlanCreateService = AimeePlanCreate<F>;
    type FsPatchService = AimeeFsPatch<F>;
    type FsReadService = AimeeFsRead<F>;
    type ImageReadService = AimeeImageRead<F>;
    type FsRemoveService = AimeeFsRemove<F>;
    type FsSearchService = AimeeFsSearch<F>;
    type FollowUpService = AimeeFollowup<F>;
    type FsUndoService = AimeeFsUndo<F>;
    type NetFetchService = AimeeFetch;
    type ShellService = AimeeShell<F>;
    type McpService = McpService<F>;
    type AuthService = AuthService<F>;
    type AgentRegistry = AimeeAgentRegistryService<F>;
    type CommandLoaderService = AimeeCommandLoaderService<F>;
    type PolicyService = AimeePolicyService<F>;
    type ProviderService = AimeeProviderService<F>;
    type WorkspaceService = crate::context_engine::AimeeWorkspaceService<F, FdDefault<F>>;
    type SkillFetchService = AimeeSkillFetch<F>;

    fn config_service(&self) -> &Self::AppConfigService {
        &self.config_service
    }

    fn conversation_service(&self) -> &Self::ConversationService {
        &self.conversation_service
    }

    fn template_service(&self) -> &Self::TemplateService {
        &self.template_service
    }

    fn attachment_service(&self) -> &Self::AttachmentService {
        &self.attachment_service
    }

    fn custom_instructions_service(&self) -> &Self::CustomInstructionsService {
        &self.custom_instructions_service
    }

    fn file_discovery_service(&self) -> &Self::FileDiscoveryService {
        self.discovery_service.as_ref()
    }

    fn mcp_config_manager(&self) -> &Self::McpConfigManager {
        self.mcp_manager.as_ref()
    }

    fn fs_create_service(&self) -> &Self::FsWriteService {
        &self.file_create_service
    }

    fn plan_create_service(&self) -> &Self::PlanCreateService {
        &self.plan_create_service
    }

    fn fs_patch_service(&self) -> &Self::FsPatchService {
        &self.file_patch_service
    }

    fn fs_read_service(&self) -> &Self::FsReadService {
        &self.file_read_service
    }

    fn fs_remove_service(&self) -> &Self::FsRemoveService {
        &self.file_remove_service
    }

    fn fs_search_service(&self) -> &Self::FsSearchService {
        &self.file_search_service
    }

    fn follow_up_service(&self) -> &Self::FollowUpService {
        &self.followup_service
    }

    fn fs_undo_service(&self) -> &Self::FsUndoService {
        &self.file_undo_service
    }

    fn net_fetch_service(&self) -> &Self::NetFetchService {
        &self.fetch_service
    }

    fn shell_service(&self) -> &Self::ShellService {
        &self.shell_service
    }

    fn mcp_service(&self) -> &Self::McpService {
        &self.mcp_service
    }

    fn auth_service(&self) -> &Self::AuthService {
        self.auth_service.as_ref()
    }

    fn agent_registry(&self) -> &Self::AgentRegistry {
        &self.agent_registry_service
    }

    fn command_loader_service(&self) -> &Self::CommandLoaderService {
        &self.command_loader_service
    }

    fn policy_service(&self) -> &Self::PolicyService {
        &self.policy_service
    }

    fn workspace_service(&self) -> &Self::WorkspaceService {
        &self.workspace_service
    }

    fn image_read_service(&self) -> &Self::ImageReadService {
        &self.image_read_service
    }
    fn skill_fetch_service(&self) -> &Self::SkillFetchService {
        &self.skill_service
    }

    fn provider_service(&self) -> &Self::ProviderService {
        &self.chat_service
    }
}

impl<
    F: EnvironmentInfra<Config = aimee_config::AimeeConfig>
        + HttpInfra
        + McpServerInfra
        + WalkerInfra
        + SnapshotRepository
        + ConversationRepository
        + KVStore
        + ChatRepository
        + ProviderRepository
        + WorkspaceIndexRepository
        + AgentRepository
        + SkillRepository
        + ValidationRepository
        + Send
        + Sync,
> aimee_app::EnvironmentInfra for AimeeServices<F>
{
    type Config = aimee_config::AimeeConfig;

    fn get_environment(&self) -> aimee_domain::Environment {
        self.infra.get_environment()
    }

    fn get_config(&self) -> anyhow::Result<aimee_config::AimeeConfig> {
        self.infra.get_config()
    }

    fn update_environment(
        &self,
        ops: Vec<aimee_domain::ConfigOperation>,
    ) -> impl std::future::Future<Output = anyhow::Result<()>> + Send {
        self.infra.update_environment(ops)
    }

    fn get_env_var(&self, key: &str) -> Option<String> {
        self.infra.get_env_var(key)
    }

    fn get_env_vars(&self) -> std::collections::BTreeMap<String, String> {
        self.infra.get_env_vars()
    }
}
