use crate::crawlers::starknet_crawler::build_address;
use crate::models::network_model::Network;
use crate::utils::mail_structure::write_mail;

use super::db::Database;
use dotenv::dotenv;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;

pub fn send_mail(receivers_email: String, message_body: String) {
    dotenv().ok();
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME not found");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not found");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .reply_to(smtp_username.parse().unwrap())
        .to(receivers_email.parse().unwrap())
        .subject("Transaction Notification")
        .header(ContentType::TEXT_PLAIN)
        .body(message_body)
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {e:?}"),
    }
}

pub async fn process_transactions_to_mail(
    transactions: Vec<&serde_json::Value>,
    blockchain: Network,
    db: Database,
) {
    for transaction in transactions {
        let transaction_hash = transaction["transaction_hash"].as_str().unwrap();
        let wallet_address = build_address(
            transaction["sender_address"]
                .as_str()
                .unwrap_or("0x0000000000000000000000")
                .to_string()
                .to_lowercase(),
        );
        let tx_url = format!("https://sepolia.starkscan.co/tx/{}", transaction_hash);
        let network = blockchain.as_str().unwrap_or("Starknet".to_string());

        match db
            .find_users_with_wallet_address(wallet_address.clone())
            .await
        {
            Ok(users) => {
                for user in users {
                    let name = user.name;
                    let email_address = user.email;
                    let email_body = write_mail(
                        name.clone(),
                        email_address.clone(),
                        network.clone(),
                        tx_url.clone(),
                        wallet_address.clone(),
                    );
                    println!("SENDING EMAIL TO: {}", name.clone());
                    send_mail(email_address.clone(), email_body);
                }
            }
            Err(e) => {
                println!("Error finding users with wallet address: {:?}", e);
                continue;
            }
        }
    }
}
