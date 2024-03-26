use super::{application::PaymentService, data_transfer_objects::webhook::Webhook};
use sha2::Sha256;
use hmac::{Hmac, Mac};

// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;


pub async fn handle_checkout_webhook(webhook:Webhook, key :&str, raw: String, raw_signature: &str, payment_service: PaymentService) -> anyhow::Result<()> {
    tracing::info!("Webhook received: {:?}", webhook);

    //verify signature
    let (timestamp, signature) = slice_raw_siganture(raw_signature, webhook.data.attributes.livemode)?;
    let mut mac = HmacSha256::new_from_slice(key[..].as_bytes())?;
    mac.update(format!("{timestamp}.{raw}").as_bytes());

    mac.verify_slice(signature.as_bytes())?;

    //validate event type
    if webhook.data.attributes.attributes_type != "checkout_session.payment.paid" {
        tracing::error!("Event type missmatch: {:?}", webhook.data.attributes.attributes_type);
        return  Ok(());
    }

    payment_service
        .mark_payment_as_paid(&webhook.data.attributes.data.id)
        .await?;

    Ok(())
}

fn slice_raw_siganture(raw_signature: &str, livemode: bool) -> anyhow::Result<(&str, &str)> {
    let parts: Vec<&str> = raw_signature.split(',').collect();

    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid input format"));
    }

    let t = parts[0].split('=').nth(1).ok_or(anyhow::anyhow!("Missing 't' value"))?;
    let te = parts[1].split('=').nth(1).ok_or(anyhow::anyhow!("Missing 'te' value"))?;
    let li = parts[2].split('=').nth(1).ok_or(anyhow::anyhow!("Missing 'li' value"))?;

    if livemode {
        Ok((t, li))
    }else{
        Ok((t, te))
    }
}
