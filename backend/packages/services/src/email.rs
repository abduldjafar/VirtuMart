use environment::Environment;
use errors::Result;
use lettre::{
    message::{Mailbox, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

#[derive(Clone)]
pub struct EmailServices {}

impl EmailServices {
    #[tracing::instrument(err, skip_all)]
    pub async fn send_verification_email(
        username: &str,
        email_reciever: &str,
        verify_token: &str,
    ) -> Result<()> {
        let environment = Environment::new();
        // Define the HTML content
        let html_content = format!(
            r#"
        <html>
            <body style="font-family: Arial, sans-serif; background-color: #f4f4f4; padding: 20px;">
                <div style="max-width: 600px; margin: 0 auto; background-color: #ffffff; padding: 20px; border-radius: 10px; box-shadow: 0 0 10px rgba(0,0,0,0.1);">
                    <h2 style="color: #333333; text-align: center;">Welcome to GymConnect!</h2>
                    <p style="font-size: 16px; color: #555555;">
                        Hi there,
                    </p>
                    <p style="font-size: 16px; color: #555555;">
                        Thank you for signing up with GymConnect! To complete your registration, please verify your account by clicking the button below:
                    </p>
                    <div style="text-align: center; margin: 30px 0;">
                        <a href="{}{}" 
                           style="background-color: #4CAF50; color: white; padding: 15px 25px; text-decoration: none; font-size: 18px; border-radius: 5px; display: inline-block;">
                            Verify My Account
                        </a>
                    </div>
                    <p style="font-size: 16px; color: #555555;">
                        If the button above doesn't work, copy and paste the following link into your browser:
                    </p>
                    <p style="font-size: 14px; color: #555555; word-wrap: break-word;">
                        <a href="{}{}" style="color: #4CAF50;">
                            {}{}
                        </a>
                    </p>
                    <p style="font-size: 16px; color: #555555;">
                        If you did not sign up for this account, please ignore this email.
                    </p>
                    <p style="font-size: 16px; color: #555555;">
                        Best regards,<br>
                        The GymConnect Team
                    </p>
                </div>
            </body>
        </html>
    "#,
            environment.host_name,
            verify_token,
            environment.host_name,
            verify_token,
            environment.host_name,
            verify_token
        );

        let from_email = "GymConnect <gymconnectdev777@gmail.com>"
            .parse::<Mailbox>()
            .unwrap();
        let to_email = format!("{} <{}>", username, email_reciever)
            .parse::<Mailbox>()
            .unwrap();

        // Define the email with HTML part
        let email = Message::builder()
            .from(from_email)
            .to(to_email)
            .subject("Verify your email !!!")
            .multipart(
                MultiPart::alternative().singlepart(SinglePart::html(html_content.to_string())),
            )
            .unwrap();

        // Set up the SMTP client credentials
        let creds = Credentials::new(
            "gymconnectdev777@gmail.com".to_owned(),
            "ostq xzll fouj ctno".to_owned(),
        );

        // Open a remote connection to the SMTP server with STARTTLS
        let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")
            .unwrap()
            .credentials(creds)
            .build();

        // Send the email
        let _ = match mailer.send(&email) {
            Ok(_) => Ok(()),
            Err(e) => Err(errors::Error::SmtpProcessingError(e.to_string())),
        };

        Ok(())
    }
}
