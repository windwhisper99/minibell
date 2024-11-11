use std::collections::HashMap;

use aws_config::SdkConfig;
use aws_sdk_dynamodb::types::{AttributeValue, PutRequest, WriteRequest};
use minibell::Error;
use serde::{de::DeserializeOwned, Serialize};

use crate::Parameters;

pub mod duty;
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

    // async fn insert_item<M: PrimaryModel>(&self, item: M) -> Result<(), Error> {
    //     let item = item.to_item()?;
    //     self.client
    //         .put_item()
    //         .table_name(&self.primary_table)
    //         .set_item(Some(item))
    //         .send()
    //         .await
    //         .map_err(|e| Error::internal(e.to_string()))?;

    //     Ok(())
    // }

    async fn query_items<M: PrimaryModel>(
        &self,
        index: Option<&str>,
        pk: &str,
        sk: &str,
    ) -> Result<Vec<M>, Error> {
        let query = self
            .client
            .query()
            .table_name(&self.primary_table)
            .key_condition_expression("#pk = :pk AND begins_with(#sk, :sk)");
        let query = if let Some(index) = index {
            query
                .index_name(index)
                .expression_attribute_names("#pk", "GSI1PK")
                .expression_attribute_names("#sk", "GSI1SK")
        } else {
            query
                .expression_attribute_names("#pk", "PK")
                .expression_attribute_names("#sk", "SK")
        };

        let items = query
            .expression_attribute_values(":pk", AttributeValue::S(pk.to_string()))
            .expression_attribute_values(":sk", AttributeValue::S(sk.to_string()))
            .send()
            .await
            .map(|i| i.items().to_vec())
            .map_err(|e| Error::internal(e.to_string()))?;

        Ok(serde_dynamo::from_items(items).map_err(|e| Error::internal(e.to_string()))?)
    }

    fn batch_insert_items(&self) -> BatchItemWrite {
        BatchItemWrite {
            client: self.client.clone(),
            table: self.primary_table.clone(),
            items: Vec::new(),
        }
    }
}

struct BatchItemWrite {
    client: aws_sdk_dynamodb::Client,
    table: String,
    items: Vec<HashMap<String, AttributeValue>>,
}

impl BatchItemWrite {
    fn add_item<M: PrimaryModel>(mut self, item: M) -> Result<Self, Error> {
        let item = item.to_item()?;
        self.items.push(item);

        Ok(Self {
            items: self.items,
            ..self
        })
    }

    fn add_items<M: PrimaryModel>(mut self, items: &[M]) -> Result<Self, Error> {
        let mut new_items = items
            .iter()
            .map(|item| item.to_item())
            .collect::<Result<Vec<_>, _>>()?;
        self.items.append(&mut new_items);

        Ok(Self {
            items: self.items,
            ..self
        })
    }

    async fn send(self) -> Result<(), Error> {
        let write_requests = self
            .items
            .into_iter()
            .map(|item| {
                PutRequest::builder()
                    .set_item(Some(item))
                    .build()
                    .map_err(|e| Error::internal(e.to_string()))
                    .map(|put_request| WriteRequest::builder().put_request(put_request).build())
            })
            .collect::<Result<Vec<_>, _>>()?;

        // Split the write requests into chunks of 25
        let write_requests = write_requests
            .chunks(25)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();

        for chunk in write_requests {
            let mut input = HashMap::new();
            input.insert(self.table.to_string(), chunk);

            self.client
                .batch_write_item()
                .set_request_items(Some(input))
                .send()
                .await
                .map_err(|e| Error::internal(e.to_string()))?;
        }
        Ok(())
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
