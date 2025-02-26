use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct User {
    pub username: String,
    pub role: String,
    pub token: String,
    pub refresh_token: Option<String>,
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

#[derive(Serialize, Deserialize)]
pub struct Stat6Message {
    pub TailerID: String
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

#[derive(Serialize, Clone, Debug)]
pub struct ShipmentPickFinishRequest {
    pub LoadId: String,
    pub FinishTime: String,
}

#[derive(Serialize)]
pub struct PickStartRequest {
    pub StartTime: String,
    pub LoadId: String,
    pub Picker: String,
}

#[derive(Serialize)]
pub struct TrailerArrivalRequest {
    pub ArrivalTime: String,
    pub LoadId: String,
    pub TrailerNum: String,
}

#[derive(Serialize, Deserialize)]
pub struct TrailerArrivalMessage {
    pub ArrivalTime: String,
    pub LoadId: String,
    pub TrailerNum: String,
}

#[derive(Serialize, Deserialize)]
pub struct PickFinishMessage {
    pub LoadId: String,
    pub FinishTime: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedByRequest {
    pub LoadId: String,
    pub VerifiedBy: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifiedByMessage {
    pub LoadId: String,
    pub VerifiedBy: String,
}

#[derive(Serialize, Deserialize)]
pub struct StartLoadingMessage {
    pub LoadId: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShipmentLoadingRequest {
    pub LoadId: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShipmentLoadingMessage {
    pub LoadId: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetShipmentDoorRequest {
    pub LoadId: String,
    pub Door: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetShipmentDoorMessage {
    pub LoadId: String,
    pub Door: String,
}

#[derive(Serialize, Deserialize)]
pub struct ShipmentDepartRequest {
    pub LoadId: String,
    pub DepartTime: String,
}

#[derive(Serialize, Deserialize)]
pub struct SetShipmentDepartMessage {
    pub LoadId: String,
    pub DepartTime: String,
}


#[derive(Serialize, Deserialize)]
pub struct PickStartMessage {
    pub LoadId: String,
    pub StartTime: String,
    pub Picker: String,
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
pub struct GmapItem {
    #[serde(rename = "PLANT")]
    pub plant: String,
    #[serde(rename = "F/U")]
    f_u: String,
    #[serde(rename = "PART")]
    pub part: String,
    #[serde(rename = "PART NAME")]
    pub part_name: String,
    #[serde(rename = "DUNS")]
    duns: String,
    #[serde(rename = "SUPPLIER NAME")]
    supplier_name: String,
    #[serde(rename = "PLANT BANK")]
    plant_bank: String,
    #[serde(rename = "PLANT BANK OVERRIDE")]               
    plant_bank_override: String,
    #[serde(rename = "PLANT BANK OVERRIDE USER")]
    plant_bank_override_user: String,
    #[serde(rename = "EFFECTIVE DATE")]
    effective_date: String,
    #[serde(rename = "PLANT CBAL")]
    plant_cbal: Option<i32>,
    #[serde(rename = "PLANT DOH")]
    pub plant_doh: String,
    #[serde(rename = "ASL BANK")]
    pub asl_bank: Option<i32>,
    #[serde(rename = "ASL QTY")]
    pub asl_qty: i32,
    #[serde(rename = "Day 1 QTY OFFSET")]
    day_1_qty_offset: Option<i32>,
    #[serde(rename = "IN TRANSIT ASL TO PLANT")]
    pub in_transit_asl_to_plant: Option<i32>,         
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ScaleItem {
    #[serde(rename = "ITEM")]
    pub item: String,
    #[serde(rename = "LOCATION")]
    pub location: String,
    #[serde(rename = "OH_QTY")]
    pub oh_quantity: i32,
    #[serde(rename = "AL_QTY")]
    pub al_quantity: i32,
    #[serde(rename = "AV_QTY")]
    pub av_quantity: i32,
    pub missing_quantity: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ScaleItemMap {
    #[serde(rename = "ITEM")]
    pub item: String,
    #[serde(rename = "OH_QTY")]
    pub oh_quantity: i32,
    #[serde(rename = "AL_QTY")]
    pub al_quantity: i32,
    #[serde(rename = "AV_QTY")]
    pub av_quantity: i32,
    pub missing_quantity: Option<i32>,
}

#[derive(Serialize)]
pub struct LoadCountRequest {
    prefix: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct Shipment {
    pub ScheduleDate: String,
    pub ScheduleTime: String,
    pub ArrivalTime: String,
    pub DepartTime: String,
    pub Dock: String,
    pub Door: String,
    pub LoadId: String,
    pub LoadNum: String,
    pub Status: String,
    pub Picker: String,
    pub PickStartTime: String,
    pub VerifiedBy: String,
    pub TrailerNum: String,
    pub PickFinishTime: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ItemCompare {
    pub part: String,
    pub scale_oh_quantity: i32,
    pub scale_al_quantity: i32,
    pub scale_missing_quantity: i32,
    pub scale_actual_quantity: i32,
    pub asl_quantity: i32,
    pub dif: i32,
    pub in_transit: i32,
    pub plant: String,
    pub plant_doh: String,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ItemDetails {
    #[serde(rename = "ITEM")]
    pub item: String,

    #[serde(rename = "VELOCITY")]
    pub velocity: String,

    #[serde(rename = "EA QTY")]
    pub ea_qty: Option<i32>,

    #[serde(rename = "EA LENGTH")]
    pub ea_length: Option<f32>,

    #[serde(rename = "EA WIDTH")]
    pub ea_width: Option<f32>,

    #[serde(rename = "EA HEIGHT")]
    pub ea_height: Option<f32>,

    #[serde(rename = "EA WEIGHT")]
    pub ea_weight: Option<f32>,

    #[serde(rename = "CTN QTY")]
    pub ctn_qty: i32,

    #[serde(rename = "CTN LENGTH")]
    pub ctn_length: Option<f32>,

    #[serde(rename = "CTN WIDTH")]
    pub ctn_width: Option<f32>,

    #[serde(rename = "CTN HEIGHT")]
    pub ctn_height: Option<f32>,

    #[serde(rename = "CTN WEIGHT")]
    pub ctn_weight: Option<f32>,

    #[serde(rename = "PAL QTY")]
    pub pal_qty: i32,

    #[serde(rename = "PAL LENGTH")]
    pub pal_length: Option<f32>,

    #[serde(rename = "PAL WIDTH")]
    pub pal_width: Option<f32>,

    #[serde(rename = "PAL HEIGHT")]
    pub pal_height: Option<f32>,

    #[serde(rename = "PAL WEIGHT")]
    pub pal_weight: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default, Clone)]
pub struct ItemMaster {
    pub part: String,
    pub desc: String,
    pub company: String,
    pub blank: String,
    pub class: String,
    pub blank2: String,
    pub velocity: String,
    pub location: String,
    pub wide: String,
    pub size: String,
    pub ea_qty: i32,
    pub ea_len: f32,
    pub ea_wid: f32,
    pub ea_hei: f32,
    pub ea_wt: f32,
    pub std_pk: i32,
    pub pri_len: f32,
    pub pri_wid: f32,
    pub pri_hei: f32,
    pub pri_wt: f32,
    pub pal_qty: i32,
    pub pal_len: f32,
    pub pal_wid: f32,
    pub pal_hei: f32,
    pub pal_wt: f32,
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ShipmentFormData {
    pub door: String,
    pub dock: String,
    pub schedule_date: String,
    pub schedule_time: String,
    pub trailer: String,
    pub picker: String,
    pub verified_by: String,
    pub load_id: String,
    pub load_num: String,
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
    pub IsStat6: bool,
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