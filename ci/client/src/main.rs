fn main() {
    println!("Yes have some sdk_dep {:?}", solana_sdk::signature::Signature::default());

    // Under development
    println!("Yes have some memo_dep 1.1 {:?}", spl_memo_1_1::id());
    println!("Yes have some token_dep 1.1 {:?}", spl_token_1_1::id());
    println!("Yes have some token_swap_dep 1.0 {:?}", spl_token_swap_1_0::id());

    // Deployed
    println!("Yes have some memo_dep 1.0 {:?}", spl_memo_1_0::id());
    println!("Yes have some token_dep 1.0 {:?}", spl_token_1_0::id());
}
