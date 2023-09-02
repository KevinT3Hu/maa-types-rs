use std::collections::HashMap;

use serde::Serialize;

pub trait Param: Serialize {
    fn json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub enum ClientType {
    Official,
    Bilibili,
    #[serde(rename = "txwy")]
    Txwy,
    YoStarEN,
    YoStarJP,
    YoStarKR,
}

#[derive(Serialize, Debug)]
pub struct StartUpParams {
    pub enable: bool,
    pub client_type: Option<ClientType>,
    pub start_game_enabled: bool,
}

impl Param for StartUpParams {}

impl Default for StartUpParams {
    fn default() -> Self {
        StartUpParams {
            enable: true,
            client_type: None,
            start_game_enabled: false,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct CloseDownParams {
    pub enable: bool,
}

impl Param for CloseDownParams {}

impl Default for CloseDownParams {
    fn default() -> Self {
        CloseDownParams { enable: true }
    }
}

#[derive(Serialize, Debug)]
pub enum Server {
    CN,
    US,
    JP,
    KR,
}

#[derive(Serialize, Debug)]
pub struct FightParams {
    pub enable: bool,
    pub stage: Option<String>,
    pub medicine: i32,
    pub expiring_medicine: i32,
    pub stone: i32,
    pub times: i32,
    pub drops: HashMap<String, i32>,
    pub report_to_penguin: bool,
    pub penguin_id: Option<String>,
    pub server: Option<Server>,
    pub client_type: Option<ClientType>,
    #[serde(rename = "DrGrandet")]
    pub dr_grandet: bool,
}

impl Param for FightParams {}

impl Default for FightParams {
    fn default() -> Self {
        FightParams {
            enable: true,
            stage: None,
            medicine: 0,
            expiring_medicine: 0,
            stone: 0,
            times: i32::MAX,
            drops: HashMap::new(),
            report_to_penguin: false,
            penguin_id: None,
            server: None,
            client_type: None,
            dr_grandet: false,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct RecruitParams {
    pub enable: bool,
    pub refresh: bool,
    pub select: Vec<i32>,
    pub confirm: Vec<i32>,
    pub times: i32,
    pub set_time: bool,
    pub expedite: bool,
    pub expedite_times: i32,
    pub skip_robot: bool,
    pub recruitment_time: HashMap<String, i32>,
    pub report_to_penguin: bool,
    pub penguin_id: Option<String>,
    pub report_to_yituliu: bool,
    pub yituliu_id: Option<String>,
    pub server: Option<Server>,
}

impl Param for RecruitParams {}

impl Default for RecruitParams {
    fn default() -> Self {
        let mut rec_time: HashMap<String, i32> = HashMap::new();

        for i in 3..=6 {
            rec_time.insert(format!("{}", i), 540);
        }

        RecruitParams {
            enable: true,
            refresh: false,
            select: Vec::new(),
            confirm: Vec::new(),
            times: 0,
            set_time: true,
            expedite: false,
            expedite_times: i32::MAX,
            skip_robot: true,
            recruitment_time: rec_time,
            report_to_penguin: false,
            penguin_id: None,
            report_to_yituliu: false,
            yituliu_id: None,
            server: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub enum InfrastFacility {
    Mfg,
    Trade,
    Power,
    Control,
    Reception,
    Office,
    Dorm,
}

#[derive(Serialize, Debug)]
pub enum DronesUsage {
    _NotUse,
    Money,
    SyntheticJade,
    CombatRecord,
    PureGold,
    OriginStone,
    Chip,
}

#[derive(Serialize, Debug)]
pub struct InfrastParams {
    pub enable: bool,
    pub mode: i32,
    pub facility: Vec<InfrastFacility>,
    pub drones: DronesUsage,
    pub threshold: f64,
    pub replenish: bool,
    pub dorm_notstationed_enabled: bool,
    pub dorm_trust_enabled: bool,
    pub filename: Option<String>,
    pub plan_index: Option<i32>,
}

impl Param for InfrastParams {}

impl Default for InfrastParams {
    fn default() -> Self {
        InfrastParams {
            enable: true,
            mode: 0,
            facility: vec![
                InfrastFacility::Mfg,
                InfrastFacility::Trade,
                InfrastFacility::Power,
                InfrastFacility::Control,
                InfrastFacility::Reception,
                InfrastFacility::Office,
                InfrastFacility::Dorm,
            ],
            drones: DronesUsage::_NotUse,
            threshold: 0.3,
            replenish: false,
            dorm_notstationed_enabled: false,
            dorm_trust_enabled: false,
            filename: None,
            plan_index: None,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MallParams {
    pub enable: bool,
    pub shopping: bool,
    pub buy_first: Option<Vec<String>>,
    pub blacklist: Option<Vec<String>>,
    pub force_shopping_if_credit_full: bool,
}

impl Param for MallParams {}

impl Default for MallParams {
    fn default() -> Self {
        MallParams {
            enable: true,
            shopping: false,
            buy_first: None,
            blacklist: None,
            force_shopping_if_credit_full: true,
        }
    }
}

pub type AwardParams = CloseDownParams;

#[derive(Serialize, Debug)]
pub struct RoguelikeParams {
    pub enable: bool,
    pub theme: Option<String>,
    pub mode: i32,
    pub starts_count: i32,
    pub investment_enabled: bool,
    pub investments_count: i32,
    pub stop_when_investment_full: bool,
    pub squad: Option<String>,
    pub roles: Option<String>,
    pub core_char: Option<String>,
    pub use_support: bool,
    pub use_nonfriend_support: bool,
    pub refresh_trader_with_dice: bool,
}

impl Param for RoguelikeParams {}

impl Default for RoguelikeParams {
    fn default() -> Self {
        RoguelikeParams {
            enable: true,
            theme: None,
            mode: 0,
            starts_count: i32::MAX,
            investment_enabled: true,
            investments_count: i32::MAX,
            stop_when_investment_full: false,
            squad: None,
            roles: None,
            core_char: None,
            use_support: false,
            use_nonfriend_support: false,
            refresh_trader_with_dice: false,
        }
    }
}

macro_rules! asst_task_param {
    ($($enumvariant: ident($content: ty),)*) => {
        #[derive(Debug)]
        pub enum AsstTaskParam {
            $($enumvariant($content),)*
        }

        impl AsstTaskParam {

            pub fn name(&self) -> String {
                match self {
                    $(AsstTaskParam::$enumvariant(..) => stringify!($enumvariant).to_string(),)*
                }
            }

            pub fn param(&self) -> String {
                match self {
                    $(AsstTaskParam::$enumvariant(content) => content.json(),)*
                }
            }

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
