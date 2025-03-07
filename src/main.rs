use bitcoincore_rpc::{bitcoin::Address, Auth, Client, RpcApi};
fn main() {
    //connect to node
    let rpc_auth = Auth::UserPass("user".to_string(), "password".to_string());
    let rpc_url = "http://192.168.1.2:18332".to_string();
    let rpc = Client::new(rpc_url, rpc_auth).unwrap();

    //get mining info
    let mining_info = rpc.get_mining_info().unwrap();
    println!("{:?}", mining_info);

    //get balance
    let balance = rpc.get_balance(None, None).unwrap();
    let sat_balance = balance.as_sat();
    let btc_balance = balance.as_btc();
    println!("sats: {:?}", sat_balance);
    println!("btc: {:?}", btc_balance);
}
