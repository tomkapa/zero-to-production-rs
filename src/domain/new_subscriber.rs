use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;

pub struct NewSubscriber {
    // We are not using `String` anymore!
    email: SubscriberEmail,
    name: SubscriberName,
}

impl NewSubscriber {
    /// parse, don't validate !!!
    /// We use hexagonal architecture, all validation should be done in the domain layer
    pub fn new(email: String, name: String) -> Result<Self, String> {
        let email = SubscriberEmail::parse(email)?;
        let name = SubscriberName::parse(name)?;
        Ok(Self { email, name })
    }

    pub fn email(&self) -> &SubscriberEmail {
        &self.email
    }

    pub fn name(&self) -> &SubscriberName {
        &self.name
    }
}