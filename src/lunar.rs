use chrono::NaiveDate;

pub const MIN_YEAR: i32 = 1900;
const BASE_MONTH: u32 = 1;
const BASE_DAY: u32 = 31;

const LUNAR_INFO: [u32; 213] = [
    0x04bd8, 0x04ae0, 0x0a570, 0x054d5, 0x0d260, 0x0d950, 0x16554, 0x056a0, 0x09ad0, 0x055d2,
    0x04ae0, 0x0a5b6, 0x0a4d0, 0x0d250, 0x1d255, 0x0b540, 0x0d6a0, 0x0ada2, 0x095b0, 0x14977,
    0x04970, 0x0a4b0, 0x0b4b5, 0x06a50, 0x06d40, 0x1ab54, 0x02b60, 0x09570, 0x052f2, 0x04970,
    0x06566, 0x0d4a0, 0x0ea50, 0x06e95, 0x05ad0, 0x02b60, 0x186e3, 0x092e0, 0x1c8d7, 0x0c950,
    0x0d4a0, 0x1d8a6, 0x0b550, 0x056a0, 0x1a5b4, 0x025d0, 0x092d0, 0x0d2b2, 0x0a950, 0x0b557,
    0x06ca0, 0x0b550, 0x15355, 0x04da0, 0x0a5d0, 0x14573, 0x052d0, 0x0a9a8, 0x0e950, 0x06aa0,
    0x0aea6, 0x0ab50, 0x04b60, 0x0aae4, 0x0a570, 0x05260, 0x0f263, 0x0d950, 0x05b57, 0x056a0,
    0x096d0, 0x04dd5, 0x04ad0, 0x0a4d0, 0x0d4d4, 0x0d250, 0x0d558, 0x0b540, 0x0b5a0, 0x195a6,
    0x095b0, 0x049b0, 0x0a974, 0x0a4b0, 0x0b27a, 0x06a50, 0x06d40, 0x0af46, 0x0ab60, 0x09570,
    0x04af5, 0x04970, 0x064b0, 0x074a3, 0x0ea50, 0x06b58, 0x05ac0, 0x0ab60, 0x096d5, 0x092e0,
    0x0c960, 0x0d954, 0x0d4a0, 0x0da50, 0x07552, 0x056a0, 0x0abb7, 0x025d0, 0x092d0, 0x0cab5,
    0x0a950, 0x0b4a0, 0x0baa4, 0x0ad50, 0x055d9, 0x04ba0, 0x0a5b0, 0x15176, 0x052b0, 0x0a930,
    0x07954, 0x06aa0, 0x0ad50, 0x05b52, 0x04b60, 0x0a6e6, 0x0a4e0, 0x0d260, 0x0ea65, 0x0d530,
    0x05aa0, 0x076a3, 0x096d0, 0x04bd7, 0x04ad0, 0x0a4d0, 0x1d0b6, 0x0d250, 0x0d520, 0x0dd45,
    0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255, 0x06d20, 0x0ada0,
    0x14b63, 0x09370, 0x049f8, 0x04970, 0x064b0, 0x168a6, 0x0ea50, 0x06b20, 0x1a6c4, 0x0aae0,
    0x0a2e0, 0x0d2e3, 0x0c960, 0x0d557, 0x0d4a0, 0x0da50, 0x05d55, 0x056a0, 0x0a6d0, 0x055d4,
    0x052d0, 0x0a9b8, 0x0a950, 0x0b4a0, 0x0b6a6, 0x0ad50, 0x055a0, 0x0aba4, 0x0a5b0, 0x052b0,
    0x0b273, 0x06930, 0x07337, 0x06aa0, 0x0ad50, 0x04b55, 0x04b60, 0x0a570, 0x054e4, 0x0d160,
    0x0e968, 0x0d520, 0x0daa0, 0x16aa6, 0x056d0, 0x04ae0, 0x0a9d4, 0x0a2d0, 0x0d150, 0x0f252,
    0x0d520, 0x0dd45, 0x0b5a0, 0x056d0, 0x055b2, 0x049b0, 0x0a577, 0x0a4b0, 0x0aa50, 0x1b255,
    0x06d20, 0x0ada0, 0x14b63,
];

const STEMS: [char; 10] = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
const BRANCHES: [char; 12] = [
    '子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥',
];
const ZODIAC: [char; 12] = [
    '鼠', '牛', '虎', '兔', '龙', '蛇', '马', '羊', '猴', '鸡', '狗', '猪',
];
const MONTH_NAMES: [char; 12] = [
    '正', '二', '三', '四', '五', '六', '七', '八', '九', '十', '冬', '腊',
];
const DAY_NAMES: [&str; 30] = [
    "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一", "十二",
    "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二", "廿三", "廿四",
    "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
];
const LUNAR_FESTIVALS: [((u8, u8), &str); 10] = [
    ((1, 1), "春节"),
    ((1, 15), "元宵节"),
    ((2, 2), "龙抬头"),
    ((5, 5), "端午节"),
    ((7, 7), "七夕节"),
    ((7, 15), "中元节"),
    ((8, 15), "中秋节"),
    ((9, 9), "重阳节"),
    ((12, 8), "腊八节"),
    ((12, 23), "小年"),
];

#[derive(Clone, Copy, Debug)]
pub struct LunarDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub is_leap: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct LunarInfo {
    pub date: LunarDate,
    pub festival: Option<&'static str>,
}

impl LunarInfo {
    pub fn display_label(&self) -> &'static str {
        self.festival.unwrap_or_else(|| day_name_for(self.date.day))
    }

    pub fn month_label(&self) -> String {
        let prefix = if self.date.is_leap { "闰" } else { "" };
        format!("{}{}月", prefix, month_name_for(self.date.month))
    }
}

pub fn max_supported_year() -> i32 {
    MIN_YEAR + (LUNAR_INFO.len() as i32) - 1
}

pub fn solar_to_lunar(date: NaiveDate) -> Option<LunarInfo> {
    let base = NaiveDate::from_ymd_opt(MIN_YEAR, BASE_MONTH, BASE_DAY)?;
    let mut offset = date.signed_duration_since(base).num_days();
    if offset < 0 {
        return None;
    }

    let mut year = MIN_YEAR;
    let max_year = max_supported_year();
    while year <= max_year {
        let year_days = lunar_year_days(year) as i64;
        if offset < year_days {
            break;
        }
        offset -= year_days;
        year += 1;
    }

    if year > max_year {
        return None;
    }

    let mut month = 1;
    let mut is_leap = false;
    let leap = leap_month(year);

    loop {
        let days_in_month = if is_leap {
            leap_days(year)
        } else {
            month_days(year, month)
        } as i64;

        if offset < days_in_month {
            break;
        }

        offset -= days_in_month;
        if leap != 0 && month == leap as i32 && !is_leap {
            is_leap = true;
        } else {
            if is_leap {
                is_leap = false;
            }
            month += 1;
        }
    }

    let day = (offset + 1) as u8;
    let mut festival = if is_leap {
        None
    } else {
        lunar_festival(month as u8, day)
    };
    if !is_leap && month == 12 {
        let last_day = month_days(year, 12);
        if day == last_day {
            festival = Some("除夕");
        }
    }

    Some(LunarInfo {
        date: LunarDate {
            year,
            month: month as u8,
            day,
            is_leap,
        },
        festival,
    })
}

pub fn gan_zhi_year(year: i32) -> String {
    let stem = STEMS[((year - 4).rem_euclid(10)) as usize];
    let branch = BRANCHES[((year - 4).rem_euclid(12)) as usize];
    format!("{}{}", stem, branch)
}

pub fn zodiac_animal(year: i32) -> char {
    ZODIAC[((year - 4).rem_euclid(12)) as usize]
}

fn lunar_festival(month: u8, day: u8) -> Option<&'static str> {
    LUNAR_FESTIVALS
        .iter()
        .find(|((m, d), _)| *m == month && *d == day)
        .map(|(_, name)| *name)
}

fn lunar_year_days(year: i32) -> i32 {
    let mut sum = 348; // 12 * 29
    let info = year_info(year).unwrap_or(0);
    let mut mask = 0x8000;
    while mask > 0x8 {
        if info & mask != 0 {
            sum += 1;
        }
        mask >>= 1;
    }
    sum + leap_days(year) as i32
}

fn leap_month(year: i32) -> u8 {
    (year_info(year).unwrap_or(0) & 0xF) as u8
}

fn leap_days(year: i32) -> u8 {
    let leap = leap_month(year);
    if leap != 0 {
        if year_info(year).unwrap_or(0) & 0x10000 != 0 {
            30
        } else {
            29
        }
    } else {
        0
    }
}

fn month_name_for(month: u8) -> char {
    let index = month.saturating_sub(1) as usize;
    MONTH_NAMES.get(index).copied().unwrap_or(MONTH_NAMES[0])
}

fn day_name_for(day: u8) -> &'static str {
    let index = day.saturating_sub(1) as usize;
    DAY_NAMES.get(index).copied().unwrap_or(DAY_NAMES[0])
}

fn month_days(year: i32, month: i32) -> u8 {
    if year < MIN_YEAR || year > max_supported_year() {
        return 29;
    }
    let info = year_info(year).unwrap_or(0);
    if info & (0x10000 >> month) != 0 {
        30
    } else {
        29
    }
}

fn year_info(year: i32) -> Option<u32> {
    if (MIN_YEAR..=max_supported_year()).contains(&year) {
        Some(LUNAR_INFO[(year - MIN_YEAR) as usize])
    } else {
        None
    }
}
