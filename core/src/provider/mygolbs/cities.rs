use crate::models::City;

pub struct MygolbsCity {
    pub city: City,
    pub api_city_name: &'static str,
    pub api_city_key: &'static str,
}

pub const CITIES: &[MygolbsCity] = &[
    MygolbsCity {
        city: City::Quanzhou,
        api_city_name: "泉州市",
        api_city_key: "qz595803",
    },
    MygolbsCity {
        city: City::Xiamen,
        api_city_name: "厦门市",
        api_city_key: "xm592801",
    },
    MygolbsCity {
        city: City::Fuzhou,
        api_city_name: "福州市",
        api_city_key: "fz591801",
    },
    MygolbsCity {
        city: City::Zhangzhou,
        api_city_name: "漳州市",
        api_city_key: "zz596301",
    },
    MygolbsCity {
        city: City::Putian,
        api_city_name: "莆田市",
        api_city_key: "pt594801",
    },
    MygolbsCity {
        city: City::Longyan,
        api_city_name: "龙岩市",
        api_city_key: "ly597801",
    },
    MygolbsCity {
        city: City::Ningde,
        api_city_name: "宁德市",
        api_city_key: "nd593801",
    },
    MygolbsCity {
        city: City::Sanming,
        api_city_name: "三明市",
        api_city_key: "sm598801",
    },
    MygolbsCity {
        city: City::Nanping,
        api_city_name: "南平市",
        api_city_key: "np599801",
    },
    MygolbsCity {
        city: City::Beijing,
        api_city_name: "北京市",
        api_city_key: "bj010801",
    },
    MygolbsCity {
        city: City::Shanghai,
        api_city_name: "上海市",
        api_city_key: "sh021801",
    },
    MygolbsCity {
        city: City::Tianjin,
        api_city_name: "天津市",
        api_city_key: "tj022801",
    },
    MygolbsCity {
        city: City::Chongqing,
        api_city_name: "重庆市",
        api_city_key: "cq023801",
    },
    MygolbsCity {
        city: City::Guangzhou,
        api_city_name: "广州市",
        api_city_key: "gz020801",
    },
    MygolbsCity {
        city: City::Dongguan,
        api_city_name: "东莞市",
        api_city_key: "dg769801",
    },
    MygolbsCity {
        city: City::Foshan,
        api_city_name: "佛山市",
        api_city_key: "fs757801",
    },
    MygolbsCity {
        city: City::Zhuhai,
        api_city_name: "珠海市",
        api_city_key: "zh756801",
    },
    MygolbsCity {
        city: City::Huizhou,
        api_city_name: "惠州市",
        api_city_key: "hz752801",
    },
    MygolbsCity {
        city: City::Zhongshan,
        api_city_name: "中山市",
        api_city_key: "zs760801",
    },
    MygolbsCity {
        city: City::Shantou,
        api_city_name: "汕头市",
        api_city_key: "st754801",
    },
    MygolbsCity {
        city: City::Nanjing,
        api_city_name: "南京市",
        api_city_key: "nj025801",
    },
    MygolbsCity {
        city: City::Suzhou,
        api_city_name: "苏州市",
        api_city_key: "sz512801",
    },
    MygolbsCity {
        city: City::Wuxi,
        api_city_name: "无锡市",
        api_city_key: "wx510801",
    },
    MygolbsCity {
        city: City::Changzhou,
        api_city_name: "常州市",
        api_city_key: "cz519801",
    },
    MygolbsCity {
        city: City::Xuzhou,
        api_city_name: "徐州市",
        api_city_key: "xz516801",
    },
    MygolbsCity {
        city: City::Hangzhou,
        api_city_name: "杭州市",
        api_city_key: "hz571801",
    },
    MygolbsCity {
        city: City::Ningbo,
        api_city_name: "宁波市",
        api_city_key: "nb574801",
    },
    MygolbsCity {
        city: City::Wenzhou,
        api_city_name: "温州市",
        api_city_key: "wz577801",
    },
    MygolbsCity {
        city: City::Hefei,
        api_city_name: "合肥市",
        api_city_key: "hf551801",
    },
    MygolbsCity {
        city: City::Nanchang,
        api_city_name: "南昌市",
        api_city_key: "nc791801",
    },
    MygolbsCity {
        city: City::Qingdao,
        api_city_name: "青岛市",
        api_city_key: "qd532801",
    },
    MygolbsCity {
        city: City::Jinan,
        api_city_name: "济南市",
        api_city_key: "jn531801",
    },
    MygolbsCity {
        city: City::Zhengzhou,
        api_city_name: "郑州市",
        api_city_key: "zz371801",
    },
    MygolbsCity {
        city: City::Wuhan,
        api_city_name: "武汉市",
        api_city_key: "wh027801",
    },
    MygolbsCity {
        city: City::Changsha,
        api_city_name: "长沙市",
        api_city_key: "cs731801",
    },
    MygolbsCity {
        city: City::Chengdu,
        api_city_name: "成都市",
        api_city_key: "cd028801",
    },
    MygolbsCity {
        city: City::Xian,
        api_city_name: "西安市",
        api_city_key: "xa029801",
    },
    MygolbsCity {
        city: City::Kunming,
        api_city_name: "昆明市",
        api_city_key: "km871801",
    },
    MygolbsCity {
        city: City::Guiyang,
        api_city_name: "贵阳市",
        api_city_key: "gy851801",
    },
    MygolbsCity {
        city: City::Lanzhou,
        api_city_name: "兰州市",
        api_city_key: "lz931801",
    },
    MygolbsCity {
        city: City::Urumqi,
        api_city_name: "乌鲁木齐市",
        api_city_key: "wlmq991801",
    },
    MygolbsCity {
        city: City::Shenyang,
        api_city_name: "沈阳市",
        api_city_key: "sy024801",
    },
    MygolbsCity {
        city: City::Harbin,
        api_city_name: "哈尔滨市",
        api_city_key: "heb451801",
    },
    MygolbsCity {
        city: City::Changchun,
        api_city_name: "长春市",
        api_city_key: "cc431801",
    },
    MygolbsCity {
        city: City::Shijiazhuang,
        api_city_name: "石家庄市",
        api_city_key: "sjz311801",
    },
    MygolbsCity {
        city: City::Taiyuan,
        api_city_name: "太原市",
        api_city_key: "ty351801",
    },
    MygolbsCity {
        city: City::Nanning,
        api_city_name: "南宁市",
        api_city_key: "nn771801",
    },
    MygolbsCity {
        city: City::Haikou,
        api_city_name: "海口市",
        api_city_key: "hk898801",
    },
];
