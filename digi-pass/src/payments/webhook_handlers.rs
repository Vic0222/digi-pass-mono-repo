use super::{application::PaymentService, data_transfer_objects::maya_webhook::MayaWebhookRequest};

pub async fn handle_maya_checkout_webhook(webhook:MayaWebhookRequest, payment_service: &PaymentService) -> anyhow::Result<()> {
    tracing::debug!("Webhook received: {:?}", webhook);

    //validate event type
    if webhook.status != "PAYMENT_SUCCESS" {
        tracing::error!("Payment failed or pneding: {:?}", &webhook.status);
        return  Ok(());
    }

    payment_service
        .mark_payment_as_paid(&webhook.id)
        .await?;

    Ok(())
}

