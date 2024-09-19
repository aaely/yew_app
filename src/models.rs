use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetArrivalTimeRequest {
    pub TrailerID: String,
    pub ArrivalTime: String,
}

#[derive(Serialize, Deserialize)]
pub struct ArrivalMessage {
    pub TrailerID: String,
    pub ArrivalTime: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct LoadInfoRequest {
    pub param: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodaysTrucksRequest {
    pub date: String,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct SidAndParts {
    pub Sid: String,
    pub Cisco: String,
    pub Part: String,
    pub Quantity: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Sids {
    pub TrailerID: String,
    pub Sids: Vec<SidAndParts>,
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

#[derive(Deserialize, Serialize)]
pub struct DateRangeTruckRequest {
    pub date1: String,
    pub date2: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Item {
    #[serde(rename = "Part")]
    pub part: String,
    #[serde(rename = "Part Name")]
    pub part_name: String,
    #[serde(rename = "Plant")]
    pub plant: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "Std Pk")]
    pub std_pk: u32,
    #[serde(rename = "Primary Length (IN)")]
    pub primary_length_in: Option<f64>,  // Made optional
    #[serde(rename = "Primary Width  (IN)")]
    pub primary_width_in: Option<f64>,   // Made optional
    #[serde(rename = "Primary Height (IN)")]
    pub primary_height_in: Option<f64>,  // Made optional
    #[serde(rename = "Primary Container Weight (LBS)")]
    pub primary_container_weight_lbs: Option<f64>,  // Made optional
    #[serde(rename = "Secondary Std Pk (Boxes/Pallet)")]
    pub secondary_std_pk: Option<u32>,   // Made optional
    #[serde(rename = "Secondary Length (IN)")]
    pub secondary_length_in: Option<f64>,  // Made optional
    #[serde(rename = "Secondary Width (IN)")]
    pub secondary_width_in: Option<f64>,   // Made optional
    #[serde(rename = "Secondary Height (IN)")]
    pub secondary_height_in: Option<f64>,  // Made optional
    #[serde(rename = "Secondary Container Weight (LBS)")]
    pub secondary_container_weight_lbs: Option<f64>,  // Made optional
    #[serde(rename = "Part Weight")]
    pub part_weight: Option<f64>,  // Made optional
    #[serde(rename = "UL L")]
    pub ul_l: Option<f64>,  // Made optional
    #[serde(rename = "UL W")]
    pub ul_w: Option<f64>,  // Made optional
    #[serde(rename = "UL H")]
    pub ul_h: Option<f64>,  // Made optional
    #[serde(rename = "Primary per Layer")]
    pub primary_per_layer: Option<u32>,  // Made optional
    #[serde(rename = "Layers per UL")]
    pub layers_per_ul: Option<u32>,  // Made optional
    #[serde(rename = "Pieces per Pallet")]
    pub pieces_per_pallet: Option<u32>,  // Made optional
    #[serde(rename = "Pallet Weight")]
    pub pallet_weight: Option<f64>,  // Made optional
    #[serde(rename = "2-20 Week Avg Releases")]
    pub avg_releases_2_20_week: Option<f64>,  // Made optional
    #[serde(rename = "ESTIMATED STOCK LEVEL")]
    pub estimated_stock_level: String,  // Made optional
    #[serde(rename = "Pallets Weekly")]
    pub pallets_weekly: Option<f64>,  // Made optional
    #[serde(rename = "Min Pallets In Stock")]
    pub min_pallets_in_stock: Option<f64>,  // Made optional
    #[serde(rename = "Average Pallets In Stock")]
    pub avg_pallets_in_stock: Option<f64>,  // Made optional
    #[serde(rename = "Max Pallets In Stock")]
    pub max_pallets_in_stock: Option<f64>,  // Made optional
    #[serde(rename = "Rack or Floor?")]
    pub rack_or_floor: Option<String>,  // Made optional
    #[serde(rename = "Rack Pick Face Length or Width")]
    pub rack_pick_face_length_or_width: Option<f64>,  // Made optional
    #[serde(rename = "Pallets per 9' Rack Level")]
    pub pallets_per_9_rack_level: Option<u32>,  // Made optional
    #[serde(rename = "Rack levels")]
    pub rack_levels: Option<u32>,  // Made optional
    #[serde(rename = "Warehouse Stack")]
    pub warehouse_stack: Option<u32>,  // Made optional
    #[serde(rename = "Stacks in Floor Storage")]
    pub stacks_in_floor_storage: Option<u32>,  // Made optional
    #[serde(rename = "Floor Pick Face Width")]
    pub floor_pick_face_width: Option<f64>,  // Made optional
    #[serde(rename = "Storage Lane Width")]
    pub storage_lane_width: Option<f64>,  // Made optional
    #[serde(rename = "Storage Lane Length")]
    pub storage_lane_length: Option<f64>,  // Made optional
    #[serde(rename = "Stacks per Lane")]
    pub stacks_per_lane: Option<String>,  // Made optional, as this might not always be numeric
    #[serde(rename = "Num of Storage Lanes")]
    pub num_of_storage_lanes: Option<u32>,  // Made optional
    #[serde(rename = "Pallet depth dim")]
    pub pallet_depth_dim: Option<f64>,  // Made optional
    #[serde(rename = "Trailer Stack")]
    pub trailer_stack: Option<u32>,  // Made optional
    #[serde(rename = "UL per 53'")]
    pub ul_per_53: Option<u32>,  // Made optional
    #[serde(rename = "UL per 40'")]
    pub ul_per_40: Option<u32>,  // Made optional
    #[serde(rename = "Average IB 40' Weekly")]
    pub avg_ib_40_weekly: Option<f64>,  // Made optional
    #[serde(rename = "Average OB 53' Weekly")]
    pub avg_ob_53_weekly: Option<f64>,  // Made optional
    #[serde(rename = "area")]
    pub area: Option<u32>,  // Made optional
}

#[derive(Default, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RecentTrailers {
    pub trailer_id: String,
    pub date: String,
    pub time: String,
    pub scac: String,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct SetScheduleRequest {
    pub TrailerID: String,
    pub ScheduleDate: String,
    pub RequestDate: String,
    pub CarrierCode: String,
    pub ScheduleTime: String,
    pub LastFreeDate: String,
    pub ContactEmail: String,
    pub Door: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MyFormData {
    pub door: String,
    pub contact_email: String,
    pub schedule_date: String,
    pub schedule_time: String,
    pub last_free_date: String,
    pub scac: String,
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