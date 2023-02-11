extern crate bins;

use chrono::Weekday::{Sat, Sun};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use itertools::Itertools;

use crate::day::Label::{Saturday, Sunday, Weekday};

pub type Week = Vec<Day>;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Day {
    pub label: Label,
    pub today: bool,
    pub this_month: bool,
    value: u32,
}

impl Day {
    fn new(datetime: DateTime<Utc>, today: DateTime<Utc>, request: DateTime<Utc>) -> Self {
        Self {
            label: match datetime.weekday() {
                Sun => Sunday,
                Sat => Saturday,
                _ => Weekday,
            },
            today: Self::same_day(datetime, today),
            this_month: Self::same_month(datetime, request),
            value: datetime.day(),
        }
    }

    fn same_day(x: DateTime<Utc>, y: DateTime<Utc>) -> bool {
        x.year() == y.year() && x.month() == y.month() && x.day() == y.day()
    }

    fn same_month(x: DateTime<Utc>, y: DateTime<Utc>) -> bool {
        x.month() == y.month()
    }

    pub fn show(&self) -> String {
        format!("{:>02}", self.value)
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Label {
    Sunday,
    Weekday,
    Saturday,
}

pub fn prev_page(request: DateTime<Utc>) -> DateTime<Utc> {
    let (y, m) = match (request.year(), request.month()) {
        (y, 1) => (y - 1, 12),
        (y, m) => (y, m - 1),
    };
    NaiveDate::from_ymd_opt(y, m, 1).unwrap().and_hms_micro_opt(0, 0, 0, 0).unwrap().and_local_timezone(Utc).unwrap()
}

pub fn next_page(request: DateTime<Utc>) -> DateTime<Utc> {
    let (y, m) = match (request.year(), request.month()) {
        (y, 12) => (y + 1, 1),
        (y, m) => (y, m + 1),
    };
    NaiveDate::from_ymd_opt(y, m, 1).unwrap().and_hms_micro_opt(0, 0, 0, 0).unwrap().and_local_timezone(Utc).unwrap()
}

pub fn get_weeks(today: DateTime<Utc>, request: DateTime<Utc>) -> Vec<Week> {
    get_left_side_days(request)
        .into_iter()
        .map(|left_side_day| create_week(left_side_day, today, request))
        .collect_vec()
}

fn get_left_side_days(request: DateTime<Utc>) -> Vec<DateTime<Utc>> {
    let first_day = request - Duration::days(request.day() as i64 - 1);

    let left_top_day = if first_day.weekday() == Sun {
        first_day
    } else {
        first_day - Duration::days(first_day.weekday().number_from_monday() as i64)
    };

    (0..5).map(|i| left_top_day + Duration::days(i * 7)).collect_vec()
}

fn create_week(left_side_day: DateTime<Utc>, today: DateTime<Utc>, request: DateTime<Utc>) -> Week {
    vec![
        vec![Day::new(left_side_day, today, request)],
        (1..6).map(|i| Day::new(left_side_day + Duration::days(i), today, request)).collect_vec(),
        vec![Day::new(left_side_day + Duration::days(6), today, request)],
    ]
    .into_iter()
    .flatten()
    .collect_vec()
}

#[cfg(test)]
mod tests {
    use chrono::{Datelike, NaiveDate, Utc};
    use rstest::rstest;

    use crate::day::{get_left_side_days, next_page, prev_page};

    #[rstest]
    #[case(2023, 1, 1, 1, 29)]
    #[case(2023, 2, 1, 29, 26)]
    #[case(2023, 2, 2, 29, 26)]
    #[case(2023, 2, 28, 29, 26)]
    #[case(2023, 3, 1, 26, 26)]
    fn test_get_left_side_days(#[case] y: i32, #[case] m: u32, #[case] d: u32, #[case] lt: u32, #[case] lb: u32) {
        let request = NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_micro_opt(0, 0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();

        let act = get_left_side_days(request);

        assert_eq!(act[0].day(), lt);
        assert_eq!(act[4].day(), lb);
    }

    #[rstest]
    #[case(2022, 12, 1, 2023, 1)]
    #[case(2022, 12, 31, 2023, 1)]
    #[case(2023, 1, 1, 2023, 2)]
    #[case(2023, 1, 31, 2023, 2)]
    fn test_next_page(#[case] y: i32, #[case] m: u32, #[case] d: u32, #[case] exp_y: i32, #[case] exp_m: u32) {
        let request = NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_micro_opt(0, 0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();

        let act = next_page(request);

        assert_eq!(act.year(), exp_y);
        assert_eq!(act.month(), exp_m);
    }

    #[rstest]
    #[case(2022, 12, 1, 2022, 11)]
    #[case(2022, 12, 31, 2022, 11)]
    #[case(2023, 1, 1, 2022, 12)]
    #[case(2023, 1, 31, 2022, 12)]
    fn test_prev_page(#[case] y: i32, #[case] m: u32, #[case] d: u32, #[case] exp_y: i32, #[case] exp_m: u32) {
        let request = NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_micro_opt(0, 0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .unwrap();

        let act = prev_page(request);

        assert_eq!(act.year(), exp_y);
        assert_eq!(act.month(), exp_m);
    }
}
