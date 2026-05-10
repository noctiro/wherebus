use crate::models::City;

pub struct ChelaileCity {
    pub city: City,
    pub city_id: &'static str,
}

pub const CITIES: &[ChelaileCity] = &[
    // 直辖市
    ChelaileCity {
        city: City::Beijing,
        city_id: "027",
    },
    ChelaileCity {
        city: City::Shanghai,
        city_id: "034",
    },
    ChelaileCity {
        city: City::Tianjin,
        city_id: "036",
    },
    ChelaileCity {
        city: City::Chongqing,
        city_id: "003",
    },
    // 河北
    ChelaileCity {
        city: City::Shijiazhuang,
        city_id: "053",
    },
    ChelaileCity {
        city: City::Tangshan,
        city_id: "056",
    },
    ChelaileCity {
        city: City::Qinhuangdao,
        city_id: "095",
    },
    ChelaileCity {
        city: City::Handan,
        city_id: "025",
    },
    ChelaileCity {
        city: City::Xingtai,
        city_id: "397",
    },
    ChelaileCity {
        city: City::Baoding,
        city_id: "069",
    },
    ChelaileCity {
        city: City::Zhangjiakou,
        city_id: "038",
    },
    ChelaileCity {
        city: City::Chengde,
        city_id: "219",
    },
    ChelaileCity {
        city: City::Cangzhou,
        city_id: "082",
    },
    ChelaileCity {
        city: City::Langfang,
        city_id: "217",
    },
    ChelaileCity {
        city: City::Hengshui,
        city_id: "220",
    },
    // 山西
    ChelaileCity {
        city: City::Taiyuan,
        city_id: "020",
    },
    ChelaileCity {
        city: City::Datong,
        city_id: "180",
    },
    ChelaileCity {
        city: City::Yangquan,
        city_id: "185",
    },
    ChelaileCity {
        city: City::Changzhi,
        city_id: "094",
    },
    ChelaileCity {
        city: City::Jincheng,
        city_id: "218",
    },
    ChelaileCity {
        city: City::Shuozhou,
        city_id: "183",
    },
    ChelaileCity {
        city: City::Jinzhong,
        city_id: "178",
    },
    ChelaileCity {
        city: City::Yuncheng,
        city_id: "186",
    },
    ChelaileCity {
        city: City::Xinzhou,
        city_id: "184",
    },
    ChelaileCity {
        city: City::Linfen,
        city_id: "181",
    },
    ChelaileCity {
        city: City::Lvliang,
        city_id: "182",
    },
    // 内蒙古
    ChelaileCity {
        city: City::Hohhot,
        city_id: "049",
    },
    ChelaileCity {
        city: City::Baotou,
        city_id: "067",
    },
    ChelaileCity {
        city: City::Chifeng,
        city_id: "210",
    },
    ChelaileCity {
        city: City::Ordos,
        city_id: "336",
    },
    ChelaileCity {
        city: City::Bayannur,
        city_id: "211",
    },
    ChelaileCity {
        city: City::Ulanqab,
        city_id: "544",
    },
    // 辽宁
    ChelaileCity {
        city: City::Shenyang,
        city_id: "035",
    },
    ChelaileCity {
        city: City::Dalian,
        city_id: "060",
    },
    ChelaileCity {
        city: City::Anshan,
        city_id: "071",
    },
    ChelaileCity {
        city: City::Fushun,
        city_id: "154",
    },
    ChelaileCity {
        city: City::Dandong,
        city_id: "465",
    },
    ChelaileCity {
        city: City::Jinzhou,
        city_id: "197",
    },
    ChelaileCity {
        city: City::Yingkou,
        city_id: "198",
    },
    ChelaileCity {
        city: City::Liaoyang,
        city_id: "128",
    },
    ChelaileCity {
        city: City::Panjin,
        city_id: "107",
    },
    ChelaileCity {
        city: City::Tieling,
        city_id: "139",
    },
    ChelaileCity {
        city: City::ChaoyangLN,
        city_id: "459",
    },
    // 吉林
    ChelaileCity {
        city: City::Changchun,
        city_id: "061",
    },
    ChelaileCity {
        city: City::JilinCity,
        city_id: "073",
    },
    ChelaileCity {
        city: City::Siping,
        city_id: "559",
    },
    ChelaileCity {
        city: City::Liaoyuan,
        city_id: "517",
    },
    ChelaileCity {
        city: City::Tonghua,
        city_id: "503",
    },
    ChelaileCity {
        city: City::Baishan,
        city_id: "308",
    },
    ChelaileCity {
        city: City::Songyuan,
        city_id: "304",
    },
    ChelaileCity {
        city: City::Baicheng,
        city_id: "561",
    },
    // 黑龙江
    ChelaileCity {
        city: City::Harbin,
        city_id: "096",
    },
    ChelaileCity {
        city: City::Qiqihar,
        city_id: "259",
    },
    ChelaileCity {
        city: City::Jixi,
        city_id: "575",
    },
    ChelaileCity {
        city: City::Shuangyashan,
        city_id: "206",
    },
    ChelaileCity {
        city: City::Daqing,
        city_id: "377",
    },
    ChelaileCity {
        city: City::Yichun,
        city_id: "142",
    },
    ChelaileCity {
        city: City::Jiamusi,
        city_id: "322",
    },
    ChelaileCity {
        city: City::Mudanjiang,
        city_id: "131",
    },
    ChelaileCity {
        city: City::Heihe,
        city_id: "563",
    },
    ChelaileCity {
        city: City::Suihua,
        city_id: "475",
    },
    // 江苏
    ChelaileCity {
        city: City::Nanjing,
        city_id: "018",
    },
    ChelaileCity {
        city: City::Wuxi,
        city_id: "054",
    },
    ChelaileCity {
        city: City::Xuzhou,
        city_id: "057",
    },
    ChelaileCity {
        city: City::Changzhou,
        city_id: "058",
    },
    ChelaileCity {
        city: City::Suzhou,
        city_id: "011",
    },
    ChelaileCity {
        city: City::Nantong,
        city_id: "055",
    },
    ChelaileCity {
        city: City::Lianyungang,
        city_id: "560",
    },
    ChelaileCity {
        city: City::Huaian,
        city_id: "062",
    },
    ChelaileCity {
        city: City::Yancheng,
        city_id: "402",
    },
    ChelaileCity {
        city: City::Yangzhou,
        city_id: "360",
    },
    ChelaileCity {
        city: City::Zhenjiang,
        city_id: "098",
    },
    ChelaileCity {
        city: City::Taizhou,
        city_id: "145",
    },
    ChelaileCity {
        city: City::Suqian,
        city_id: "247",
    },
    // 浙江
    ChelaileCity {
        city: City::Hangzhou,
        city_id: "004",
    },
    ChelaileCity {
        city: City::Ningbo,
        city_id: "045",
    },
    ChelaileCity {
        city: City::Wenzhou,
        city_id: "048",
    },
    ChelaileCity {
        city: City::Jiaxing,
        city_id: "028",
    },
    ChelaileCity {
        city: City::Huzhou,
        city_id: "515",
    },
    ChelaileCity {
        city: City::Shaoxing,
        city_id: "044",
    },
    ChelaileCity {
        city: City::Jinhua,
        city_id: "187",
    },
    ChelaileCity {
        city: City::Quzhou,
        city_id: "188",
    },
    ChelaileCity {
        city: City::Zhoushan,
        city_id: "301",
    },
    ChelaileCity {
        city: City::TaizhouZJ,
        city_id: "068",
    },
    // 安徽
    ChelaileCity {
        city: City::Hefei,
        city_id: "005",
    },
    ChelaileCity {
        city: City::Wuhu,
        city_id: "296",
    },
    ChelaileCity {
        city: City::Bengbu,
        city_id: "390",
    },
    ChelaileCity {
        city: City::Huainan,
        city_id: "114",
    },
    ChelaileCity {
        city: City::Maanshan,
        city_id: "202",
    },
    ChelaileCity {
        city: City::Huaibei,
        city_id: "370",
    },
    ChelaileCity {
        city: City::Tongling,
        city_id: "214",
    },
    ChelaileCity {
        city: City::Anqing,
        city_id: "297",
    },
    ChelaileCity {
        city: City::Chuzhou,
        city_id: "262",
    },
    ChelaileCity {
        city: City::Fuyang,
        city_id: "199",
    },
    ChelaileCity {
        city: City::SuzhouAH,
        city_id: "374",
    },
    ChelaileCity {
        city: City::Luan,
        city_id: "518",
    },
    ChelaileCity {
        city: City::Bozhou,
        city_id: "200",
    },
    // 福建
    ChelaileCity {
        city: City::Fuzhou,
        city_id: "047",
    },
    ChelaileCity {
        city: City::Putian,
        city_id: "051",
    },
    ChelaileCity {
        city: City::Sanming,
        city_id: "204",
    },
    ChelaileCity {
        city: City::Quanzhou,
        city_id: "059",
    },
    ChelaileCity {
        city: City::Zhangzhou,
        city_id: "138",
    },
    ChelaileCity {
        city: City::Nanping,
        city_id: "205",
    },
    ChelaileCity {
        city: City::Longyan,
        city_id: "203",
    },
    ChelaileCity {
        city: City::Ningde,
        city_id: "215",
    },
    // 江西
    ChelaileCity {
        city: City::Nanchang,
        city_id: "022",
    },
    ChelaileCity {
        city: City::Jingdezhen,
        city_id: "398",
    },
    ChelaileCity {
        city: City::Pingxiang,
        city_id: "348",
    },
    ChelaileCity {
        city: City::Jiujiang,
        city_id: "254",
    },
    ChelaileCity {
        city: City::Xinyu,
        city_id: "195",
    },
    ChelaileCity {
        city: City::Yingtan,
        city_id: "196",
    },
    ChelaileCity {
        city: City::Ganzhou,
        city_id: "039",
    },
    ChelaileCity {
        city: City::Jian,
        city_id: "258",
    },
    ChelaileCity {
        city: City::FuzhouJX,
        city_id: "394",
    },
    ChelaileCity {
        city: City::Shangrao,
        city_id: "562",
    },
    // 山东
    ChelaileCity {
        city: City::Jinan,
        city_id: "041",
    },
    ChelaileCity {
        city: City::Qingdao,
        city_id: "009",
    },
    ChelaileCity {
        city: City::Zibo,
        city_id: "108",
    },
    ChelaileCity {
        city: City::Zaozhuang,
        city_id: "120",
    },
    ChelaileCity {
        city: City::Dongying,
        city_id: "121",
    },
    ChelaileCity {
        city: City::Yantai,
        city_id: "052",
    },
    ChelaileCity {
        city: City::Weifang,
        city_id: "122",
    },
    ChelaileCity {
        city: City::Jining,
        city_id: "091",
    },
    ChelaileCity {
        city: City::Taian,
        city_id: "093",
    },
    ChelaileCity {
        city: City::Weihai,
        city_id: "112",
    },
    ChelaileCity {
        city: City::Rizhao,
        city_id: "117",
    },
    ChelaileCity {
        city: City::Linyi,
        city_id: "078",
    },
    ChelaileCity {
        city: City::Dezhou,
        city_id: "124",
    },
    ChelaileCity {
        city: City::Liaocheng,
        city_id: "125",
    },
    ChelaileCity {
        city: City::Binzhou,
        city_id: "123",
    },
    ChelaileCity {
        city: City::Heze,
        city_id: "126",
    },
    // 河南
    ChelaileCity {
        city: City::Zhengzhou,
        city_id: "010",
    },
    ChelaileCity {
        city: City::Kaifeng,
        city_id: "024",
    },
    ChelaileCity {
        city: City::Pingdingshan,
        city_id: "171",
    },
    ChelaileCity {
        city: City::Anyang,
        city_id: "030",
    },
    ChelaileCity {
        city: City::Hebi,
        city_id: "466",
    },
    ChelaileCity {
        city: City::Xinxiang,
        city_id: "033",
    },
    ChelaileCity {
        city: City::Jiaozuo,
        city_id: "026",
    },
    ChelaileCity {
        city: City::Puyang,
        city_id: "170",
    },
    ChelaileCity {
        city: City::Xuchang,
        city_id: "023",
    },
    ChelaileCity {
        city: City::Luohe,
        city_id: "410",
    },
    ChelaileCity {
        city: City::Sanmenxia,
        city_id: "389",
    },
    ChelaileCity {
        city: City::Nanyang,
        city_id: "029",
    },
    ChelaileCity {
        city: City::Shangqiu,
        city_id: "031",
    },
    ChelaileCity {
        city: City::Xinyang,
        city_id: "450",
    },
    ChelaileCity {
        city: City::Zhoukou,
        city_id: "443",
    },
    ChelaileCity {
        city: City::Zhumadian,
        city_id: "032",
    },
    // 湖北
    ChelaileCity {
        city: City::Wuhan,
        city_id: "000",
    },
    ChelaileCity {
        city: City::Huangshi,
        city_id: "513",
    },
    ChelaileCity {
        city: City::Shiyan,
        city_id: "365",
    },
    ChelaileCity {
        city: City::Yichang,
        city_id: "155",
    },
    ChelaileCity {
        city: City::Xiangyang,
        city_id: "216",
    },
    ChelaileCity {
        city: City::Jingmen,
        city_id: "453",
    },
    ChelaileCity {
        city: City::Xiaogan,
        city_id: "524",
    },
    ChelaileCity {
        city: City::Jingzhou,
        city_id: "331",
    },
    ChelaileCity {
        city: City::Huanggang,
        city_id: "570",
    },
    ChelaileCity {
        city: City::Xianning,
        city_id: "298",
    },
    ChelaileCity {
        city: City::Suizhou,
        city_id: "463",
    },
    // 湖南
    ChelaileCity {
        city: City::Changsha,
        city_id: "066",
    },
    ChelaileCity {
        city: City::Zhuzhou,
        city_id: "601",
    },
    ChelaileCity {
        city: City::Xiangtan,
        city_id: "173",
    },
    ChelaileCity {
        city: City::Hengyang,
        city_id: "469",
    },
    ChelaileCity {
        city: City::Shaoyang,
        city_id: "174",
    },
    ChelaileCity {
        city: City::Yueyang,
        city_id: "070",
    },
    ChelaileCity {
        city: City::Changde,
        city_id: "315",
    },
    ChelaileCity {
        city: City::Zhangjiajie,
        city_id: "177",
    },
    ChelaileCity {
        city: City::Yiyang,
        city_id: "480",
    },
    ChelaileCity {
        city: City::Chenzhou,
        city_id: "212",
    },
    ChelaileCity {
        city: City::Yongzhou,
        city_id: "176",
    },
    ChelaileCity {
        city: City::Huaihua,
        city_id: "314",
    },
    ChelaileCity {
        city: City::Loudi,
        city_id: "162",
    },
    // 广东
    ChelaileCity {
        city: City::Foshan,
        city_id: "019",
    },
    ChelaileCity {
        city: City::Zhongshan,
        city_id: "021",
    },
    ChelaileCity {
        city: City::Dongguan,
        city_id: "008",
    },
    ChelaileCity {
        city: City::Huizhou,
        city_id: "016",
    },
    ChelaileCity {
        city: City::Zhuhai,
        city_id: "242",
    },
    ChelaileCity {
        city: City::Shantou,
        city_id: "345",
    },
    ChelaileCity {
        city: City::Jiangmen,
        city_id: "134",
    },
    ChelaileCity {
        city: City::Zhanjiang,
        city_id: "157",
    },
    ChelaileCity {
        city: City::Maoming,
        city_id: "251",
    },
    ChelaileCity {
        city: City::Zhaoqing,
        city_id: "521",
    },
    ChelaileCity {
        city: City::Meizhou,
        city_id: "277",
    },
    ChelaileCity {
        city: City::Shanwei,
        city_id: "159",
    },
    ChelaileCity {
        city: City::Heyuan,
        city_id: "278",
    },
    ChelaileCity {
        city: City::Yangjiang,
        city_id: "158",
    },
    ChelaileCity {
        city: City::Qingyuan,
        city_id: "160",
    },
    ChelaileCity {
        city: City::Chaozhou,
        city_id: "113",
    },
    ChelaileCity {
        city: City::Jieyang,
        city_id: "270",
    },
    ChelaileCity {
        city: City::Yunfu,
        city_id: "161",
    },
    ChelaileCity {
        city: City::Shaoguan,
        city_id: "241",
    },
    // 广西
    ChelaileCity {
        city: City::Nanning,
        city_id: "046",
    },
    ChelaileCity {
        city: City::Liuzhou,
        city_id: "140",
    },
    ChelaileCity {
        city: City::Guilin,
        city_id: "361",
    },
    ChelaileCity {
        city: City::Wuzhou,
        city_id: "169",
    },
    ChelaileCity {
        city: City::Beihai,
        city_id: "267",
    },
    ChelaileCity {
        city: City::Guigang,
        city_id: "316",
    },
    ChelaileCity {
        city: City::Yulin,
        city_id: "440",
    },
    ChelaileCity {
        city: City::Baise,
        city_id: "551",
    },
    ChelaileCity {
        city: City::Hezhou,
        city_id: "303",
    },
    ChelaileCity {
        city: City::Hechi,
        city_id: "300",
    },
    ChelaileCity {
        city: City::Laibin,
        city_id: "372",
    },
    // 海南
    ChelaileCity {
        city: City::Haikou,
        city_id: "129",
    },
    ChelaileCity {
        city: City::Sanya,
        city_id: "074",
    },
    ChelaileCity {
        city: City::Danzhou,
        city_id: "558",
    },
    // 四川
    ChelaileCity {
        city: City::Chengdu,
        city_id: "007",
    },
    ChelaileCity {
        city: City::Zigong,
        city_id: "371",
    },
    ChelaileCity {
        city: City::Panzhihua,
        city_id: "192",
    },
    ChelaileCity {
        city: City::Luzhou,
        city_id: "344",
    },
    ChelaileCity {
        city: City::Mianyang,
        city_id: "133",
    },
    ChelaileCity {
        city: City::Neijiang,
        city_id: "130",
    },
    ChelaileCity {
        city: City::Leshan,
        city_id: "191",
    },
    ChelaileCity {
        city: City::Nanchong,
        city_id: "526",
    },
    ChelaileCity {
        city: City::Meishan,
        city_id: "557",
    },
    ChelaileCity {
        city: City::Yibin,
        city_id: "193",
    },
    ChelaileCity {
        city: City::Dazhou,
        city_id: "320",
    },
    ChelaileCity {
        city: City::Yaan,
        city_id: "194",
    },
    ChelaileCity {
        city: City::Bazhong,
        city_id: "282",
    },
    ChelaileCity {
        city: City::Ziyang,
        city_id: "263",
    },
    // 贵州
    ChelaileCity {
        city: City::Guiyang,
        city_id: "083",
    },
    ChelaileCity {
        city: City::Liupanshui,
        city_id: "179",
    },
    ChelaileCity {
        city: City::Zunyi,
        city_id: "166",
    },
    ChelaileCity {
        city: City::Anshun,
        city_id: "305",
    },
    ChelaileCity {
        city: City::Bijie,
        city_id: "312",
    },
    ChelaileCity {
        city: City::Tongren,
        city_id: "520",
    },
    // 云南
    ChelaileCity {
        city: City::Kunming,
        city_id: "081",
    },
    ChelaileCity {
        city: City::Qujing,
        city_id: "165",
    },
    ChelaileCity {
        city: City::Yuxi,
        city_id: "467",
    },
    ChelaileCity {
        city: City::Zhaotong,
        city_id: "290",
    },
    ChelaileCity {
        city: City::Lijiang,
        city_id: "116",
    },
    ChelaileCity {
        city: City::Puer,
        city_id: "273",
    },
    ChelaileCity {
        city: City::Lincang,
        city_id: "468",
    },
    // 西藏
    ChelaileCity {
        city: City::Lhasa,
        city_id: "043",
    },
    ChelaileCity {
        city: City::Shigatse,
        city_id: "528",
    },
    ChelaileCity {
        city: City::Nyingchi,
        city_id: "409",
    },
    ChelaileCity {
        city: City::Shannan,
        city_id: "433",
    },
    // 陕西
    ChelaileCity {
        city: City::Xian,
        city_id: "076",
    },
    ChelaileCity {
        city: City::Tongchuan,
        city_id: "531",
    },
    ChelaileCity {
        city: City::Baoji,
        city_id: "275",
    },
    ChelaileCity {
        city: City::Xianyang,
        city_id: "408",
    },
    ChelaileCity {
        city: City::YulinSX,
        city_id: "529",
    },
    // 甘肃
    ChelaileCity {
        city: City::Jinchang,
        city_id: "602",
    },
    ChelaileCity {
        city: City::Baiyin,
        city_id: "355",
    },
    ChelaileCity {
        city: City::Tianshui,
        city_id: "366",
    },
    ChelaileCity {
        city: City::Zhangye,
        city_id: "539",
    },
    ChelaileCity {
        city: City::Jiuquan,
        city_id: "553",
    },
    ChelaileCity {
        city: City::Qingyang,
        city_id: "552",
    },
    ChelaileCity {
        city: City::Dingxi,
        city_id: "413",
    },
    ChelaileCity {
        city: City::Longnan,
        city_id: "115",
    },
    // 青海
    ChelaileCity {
        city: City::Xining,
        city_id: "566",
    },
    ChelaileCity {
        city: City::Haidong,
        city_id: "411",
    },
    // 宁夏
    ChelaileCity {
        city: City::Yinchuan,
        city_id: "100",
    },
    ChelaileCity {
        city: City::Wuzhong,
        city_id: "163",
    },
    ChelaileCity {
        city: City::Guyuan,
        city_id: "164",
    },
    ChelaileCity {
        city: City::Zhongwei,
        city_id: "499",
    },
    // 新疆
    ChelaileCity {
        city: City::Urumqi,
        city_id: "001",
    },
    ChelaileCity {
        city: City::Karamay,
        city_id: "289",
    },
    ChelaileCity {
        city: City::Turpan,
        city_id: "104",
    },
    ChelaileCity {
        city: City::Hami,
        city_id: "462",
    },
];
