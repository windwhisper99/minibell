use std::sync::Arc;

use async_trait::async_trait;
use minibell::{
    duty::{Duty, DutyCategory, DutyPhrase, DutyRepository},
    Error,
};
use serde::{Deserialize, Serialize};
use shaku::Component;

use super::{DynamoClient, PrimaryModel};

#[derive(Debug, Deserialize, Serialize)]
pub struct DutyCategoryModel {
    id: String,
    name: String,
    parent: Option<String>,
    sort: i32,
}

impl From<&DutyCategory> for DutyCategoryModel {
    fn from(value: &DutyCategory) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            parent: value.parent.clone(),
            sort: value.sort,
        }
    }
}

impl Into<DutyCategory> for DutyCategoryModel {
    fn into(self) -> DutyCategory {
        DutyCategory {
            id: self.id,
            name: self.name,
            parent: self.parent,
            sort: self.sort,
        }
    }
}

impl PrimaryModel for DutyCategoryModel {
    fn data_type(&self) -> String {
        "DutyCategory".to_string()
    }

    fn primary_key(&self) -> String {
        "DUTY_CATEGORY".to_string()
    }

    fn sort_key(&self) -> String {
        format!("DUTY_CATEGORY#{}", self.id)
    }

    /// Query list of category by parent and sort
    fn gsi1(&self) -> Option<(String, String)> {
        Some((
            match &self.parent {
                Some(parent) => format!("DUTY_CATEGORY#{}", parent),
                None => "DUTY_CATEGORY".to_string(),
            },
            format!("DUTY_CATEGORY#{:0>8}", self.sort),
        ))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DutyModel {
    pub id: String,
    pub category: String,

    pub name: String,
    pub description: Option<String>,
    pub short_name: Option<String>,
    pub patch: f64,
    pub image: String,
    pub sort: i32,
}

impl From<&Duty> for DutyModel {
    fn from(value: &Duty) -> Self {
        Self {
            id: value.id.clone(),
            category: value.category.clone(),
            name: value.name.clone(),
            description: value.description.clone(),
            short_name: value.short_name.clone(),
            patch: value.patch,
            image: value.image.clone(),
            sort: value.sort,
        }
    }
}

impl Into<Duty> for DutyModel {
    fn into(self) -> Duty {
        Duty {
            id: self.id,
            category: self.category,
            name: self.name,
            description: self.description,
            short_name: self.short_name,
            patch: self.patch,
            image: self.image,
            sort: self.sort,
        }
    }
}

impl PrimaryModel for DutyModel {
    fn data_type(&self) -> String {
        "Duty".to_string()
    }

    fn primary_key(&self) -> String {
        "DUTY".to_string()
    }

    fn sort_key(&self) -> String {
        format!("DUTY#{}", self.id)
    }

    /// Query list of duties by category and sort
    fn gsi1(&self) -> Option<(String, String)> {
        Some((
            format!("DUTY#{}", self.category),
            format!("DUTY#{:0>8}", self.sort),
        ))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct DutyPhraseModel {
    duty_id: String,
    name: String,
    progression: f64,
}

impl From<(&Duty, &DutyPhrase)> for DutyPhraseModel {
    fn from((duty, pharse): (&Duty, &DutyPhrase)) -> Self {
        Self {
            duty_id: duty.id.to_string(),
            name: pharse.name.to_string(),
            progression: pharse.progression,
        }
    }
}

impl Into<DutyPhrase> for DutyPhraseModel {
    fn into(self) -> DutyPhrase {
        DutyPhrase {
            name: self.name,
            progression: self.progression,
        }
    }
}

impl PrimaryModel for DutyPhraseModel {
    fn data_type(&self) -> String {
        "DutyPhrase".to_string()
    }

    fn primary_key(&self) -> String {
        "DUTY_PHRASE".to_string()
    }

    fn sort_key(&self) -> String {
        format!("DUTY#{}#{:0>8}", self.duty_id, self.progression * 1000f64)
    }
}

#[derive(Debug, Component)]
#[shaku(interface = DutyRepository)]
pub struct DutyRepoImpl {
    db: Arc<DynamoClient>,
}

impl DutyRepoImpl {
    async fn get_category(&self, category: &str) -> Result<DutyCategory, Error> {
        self.db
            .get_item::<DutyCategoryModel>("DUTY_CATEGORY", &format!("DUTY_CATEGORY#{}", category))
            .await
            .map(|m| m.into())
    }
}

#[async_trait]
impl DutyRepository for DutyRepoImpl {
    /// Insert a category
    /// Create or update if exists
    async fn insert_categories(&self, categories: &[DutyCategory]) -> Result<(), Error> {
        self.db
            .batch_insert_items()
            .add_items::<DutyCategoryModel>(&categories.iter().map(From::from).collect::<Vec<_>>())?
            .send()
            .await
    }

    /// Insert a duty
    /// Create or update if exists
    async fn insert_duties(&self, duties: &[(Duty, &[DutyPhrase])]) -> Result<(), Error> {
        let mut command = self.db.batch_insert_items();
        for (duty, pharses) in duties {
            command = command.add_item(DutyModel::from(duty))?.add_items(
                &pharses
                    .iter()
                    .map(|pharse| DutyPhraseModel::from((duty, pharse)))
                    .collect::<Vec<_>>(),
            )?;
        }

        command.send().await
    }

    /// List all categories or list categories by parent
    async fn list_categories(&self, parent: Option<&str>) -> Result<Vec<DutyCategory>, Error> {
        self.db
            .query_items::<DutyCategoryModel>(
                Some("GSI1"),
                "DUTY_CATEGORY",
                &match parent {
                    Some(parent) => format!("DUTY_CATEGORY#{}", parent),
                    None => "DUTY_CATEGORY".to_string(),
                },
            )
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
    }

    /// List all duties given category
    async fn list_duties(&self, category: &str) -> Result<Vec<Duty>, Error> {
        self.db
            .query_items::<DutyModel>(Some("GSI1"), &format!("DUTY#{}", category), "DUTY")
            .await
            .map(|items| items.into_iter().map(Into::into).collect())
    }

    /// List all categories and duties
    /// Return the parent category, all sub categories and all duties
    async fn list_categories_and_duties(
        &self,
        parent: &str,
    ) -> Result<(DutyCategory, Vec<DutyCategory>, Vec<Duty>), Error> {
        let query_parent = self.get_category(&parent);
        let query_categories = self.list_categories(Some(parent));
        let query_duties = self.list_duties(parent);

        futures::try_join!(query_parent, query_categories, query_duties)
            .map(|(parent, categories, duties)| (parent, categories, duties))
    }

    /// Get a duty will pharse
    async fn get_duty(&self, duty_id: &str) -> Result<(Duty, Vec<DutyPhrase>), Error> {
        let get_duty_sk = format!("DUTY#{}", duty_id);
        let get_duty = self.db.get_item::<DutyModel>("DUTY", &get_duty_sk);
        let query_phrases_sk = format!("DUTY#{}", duty_id);
        let query_phrases =
            self.db
                .query_items::<DutyPhraseModel>(None, "DUTY_PHRASE", &query_phrases_sk);

        futures::try_join!(get_duty, query_phrases)
            .map(|(duty, phrases)| (duty.into(), phrases.into_iter().map(Into::into).collect()))
    }
}

#[cfg(test)]
mod tests {
    use minibell::duty::{Duty, DutyPhrase};

    use crate::dynamodb::{
        duty::{DutyModel, DutyPhraseModel},
        PrimaryModel,
    };

    #[test]
    fn demo() {
        let duty = Duty {
            id: "1a863f1ea3b".to_string(),
            category: "ultimates".to_string(),
            name: "The Unending Coil of Bahamut".to_string(),
            description: None,
            short_name: Some("UWU".to_string()),
            patch: 4.11,
            image: "https://lds-img.finalfantasyxiv.com/itemicon/1a/1aa609c136b7632b4fb51293c24580f7bb50203a.png".to_string(),
            sort: 1,
        };

        let pharse = DutyPhrase {
            name: "Phase 1".to_string(),
            progression: 2.4,
        };
        println!("{:#?}", DutyModel::from(&duty).to_item().unwrap());
        println!(
            "{:#?}",
            DutyPhraseModel::from((&duty, &pharse)).to_item().unwrap()
        );
    }
}
