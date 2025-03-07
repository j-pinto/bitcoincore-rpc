use bitcoincore_rpc::{Auth, Client, RpcApi};
fn main() {
    let rpc_auth = Auth::UserPass("user".to_string(), "password".to_string());
    let rpc_url = "http://192.168.1.2:18332".to_string();
    
    let rpc = Client::new(rpc_url, rpc_auth).unwrap();

    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:?}", mining_info);
}
