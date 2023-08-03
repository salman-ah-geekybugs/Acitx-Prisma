use chrono::{Utc, Local};
use tokio_schedule::{every,Job};

use crate::prisma::PrismaClient;

pub async fn start_test_cron(client: &PrismaClient){
	let task = every(1).seconds().in_timezone(&Utc).perform(|| async {
		println!("schedule_task event - {:?}", Local::now())
	});
	task.await;
}