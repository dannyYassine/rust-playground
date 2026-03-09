use component::Injectable;
use tokio::sync::Mutex;

#[derive(Injectable)]
pub struct IncrementCounterUseCase {}

impl IncrementCounterUseCase {
    pub async fn execute(&self, count: &Mutex<isize>) -> isize {
        let mut counter = count.lock().await;
        *counter += 1;
        *counter
    }
}
