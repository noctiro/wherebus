#!/usr/bin/env python3
"""
车来了 API 调试 — 自动串联查询链路
流程: encryptedNearlines → encryptedLineDetail
重点: 验证到站信息解析（desc/state/distanceToSp + buses.travelTime）
"""

import hashlib
import json
from base64 import b64decode

import requests
from Crypto.Cipher import AES
from Crypto.Util.Padding import unpad

API = "https://web.chelaile.net.cn/api"
HEADERS = {
    "Referer": "https://web.chelaile.net.cn/",
    "User-Agent": "Mozilla/5.0 (Linux; Android 14; Pixel 8) AppleWebKit/537.36",
}

CITY_ID = "034"
LAT = "31.2304"
LNG = "121.4737"

SIGN_SALT = "qwihrnbtmj"
AES_KEY = b"422556651C7F7B2B5C266EED06068230"


def sign(params: list[tuple[str, str]]) -> str:
    concat = "&".join(f"{k}={v}" for k, v in params)
    return hashlib.md5((concat + SIGN_SALT).encode()).hexdigest()


def decrypt(encrypted: str) -> dict:
    ciphertext = b64decode(encrypted)
    cipher = AES.new(AES_KEY, AES.MODE_ECB)
    plaintext = unpad(cipher.decrypt(ciphertext), AES.block_size)
    return json.loads(plaintext.decode("utf-8"))


def common_params() -> dict:
    return {
        "s": "h5", "v": "9.1.2", "vc": "1", "src": "webapp_default",
        "userId": "browser_wherebus", "h5Id": "browser_wherebus",
        "sign": "1", "cityId": CITY_ID,
    }


def get(path: str, extra: dict) -> dict:
    resp = requests.get(f"{API}/{path}", params={**extra, **common_params()}, headers=HEADERS, timeout=10)
    resp.raise_for_status()
    text = resp.text
    if text.startswith("**YGKJ"):
        text = text[6:]
    if text.endswith("YGKJ##"):
        text = text[:-6]
    return json.loads(text)


def nearby_lines():
    s = sign([("cityId", CITY_ID), ("lat", LAT), ("lng", LNG), ("gpstype", "gcj")])
    data = get("bus/stop!encryptedNearlines.action", {
        "cryptoSign": s, "lat": LAT, "lng": LNG,
        "geo_lat": LAT, "geo_lng": LNG, "gpstype": "gcj",
    })
    encrypted = data.get("jsonr", {}).get("data", {}).get("encryptResult")
    if encrypted:
        return decrypt(encrypted)
    return data.get("jsonr", {}).get("data", {})


def fetch_line_detail(line_id: str, line_name: str, direction: str,
                      station_name: str, target_order: int):
    s = sign([
        ("lineId", line_id), ("lineName", line_name), ("direction", direction),
        ("stationName", station_name), ("nextStationName", ""),
        ("lineNo", ""), ("targetOrder", str(target_order)),
    ])
    data = get("bus/line!encryptedLineDetail.action", {
        "lineId": line_id, "lineName": line_name, "direction": direction,
        "stationName": station_name, "nextStationName": "", "lineNo": "",
        "targetOrder": str(target_order), "cryptoSign": s,
    })
    encrypted = data.get("jsonr", {}).get("data", {}).get("encryptResult")
    if encrypted:
        return decrypt(encrypted)
    return data.get("jsonr", {}).get("data", {})


def simulate_parse_arrival(line: dict, distance_to_sp: int) -> str:
    """模拟 Rust 端 parse_chelaile_arrival 逻辑"""
    desc = line.get("desc", "").strip()
    state = line.get("state", 0)

    if state == -3 or "末班时间已过" in desc:
        return "NoService (末班已过)"
    if state == -1 or "等待发车" in desc or "未发车" in desc:
        return "NoService (等待发车)"
    if not desc:
        return "Unknown (空desc)"
    if any(k in desc for k in ("即将到站", "进站中", "已到站")):
        return "Arriving"

    stations = extract_stations(desc)
    minutes = extract_minutes(desc)
    dist = distance_to_sp if distance_to_sp > 0 else None

    if stations or minutes:
        if not minutes and dist:
            minutes = max(1, dist // 400)
        return f"Approaching(stations={stations or 0}, minutes={minutes}, dist={dist})"
    return f"Unknown (desc=\"{desc}\")"


def extract_stations(text: str) -> int | None:
    idx = text.find("站")
    if idx < 0:
        return None
    before = text[:idx]
    num = ""
    for c in reversed(before):
        if c.isdigit():
            num = c + num
        else:
            break
    return int(num) if num else None


def extract_minutes(text: str) -> int | None:
    if "小于" in text or "不到" in text:
        return 0
    idx = text.find("分")
    if idx < 0:
        return None
    before = text[:idx]
    num = ""
    for c in reversed(before):
        if c.isdigit():
            num = c + num
        else:
            break
    return int(num) if num else None


def run():
    print(f"城市: {CITY_ID}  坐标: {LAT}, {LNG}\n")

    # Step 1: 附近站点+线路
    nearby = nearby_lines()
    stations = nearby.get("nearLines", [])
    print(f"[nearlines] {len(stations)} 个站点")

    # Step 2: 打印每条线路的关键字段 + 解析验证
    for si, station in enumerate(stations[:3]):
        sn = station.get("sn", "")
        dist = station.get("distance", 0)
        lines = station.get("lines", [])
        print(f"\n  站点 {si+1}: {sn} ({dist}m) — {len(lines)} 条线路")

        for lw in lines[:6]:
            line = lw.get("line", {})
            target = lw.get("targetStation", {})
            distance_to_sp = target.get("distanceToSp", 0)

            parsed = simulate_parse_arrival(line, distance_to_sp)
            print(f"\n    [{line.get('name','')}] state={line.get('state')} desc=\"{line.get('desc','')}\"")
            print(f"      targetStation.order={target.get('order')} distanceToSp={distance_to_sp}")
            print(f"      → 解析: {parsed}")

    # Step 3: lineDetail 实时车辆
    print("\n" + "=" * 60)
    print("[lineDetail] 实时车辆对比")
    print("=" * 60)

    checked = 0
    for station in stations[:3]:
        if checked >= 3:
            break
        sn = station.get("sn", "")
        for lw in station.get("lines", []):
            if checked >= 3:
                break
            line = lw.get("line", {})
            line_id = line.get("lineId", "")
            name = line.get("name", "")
            direction = str(line.get("direction", 0))
            target = lw.get("targetStation", {})
            order = target.get("order", 1)
            if not line_id:
                continue

            try:
                detail = fetch_line_detail(line_id, name, direction, sn, order)
            except Exception as e:
                print(f"\n  {name}: 请求失败 ({e})")
                checked += 1
                continue

            buses = detail.get("buses", [])
            print(f"\n  {name} (dir={direction}, station={sn}, order={order}):")
            if buses:
                for b in buses[:3]:
                    stations_away = order - b.get("order", 0) if b.get("order", 0) < order else 0
                    travel_min = int(b.get("travelTime", 0)) // 60
                    print(f"    bus: order={b.get('order')} travelTime={b.get('travelTime')}s "
                          f"({travel_min}min) dist={b.get('distanceToSc')} "
                          f"state={b.get('state')} timeStr=\"{b.get('timeStr','')}\"")
                    print(f"         → stations_away={stations_away} minutes_away={travel_min}")
            else:
                print(f"    (无在途车辆)")
            checked += 1


if __name__ == "__main__":
    run()
