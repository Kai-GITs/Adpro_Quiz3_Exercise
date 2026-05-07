use exercise_2::quizprep::quiz_service_server::QuizServiceServer;
use exercise_2::QuizApp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let service = QuizApp::with_seed_data();

    println!("QuizService listening on {addr}");
    tonic::transport::Server::builder()
        .add_service(QuizServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
