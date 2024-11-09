use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::AttributeValue;
use minibell::Error;
use serde::{de::DeserializeOwned, Serialize};

use crate::Parameters;

pub mod member;

#[derive(Debug)]
pub struct DynamoClient {
    pub(super) client: aws_sdk_dynamodb::Client,

    pub(super) primary_table: String,
}

impl DynamoClient {
    pub(crate) fn new(sdkconfig: &SdkConfig, parameters: &Parameters) -> Self {
        Self {
            client: aws_sdk_dynamodb::Client::new(&sdkconfig),

            primary_table: parameters.primary_table.to_string(),
        }
    }

    async fn get_item<M: PrimaryModel>(&self, pk: &str, sk: &str) -> Result<M, Error> {
        let item = self
            .client
            .get_item()
            .table_name(&self.primary_table)
            .key("PK", AttributeValue::S(pk.to_string()))
            .key("SK", AttributeValue::S(sk.to_string()))
            .send()
            .await
            .map_err(|e| Error::internal(e.to_string()))?
            .item
            .ok_or(Error::ItemNotFound)?;
        let member: M =
            serde_dynamo::from_item(item).map_err(|e| Error::internal(e.to_string()))?;

        Ok(member.into())
    }
}

trait PrimaryModel: DeserializeOwned + Serialize {
    fn primary_key(&self) -> String;
    fn sort_key(&self) -> String;
    fn data_type(&self) -> String;

    fn gsi1(&self) -> Option<(String, String)> {
        None
    }

    fn gsi2(&self) -> Option<(String, String)> {
        None
    }

    fn to_item(&self) -> Result<HashMap<String, AttributeValue>, Error> {
        let mut hash_map: HashMap<String, AttributeValue> =
            serde_dynamo::to_item(self).map_err(|e| Error::internal(e.to_string()))?;
        hash_map.insert("PK".to_string(), AttributeValue::S(self.primary_key()));
        hash_map.insert("SK".to_string(), AttributeValue::S(self.sort_key()));
        hash_map.insert("data_type".to_string(), AttributeValue::S(self.data_type()));

        if let Some((gpk1, gsk1)) = self.gsi1() {
            hash_map.insert("GSI1PK".to_string(), AttributeValue::S(gpk1));
            hash_map.insert("GSI1SK".to_string(), AttributeValue::S(gsk1));
        }

        if let Some((gpk2, gsk2)) = self.gsi2() {
            hash_map.insert("GSI2PK".to_string(), AttributeValue::S(gpk2));
            hash_map.insert("GSI2SK".to_string(), AttributeValue::S(gsk2));
        }

        Ok(hash_map)
    }
}
