// test-link: https://buy.stripe.com/test_8x28wP4eY2AE8uh2subwk00
use axum::{
    Error,
    body::Body,
    extract::FromRequest,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use stripe::{Event, EventObject, EventType};

use crate::auth::basicauth::update_bal;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub struct StripeEvent(Event);

impl<S> FromRequest<S> for StripeEvent
where
    String: FromRequest<S>,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        dotenv::dotenv().ok();
        let signature = if let Some(sig) = req.headers().get("stripe-signature") {
            sig.to_owned()
        } else {
            return Err(StatusCode::BAD_REQUEST.into_response());
        };

        let payload = String::from_request(req, state)
            .await
            .map_err(IntoResponse::into_response)?;

        Ok(Self(
            stripe::Webhook::construct_event(
                &payload,
                signature.to_str().unwrap(),
                &std::env::var("WHSTRIPE").expect("WHSTRIPE env key not set"),
            )
            .map_err(|_| StatusCode::BAD_REQUEST.into_response())?,
        ))
    }
}

pub async fn handle_webhook(StripeEvent(event): StripeEvent) {
    match event.type_ {
        EventType::CheckoutSessionCompleted => {
            if let EventObject::CheckoutSession(session) = event.data.object {
                println!("Details: {:#?}", session.customer_details);
                let details = match &session.customer_details {
                    Some(d) => d,
                    None => return,
                };

                println!("Email: {:#?}", details.email);
                let email = match &details.email {
                    Some(e) => e,
                    None => return,
                };

                let amount_total = match session.amount_total {
                    Some(a) => a / 100,
                    None => return,
                };

                println!("Amount total: {:?}", amount_total);

                update_bal(email.to_owned(), amount_total as i32)
                    .await
                    .expect("Error update_bal");
            }
        }
        _ => println!("Unknown event encountered in webhook: {:?}", event.type_),
    }
}
