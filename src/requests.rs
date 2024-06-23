use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: Option<String>,
    pub user: UserResponse,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct Schedule {
    pub ScheduleDate: String,
    pub ScheduleTime: String,
    pub ArrivalTime: String,
    pub CarrierCode: String,
    pub ContactEmail: String,
    pub DoorNumber: String,
    pub IsHot: bool,
    pub LastFreeDate: String,
    pub LoadStatus: String,
    pub RequestDate: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
pub struct TrailerResponse {
    pub TrailerID: String,
    pub Schedule: Schedule,
    pub CiscoIDs: Vec<String>,
}