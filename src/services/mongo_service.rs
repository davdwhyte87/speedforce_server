use mongodb::{Client, Database};
use mongodb::options::ClientOptions;

pub struct MongoService{
    pub db:Database
}

impl MongoService{
   pub async fn  init()->MongoService{
       // Parse a connection string into an options struct.
       let mut client_options = ClientOptions::parse("mongodb://localhost/hdos").await.unwrap();

       // Manually set an option.
       client_options.app_name = Some("hdos".to_string());

       // Get a handle to the deployment.
       let client = Client::with_options(client_options);


       let db = client.unwrap().database("hdos");

       return MongoService{db}
   }
}