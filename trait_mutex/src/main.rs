use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
enum TestError {
    Internal,
}

#[async_trait]
trait State: Send + Sync + 'static {
    async fn execute(&mut self) -> Result<(), TestError>;
}

#[async_trait]
trait Shim: Send + Sync + 'static {
    async fn run(&mut self, data: Vec<u8>) -> Result<(), TestError>;
}

struct MyState {
    shim: Arc<Mutex<dyn Shim>>,
}

#[async_trait]
impl State for MyState {
    async fn execute(&mut self) -> Result<(), TestError> {
        {
            let mut shimptr = self.shim.lock().await;
            shimptr.run(vec![]).await?;
        }

        Ok(())
    }
}

struct MyShim {}

#[async_trait]
impl Shim for MyShim {
    async fn run(&mut self, data: Vec<u8>) -> Result<(), TestError> {
        println!("{}", data.len());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), TestError> {
    let mut state = MyState {
        shim: Arc::new(Mutex::new(MyShim {})),
    };

    state.execute().await?;

    println!("End!");

    Ok(())
}
