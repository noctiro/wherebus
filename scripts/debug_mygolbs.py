#!/usr/bin/env python3
"""
掌上公交 API 调试 — 自动串联查询链路
流程: CMD 106 附近站点 → CMD 115 站点线路 → CMD 104 实时到站
重点: 验证到站信息解析（neartext/neartime/nearnum/neardis vs routeOnStationRTimeInfoList）
"""

import json
import requests

API = "https://h5.mygolbs.com/ApiData.do"
HEADERS = {
    "Referer": "https://h5.mygolbs.com/",
    "Origin": "https://h5.mygolbs.com",
    "User-Agent": "Mozilla/5.0 (Linux; Android 12) AppleWebKit/537.36",
}

CITY_NAME = "泉州市"
CITY_KEY = "qz595803"
LAT = "25.028607"
LNG = "118.801630"


def post(params: dict) -> dict:
    resp = requests.post(API, data=params, headers=HEADERS, timeout=10)
    resp.raise_for_status()
    return resp.json()


def cmd106(lat=LAT, lng=LNG):
    return post({"CMD": "106", "CITYNAME": CITY_NAME, "CITYKEY": CITY_KEY, "LAT": lat, "LNG": lng})


def cmd115(station_name: str, lat=LAT, lng=LNG):
    return post({
        "CMD": "115", "CITYNAME": CITY_NAME, "CITYKEY": CITY_KEY,
        "STATIONNAME": station_name, "MYLAT": lat, "MYLNG": lng, "ALL": "0",
    })


def cmd104(line_name: str, direction: str, order: int):
    return post({
        "CMD": "104", "CITYNAME": CITY_NAME, "CITYKEY": CITY_KEY,
        "LINENAME": line_name, "DIRECTION": direction, "STATIONORDER": str(order),
    })


def cmd103(line_name: str, direction: str):
    return post({
        "CMD": "103", "CITYNAME": CITY_NAME, "CITYKEY": CITY_KEY,
        "LINENAME": line_name, "DIRECTION": direction,
    })


def simulate_parse_arrival(l: dict) -> str:
    """模拟 Rust 端 parse_arrival 逻辑，验证解析结果"""
    neartext = l.get("neartext", "").strip()
    neartime = l.get("neartime", "").strip()
    nearnum = l.get("nearnum", 0)
    neardis = l.get("neardis", "").strip()

    if not neartext:
        return "Unknown"
    if any(k in neartext for k in ("即将到站", "进站中", "已到站")):
        return "Arriving"
    if nearnum < 0 or any(k in neartext for k in ("等待发车", "未发车", "停运", "收班", "非运营")):
        return "NoService"

    # stations
    stations = nearnum if nearnum > 0 else extract_stations(neartext)
    # minutes: neartime > text > distance estimate
    minutes = parse_time_field(neartime) or extract_minutes(neartext) or estimate_from_distance(neardis)
    # distance
    distance = parse_distance(neardis)

    if stations or minutes:
        return f"Approaching(stations={stations or 0}, minutes={minutes}, dist={distance})"
    return "Unknown"


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


def parse_time_field(neartime: str) -> int | None:
    if not neartime:
        return None
    if "小于" in neartime or "不到" in neartime:
        return 0
    num = "".join(c for c in neartime if c.isdigit())
    return int(num) if num else None


def parse_distance(neardis: str) -> int | None:
    if not neardis:
        return None
    for suffix in ("km", "公里"):
        if neardis.endswith(suffix):
            try:
                return int(float(neardis[:-len(suffix)].strip()) * 1000)
            except ValueError:
                return None
    for suffix in ("m", "米"):
        if neardis.endswith(suffix):
            try:
                return int(float(neardis[:-len(suffix)].strip()))
            except ValueError:
                return None
    try:
        return int(float(neardis))
    except ValueError:
        return None


def estimate_from_distance(neardis: str) -> int | None:
    d = parse_distance(neardis)
    if d and d > 0:
        return max(1, d // 400)
    return None


def run():
    print(f"城市: {CITY_NAME} ({CITY_KEY})  坐标: {LAT}, {LNG}\n")

    # Step 1: 附近站点
    nearby = cmd106()
    stations = nearby.get("data", [])
    print(f"[106] 附近 {len(stations)} 个站点:")
    for s in stations[:8]:
        print(f"  {s['name']} ({s['dis']}m)")

    # Step 2: 逐站查 CMD 115
    active_station = None
    lines_data = []
    for s in stations[:6]:
        name = s.get("name", "")
        if not name:
            continue
        resp = cmd115(name)
        lines_data = resp.get("data", [])
        if lines_data:
            active_station = name
            break

    if not active_station:
        print("\n[115] 所有站点均无实时数据")
        print("      跳转 CMD 104 直接查询...\n")
        fallback_cmd104()
        return

    # Step 3: CMD 115 完整字段 + 解析验证
    print(f"\n[115] {active_station} — {len(lines_data)} 条线路")
    print("=" * 70)
    for i, l in enumerate(lines_data):
        parsed = simulate_parse_arrival(l)
        print(f"\n  线路 {i+1}: {l.get('lineName', '?')}")
        print(f"    原始: {json.dumps(l, ensure_ascii=False)}")
        print(f"    解析: {parsed}")
    print("=" * 70)

    # Step 4: CMD 104 对比（验证 routeOnStationRTimeInfoList）
    print(f"\n[104] 对比 CMD 115 vs CMD 104:")
    checked = 0
    for l in lines_data:
        if checked >= 3:
            break
        line_name = l.get("lineName", "")
        order = l.get("stationOrder", 1)
        if not line_name:
            continue
        try:
            data = cmd104(line_name, "1", order)
        except Exception as e:
            print(f"  {line_name}: 请求失败 ({e})")
            continue

        rtime = data.get("routeOnStationRTimeInfoList", [])
        has_real = data.get("hasReal", 0)
        run_state = data.get("runState", -1)
        print(f"\n  {line_name} 站序{order} (hasReal={has_real} runState={run_state}):")
        print(f"    CMD115: neartext=\"{l.get('neartext','')}\" neartime=\"{l.get('neartime','')}\" "
              f"nearnum={l.get('nearnum',0)} neardis=\"{l.get('neardis','')}\"")
        if rtime:
            for r in rtime:
                print(f"    CMD104: {json.dumps(r, ensure_ascii=False)}")
        else:
            print(f"    CMD104: (无到站预估)")
        checked += 1


def fallback_cmd104():
    """CMD 115 无数据时，用 CMD 119 获取线路列表再查 CMD 104"""
    all_lines_resp = post({
        "CMD": "119", "CITYNAME": CITY_NAME, "CITYKEY": CITY_KEY, "KEY": "",
    })
    all_lines = all_lines_resp.get("buslines", [])
    if not all_lines:
        print("  CMD 119 也无数据")
        return

    print(f"  [119] 共 {len(all_lines)} 条线路，检查前5条:")
    checked = 0
    for line in all_lines[:10]:
        if checked >= 5:
            break
        line_name = line.get("lineName", "")
        if not line_name:
            continue
        try:
            data = cmd104(line_name, "1", 3)
        except Exception:
            continue
        rtime = data.get("routeOnStationRTimeInfoList", [])
        has_real = data.get("hasReal", 0)
        run_state = data.get("runState", -1)
        if has_real == 0 and not rtime:
            continue
        print(f"\n  {line_name} (hasReal={has_real} runState={run_state}):")
        for r in rtime[:3]:
            print(f"    {json.dumps(r, ensure_ascii=False)}")
        buses = data.get("list", [])
        if buses:
            print(f"    buses[0]: {json.dumps(buses[0], ensure_ascii=False)}")
        checked += 1

    if checked == 0:
        print("  所有线路均无实时数据（非运营时段）")


if __name__ == "__main__":
    run()
