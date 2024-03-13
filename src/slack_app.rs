use slack_morphism::prelude::*;
use std::sync::{Arc, Mutex};

pub async fn run_slack_socket_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(SlackClient::new(SlackClientHyperConnector::new()?));

    let socket_mode_callbacks = SlackSocketModeListenerCallbacks::new()
        .with_command_events(command_events_function)
        .with_interaction_events(interaction_events_function)
        .with_push_events(push_events_sm_function);

    let test_state = Arc::new(Mutex::new(0));
    let listener_environment = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(error_handler)
            .with_user_state(test_state.clone()),
    );

    let socket_mode_listener = SlackClientSocketModeListener::new(
        &SlackClientSocketModeConfig::new(),
        listener_environment.clone(),
        socket_mode_callbacks,
    );

    let app_token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_APP_TOKEN")?.into();
    let app_token: SlackApiToken = SlackApiToken::new(app_token_value);

    socket_mode_listener.listen_for(&app_token).await?;

    socket_mode_listener.serve().await;

    Ok(())
}

fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

fn error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> HttpStatusCode {
    println!("{:#?}", err);

    // This return value should be OK if we want to return successful ack to the Slack server using Web-sockets
    // https://api.slack.com/apis/connections/socket-implement#acknowledge
    // so that Slack knows whether to retry
    HttpStatusCode::OK
}

async fn command_events_function(
    event: SlackCommandEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<SlackCommandEventResponse, Box<dyn std::error::Error + Send + Sync>> {
    println!("{:#?}", event);

    Ok(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Testing the command event func".into()),
    ))
}

async fn push_events_sm_function(
    event: SlackPushEventCallback,
    _client: Arc<SlackHyperClient>,
    states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Push event: {:#?}", event);

    let storage = states.write().await;

    let app_state: &Arc<Mutex<i32>> = storage.get_user_state().unwrap();
    let app_state = app_state.to_owned();

    let mut data = app_state.lock().unwrap();

    *data += 1;
    println!("State now {:?}", data);

    Ok(())
}

async fn interaction_events_function(
    event: SlackInteractionEvent,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Interaction event: {:#?}", event);
    Ok(())
}
