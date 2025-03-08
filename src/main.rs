use std::collections::HashMap;

use bitcoincore_rpc::{ bitcoin::Amount, json, Auth, Client, RpcApi};
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

    //list all UTXOs
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
    
    //select first UTXO to use in the transaction
    let selected_tx = &unspent[0];
    
    println!("selected unspent transaction: {:#?}", selected_tx);

    //make transaction inputs
    let txid = selected_tx.txid;
    let vout = selected_tx.vout;
    let tx_input = json::CreateRawTransactionInput {
        txid: txid,
        vout: vout,
        sequence: None,
    };

    //make transaction outputs
    let amount = selected_tx.amount;
    let fee = Amount::from_sat(1000);
    let amount_to_spend = amount - fee;
    let mut tx_output = HashMap::new();
    tx_output.insert(my_address.to_string(), amount_to_spend);

    //make unsigned transaction
    let unsigned_tx = rpc
        .create_raw_transaction(&[tx_input], &tx_output, None, None)
        .unwrap();
    
    //sign transaction
    let signed_tx = rpc
        .sign_raw_transaction_with_wallet(&unsigned_tx, None, None)
        .unwrap();
    
    println!("signed tx {:?}", signed_tx.transaction().unwrap());

    //send transaction
    let txid_sent = rpc
        .send_raw_transaction(&signed_tx.transaction().unwrap())
        .unwrap();

    println!("{:?}", txid_sent);
}