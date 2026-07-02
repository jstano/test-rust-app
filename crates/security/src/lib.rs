use serde::{Deserialize, Serialize};
use ss::{Claims, SecurityContext};
use stano_security as ss;
use tokio::task_local;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppClaims {
    pub email: String,
    pub role: Role,
}

pub type AppSecurityContext = SecurityContext<AppClaims>;
pub type AppClaimsType = Claims<AppClaims>;

task_local! {
    pub static SECURITY_CONTEXT: AppSecurityContext;
}
