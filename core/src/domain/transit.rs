// ─── 枚举类型 ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ServiceTime {
    minutes_since_midnight: u16,
}

impl ServiceTime {
    pub fn parse(raw: &str) -> Option<Self> {
        let raw = raw.trim();
        let (hour, minute) = raw.split_once(':')?;
        let hour: u16 = hour.parse().ok()?;
        let minute: u16 = minute.parse().ok()?;
        if hour > 23 || minute > 59 {
            return None;
        }
        Some(Self {
            minutes_since_midnight: hour * 60 + minute,
        })
    }

    pub fn hour(self) -> u16 {
        self.minutes_since_midnight / 60
    }

    pub fn minute(self) -> u16 {
        self.minutes_since_midnight % 60
    }
}

impl core::fmt::Display for ServiceTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Terminal {
    Named(String),
    Unknown,
}

impl Terminal {
    pub fn named(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        if raw.trim().is_empty() {
            Self::Unknown
        } else {
            Self::Named(raw)
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Named(value) => Some(value.as_str()),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Fare {
    Unknown,
    Text(String),
}

impl Default for Fare {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Fare {
    pub fn text(raw: impl Into<String>) -> Self {
        let raw = raw.into();
        if raw.trim().is_empty() {
            Self::Unknown
        } else {
            Self::Text(raw)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ContactInfo {
    Unknown,
    Phone(String),
}

impl Default for ContactInfo {
    fn default() -> Self {
        Self::Unknown
    }
}

impl ContactInfo {
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct LineMeta {
    pub first_service: Option<ServiceTime>,
    pub last_service: Option<ServiceTime>,
    pub fare: Fare,
    pub company: Option<String>,
    pub contact: ContactInfo,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct EndpointsView {
    pub origin: String,
    pub origin_alias: Option<String>,
    pub terminus: String,
    pub terminus_alias: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteTopology {
    pub start: Terminal,
    pub end: Terminal,
    pub stations: Vec<LineStop>,
    pub track_points: Vec<(f64, f64)>,
}

/// 线路运营状态
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RunState {
    /// 正常运营，有实时车辆数据
    Running,
    /// 未到运营时间（首班车未发出）
    NotOperating,
    /// 已过末班，停止运营
    Stopped,
    /// 无法获取实时数据（GPS 离线等）
    NoRealtime,
}

/// 站点停靠状态
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StopStatus {
    /// 正常停靠
    Normal,
    /// 不停靠（飞站）
    NotStopping,
    /// 仅上客
    BoardOnly,
    /// 仅下客
    AlightOnly,
    /// 临时增设站
    Temporary,
    /// 招呼站（按需停靠）
    OnDemand,
    /// 快线跳站
    Express,
}

impl From<u8> for StopStatus {
    fn from(v: u8) -> Self {
        match v {
            0 => Self::NotStopping,
            2 => Self::BoardOnly,
            3 => Self::AlightOnly,
            4 => Self::Temporary,
            5 | 6 => Self::OnDemand,
            8 => Self::Express,
            _ => Self::Normal,
        }
    }
}

/// 车厢拥挤程度
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CrowdLevel {
    Unknown,
    /// 宽敞有座
    Spacious,
    /// 适中
    Normal,
    /// 拥挤站立
    Crowded,
    /// 满载
    Full,
}

/// 路段拥堵等级
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CongestionLevel {
    /// 畅通
    Smooth,
    /// 缓行
    Slow,
    /// 拥堵
    Congested,
    Unknown,
}

// ─── 附近站点 ───

/// 附近的公交站点
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Station {
    /// 站点 ID（车来了: sId，掌上公交: 无）
    pub id: Option<String>,
    pub name: String,
    /// 括号内的别名
    pub alias: Option<String>,
    pub lat: f64,
    pub lng: f64,
    /// 距用户直线距离（米）
    pub distance_m: u32,
    /// 同名站点数量（掌上公交提供）
    pub same_name_count: u32,
}

// ─── 站点经过的线路 ───

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ArrivalEstimate {
    Arriving,
    Approaching {
        stations_away: u32,
        minutes_away: Option<u32>,
        distance_m: Option<u32>,
    },
    NoService,
    Unknown,
}

impl ArrivalEstimate {
    pub fn proximity_score(&self) -> u32 {
        match self {
            Self::Arriving => 0,
            Self::Approaching { stations_away, .. } => (*stations_away).max(1),
            Self::NoService => 998,
            Self::Unknown => 999,
        }
    }

    pub fn state_tag(&self) -> &'static str {
        match self {
            Self::Arriving => "Arriving",
            Self::Approaching { .. } => "Approaching",
            Self::NoService => "NoService",
            Self::Unknown => "Unknown",
        }
    }
}

/// 经过某站点的线路摘要（附近页面用）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineSummary {
    pub id: Option<String>,
    pub name: String,
    pub direction_id: String,
    pub endpoints: EndpointsView,
    pub arrival: ArrivalEstimate,
    pub station_order: u32,
    pub run_state: RunState,
}

// ─── 线路详情 ───

/// 线路完整信息（含全部站点）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineDetail {
    pub id: Option<String>,
    pub name: String,
    pub direction_id: String,
    pub reverse_id: Option<String>,
    pub topology: RouteTopology,
    pub meta: LineMeta,
}

/// 线路中的一个站点
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LineStop {
    pub id: Option<String>,
    pub name: String,
    pub alias: Option<String>,
    /// 站序（从 1 开始）
    pub order: u32,
    pub lat: f64,
    pub lng: f64,
    pub status: StopStatus,
    /// 对应轨迹点索引（掌上公交提供，用于将站点对齐到轨迹线）
    pub track_index: Option<u32>,
}

// ─── 实时数据 ───

/// 线路实时运营数据
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RealTimeData {
    pub run_state: RunState,
    /// 计划发车时间（无实时数据时的参考）
    pub plan_time: Option<String>,
    /// 在途车辆
    pub buses: Vec<BusPosition>,
    /// 各站到离站统计（掌上公交提供）
    pub station_arrivals: Vec<StationArrival>,
    /// 站间路段拥堵信息（掌上公交提供）
    pub segments: Vec<RouteSegment>,
    /// 到站预估列表（CMD 104 routeOnStationRTimeInfoList）
    pub arrival_estimates: Vec<ArrivalDetail>,
}

/// CMD 104 返回的结构化到站预估
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArrivalDetail {
    pub stations_away: u32,
    pub minutes_away: u32,
    pub distance_m: u32,
}

/// 单辆公交车的实时位置
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BusPosition {
    /// 车辆标识（掌上公交: 车牌号，车来了: busId）
    pub bus_id: String,
    /// 当前所在站序
    pub station_index: u32,
    /// true = 正在进站，false = 已离站
    pub is_arriving: bool,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    /// 车头朝向角度（掌上公交提供）
    pub angle: Option<f64>,
    /// 距目标站距离（米）
    pub distance_to_station: Option<f64>,
    /// 预计到达秒数（车来了提供）
    pub travel_time_secs: Option<u32>,
    /// 目标站名称
    pub station_name: Option<String>,
    /// 车厢拥挤度（掌上公交提供）
    pub crowd_status: CrowdLevel,
    /// 对应轨迹线段索引（掌上公交提供）
    pub track_segment_index: Option<i32>,
    /// 状态描述（车来了提供，如 "还有9分钟"）
    pub state_description: Option<String>,
}

/// 站点到离站车辆统计
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StationArrival {
    pub station_index: u32,
    pub station_name: Option<String>,
    /// 正在进站的车辆数
    pub arriving_count: u32,
    /// 已离站的车辆数
    pub leaving_count: u32,
}

/// 站间路段信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RouteSegment {
    /// 路段长度（米）
    pub distance: Option<f64>,
    /// 平均速度（km/h）
    pub speed: Option<f64>,
    pub congestion: CongestionLevel,
}

// ─── 线路搜索 ───

/// 线路列表项（搜索结果）
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BusRoute {
    pub id: Option<String>,
    pub name: String,
    pub direction_id: String,
    pub terminals: (Terminal, Terminal),
    pub endpoints: EndpointsView,
    pub company: Option<String>,
}
