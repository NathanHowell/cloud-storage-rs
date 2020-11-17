use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
pub use crate::resources::common::{Entity, ProjectTeam, Role};

/// The BucketAccessControl resource represents the Access Control Lists (ACLs) for buckets within
/// Google Cloud Storage. ACLs let you specify who has access to your data and to what extent.
///
/// ```text,ignore
/// Important: This method fails with a 400 Bad Request response for buckets with uniform
/// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
/// control access instead.
/// ```
///
/// There are three roles that can be assigned to an entity:
///
/// * READERs can get the bucket, though no acl property will be returned, and list the bucket's
/// objects.
/// * WRITERs are READERs, and they can insert objects into the bucket and delete the bucket's
/// objects.
/// * OWNERs are WRITERs, and they can get the acl property of a bucket, update a bucket, and call
/// all BucketAccessControl methods on the bucket.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BucketAccessControl {
    /// The kind of item this is. For bucket access control entries, this is always
    /// `storage#bucketAccessControl`.
    pub kind: String,
    /// The ID of the access-control entry.
    pub id: String,
    /// The link to this access-control entry.
    pub self_link: String,
    /// The name of the bucket.
    pub bucket: String,
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
}

/// Model that can be used to create a new BucketAccessControl object.
#[derive(Clone, Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewBucketAccessControl {
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

impl BucketAccessControl {
    /// Create a new `BucketAccessControl` using the provided `NewBucketAccessControl`, related to
    /// the `Bucket` provided by the `bucket_name` argument.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, NewBucketAccessControl};
    /// use cloud_storage::bucket_access_control::{Role, Entity};
    ///
    /// let new_bucket_access_control = NewBucketAccessControl {
    ///     entity: Entity::AllUsers,
    ///     role: Role::Reader,
    /// };
    /// BucketAccessControl::create("mybucket", new_bucket_access_control).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        cloud_storage: &crate::Client,
        bucket: &str,
        new_bucket_access_control: NewBucketAccessControl,
    ) -> crate::Result<Self> {
        let url = format!("{}/b/{}/acl", crate::BASE_URL, bucket);
        cloud_storage
            .post(&url)
            .await?
            .json(&new_bucket_access_control)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
    }

    /// Returns all `BucketAccessControl`s related to this bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::BucketAccessControl;
    ///
    /// let acls = BucketAccessControl::list("mybucket").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(cloud_storage: &crate::Client, bucket: &str) -> crate::Result<Vec<Self>> {
        let url = format!("{}/b/{}/acl", crate::BASE_URL, bucket);
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<ListResponse<Self>>>()
            .await?
            .into_result()
            .map(|response| response.items)
    }

    /// Returns the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(
        cloud_storage: &crate::Client,
        bucket: &str,
        entity: &Entity,
    ) -> crate::Result<Self> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, bucket, entity);
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
    }

    /// Update this `BucketAccessControl`.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let mut acl = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// acl.entity = Entity::AllAuthenticatedUsers;
    /// acl.update().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(self, cloud_storage: &crate::Client) -> crate::Result<Self> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, self.bucket, self.entity);
        cloud_storage
            .put(&url)
            .await?
            .json(&self)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    /// ### Example
    /// ```rust,no_run
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::bucket_access_control::{BucketAccessControl, Entity};
    ///
    /// let controls = BucketAccessControl::read("mybucket", &Entity::AllUsers).await?;
    /// controls.delete().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(self, cloud_storage: &crate::Client) -> crate::Result<()> {
        let url = format!("{}/b/{}/acl/{}", crate::BASE_URL, self.bucket, self.entity);
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
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&client, &bucket.name, new_bucket_access_control).await?;
        Ok(())
    }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        BucketAccessControl::list(&client, &bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        let bucket = crate::read_test_bucket().await;
        BucketAccessControl::read(&client, &bucket.name, &Entity::AllUsers).await?;
        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        // use a separate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket(&client, "test-update-bucket-access-controls").await;
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&client, &bucket.name, new_bucket_access_control).await?;
        let mut acl = BucketAccessControl::read(&client, &bucket.name, &Entity::AllUsers).await?;
        acl.entity = Entity::AllAuthenticatedUsers;
        acl.update(&client).await?;
        bucket.delete(&client).await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let client = crate::Client::new()?;
        // use a separate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket(&client, "test-delete-bucket-access-controls").await;
        let new_bucket_access_control = NewBucketAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        BucketAccessControl::create(&client, &bucket.name, new_bucket_access_control).await?;
        let acl = BucketAccessControl::read(&client, &bucket.name, &Entity::AllUsers).await?;
        acl.delete(&client).await?;
        bucket.delete(&client).await?;
        Ok(())
    }
}
