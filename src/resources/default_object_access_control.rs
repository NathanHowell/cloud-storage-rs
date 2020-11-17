#![allow(unused_imports)]

use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
pub use crate::resources::common::{Entity, ProjectTeam, Role};

/// The DefaultObjectAccessControls resources represent the Access Control Lists (ACLs) applied to a
/// new object within Google Cloud Storage when no ACL was provided for that object. ACLs let you
/// specify who has access to your data and to what extent.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultObjectAccessControl {
    /// The kind of item this is. For object access control entries, this is always
    /// storage#objectAccessControl.
    pub kind: String,
    /// The entity holding the permission, in one of the following forms:
    ///
    /// * `user-userId`
    /// * `user-email`
    /// * `group-groupId`
    /// * `group-email`
    /// * `domain-domain`
    /// * `project-team-projectId`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    ///
    /// Examples:
    ///
    /// * The user liz@example.com would be user-liz@example.com.
    /// * The group example@googlegroups.com would be group-example@googlegroups.com.
    /// * To refer to all members of the G Suite for Business domain example.com, the entity would
    /// be domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
    /// The email address associated with the entity, if any.
    pub email: Option<String>,
    /// The ID for the entity, if any.
    pub entity_id: Option<String>,
    /// The domain associated with the entity, if any.
    pub domain: Option<String>,
    /// The project team associated with the entity, if any.
    pub project_team: Option<ProjectTeam>,
    /// HTTP 1.1 Entity tag for the access-control entry.
    pub etag: String,
    /// The bucket this resource belongs to.
    #[serde(default)]
    pub bucket: String, // this field is not returned by Google, but we populate it manually for the
                        // convenience of the end user.
}

/// Model that can be used to create a new DefaultObjectAccessControl object.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewDefaultObjectAccessControl {
    /// The entity holding the permission, in one of the following forms:
    ///
    /// * `user-userId`
    /// * `user-email`
    /// * `group-groupId`
    /// * `group-email`
    /// * `domain-domain`
    /// * `project-team-projectId`
    /// * `allUsers`
    /// * `allAuthenticatedUsers`
    ///
    /// Examples:
    ///
    /// * The user liz@example.com would be user-liz@example.com.
    /// * The group example@googlegroups.com would be group-example@googlegroups.com.
    /// * To refer to all members of the G Suite for Business domain example.com, the entity would
    /// be domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
}

impl DefaultObjectAccessControl {
    /// Create a new `DefaultObjectAccessControl` entry on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{
    ///     DefaultObjectAccessControl, NewDefaultObjectAccessControl, Role, Entity,
    /// };
    ///
    /// let new_acl = NewDefaultObjectAccessControl {
    ///     entity: Entity::AllAuthenticatedUsers,
    ///     role: Role::Reader,
    /// };
    /// let default_acl = DefaultObjectAccessControl::create("mybucket", &new_acl).await?;
    /// # default_acl.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        cloud_storage: &crate::Client,
        bucket: &str,
        new_acl: NewDefaultObjectAccessControl,
    ) -> crate::Result<Self> {
        let url = format!("{}/b/{}/defaultObjectAcl", crate::BASE_URL, bucket);
        cloud_storage
            .post(&url)
            .await?
            .json(&new_acl)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
            .map(|mut s| {
                s.bucket = bucket.to_string();
                s
            })
    }

    /// Retrieves default object ACL entries on the specified bucket.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::DefaultObjectAccessControl;
    ///
    /// let default_acls = DefaultObjectAccessControl::list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(cloud_storage: &crate::Client, bucket: &str) -> crate::Result<Vec<Self>> {
        let url = format!("{}/b/{}/defaultObjectAcl", crate::BASE_URL, bucket);
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<ListResponse<Self>>>()
            .await?
            .into_result()
            .map(|s| {
                s.items
                    .into_iter()
                    .map(|mut item| {
                        item.bucket = bucket.to_string();
                        item
                    })
                    .collect()
            })
    }

    /// Read a single `DefaultObjectAccessControl`.
    /// The `bucket` argument is the name of the bucket whose `DefaultObjectAccessControl` is to be
    /// read, and the `entity` argument is the entity holding the permission. Options are
    /// Can be "user-`userId`", "user-`email_address`", "group-`group_id`", "group-`email_address`",
    /// "allUsers", or "allAuthenticatedUsers".
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let default_acl = DefaultObjectAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        cloud_storage: &crate::Client,
        bucket: &str,
        entity: &Entity,
    ) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            bucket,
            entity
        );
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
            .map(|mut s| {
                s.bucket = bucket.to_string();
                s
            })
    }

    /// Update the current `DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// default_acl.entity = Entity::AllAuthenticatedUsers;
    /// default_acl.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(self, cloud_storage: &crate::Client) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            self.bucket,
            self.entity
        );
        cloud_storage
            .put(&url)
            .await?
            .json(&self)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
            .map(|mut s| {
                s.bucket = self.bucket.to_string();
                s
            })
    }

    /// Delete this 'DefaultObjectAccessControl`.
    /// ### Important
    /// Important: This method fails with a `400 Bad Request` response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::default_object_access_control::{DefaultObjectAccessControl, Entity};
    ///
    /// let mut default_acl = DefaultObjectAccessControl::read("my_bucket", &Entity::AllUsers).await?;
    /// default_acl.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self, cloud_storage: &crate::Client) -> Result<(), crate::Error> {
        let url = format!(
            "{}/b/{}/defaultObjectAcl/{}",
            crate::BASE_URL,
            self.bucket,
            self.entity
        );
        cloud_storage
            .delete(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<()>>()
            .await?
            .into_result()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        let new_acl = NewDefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        DefaultObjectAccessControl::create(&client, &bucket.name, new_acl).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        DefaultObjectAccessControl::read(&client, &bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        DefaultObjectAccessControl::list(&client, &bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        let new_acl = NewDefaultObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        let mut default_acl =
            DefaultObjectAccessControl::create(&client, &bucket.name, new_acl).await?;
        default_acl.entity = Entity::AllAuthenticatedUsers;
        default_acl.update(&client).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        let default_acl =
            DefaultObjectAccessControl::read(&client, &bucket.name, &Entity::AllAuthenticatedUsers)
                .await?;
        default_acl.delete(&client).await?;
        Ok(())
    }
}
