use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

pub async fn connect_db() -> Result<Client, mongodb::error::Error> {
    let uri = "mongodb+srv://rudra:jobshub@jobshubdb.xmysr5j.mongodb.net/jobshubdb";
    let mut client_options = ClientOptions::parse_async(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    Client::with_options(client_options)
}
