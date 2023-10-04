use serde::Serialize;

use crate::client::{Client, Response};
use crate::ids::SubscriptionId;
use crate::resources::{CreateSubscriptionItems, Subscription};

#[derive(Clone, Debug, Default, Serialize)]
pub struct CancelSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_period_end: Option<bool>,
}

impl CancelSubscription {
    pub fn new() -> CancelSubscription {
        CancelSubscription { at_period_end: None }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ResumeSubscription {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<String>,
}

impl ResumeSubscription {
    pub fn new() -> ResumeSubscription {
        ResumeSubscription { billing_cycle_anchor: None }
    }
}

impl Subscription {
    /// Cancels a subscription.
    ///
    /// For more details see <https://stripe.com/docs/api#cancel_subscription>.
    pub fn cancel(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: CancelSubscription,
    ) -> Response<Subscription> {
        client.delete_query(&format!("/subscriptions/{}", subscription_id), params)
    }

    /// Resumes a canceled subscription.
    ///
    /// For more details see <https://stripe.com/docs/api/subscriptions/resume>.
    pub fn resume(
        client: &Client,
        subscription_id: &SubscriptionId,
        params: ResumeSubscription,
    ) -> Response<Subscription> {
        client.post_form(&format!("/subscriptions/{}/resume", subscription_id), params)
    }
}

impl CreateSubscriptionItems {
    pub fn new() -> Self {
        Default::default()
    }
}
