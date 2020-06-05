use crate::error::{Error, GoogleResponse};
pub use crate::resources::bucket::Owner;
use crate::resources::common::ListResponse;
use crate::resources::object_access_control::ObjectAccessControl;
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};

/// A resource representing a file in Google Cloud Storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Object {
    /// The kind of item this is. For objects, this is always `storage#object`.
    pub kind: String,
    /// The ID of the object, including the bucket name, object name, and generation number.
    pub id: String,
    /// The link to this object.
    pub self_link: String,
    /// The name of the object. Required if not specified by URL parameter.
    pub name: String,
    /// The name of the bucket containing this object.
    pub bucket: String,
    /// The content generation of this object. Used for object versioning.
    #[serde(deserialize_with = "crate::from_str")]
    pub generation: i64,
    /// The version of the metadata for this object at this generation. Used for preconditions and
    /// for detecting changes in metadata. A metageneration number is only meaningful in the context
    /// of a particular generation of a particular object.
    #[serde(deserialize_with = "crate::from_str")]
    pub metageneration: i64,
    /// Content-Type of the object data. If an object is stored without a Content-Type, it is served
    /// as application/octet-stream.
    pub content_type: Option<String>,
    /// The creation time of the object in RFC 3339 format.
    pub time_created: chrono::DateTime<chrono::Utc>,
    /// The modification time of the object metadata in RFC 3339 format.
    pub updated: chrono::DateTime<chrono::Utc>,
    /// The deletion time of the object in RFC 3339 format. Returned if and only if this version of
    /// the object is no longer a live version, but remains in the bucket as a noncurrent version.
    pub time_deleted: Option<chrono::DateTime<chrono::Utc>>,
    /// Whether or not the object is subject to a temporary hold.
    pub temporary_hold: Option<bool>,
    /// Whether or not the object is subject to an event-based hold.
    pub event_based_hold: Option<bool>,
    /// The earliest time that the object can be deleted, based on a bucket's retention policy, in
    /// RFC 3339 format.
    pub retention_expiration_time: Option<chrono::DateTime<chrono::Utc>>,
    /// Storage class of the object.
    pub storage_class: String,
    /// The time at which the object's storage class was last changed. When the object is initially
    /// created, it will be set to timeCreated.
    pub time_storage_class_updated: chrono::DateTime<chrono::Utc>,
    /// Content-Length of the data in bytes.
    #[serde(deserialize_with = "crate::from_str")]
    pub size: u64,
    /// MD5 hash of the data; encoded using base64. For more information about using the MD5 hash,
    /// see Hashes and ETags: Best Practices.
    pub md5_hash: Option<String>,
    /// Media download link.
    pub media_link: String,
    /// Content-Encoding of the object data.
    pub content_encoding: Option<String>,
    /// Content-Disposition of the object data.
    pub content_disposition: Option<String>,
    /// Content-Language of the object data.
    pub content_language: Option<String>,
    /// Cache-Control directive for the object data. If omitted, and the object is accessible to all
    /// anonymous users, the default will be public, max-age=3600.
    pub cache_control: Option<String>,
    /// User-provided metadata, in key/value pairs.
    pub metadata: Option<std::collections::HashMap<String, String>>,
    /// Access controls on the object, containing one or more objectAccessControls Resources. If
    /// iamConfiguration.uniformBucketLevelAccess.enabled is set to true, this field is omitted in
    /// responses, and requests that specify this field fail.
    pub acl: Option<Vec<ObjectAccessControl>>,
    /// The owner of the object. This will always be the uploader of the object. If
    /// `iamConfiguration.uniformBucketLevelAccess.enabled` is set to true, this field does not
    /// apply, and is omitted in responses.
    pub owner: Option<Owner>,
    /// CRC32c checksum, as described in RFC 4960, Appendix B; encoded using base64 in big-endian
    /// byte order. For more information about using the CRC32c checksum, see Hashes and ETags: Best
    /// Practices.
    pub crc32c: String,
    /// Number of underlying components that make up a composite object. Components are accumulated
    /// by compose operations, counting 1 for each non-composite source object and componentCount
    /// for each composite source object. Note: componentCount is included in the metadata for
    /// composite objects only.
    #[serde(default, deserialize_with = "crate::from_str_opt")]
    pub component_count: Option<i32>,
    /// HTTP 1.1 Entity tag for the object.
    pub etag: String,
    /// Metadata of customer-supplied encryption key, if the object is encrypted by such a key.
    pub customer_encryption: Option<CustomerEncrypton>,
    /// Cloud KMS Key used to encrypt this object, if the object is encrypted by such a key.
    pub kms_key_name: Option<String>,
}

/// Contains data about how a user might encrypt their files in Google Cloud Storage.
#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerEncrypton {
    /// The encryption algorithm.
    pub encryption_algorithm: String,
    /// SHA256 hash value of the encryption key.
    pub key_sha256: String,
}

/// The request that is supplied to perform `Object::compose`.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ComposeRequest {
    /// The kind of item this is. Will always be `storage#composeRequest`.
    pub kind: String,
    /// The list of source objects that will be concatenated into a single object.
    pub source_objects: Vec<SourceObject>,
    /// Properties of the resulting object.
    pub destination: Option<Object>,
}

/// A SourceObject represents one of the objects that is to be composed.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceObject {
    /// The source object's name. All source objects must have the same storage class and reside in
    /// the same bucket.
    pub name: String,
    /// The generation of this object to use as the source.
    pub generation: Option<i64>,
    /// Conditions that must be met for this operation to execute.
    pub object_preconditions: Option<ObjectPrecondition>,
}

/// Allows conditional copying of this file.
#[derive(Debug, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPrecondition {
    /// Only perform the composition if the generation of the source object that would be used
    /// matches this value. If this value and a generation are both specified, they must be the same
    /// value or the call will fail.
    pub if_generation_match: i64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ObjectList {
    kind: String,
    items: Vec<Object>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct RewriteResponse {
    kind: String,
    total_bytes_rewritten: String,
    object_size: String,
    done: bool,
    resource: Object,
}

impl Object {
    /// Create a new object.
    /// Upload a file as that is loaded in memory to google cloud storage, where it will be
    /// interpreted according to the mime type you specified.
    /// ## Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::Object;
    ///
    /// let file: Vec<u8> = read_cute_cat("cat.png");
    /// Object::create("cat-photos", &file, "recently read cat.png", "image/png")
    ///     .expect("cat not uploaded");
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create(
        bucket: &str,
        file: &[u8],
        filename: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        // has its own url for some reason
        const BASE_URL: &str = "https://www.googleapis.com/upload/storage/v1/b";
        let client = reqwest::Client::new();
        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(&bucket),
            percent_encode(&filename),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.to_string().parse()?);
        headers.insert(CONTENT_LENGTH, file.len().to_string().parse()?);
        let response = client
            .post(url)
            .headers(headers)
            .body(file.to_owned())
            .send()
            .await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(Error::new(&response.text().await?))
        }
    }

    /// Create a new object. This works in the same way as `Object::create`, except it does not need
    /// to load the entire file in ram.
    /// ## Example
    /// ```rust,no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # fn read_cute_cat(_in: &str) -> Vec<u8> { vec![0, 1] }
    /// use cloud_storage::Object;
    ///
    /// let mut file = std::io::Cursor::new(read_cute_cat("cat.png"));
    /// Object::create_streamed("cat-photos", file, 10, "recently read cat.png", "image/png")
    ///     .expect("cat not uploaded");
    /// Ok(())
    /// # }
    /// ```
    pub async fn create_streamed<B: Into<reqwest::Body>>(
        bucket: &str,
        body: B,
        length: u64,
        filename: &str,
        mime_type: &str,
    ) -> Result<Self, Error> {
        use reqwest::header::{CONTENT_LENGTH, CONTENT_TYPE};

        // has its own url for some reason
        const BASE_URL: &str = "https://www.googleapis.com/upload/storage/v1/b";
        let client = reqwest::Client::new();
        let url = &format!(
            "{}/{}/o?uploadType=media&name={}",
            BASE_URL,
            percent_encode(&bucket),
            percent_encode(&filename),
        );
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_TYPE, mime_type.to_string().parse()?);
        headers.insert(CONTENT_LENGTH, length.to_string().parse()?);
        let response = client.post(url).headers(headers).body(body).send().await?;
        if response.status() == 200 {
            Ok(serde_json::from_str(&response.text().await?)?)
        } else {
            Err(Error::new(&response.text().await?))
        }
    }

    /// Obtain a list of objects within this Bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let all_objects = Object::list("my_bucket")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(bucket: &str) -> Result<Vec<Self>, Error> {
        Self::list_from(bucket, None, None).await
    }

    /// Obtain a list of objects by prefix within this Bucket .
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let all_objects = Object::list_prefix("my_bucket", "prefix/")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_prefix(bucket: &str, prefix: &str) -> Result<Vec<Self>, Error> {
        Self::list_from(bucket, Some(prefix), None).await
    }

    async fn list_from(
        bucket: &str,
        prefix: Option<&str>,
        page_token: Option<&str>,
    ) -> Result<Vec<Self>, Error> {
        panic!();
        // let url = format!("{}/b/{}/o", crate::BASE_URL, percent_encode(bucket));
        // let client = reqwest::Client::new();
        // let mut query = if let Some(page_token) = page_token {
        //     vec![("pageToken", page_token)]
        // } else {
        //     vec![]
        // };
        // if let Some(prefix) = prefix {
        //     query.push(("prefix", prefix));
        // };
        //
        // let result: GoogleResponse<ListResponse<Self>> = client
        //     .get(&url)
        //     .query(&query)
        //     .headers(crate::get_headers()?)
        //     .send()
        //     .await?
        //     .json()
        //     .await?;
        // match result {
        //     GoogleResponse::Success(mut s) => {
        //         if let Some(page_token) = s.next_page_token {
        //             s.items
        //                 .extend(Self::list_from(bucket, prefix, Some(&page_token)).await?.into_iter());
        //         }
        //         Ok(s.items)
        //     }
        //     GoogleResponse::Error(e) => Err(e.into()),
        // }
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let object = Object::read("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn read(bucket: &str, file_name: &str) -> Result<Self, Error> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let client = reqwest::Client::new();
        let result: GoogleResponse<Self> = client
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Download the content of the object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let bytes = Object::download("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn download(bucket: &str, file_name: &str) -> Result<bytes::Bytes, Error> {
        let url = format!(
            "{}/b/{}/o/{}?alt=media",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let client = reqwest::Client::new();
        Ok(client
            .get(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?
            .bytes()
            .await?)
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let mut object = Object::read("my_bucket", "path/to/my/file.png")?;
    /// object.content_type = Some("application/xml".to_string());
    /// object.update();
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update(&self) -> Result<Self, Error> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(&self.bucket),
            percent_encode(&self.name),
        );
        let client = reqwest::Client::new();
        let result: GoogleResponse<Self> = client
            .put(&url)
            .headers(crate::get_headers().await?)
            .json(&self)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Deletes a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::Object;
    ///
    /// let mut object = Object::delete("my_bucket", "path/to/my/file.png")?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn delete(bucket: &str, file_name: &str) -> Result<(), Error> {
        let url = format!(
            "{}/b/{}/o/{}",
            crate::BASE_URL,
            percent_encode(bucket),
            percent_encode(file_name),
        );
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .headers(crate::get_headers().await?)
            .send()
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(Error::Google(response.json().await?))
        }
    }

    /// Obtains a single object with the specified name in the specified bucket.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest, SourceObject};
    ///
    /// let obj1 = Object::read("my_bucket", "file1")?;
    /// let obj2 = Object::read("my_bucket", "file2")?;
    /// let compose_request = ComposeRequest {
    ///     kind: "storage#composeRequest".to_string(),
    ///     source_objects: vec![
    ///         SourceObject {
    ///             name: obj1.name.clone(),
    ///             generation: None,
    ///             object_preconditions: None,
    ///         },
    ///         SourceObject {
    ///             name: obj2.name.clone(),
    ///             generation: None,
    ///             object_preconditions: None,
    ///         },
    ///     ],
    ///     destination: None,
    /// };
    /// let obj3 = Object::compose("my_bucket", &compose_request, "test-concatted-file")?;
    /// // obj3 is now a file with the content of obj1 and obj2 concatted together.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn compose(
        bucket: &str,
        req: &ComposeRequest,
        destination_object: &str,
    ) -> Result<Self, Error> {
        let url = format!(
            "{}/b/{}/o/{}/compose",
            crate::BASE_URL,
            percent_encode(&bucket),
            percent_encode(&destination_object)
        );
        let client = reqwest::Client::new();
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(crate::get_headers().await?)
            .json(req)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Copy this object to the target bucket and path
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1")?;
    /// let obj2 = obj1.copy("my_other_bucket", "file2")?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn copy(&self, destination_bucket: &str, path: &str) -> Result<Self, Error> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/copyTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&self.bucket),
            sObject = percent_encode(&self.name),
            dBucket = percent_encode(&destination_bucket),
            dObject = percent_encode(&path),
        );
        let client = reqwest::Client::new();
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let result: GoogleResponse<Self> = client
            .post(&url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Moves a file from the current location to the target bucket and path.
    ///
    /// ## Limitations
    /// This function does not yet support rewriting objects to another
    /// * Geographical Location,
    /// * Encryption,
    /// * Storage class.
    /// These limitations mean that for now, the rewrite and the copy methods do the same thing.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::Object;
    ///
    /// let obj1 = Object::read("my_bucket", "file1")?;
    /// let obj2 = obj1.rewrite("my_other_bucket", "file2")?;
    /// // obj2 is now a copy of obj1.
    /// # Ok(())
    /// # }
    /// ```
    pub async fn rewrite(&self, destination_bucket: &str, path: &str) -> Result<Self, Error> {
        use reqwest::header::CONTENT_LENGTH;

        let url = format!(
            "{base}/b/{sBucket}/o/{sObject}/rewriteTo/b/{dBucket}/o/{dObject}",
            base = crate::BASE_URL,
            sBucket = percent_encode(&self.bucket),
            sObject = percent_encode(&self.name),
            dBucket = percent_encode(destination_bucket),
            dObject = percent_encode(path),
        );
        let client = reqwest::Client::new();
        let mut headers = crate::get_headers().await?;
        headers.insert(CONTENT_LENGTH, "0".parse()?);
        let result: GoogleResponse<RewriteResponse> = client
            .post(&url)
            .headers(headers)
            .send()
            .await?
            .json()
            .await?;
        match result {
            GoogleResponse::Success(s) => Ok(s.resource),
            GoogleResponse::Error(e) => Err(e.into()),
        }
    }

    /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    /// which is valid for `duration` seconds, and lets the posessor download the file contents
    /// without any authentication.
    /// ### Example
    /// ```no_run
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// use cloud_storage::object::{Object, ComposeRequest};
    ///
    /// let obj1 = Object::read("my_bucket", "file1")?;
    /// let url = obj1.download_url(50)?;
    /// // url is now a url to which an unauthenticated user can make a request to download a file
    /// // for 50 seconds.
    /// # Ok(())
    /// # }
    /// ```
    pub fn download_url(&self, duration: u32) -> Result<String, Error> {
        self.sign(&self.name, duration, "GET")
    }

    // /// Creates a [Signed Url](https://cloud.google.com/storage/docs/access-control/signed-urls)
    // /// which is valid for `duration` seconds, and lets the posessor upload new file contents.
    // /// without any authentication.
    // pub fn upload_url(&self, duration: u32) -> Result<String, Error> {
    //     self.sign(&self.name, duration, "POST")
    // }

    #[inline(always)]
    fn sign(&self, file_path: &str, duration: u32, http_verb: &str) -> Result<String, Error> {
        use openssl::sha;

        if duration > 604800 {
            let msg = format!(
                "duration may not be greater than 604800, but was {}",
                duration
            );
            return Err(Error::Other(msg));
        }

        // 1 construct the canonical reques
        let issue_date = chrono::Utc::now();
        let file_path = self.path_to_resource(file_path);
        let query_string = Self::get_canonical_query_string(&issue_date, duration);
        let canonical_request = self.get_canonical_request(&file_path, &query_string, http_verb);

        // 2 get hex encoded SHA256 hash the canonical request
        let hash = sha::sha256(canonical_request.as_bytes());
        let hex_hash = hex::encode(hash);

        // 3 construct the string to sign
        let string_to_sign = format!(
            "{signing_algorithm}\n\
            {current_datetime}\n\
            {credential_scope}\n\
            {hashed_canonical_request}",
            signing_algorithm = "GOOG4-RSA-SHA256",
            current_datetime = issue_date.format("%Y%m%dT%H%M%SZ"),
            credential_scope = Self::get_credential_scope(&issue_date),
            hashed_canonical_request = hex_hash,
        );

        // 4 sign the string to sign with RSA - SHA256
        let buffer = Self::sign_str(&string_to_sign);
        let signature = hex::encode(&buffer?);

        // 5 construct the signed url
        Ok(format!(
            "https://storage.googleapis.com{path_to_resource}?\
            {query_string}&\
            X-Goog-Signature={request_signature}",
            path_to_resource = file_path,
            query_string = query_string,
            request_signature = signature,
        ))
    }

    #[inline(always)]
    fn get_canonical_request(&self, path: &str, query_string: &str, http_verb: &str) -> String {
        format!(
            "{http_verb}\n\
            {path_to_resource}\n\
            {canonical_query_string}\n\
            {canonical_headers}\n\
            \n\
            {signed_headers}\n\
            {payload}",
            http_verb = http_verb,
            path_to_resource = path,
            canonical_query_string = query_string,
            canonical_headers = "host:storage.googleapis.com",
            signed_headers = "host",
            payload = "UNSIGNED-PAYLOAD",
        )
    }

    #[inline(always)]
    fn get_canonical_query_string(date: &chrono::DateTime<chrono::Utc>, exp: u32) -> String {
        let credential = format!(
            "{authorizer}/{scope}",
            authorizer = crate::SERVICE_ACCOUNT.client_email,
            scope = Self::get_credential_scope(date),
        );
        format!(
            "X-Goog-Algorithm={algo}&\
            X-Goog-Credential={cred}&\
            X-Goog-Date={date}&\
            X-Goog-Expires={exp}&\
            X-Goog-SignedHeaders={signed}",
            algo = "GOOG4-RSA-SHA256",
            cred = percent_encode(&credential),
            date = date.format("%Y%m%dT%H%M%SZ"),
            exp = exp,
            signed = "host",
        )
    }

    #[inline(always)]
    fn path_to_resource(&self, path: &str) -> String {
        format!(
            "/{bucket}/{file_path}",
            bucket = self.bucket,
            file_path = percent_encode_noslash(path),
        )
    }

    #[inline(always)]
    fn get_credential_scope(date: &chrono::DateTime<chrono::Utc>) -> String {
        format!("{}/henk/storage/goog4_request", date.format("%Y%m%d"))
    }

    #[inline(always)]
    fn sign_str(message: &str) -> Result<Vec<u8>, Error> {
        use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};

        let key = PKey::private_key_from_pem(crate::SERVICE_ACCOUNT.private_key.as_bytes())?;
        let mut signer = Signer::new(MessageDigest::sha256(), &key)?;
        signer.update(message.as_bytes())?;
        Ok(signer.sign_to_vec()?)
    }
}

const ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'*')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_');

const NOSLASH_ENCODE_SET: &AsciiSet = &ENCODE_SET.remove(b'/').remove(b'~');

// We need to be able to percent encode stuff, but without touching the slashes in filenames. To
// this end we create an implementation that does this, without touching the slashes.
fn percent_encode_noslash(input: &str) -> String {
    utf8_percent_encode(input, NOSLASH_ENCODE_SET).to_string()
}

fn percent_encode(input: &str) -> String {
    utf8_percent_encode(input, ENCODE_SET).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn create() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, &[0, 1], "test-create", "text/plain").await?;
        Ok(())
    }

    // #[tokio::test]
    // async fn create_streamed() -> Result<(), Box<dyn std::error::Error>> {
    //     let bucket = crate::read_test_bucket().await;
    //     let cursor = std::io::Cursor::new([0, 1]);
    //     Object::create_streamed(
    //         &bucket.name,
    //         cursor,
    //         2,
    //         "test-create-streamed",
    //         "text/plain",
    //     ).await?;
    //     Ok(())
    // }

    #[tokio::test]
    async fn list() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::read_test_bucket().await;
        Object::list(&test_bucket.name).await?;
        Ok(())
    }

    #[tokio::test]
    async fn list_prefix() -> Result<(), Box<dyn std::error::Error>> {
        let test_bucket = crate::read_test_bucket().await;

        let prefix_names = [
            "test-list-prefix/1",
            "test-list-prefix/2",
            "test-list-prefix/sub/1",
            "test-list-prefix/sub/2",
        ];

        for name in &prefix_names {
            Object::create(&test_bucket.name, &[0, 1], name, "text/plain").await?;
        }

        let list = Object::list_prefix(&test_bucket.name, "test-list-prefix/").await?;
        assert_eq!(list.len(), 4);
        let list = Object::list_prefix(&test_bucket.name, "test-list-prefix/sub").await?;
        assert_eq!(list.len(), 2);
        Ok(())
    }

    #[tokio::test]
    async fn read() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, &[0, 1], "test-read", "text/plain").await?;
        Object::read(&bucket.name, "test-read").await?;
        Ok(())
    }

    #[tokio::test]
    async fn download() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let content = b"hello world";
        Object::create(
            &bucket.name,
            content,
            "test-download",
            "application/octet-stream",
        ).await?;

        let data = Object::download(&bucket.name, "test-download").await?;
        assert_eq!(data.as_ref(), content);

        Ok(())
    }

    #[tokio::test]
    async fn update() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let mut obj = Object::create(&bucket.name, &[0, 1], "test-update", "text/plain").await?;
        obj.content_type = Some("application/xml".to_string());
        obj.update().await?;
        Ok(())
    }

    #[tokio::test]
    async fn delete() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        Object::create(&bucket.name, &[0, 1], "test-delete", "text/plain").await?;

        Object::delete(&bucket.name, "test-delete").await?;

        let list = Object::list_prefix(&bucket.name, "test-delete").await?;
        assert!(list.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn compose() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let obj1 = Object::create(&bucket.name, &[0, 1], "test-compose-1", "text/plain").await?;
        let obj2 = Object::create(&bucket.name, &[2, 3], "test-compose-2", "text/plain").await?;
        let compose_request = ComposeRequest {
            kind: "storage#composeRequest".to_string(),
            source_objects: vec![
                SourceObject {
                    name: obj1.name.clone(),
                    generation: None,
                    object_preconditions: None,
                },
                SourceObject {
                    name: obj2.name.clone(),
                    generation: None,
                    object_preconditions: None,
                },
            ],
            destination: None,
        };
        let obj3 = Object::compose(&bucket.name, &compose_request, "test-concatted-file").await?;
        let url = obj3.download_url(100)?;
        let content = reqwest::get(&url).await?.text().await?;
        assert_eq!(content.as_bytes(), &[0, 1, 2, 3]);
        Ok(())
    }

    #[tokio::test]
    async fn copy() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let original = Object::create(&bucket.name, &[2, 3], "test-copy", "text/plain").await?;
        original.copy(&bucket.name, "test-copy - copy").await?;
        Ok(())
    }

    #[tokio::test]
    async fn rewrite() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let obj = Object::create(&bucket.name, &[0, 1], "test-rewrite", "text/plain").await?;
        let obj = obj.rewrite(&bucket.name, "test-rewritten").await?;
        let url = obj.download_url(100)?;
        let client = reqwest::Client::new();
        let download = client.head(&url).send().await?;
        assert_eq!(download.status().as_u16(), 200);
        Ok(())
    }

    #[tokio::test]
    async fn test_url_encoding() -> Result<(), Box<dyn std::error::Error>> {
        let bucket = crate::read_test_bucket().await;
        let complicated_names = [
            "asdf",
            "asdf+1",
            "asdf&&+1?=3,,-_()*&^%$#@!`~{}[]\\|:;\"'<>,.?/äöüëß",
            "https://www.google.com",
            "परिक्षण फाईल",
            "测试很重要",
        ];
        for name in &complicated_names {
            let _obj = Object::create(&bucket.name, &[0, 1], name, "text/plain").await?;
            let obj = Object::read(&bucket.name, &name).await.unwrap();
            let url = obj.download_url(100)?;
            let client = reqwest::Client::new();
            let download = client.head(&url).send().await?;
            assert_eq!(download.status().as_u16(), 200);
        }
        Ok(())
    }
}
