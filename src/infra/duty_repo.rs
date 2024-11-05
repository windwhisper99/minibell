use std::sync::Arc;

use serde::Deserialize;
use surrealdb::RecordId;

use crate::domain::{duty, Error};

use super::Database;

#[derive(Debug, Deserialize)]
struct DutyCategoryModel {
    id: String,
    name: String,
    duties: Vec<DutyReviewModel>,
}

impl Into<duty::DutyCategory> for DutyCategoryModel {
    fn into(self) -> duty::DutyCategory {
        duty::DutyCategory {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<duty::DutyCategoryWithDutiesReview> for DutyCategoryModel {
    fn into(self) -> duty::DutyCategoryWithDutiesReview {
        duty::DutyCategoryWithDutiesReview {
            id: self.id,
            name: self.name,
            duties: self.duties.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct DutyReviewModel {
    pub id: String,
    pub name: String,
    pub short_name: String,
}

impl Into<duty::DutyReview> for DutyReviewModel {
    fn into(self) -> duty::DutyReview {
        duty::DutyReview {
            id: self.id,
            name: self.name,
            short_name: self.short_name,
        }
    }
}

#[derive(Debug, Deserialize)]
struct PhaseModel {
    name: String,
    progression: f64,
}

impl Into<duty::Phase> for PhaseModel {
    fn into(self) -> duty::Phase {
        duty::Phase {
            name: self.name,
            progression: self.progression,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DutyModel {
    id: String,
    name: String,
    short_name: String,
    image_url: String,

    phases: Vec<PhaseModel>,
}

impl Into<duty::DutyReview> for DutyModel {
    fn into(self) -> duty::DutyReview {
        duty::DutyReview {
            id: self.id,
            name: self.name,
            short_name: self.short_name,
        }
    }
}

impl Into<duty::Duty> for DutyModel {
    fn into(self) -> duty::Duty {
        duty::Duty {
            id: self.id,
            name: self.name,
            short_name: self.short_name,
            image_url: self.image_url,

            phases: self.phases.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Clone)]
pub struct DutyRepo {
    db: Arc<Database>,
}

impl DutyRepo {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

impl duty::DutyRepo for &DutyRepo {
    async fn find_all(&self) -> Result<Vec<duty::DutyCategoryWithDutiesReview>, Error> {
        self.db
            .query(
                "SELECT 
                    *,
                    id.id(),
                    duties.*.{id:id.id(), name, short_name}
                FROM duty_category_view",
            )
            .await?
            .check()?
            .take::<Vec<DutyCategoryModel>>(0)
            .map(|categories| {
                categories
                    .into_iter()
                    .map(|category| category.into())
                    .collect::<Vec<duty::DutyCategoryWithDutiesReview>>()
            })
            .map_err(Into::into)
    }

    async fn find_by_id(&self, id: &str) -> Result<duty::Duty, Error> {
        self.db
            .query(
                "SELECT *, id.id() FROM ONLY duty_view
                WHERE id = $id LIMIT 1",
            )
            .bind(("id", RecordId::from_table_key("duty_view", id)))
            .await?
            .check()?
            .take::<Option<DutyModel>>(0)?
            .map(Into::into)
            .ok_or_else(|| Error::NotFound)
    }
}
