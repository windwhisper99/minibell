use infra::BootstrapConfig;
use minibell::{
    duty,
    usecases::{insert_duties, insert_duty_categories, UseCase},
    AccessType,
};
use serde::Deserialize;
use shaku::HasComponent;

#[derive(Debug, Deserialize)]
struct Manifest {
    categories: Option<Vec<CategoryManifest>>,
    duties: Option<Vec<DutyManifest>>,
}

#[derive(Debug, Deserialize)]
struct CategoryManifest {
    id: String,
    name: String,
    parent: Option<String>,
    sort: i32,
}

impl Into<duty::DutyCategory> for CategoryManifest {
    fn into(self) -> duty::DutyCategory {
        duty::DutyCategory {
            id: self.id.to_string(),
            name: self.name.to_string(),
            parent: self.parent.clone(),
            sort: self.sort,
        }
    }
}

#[derive(Debug, Deserialize)]
struct DutyManifest {
    id: String,
    category: String,

    name: String,
    description: Option<String>,
    short_name: Option<String>,
    patch: f64,
    image: String,
    sort: i32,

    phrases: Option<Vec<DutyPhraseManifest>>,
}

impl Into<duty::Duty> for DutyManifest {
    fn into(self) -> duty::Duty {
        duty::Duty {
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

#[derive(Debug, Clone, Deserialize)]
struct DutyPhraseManifest {
    name: String,
    progression: f64,
}

impl Into<duty::DutyPhrase> for DutyPhraseManifest {
    fn into(self) -> duty::DutyPhrase {
        duty::DutyPhrase {
            name: self.name,
            progression: self.progression,
        }
    }
}

pub async fn upload_duty(file: &str, config: &str) {
    let infra = infra::bootstrap(BootstrapConfig {
        secret_manager_key: Some(config.to_string()),
    })
    .await
    .expect("Failed to bootstrap infra");

    // Read file
    let file = std::fs::read_to_string(file).expect("Unable to read file");
    // Parse file (yml)
    let manifest: Manifest = serde_yaml::from_str(&file).expect("Unable to parse file");

    let insert_categories = insert_duty_categories::InsertDutyCategories {
        duty_repo: infra.resolve_ref(),
    };
    let insert_categories = insert_categories.execute(
        &AccessType::System,
        insert_duty_categories::Input {
            categories: manifest
                .categories
                .unwrap_or_default()
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        },
    );

    let insert_duties = insert_duties::InsertDuties {
        duty_repo: infra.resolve_ref(),
    };
    let insert_duties = insert_duties.execute(
        &AccessType::System,
        insert_duties::Input {
            duties: manifest
                .duties
                .unwrap_or_default()
                .into_iter()
                .map(|duty| {
                    let phrases = duty
                        .phrases
                        .clone()
                        .unwrap_or_default()
                        .into_iter()
                        .map(Into::into)
                        .collect::<Vec<_>>();

                    (duty.into(), phrases)
                })
                .collect::<Vec<_>>(),
        },
    );

    // Wait for both to finish
    futures::future::try_join(insert_categories, insert_duties)
        .await
        .unwrap();
}
