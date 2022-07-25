use crate::error::prelude::*;
use crate::config::load_config;
use crate::application::Application;

pub async fn build_application() -> SResult<Application> {
    let config = load_config()?; 
    Application::build(config).await
}
