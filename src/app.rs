use crate::action::Action;
use crate::ui::get_items_for_view;
use crate::view::{NavigationStack, ViewType};
use orbit_core::{load_config, HiveClient, OrbitConfig};
use ratatui::widgets::ListState;

pub struct App {
    pub running: bool,
    pub selected_index: usize,
    pub list_state: ListState,
    pub navigation: NavigationStack,
    pub config: Option<OrbitConfig>,
    pub profiles: Vec<String>,
    pub error: Option<String>,
    pub selected_profile: Option<String>,
    pub projects: Vec<String>,
    pub targets: Vec<String>,
    pub services: Vec<String>,
}

impl App {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let (config, profiles, error) = match load_config() {
            Ok(cfg) => {
                let profile_names: Vec<String> = cfg.profiles.keys().cloned().collect();
                (Some(cfg), profile_names, None)
            }
            Err(e) => (
                None,
                Vec::new(),
                Some(format!("Error loading config: {}", e)),
            ),
        };

        Self {
            running: true,
            selected_index: 0,
            list_state,
            navigation: NavigationStack::new(),
            config,
            profiles,
            error,
            projects: Vec::new(),
            targets: Vec::new(),
            services: Vec::new(),
            selected_profile: None,
        }
    }

    pub fn reset_selection(&mut self) {
        self.selected_index = 0;
        self.list_state.select(Some(0));
    }

    pub async fn handle_select(&mut self) {
        let items = get_items_for_view(&self.navigation.current, self);
        let selected_item = items.get(self.selected_index).cloned().unwrap_or_default();

        match &self.navigation.current {
            ViewType::ProfileSelect => {
                self.selected_profile = Some(selected_item.clone());
                if let Err(e) = self.load_projects().await {
                    self.error = Some(e);
                    return;
                }
                self.navigation.push(ViewType::Projects);
                self.reset_selection();
            }
            ViewType::Projects => {
                let project = selected_item;
                if let Err(e) = self.load_targets(&project).await {
                    self.error = Some(e);
                    return;
                }
                self.navigation.push(ViewType::Targets { project });
                self.reset_selection();
            }
            ViewType::Targets { project } => {
                let project = project.clone();
                let target = selected_item;

                if let Err(e) = self.load_services(&project, &target).await {
                    self.error = Some(e);
                    return;
                }
                self.navigation.push(ViewType::Services {
                    project,
                    target,
                });
                self.reset_selection();
            }
            ViewType::Services { project, target } => {
                self.navigation.push(ViewType::Schema {
                    project: project.clone(),
                    target: target.clone(),
                    service: selected_item,
                });
                self.reset_selection();
            }
            ViewType::Schema { .. } => {}
        };
    }

    async fn load_projects(&mut self) -> Result<(), String> {
        let config = self.config.as_ref().ok_or("No config loaded")?;
        let client = HiveClient::new(config, None).map_err(|e| e.to_string())?;

        let org_data = client.list_projects().await.map_err(|e| e.to_string())?;

        self.projects = org_data
            .organization
            .projects
            .edges
            .iter()
            .filter_map(|edge| edge.node.slug.clone())
            .collect();

        self.error = None;
        Ok(())
    }

    async fn load_targets(&mut self, project: &str) -> Result<(), String> {
        let config = self.config.as_ref().ok_or("No config loaded")?;
        let client = HiveClient::new(config, None).map_err(|e| e.to_string())?;

        let project_data = client
            .targets_by_project(project)
            .await
            .map_err(|e| e.to_string())?;

        self.targets = project_data
            .project
            .project_targets
            .map(|t| t.edges.iter().map(|e| e.node.slug.clone()).collect())
            .unwrap_or_default();

        self.error = None;
        Ok(())
    }

    async fn load_services(&mut self, project: &str, target: &str) -> Result<(), String> {
        let config = self.config.as_ref().ok_or("No config loaded")?;
        let client = HiveClient::new(config, None).map_err(|e| e.to_string())?;

        let version_data = client
            .services_by_target(project, target)
            .await
            .map_err(|e| e.to_string())?;

        self.services = version_data
            .latest_version
            .map(|v| {
                v.schemas
                    .edges
                    .iter()
                    .map(|e| e.node.service.clone())
                    .collect()
            })
            .unwrap_or_default();

        self.error = None;
        Ok(())
    }

    pub async fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::NavigateUp => {
                self.selected_index = self.selected_index.saturating_sub(1);
                self.list_state.select(Some(self.selected_index));
            }
            Action::NavigateDown => {
                let max = get_items_for_view(&self.navigation.current, self)
                    .len()
                    .saturating_sub(1);
                if self.selected_index < max {
                    self.selected_index = self.selected_index.saturating_add(1);
                    self.list_state.select(Some(self.selected_index));
                }
            }
            Action::Select => {
                self.handle_select().await;
            }
            Action::Back => {
                if self.navigation.pop() {
                    self.reset_selection();
                }
            }
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
