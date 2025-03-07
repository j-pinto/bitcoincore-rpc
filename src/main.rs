use bitcoincore_rpc::{ bitcoin::{util::amount, Amount}, json, Auth, Client, RpcApi};
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

    //new address
    let my_address = rpc
        .get_new_address(None, Option::Some(json::AddressType::Bech32))
        .unwrap();
    println!("my address: {:?}", my_address);

    //select first UTXO
    let unspent = rpc
        .list_unspent(
            None,
            None,
            None,
            None,
            Option::Some(json::ListUnspentQueryOptions {
                minimum_amount: Option::Some(Amount::from_btc(0.00000001).unwrap()),
                maximum_amount: None,
                maximum_count: None,
                minimum_sum_amount: None,
            }),
        )
        .unwrap();
    
    let selected_tx = &unspent[0];
    
    println!("selected unspent transaction: {:#?}", selected_tx);
}
