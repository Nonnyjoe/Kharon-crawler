pub fn write_mail(
    name: String,
    user_email: String,
    blockchain: String,
    transaction_url: String,
    wallet_address: String,
) -> String {
    let address_structure: String = format!(
        "{}.....{}",
        wallet_address[0..7].to_string(),
        wallet_address[wallet_address.len() - 7..wallet_address.len()].to_string()
    );

    let body = format!(
        "
        Dear {},
        Your wallet address {}, Just trigered a transaction on {}, Find more details about this transaction by accessing starkscan through this link:
        {}.

        You've received this mail because you subscribed for notifications to this email {}, on Kharon. 
        Thank you. 
    
        ", name, address_structure, blockchain, transaction_url, user_email
    );

    return body;
}
