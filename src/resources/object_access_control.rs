#![allow(unused_imports)]

use crate::error::GoogleResponse;
use crate::resources::common::ListResponse;
pub use crate::resources::common::{Entity, ProjectTeam, Role};

/// The ObjectAccessControls resources represent the Access Control Lists (ACLs) for objects within
/// Google Cloud Storage. ACLs let you specify who has access to your data and to what extent.
///
/// ```text,ignore
/// Important: The methods for this resource fail with a 400 Bad Request response for buckets with
/// uniform bucket-level access enabled. Use storage.buckets.getIamPolicy and
/// storage.buckets.setIamPolicy to control access instead.
/// ```
///
/// There are two roles that can be assigned to an entity:
///
/// READERs can get an object, though the acl property will not be revealed.
/// OWNERs are READERs, and they can get the acl property, update an object, and call all
/// objectAccessControls methods on the object. The owner of an object is always an OWNER.
///
/// For more information, see Access Control, with the caveat that this API uses READER and OWNER
/// instead of READ and FULL_CONTROL.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectAccessControl {
    /// The kind of item this is. For object access control entries, this is always
    /// `storage#objectAccessControl`.
    pub kind: String,
    /// The ID of the access-control entry.
    pub id: String,
    /// The link to this access-control entry.
    pub self_link: String,
    /// The name of the bucket.
    pub bucket: String,
    /// The name of the object, if applied to an object.
    pub object: String,
    /// The content generation of the object, if applied to an object.
    pub generation: Option<String>,
    /// The entity holding the permission, in one of the following forms:
    ///
    /// user-userId
    /// user-email
    /// group-groupId
    /// group-email
    /// domain-domain
    /// project-team-projectId
    /// allUsers
    /// allAuthenticatedUsers
    ///
    /// Examples:
    ///
    /// The user liz@example.com would be user-liz@example.com.
    /// The group example@googlegroups.com would be group-example@googlegroups.com.
    /// To refer to all members of the G Suite for Business domain example.com, the entity would be
    /// domain-example.com.
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

/// Used to create a new `ObjectAccessControl` object.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NewObjectAccessControl {
    /// The entity holding the permission, in one of the following forms:
    ///
    /// user-userId
    /// user-email
    /// group-groupId
    /// group-email
    /// domain-domain
    /// project-team-projectId
    /// allUsers
    /// allAuthenticatedUsers
    ///
    /// Examples:
    ///
    /// The user liz@example.com would be user-liz@example.com.
    /// The group example@googlegroups.com would be group-example@googlegroups.com.
    /// To refer to all members of the G Suite for Business domain example.com, the entity would be
    /// domain-example.com.
    pub entity: Entity,
    /// The access permission for the entity.
    pub role: Role,
}

#[allow(unused)]
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObjectAccessControlList {
    kind: String,
    items: Vec<ObjectAccessControl>,
}

impl ObjectAccessControl {
    /// Creates a new ACL entry on the specified `object`.
    ///
    /// ### Important
    /// This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    #[tracing::instrument]
    pub async fn create(
        cloud_storage: &crate::Client,
        bucket: &str,
        object: &str,
        new_object_access_control: &NewObjectAccessControl,
    ) -> crate::Result<Self> {
        let url = format!("{}/b/{}/o/{}/acl", crate::BASE_URL, bucket, object);
        cloud_storage
            .post(&url)
            .await?
            .json(new_object_access_control)
            .send()
            .await?
            .json::<GoogleResponse<Self>>()
            .await?
            .into_result()
    }

    /// Retrieves `ACL` entries on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    #[tracing::instrument]
    pub async fn list(
        cloud_storage: &crate::Client,
        bucket: &str,
        object: &str,
    ) -> crate::Result<Vec<Self>> {
        let url = format!("{}/b/{}/o/{}/acl", crate::BASE_URL, bucket, object);
        cloud_storage
            .get(&url)
            .await?
            .send()
            .await?
            .json::<GoogleResponse<ListResponse<Self>>>()
            .await?
            .into_result()
            .map(|s| s.items)
    }

    /// Returns the `ACL` entry for the specified entity on the specified bucket.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    #[tracing::instrument]
    pub async fn read(
        cloud_storage: &crate::Client,
        bucket: &str,
        object: &str,
        entity: &Entity,
    ) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            bucket,
            object,
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
    }

    /// Updates an ACL entry on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    #[tracing::instrument]
    pub async fn update(self, cloud_storage: &crate::Client) -> crate::Result<Self> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            self.bucket,
            self.object,
            self.entity,
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
    }

    /// Permanently deletes the ACL entry for the specified entity on the specified object.
    ///
    /// ### Important
    /// Important: This method fails with a 400 Bad Request response for buckets with uniform
    /// bucket-level access enabled. Use `Bucket::get_iam_policy` and `Bucket::set_iam_policy` to
    /// control access instead.
    #[tracing::instrument]
    pub async fn delete(self, cloud_storage: &crate::Client) -> crate::Result<()> {
        let url = format!(
            "{}/b/{}/o/{}/acl/{}",
            crate::BASE_URL,
            self.bucket,
            self.object,
            self.entity,
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
    use crate::Object;

    #[tokio::test]
    async fn create() {
        let client = crate::Client::new().unwrap();
        let bucket = crate::read_test_bucket().await;
        Object::create(
            &client,
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-create",
            "text/plain",
        )
        .await
        .unwrap();
        let new_bucket_access_control = NewObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        ObjectAccessControl::create(
            &client,
            &bucket.name,
            "test-object-access-controls-create",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn list() {
        let client = crate::Client::new().unwrap();
        let bucket = crate::read_test_bucket().await;
        Object::create(
            &client,
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-list",
            "text/plain",
        )
        .await
        .unwrap();
        ObjectAccessControl::list(&client, &bucket.name, "test-object-access-controls-list")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn read() {
        let client = crate::Client::new().unwrap();
        let bucket = crate::read_test_bucket().await;
        Object::create(
            &client,
            &bucket.name,
            vec![0, 1],
            "test-object-access-controls-read",
            "text/plain",
        )
        .await
        .unwrap();
        let new_bucket_access_control = NewObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        ObjectAccessControl::create(
            &client,
            &bucket.name,
            "test-object-access-controls-read",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
        ObjectAccessControl::read(
            &client,
            &bucket.name,
            "test-object-access-controls-read",
            &Entity::AllUsers,
        )
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn update() {
        let client = crate::Client::new().unwrap();
        // use a separate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket(&client, "test-object-access-controls-update").await;
        let new_bucket_access_control = NewObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        Object::create(
            &client,
            &bucket.name,
            vec![0, 1],
            "test-update",
            "text/plain",
        )
        .await
        .unwrap();
        ObjectAccessControl::create(
            &client,
            &bucket.name,
            "test-update",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
        let mut acl =
            ObjectAccessControl::read(&client, &bucket.name, "test-update", &Entity::AllUsers)
                .await
                .unwrap();
        acl.entity = Entity::AllAuthenticatedUsers;
        acl.update(&client).await.unwrap();
        Object::delete(&client, &bucket.name, "test-update")
            .await
            .unwrap();
        bucket.delete(&client).await.unwrap();
    }

    #[tokio::test]
    async fn delete() {
        let client = crate::Client::new().unwrap();
        // use a separate bucket to prevent synchronization issues
        let bucket = crate::create_test_bucket(&client, "test-object-access-controls-delete").await;
        let new_bucket_access_control = NewObjectAccessControl {
            entity: Entity::AllUsers,
            role: Role::Reader,
        };
        Object::create(
            &client,
            &bucket.name,
            vec![0, 1],
            "test-delete",
            "text/plain",
        )
        .await
        .unwrap();
        ObjectAccessControl::create(
            &client,
            &bucket.name,
            "test-delete",
            &new_bucket_access_control,
        )
        .await
        .unwrap();
        let acl =
            ObjectAccessControl::read(&client, &bucket.name, "test-delete", &Entity::AllUsers)
                .await
                .unwrap();
        acl.delete(&client).await.unwrap();
        Object::delete(&client, &bucket.name, "test-delete")
            .await
            .unwrap();
        bucket.delete(&client).await.unwrap();
    }
}
