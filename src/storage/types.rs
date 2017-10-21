/// Storage kind.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum StorageKind {
    /// Conventional storage.
    Conventional,
    /// Low-frequency storage.
    LowFrequency,
}


const STORAGE_KIND_CONVENTIONAL: u64 = 0;
const STORAGE_KIND_LF: u64 = 1;


impl ::serde::Serialize for StorageKind {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_u64(match self {
            &StorageKind::Conventional => STORAGE_KIND_CONVENTIONAL,
            &StorageKind::LowFrequency => STORAGE_KIND_LF,
        })
    }
}


impl<'de> ::serde::Deserialize<'de> for StorageKind {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> ::serde::de::Visitor<'de> for Visitor {
            type Value = StorageKind;

            fn expecting(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str("storage kind constant (0 or 1)")
            }

            fn visit_u64<E>(self, value: u64) -> Result<StorageKind, E>
            where
                E: ::serde::de::Error,
            {
                match value {
                    STORAGE_KIND_CONVENTIONAL => Ok(StorageKind::Conventional),
                    STORAGE_KIND_LF => Ok(StorageKind::LowFrequency),
                    _ => Err(E::custom(format!("unknown StorageKind: {}", value))),
                }
            }
        }

        deserializer.deserialize_u64(Visitor)
    }
}


/// Scope setting for put policies.
pub enum PutScope {
    /// Allow only new uploads into the specified bucket.
    Bucket(String),
    /// Allow new uploads and updates into the specified bucket, with exactly the
    /// specified key.
    BucketKey(String, String),
    /// Allow only new uploads into the specified bucket, with exactly the
    /// specified key.
    BucketKeyInsertOnly(String, String),
    /// Allow only uploads into the specified bucket, with the specified key prefix.
    BucketKeyPrefix(String, String),
}


impl Into<String> for PutScope {
    fn into(self) -> String {
        match self {
            PutScope::Bucket(bkt) => bkt,
            PutScope::BucketKey(mut tmp, k) |
            PutScope::BucketKeyInsertOnly(mut tmp, k) |
            PutScope::BucketKeyPrefix(mut tmp, k) => {
                tmp.push(':');
                tmp.push_str(&k);
                tmp
            }
        }
    }
}


impl PutScope {
    pub(crate) fn is_prefixal_scope(&self) -> bool {
        match self {
            &PutScope::Bucket(_) |
            &PutScope::BucketKey(_, _) |
            &PutScope::BucketKeyInsertOnly(_, _) => false,
            &PutScope::BucketKeyPrefix(_, _) => true,
        }
    }

    pub(crate) fn need_set_insert_only(&self) -> bool {
        match self {
            &PutScope::Bucket(_) |
            &PutScope::BucketKey(_, _) |
            &PutScope::BucketKeyPrefix(_, _) => false,
            &PutScope::BucketKeyInsertOnly(_, _) => true,
        }
    }
}


/// [Put policy][put-policy] for resources.
///
/// Since there are many available options, please use [PutPolicyBuilder] to
/// construct put policy instances.
///
/// [put-policy]: https://developer.qiniu.com/kodo/manual/1206/put-policy
/// [PutPolicyBuilder]: ./struct.PutPolicyBuilder.html
#[derive(Clone, Default, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct PutPolicy {
    #[serde(rename = "scope")]
    bucket: String,
    #[serde(rename = "isPrefixalScope", skip_serializing_if = "Option::is_none")]
    is_prefixal_scope: Option<isize>,
    #[serde(rename = "deadline")]
    unix_timestamp: u32,
    #[serde(rename = "insertOnly", skip_serializing_if = "Option::is_none")]
    insert_only: Option<isize>,
    #[serde(rename = "endUser", skip_serializing_if = "Option::is_none")]
    end_user_id: Option<String>,
    #[serde(rename = "returnUrl", skip_serializing_if = "Option::is_none")]
    redirect_url: Option<String>,
    #[serde(rename = "returnBody", skip_serializing_if = "Option::is_none")]
    response_body_for_app_client: Option<String>,
    #[serde(rename = "callbackUrl", skip_serializing_if = "Option::is_none")]
    request_url_for_app_server: Option<String>,
    #[serde(rename = "callbackHost", skip_serializing_if = "Option::is_none")]
    request_host_for_app_server: Option<String>,
    #[serde(rename = "callbackBody", skip_serializing_if = "Option::is_none")]
    request_body_for_app_server: Option<String>,
    #[serde(rename = "callbackBodyType", skip_serializing_if = "Option::is_none")]
    request_body_type_for_app_server: Option<String>,
    #[serde(rename = "persistentOps", skip_serializing_if = "Option::is_none")]
    persistent_ops_cmds: Option<String>,
    #[serde(rename = "persistentNotifyUrl", skip_serializing_if = "Option::is_none")]
    persistent_notify_url: Option<String>,
    #[serde(rename = "persistentPipeline", skip_serializing_if = "Option::is_none")]
    persistent_pipeline: Option<String>,
    #[serde(rename = "saveKey", skip_serializing_if = "Option::is_none")]
    save_key: Option<String>,
    #[serde(rename = "fsizeMin", skip_serializing_if = "Option::is_none")]
    file_size_min: Option<i64>,
    #[serde(rename = "fsizeLimit", skip_serializing_if = "Option::is_none")]
    file_size_limit: Option<i64>,
    #[serde(rename = "detectMime", skip_serializing_if = "Option::is_none")]
    auto_detect_mime_type: Option<isize>,
    #[serde(rename = "mimeLimit", skip_serializing_if = "Option::is_none")]
    mime_limit: Option<String>,
    #[serde(rename = "fileType", skip_serializing_if = "Option::is_none")]
    file_type: Option<StorageKind>,
}


/// Builder of put policy.
pub struct PutPolicyBuilder {
    inner: PutPolicy,
}


fn to_option_int(x: bool) -> Option<isize> {
    if x { Some(1) } else { None }
}


impl PutPolicyBuilder {
    /// Begin constructing a put policy.
    pub fn new(scope: PutScope, deadline: u32) -> Self {
        let is_prefixal_scope = to_option_int(scope.is_prefixal_scope());
        let insert_only = to_option_int(scope.need_set_insert_only());
        let scope = scope.into();

        Self {
            inner: PutPolicy {
                bucket: scope,
                unix_timestamp: deadline,
                is_prefixal_scope: is_prefixal_scope,
                insert_only: insert_only,

                ..PutPolicy::default()
            },
        }
    }

    /// Return the constructed put policy.
    pub fn build(self) -> PutPolicy {
        self.inner
    }

    /// Set the redirect URL on upload success.
    pub fn redirect_url(mut self, url: String) -> Self {
        self.inner.redirect_url = Some(url);
        self
    }

    /// Set the callback body on upload success.
    pub fn return_body(mut self, body: String) -> Self {
        self.inner.response_body_for_app_client = Some(body);
        self
    }

    // TODO: remaining fields
}
