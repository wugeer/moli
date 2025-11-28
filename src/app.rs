use chrono::{Datelike, Duration, Local, NaiveDate, NaiveDateTime};

use crate::lunar;

#[derive(Clone, Copy, Debug)]
pub enum HolidayCategory {
    Statutory,
    Traditional,
    OtherTraditional,
}

impl HolidayCategory {
    pub fn label(self) -> &'static str {
        match self {
            HolidayCategory::Statutory => "法定节假日",
            HolidayCategory::Traditional => "传统节日",
            HolidayCategory::OtherTraditional => "民俗节日",
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HolidayInfo {
    pub name: &'static str,
    pub category: HolidayCategory,
    pub note: &'static str,
}

#[derive(Clone, Copy, Debug)]
struct SolarHoliday {
    info: HolidayInfo,
    month: u32,
    day: u32,
}

const HOLIDAY_SPRING_FESTIVAL: HolidayInfo = HolidayInfo {
    name: "春节",
    category: HolidayCategory::Statutory,
    note: "农历正月初一 · 放假4天（除夕至初三）",
};
const HOLIDAY_SPRING_EVE: HolidayInfo = HolidayInfo {
    name: "除夕",
    category: HolidayCategory::Statutory,
    note: "春节前夜 · 合家团圆",
};
const HOLIDAY_NEW_YEAR: HolidayInfo = HolidayInfo {
    name: "元旦",
    category: HolidayCategory::Statutory,
    note: "公历1月1日 · 放假1天",
};
const HOLIDAY_LABOR_DAY: HolidayInfo = HolidayInfo {
    name: "劳动节",
    category: HolidayCategory::Statutory,
    note: "公历5月1日 · 放假2天",
};
const HOLIDAY_DRAGON_BOAT: HolidayInfo = HolidayInfo {
    name: "端午节",
    category: HolidayCategory::Statutory,
    note: "农历五月初五 · 放假1天",
};
const HOLIDAY_MID_AUTUMN: HolidayInfo = HolidayInfo {
    name: "中秋节",
    category: HolidayCategory::Statutory,
    note: "农历八月十五 · 放假1天",
};
const HOLIDAY_NATIONAL_DAY: HolidayInfo = HolidayInfo {
    name: "国庆节",
    category: HolidayCategory::Statutory,
    note: "公历10月1日至3日 · 放假3天",
};
const HOLIDAY_QINGMING: HolidayInfo = HolidayInfo {
    name: "清明节",
    category: HolidayCategory::Statutory,
    note: "清明时节 · 踏青祭祖 · 放假1天",
};
const HOLIDAY_LANTERN: HolidayInfo = HolidayInfo {
    name: "元宵节",
    category: HolidayCategory::Traditional,
    note: "农历正月十五 · 元宵赏灯",
};
const HOLIDAY_QIXI: HolidayInfo = HolidayInfo {
    name: "七夕节",
    category: HolidayCategory::Traditional,
    note: "农历七月初七 · 牛郎织女传说",
};
const HOLIDAY_CHONGYANG: HolidayInfo = HolidayInfo {
    name: "重阳节",
    category: HolidayCategory::Traditional,
    note: "农历九月初九 · 登高敬老",
};
const HOLIDAY_LONGTAITOU: HolidayInfo = HolidayInfo {
    name: "龙抬头",
    category: HolidayCategory::OtherTraditional,
    note: "农历二月初二 · 春耕开犁",
};
const HOLIDAY_ZHONGYUAN: HolidayInfo = HolidayInfo {
    name: "中元节",
    category: HolidayCategory::OtherTraditional,
    note: "农历七月十五 · 中元祭祖",
};
const HOLIDAY_LABA: HolidayInfo = HolidayInfo {
    name: "腊八节",
    category: HolidayCategory::OtherTraditional,
    note: "农历腊月初八 · 喝腊八粥",
};
const HOLIDAY_DONGZHI: HolidayInfo = HolidayInfo {
    name: "冬至",
    category: HolidayCategory::OtherTraditional,
    note: "冬至日 · 最重要节气之一",
};

const SOLAR_HOLIDAYS: &[SolarHoliday] = &[
    SolarHoliday {
        info: HOLIDAY_NEW_YEAR,
        month: 1,
        day: 1,
    },
    SolarHoliday {
        info: HOLIDAY_LABOR_DAY,
        month: 5,
        day: 1,
    },
    SolarHoliday {
        info: HOLIDAY_NATIONAL_DAY,
        month: 10,
        day: 1,
    },
];

const SOLAR_TERM_NAMES: [&str; 24] = [
    "小寒", "大寒", "立春", "雨水", "惊蛰", "春分", "清明", "谷雨", "立夏", "小满", "芒种", "夏至",
    "小暑", "大暑", "立秋", "处暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "冬至",
];

const SOLAR_TERM_OFFSETS: [i64; 24] = [
    0, 21208, 42467, 63836, 85337, 107014, 128867, 150921, 173149, 195551, 218072, 240693, 263343,
    285989, 308563, 331033, 353350, 375494, 397447, 419210, 440795, 462224, 483532, 504758,
];

const SOLAR_TERM_BASE_YEAR: i32 = 1900;
const SOLAR_TERM_MIN_YEAR: i32 = 1900;
const SOLAR_TERM_MAX_YEAR: i32 = 2100;
const SOLAR_TERM_YEAR_MS: f64 = 31_556_925_974.7;

#[derive(Clone, Copy, Debug)]
pub struct DayCell {
    pub date: NaiveDate,
    pub is_current_month: bool,
    pub is_today: bool,
    pub is_selected: bool,
    pub lunar: Option<lunar::LunarInfo>,
    pub holiday: Option<HolidayInfo>,
    pub solar_term: Option<&'static str>,
}

pub struct App {
    today: NaiveDate,
    view_year: i32,
    view_month: u32,
    selected_day: u32,
}

impl App {
    pub fn new() -> Self {
        let today = Local::now().date_naive();
        Self {
            view_year: today.year(),
            view_month: today.month(),
            selected_day: today.day(),
            today,
        }
    }

    pub fn view_year(&self) -> i32 {
        self.view_year
    }

    pub fn view_month(&self) -> u32 {
        self.view_month
    }

    pub fn today(&self) -> NaiveDate {
        self.today
    }

    pub fn selected_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(self.view_year, self.view_month, self.selected_day)
            .expect("invalid selected date")
    }

    pub fn month_rows(&self) -> Vec<Vec<DayCell>> {
        let first_day = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1).unwrap();
        let offset = first_day.weekday().num_days_from_monday() as i64;
        let start = first_day
            .checked_sub_signed(Duration::days(offset))
            .unwrap();
        let mut cursor = start;
        let mut rows = Vec::with_capacity(6);
        for _ in 0..6 {
            let mut week = Vec::with_capacity(7);
            for _ in 0..7 {
                let is_current_month =
                    cursor.month() == self.view_month && cursor.year() == self.view_year;
                let is_today = cursor == self.today;
                let is_selected = cursor == self.selected_date();
                let lunar = lunar::solar_to_lunar(cursor);
                let solar_term = solar_term_name(cursor);
                let holiday = holiday_for(cursor, lunar.as_ref(), solar_term);
                week.push(DayCell {
                    date: cursor,
                    is_current_month,
                    is_today,
                    is_selected,
                    lunar,
                    holiday,
                    solar_term,
                });
                cursor = cursor.succ_opt().unwrap();
            }
            rows.push(week);
        }
        rows
    }

    pub fn selected_lunar(&self) -> Option<lunar::LunarInfo> {
        lunar::solar_to_lunar(self.selected_date())
    }

    pub fn selected_solar_term(&self) -> Option<&'static str> {
        solar_term_name(self.selected_date())
    }

    pub fn selected_holiday(&self) -> Option<HolidayInfo> {
        let date = self.selected_date();
        let lunar = lunar::solar_to_lunar(date);
        let solar_term = self.selected_solar_term();
        holiday_for(date, lunar.as_ref(), solar_term)
    }

    pub fn month_anchor_lunar(&self) -> Option<lunar::LunarInfo> {
        let anchor = NaiveDate::from_ymd_opt(self.view_year, self.view_month, 1).unwrap();
        lunar::solar_to_lunar(anchor)
    }

    pub fn prev_month(&mut self) {
        if self.view_month == 1 {
            self.view_month = 12;
            self.view_year -= 1;
        } else {
            self.view_month -= 1;
        }
        self.constrain_year();
        self.sync_day();
    }

    pub fn next_month(&mut self) {
        if self.view_month == 12 {
            self.view_month = 1;
            self.view_year += 1;
        } else {
            self.view_month += 1;
        }
        self.constrain_year();
        self.sync_day();
    }

    pub fn prev_year(&mut self) {
        self.view_year -= 1;
        self.constrain_year();
        self.sync_day();
    }

    pub fn next_year(&mut self) {
        self.view_year += 1;
        self.constrain_year();
        self.sync_day();
    }

    pub fn back_to_today(&mut self) {
        self.view_year = self.today.year();
        self.view_month = self.today.month();
        self.selected_day = self.today.day();
    }

    pub fn move_selection(&mut self, delta_days: i64) {
        let current = self.selected_date();
        if let Some(mut new_date) = current.checked_add_signed(Duration::days(delta_days)) {
            let min_date = NaiveDate::from_ymd_opt(lunar::MIN_YEAR, 1, 1).unwrap();
            let max_date = NaiveDate::from_ymd_opt(lunar::max_supported_year(), 12, 31).unwrap();
            if new_date < min_date {
                new_date = min_date;
            } else if new_date > max_date {
                new_date = max_date;
            }
            self.view_year = new_date.year();
            self.view_month = new_date.month();
            self.selected_day = new_date.day();
        }
    }

    fn sync_day(&mut self) {
        let max_day = days_in_month(self.view_year, self.view_month);
        if self.selected_day > max_day {
            self.selected_day = max_day;
        }
    }

    fn constrain_year(&mut self) {
        let min_year = lunar::MIN_YEAR;
        let max_year = lunar::max_supported_year();
        if self.view_year < min_year {
            self.view_year = min_year;
        } else if self.view_year > max_year {
            self.view_year = max_year;
        }
    }
}

fn days_in_month(year: i32, month: u32) -> u32 {
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let next = match month {
        12 => NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap(),
        _ => NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap(),
    };
    (next - first).num_days() as u32
}

fn holiday_for(
    date: NaiveDate,
    lunar: Option<&lunar::LunarInfo>,
    solar_term: Option<&'static str>,
) -> Option<HolidayInfo> {
    solar_holiday(date)
        .or_else(|| qingming_holiday(solar_term))
        .or_else(|| lunar_statutory_holiday(lunar))
        .or_else(|| major_traditional_holiday(lunar))
        .or_else(|| other_traditional_holiday(lunar, solar_term))
}

fn solar_holiday(date: NaiveDate) -> Option<HolidayInfo> {
    SOLAR_HOLIDAYS.iter().find_map(|holiday| {
        let target = NaiveDate::from_ymd_opt(date.year(), holiday.month, holiday.day)?;
        if date == target {
            Some(holiday.info)
        } else {
            None
        }
    })
}

fn qingming_holiday(solar_term: Option<&'static str>) -> Option<HolidayInfo> {
    solar_term
        .filter(|term| *term == "清明")
        .map(|_| HOLIDAY_QINGMING)
}

fn lunar_statutory_holiday(lunar: Option<&lunar::LunarInfo>) -> Option<HolidayInfo> {
    let info = lunar?;
    let month = info.date.month;
    let day = info.date.day;
    if info.festival == Some("除夕") {
        return Some(HOLIDAY_SPRING_EVE);
    }
    if month == 1 && day <= 3 {
        return Some(HOLIDAY_SPRING_FESTIVAL);
    }
    match (month, day) {
        (5, 5) => Some(HOLIDAY_DRAGON_BOAT),
        (8, 15) => Some(HOLIDAY_MID_AUTUMN),
        _ => None,
    }
}

fn major_traditional_holiday(lunar: Option<&lunar::LunarInfo>) -> Option<HolidayInfo> {
    let info = lunar?;
    match (info.date.month, info.date.day) {
        (1, 15) => Some(HOLIDAY_LANTERN),
        (7, 7) => Some(HOLIDAY_QIXI),
        (9, 9) => Some(HOLIDAY_CHONGYANG),
        _ => None,
    }
}

fn other_traditional_holiday(
    lunar: Option<&lunar::LunarInfo>,
    solar_term: Option<&'static str>,
) -> Option<HolidayInfo> {
    if let Some(info) = lunar {
        match (info.date.month, info.date.day) {
            (2, 2) => return Some(HOLIDAY_LONGTAITOU),
            (7, 15) => return Some(HOLIDAY_ZHONGYUAN),
            (12, 8) => return Some(HOLIDAY_LABA),
            _ => {}
        }
    }
    if solar_term == Some("冬至") {
        return Some(HOLIDAY_DONGZHI);
    }
    None
}

fn solar_term_name(date: NaiveDate) -> Option<&'static str> {
    if !(SOLAR_TERM_MIN_YEAR..=SOLAR_TERM_MAX_YEAR).contains(&date.year()) {
        return None;
    }
    let base = solar_term_base_datetime()?;
    SOLAR_TERM_NAMES
        .iter()
        .enumerate()
        .find_map(|(idx, &name)| {
            solar_term_date_from_base(base, date.year(), idx)
                .and_then(|term_date| (term_date == date).then_some(name))
        })
}

fn solar_term_base_datetime() -> Option<NaiveDateTime> {
    NaiveDate::from_ymd_opt(SOLAR_TERM_BASE_YEAR, 1, 6)?.and_hms_opt(2, 5, 0)
}

fn solar_term_date_from_base(base: NaiveDateTime, year: i32, index: usize) -> Option<NaiveDate> {
    let offset = solar_term_offset_ms(year, index)?;
    base.checked_add_signed(Duration::milliseconds(offset))
        .map(|dt| dt.date())
}

fn solar_term_offset_ms(year: i32, index: usize) -> Option<i64> {
    let minutes = *SOLAR_TERM_OFFSETS.get(index)?;
    let year_offset = (year - SOLAR_TERM_BASE_YEAR) as f64 * SOLAR_TERM_YEAR_MS;
    let term_offset = minutes as f64 * 60_000.0;
    Some((year_offset + term_offset).round() as i64)
}
