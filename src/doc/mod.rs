use utoipa::OpenApi;
use crate::server::*;
/// Kore HTTP
///
/// This API provides interaction with Kore Ledger nodes using the HTTP protocol.
/// It allows sending and retrieving various types of requests and managing subjects.
/// The API is documented with OpenAPI for easy integration and use.
/// 
/// # Configuration
/// 
/// This client uses a single configuration variable, which is set through an environment variable.
/// Ensure that the environment variable is properly configured before using this API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Kore HTTP",
        description = "This API provides interaction with Kore Ledger nodes using the HTTP protocol. It allows sending and retrieving various types of requests and managing subjects. The API is documented with OpenAPI for easy integration and use.",
        version = "0.1.0",
        contact(
            name = "Kore Information",
            url = "https://www.kore-ledger.net/",
            email = "info@kore-ledger.net"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    paths(
        send_event_request,
        get_request_state,
        get_approval,
        patch_approval,
        put_auth,
        get_all_auth_subjects,
        get_witnesses_subject,
        delete_auth_subject,
        update_subject,
        get_all_govs,
        get_all_subjects,
        get_events,
        get_state,
        get_signatures,
        get_controller_id,
        get_peer_id,
        get_first_or_end_events,
        get_event_sn
    ),
    tags(
        (name = "Auth", description = "Endpoints related to authorization."),
        (name = "Update", description = "Endpoints related to Update."),
        (name = "Governances", description = "Endpoints related to Governances."),
        (name = "States", description = "Endpoints related to States."),
        (name = "Events", description = "Endpoints related to Events."),
        (name = "Approvals", description = "Endpoints related to request approvals."),
        (name = "Requests", description = "Endpoints for managing event requests."),
        (name = "Subjects", description = "Endpoints for managing subjects and their data."),
        (name = "Others", description = "Miscellaneous endpoints for node identification and configuration."),
    )
)]
pub struct ApiDoc;