use crate::common::sleep_while;
use log::info;

pub fn do_homework(work: &str) {
    info!("{} ödevine çalışmaya başladım", work);
    sleep_while(4.0);
    info!("Ödevler bitti");
}
