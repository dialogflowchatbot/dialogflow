pub(crate) const DEMO_COLLECT: &str = include_str!("demo_collect.txt");
pub(crate) const DEMO_NOTIFY: &str = include_str!("demo_notify.txt");
pub(crate) const DEMO_REPAY: &str = include_str!("demo_repay.txt");
pub(crate) const DEMO_COLLECT_EN: &str = include_str!("demo_collect_en.txt");
pub(crate) const DEMO_NOTIFY_EN: &str = include_str!("demo_notify_en.txt");
pub(crate) const DEMO_REPAY_EN: &str = include_str!("demo_repay_en.txt");

pub(crate) fn get_demo<'a>(is_en: bool, name: &'a str) -> Option<&'static str> {
    if name.eq("demo-collect") {
        Some(if is_en { DEMO_COLLECT_EN } else { DEMO_COLLECT })
    } else if name.eq("demo-notify") {
        Some(if is_en { DEMO_NOTIFY_EN } else { DEMO_NOTIFY })
    } else if name.eq("demo-repay") {
        Some(if is_en { DEMO_REPAY_EN } else { DEMO_REPAY })
    } else {
        None
    }
}
