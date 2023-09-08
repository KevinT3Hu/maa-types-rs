use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_default::DefaultFromSerde;

pub trait Param: Serialize {
    fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientType {
    Official,
    Bilibili,
    #[serde(rename = "txwy")]
    Txwy,
    YoStarEN,
    YoStarJP,
    YoStarKR,
}

// This is something silly right now, a workaround for serde default value which only supports using a function
fn def_true() -> bool {
    true
}

fn def_i32_max() -> i32 {
    i32::MAX
}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct StartUpParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub client_type: Option<ClientType>,
    #[serde(default)]
    pub start_game_enabled: bool,
}

impl Param for StartUpParams {}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct CloseDownParams {
    #[serde(default = "def_true")]
    pub enable: bool,
}

impl Param for CloseDownParams {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Server {
    CN,
    US,
    JP,
    KR,
}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct FightParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub stage: Option<String>,
    #[serde(default)]
    pub medicine: i32,
    #[serde(default)]
    pub expiring_medicine: i32,
    #[serde(default)]
    pub stone: i32,
    #[serde(default = "def_i32_max")]
    pub times: i32,
    #[serde(default)]
    pub drops: HashMap<String, i32>,
    #[serde(default)]
    pub report_to_penguin: bool,
    #[serde(default)]
    pub penguin_id: Option<String>,
    #[serde(default)]
    pub server: Option<Server>,
    #[serde(default)]
    pub client_type: Option<ClientType>,
    #[serde(rename = "DrGrandet")]
    #[serde(default)]
    pub dr_grandet: bool,
}

impl Param for FightParams {}

fn def_rec_time() -> HashMap<String, i32> {
    let mut rec_time: HashMap<String, i32> = HashMap::new();

    for i in 3..=6 {
        rec_time.insert(format!("{}", i), 540);
    }

    rec_time
}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct RecruitParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub refresh: bool,
    #[serde(default)]
    pub select: Vec<i32>,
    #[serde(default)]
    pub confirm: Vec<i32>,
    #[serde(default)]
    pub times: i32,
    #[serde(default = "def_true")]
    pub set_time: bool,
    #[serde(default)]
    pub expedite: bool,
    #[serde(default = "def_i32_max")]
    pub expedite_times: i32,
    #[serde(default = "def_true")]
    pub skip_robot: bool,
    #[serde(default = "def_rec_time")]
    pub recruitment_time: HashMap<String, i32>,
    #[serde(default)]
    pub report_to_penguin: bool,
    #[serde(default)]
    pub penguin_id: Option<String>,
    #[serde(default)]
    pub report_to_yituliu: bool,
    #[serde(default)]
    pub yituliu_id: Option<String>,
    #[serde(default)]
    pub server: Option<Server>,
}

impl Param for RecruitParams {}

#[derive(Serialize, Deserialize, Debug)]
pub enum InfrastFacility {
    Mfg,
    Trade,
    Power,
    Control,
    Reception,
    Office,
    Dorm,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DronesUsage {
    _NotUse,
    Money,
    SyntheticJade,
    CombatRecord,
    PureGold,
    OriginStone,
    Chip,
}

impl Default for DronesUsage {
    fn default() -> Self {
        DronesUsage::_NotUse
    }
}

fn def_facility() -> Vec<InfrastFacility> {
    vec![
        InfrastFacility::Mfg,
        InfrastFacility::Trade,
        InfrastFacility::Power,
        InfrastFacility::Control,
        InfrastFacility::Reception,
        InfrastFacility::Office,
        InfrastFacility::Dorm,
    ]
}

fn def_threshold() -> f64 {
    0.3
}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct InfrastParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub mode: i32,
    #[serde(default = "def_facility")]
    pub facility: Vec<InfrastFacility>,
    #[serde(default)]
    pub drones: DronesUsage,
    #[serde(default = "def_threshold")]
    pub threshold: f64,
    #[serde(default)]
    pub replenish: bool,
    #[serde(default)]
    pub dorm_notstationed_enabled: bool,
    #[serde(default)]
    pub dorm_trust_enabled: bool,
    #[serde(default)]
    pub filename: Option<String>,
    #[serde(default)]
    pub plan_index: Option<i32>,
}

impl Param for InfrastParams {}

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct MallParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub shopping: bool,
    #[serde(default)]
    pub buy_first: Option<Vec<String>>,
    #[serde(default)]
    pub blacklist: Option<Vec<String>>,
    #[serde(default = "def_true")]
    pub force_shopping_if_credit_full: bool,
}

impl Param for MallParams {}

pub type AwardParams = CloseDownParams;

#[derive(Serialize, Deserialize, Debug, DefaultFromSerde)]
pub struct RoguelikeParams {
    #[serde(default = "def_true")]
    pub enable: bool,
    #[serde(default)]
    pub theme: Option<String>,
    #[serde(default)]
    pub mode: i32,
    #[serde(default = "def_i32_max")]
    pub starts_count: i32,
    #[serde(default = "def_true")]
    pub investment_enabled: bool,
    #[serde(default = "def_i32_max")]
    pub investments_count: i32,
    #[serde(default)]
    pub stop_when_investment_full: bool,
    #[serde(default)]
    pub squad: Option<String>,
    #[serde(default)]
    pub roles: Option<String>,
    #[serde(default)]
    pub core_char: Option<String>,
    #[serde(default)]
    pub use_support: bool,
    #[serde(default)]
    pub use_nonfriend_support: bool,
    #[serde(default)]
    pub refresh_trader_with_dice: bool,
}

impl Param for RoguelikeParams {}

macro_rules! asst_task_param {
    ($($enumvariant: ident($content: ty),)*) => {
        /// AsstTaskParam is a enum that contains all the task parameters
        /// 
        /// All the task parameters implement [Param](Param) and [Default](std::default::Default) trait
        /// 
        /// Example:
        /// ```
        /// use maa_types::task::AsstTaskParam;
        /// use maa_types::task::StartUpParams;
        /// 
        /// let startup = AsstTaskParam::StartUp(StartUpParams::default());
        /// assert_eq!(startup.name(), "StartUp");
        #[derive(Debug)]
        pub enum AsstTaskParam {
            $($enumvariant($content),)*
        }

        impl AsstTaskParam {

            /// Return the name of the task parameter
            pub fn name(&self) -> String {
                match self {
                    $(AsstTaskParam::$enumvariant(..) => stringify!($enumvariant).to_string(),)*
                }
            }

            /// Return the json string of the task parameter
            pub fn param(&self) -> String {
                match self {
                    $(AsstTaskParam::$enumvariant(content) => content.json(),)*
                }
            }

            /// Return whether the task parameter is enabled
            pub fn enabled(&self) -> bool {
                match self {
                    $(AsstTaskParam::$enumvariant(content) => content.enable,)*
                }
            }
        }
    };
}

asst_task_param! {
    StartUp(StartUpParams),
    CloseDown(CloseDownParams),
    Fight(FightParams),
    Recruit(RecruitParams),
    Infrast(InfrastParams),
    Mall(MallParams),
    Award(AwardParams),
    Roguelike(RoguelikeParams),
}
