use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoadInfoRequest {
    pub param: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UserResponse {
    pub username: String,
    pub role: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct SidParts {
    pub Sid: Sid,
    pub Parts: Vec<Part>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Sid {
    pub CiscoID: String,
    pub id: String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Part {
    pub partNumber: String,
    pub quantity: i32,
}



#[derive(Deserialize, Clone, Debug)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: Option<String>,
    pub user: UserResponse,
}

#[derive(Deserialize, Clone, Debug)]
pub struct TrailerSchedule {
    pub TrailerID: String,
    pub Schedule: Schedule,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HotTrailerRequest {
    pub TrailerID: String,
}