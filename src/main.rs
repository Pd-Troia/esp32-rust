use webserver::web_server;

mod webserver;

fn main() {   
   dotenv::dotenv().ok();
   let _ = web_server();
}
 