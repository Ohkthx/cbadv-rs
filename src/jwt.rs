use std::str;
use std::sync::Arc;

use base64::engine::general_purpose::{STANDARD_NO_PAD, URL_SAFE_NO_PAD};
use base64::Engine;
use openssl::ec::EcKey;
use openssl::pkey::PKey;
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;
use ring::signature::{self, EcdsaKeyPair};
use serde::Serialize;

use crate::errors::CbError;
use crate::time;
use crate::types::CbResult;

#[derive(Serialize)]
struct Header<'a> {
    alg: &'a str,
    kid: String,
    nonce: String,
}

#[derive(Serialize)]
struct Payload<'a> {
    sub: String,
    iss: &'a str,
    nbf: u64,
    exp: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    uri: Option<String>,
}

#[derive(Debug)]
pub(crate) struct Jwt {
    /// API Key provided by the service.
    api_key: String,
    /// Pre-initialized ECDSA signing key pair.
    signing_key: Arc<EcdsaKeyPair>,
    /// RNG for signing.
    rng: SystemRandom,
}

impl Clone for Jwt {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            signing_key: Arc::clone(&self.signing_key),
            rng: SystemRandom::new(),
        }
    }
}

impl Jwt {
    /// Create a new instance of the JSON Web Token (Jwt) used to generate new tokens.
    pub(crate) fn new(api_key: &str, api_secret: &str) -> CbResult<Self> {
        let secret = Self::format_key(api_secret.as_bytes())?;

        // Initialize SystemRandom.
        let rng = SystemRandom::new();

        // Initialize the EcdsaKeyPair once with the RNG.
        let signing_key = EcdsaKeyPair::from_pkcs8(Self::get_alg(), &secret, &rng)
            .map_err(|why| CbError::BadSignature(why.to_string()))?;

        Ok(Self {
            api_key: api_key.to_string(),
            signing_key: Arc::new(signing_key),
            rng,
        })
    }

    #[inline]
    pub(crate) fn build_uri(method: &str, root: &str, url: &str) -> String {
        format!("{method} {root}{url}")
    }

    /// Creates the header for the message.
    fn build_header(&self) -> CbResult<Header<'static>> {
        // Generate 48 random bytes for the nonce (resulting in 64 Base64 characters)
        let mut nonce_bytes = [0u8; 48];
        self.rng
            .fill(&mut nonce_bytes)
            .map_err(|why| CbError::BadSignature(format!("RNG error: {why:?}")))?;

        Ok(Header {
            alg: "ES256",
            kid: self.api_key.clone(),
            nonce: URL_SAFE_NO_PAD.encode(nonce_bytes),
        })
    }

    /// Creates the payload for the message.
    fn build_payload(&self, uri: Option<&str>) -> Payload<'static> {
        let now = time::now();
        Payload {
            sub: self.api_key.clone(),
            iss: "coinbase-cloud",
            nbf: now,
            exp: now + 120,
            uri: uri.map(String::from),
        }
    }

    /// Encodes (base64) a raw byte slice (`&[u8]`).
    #[inline]
    fn to_base64(input: &[u8]) -> String {
        URL_SAFE_NO_PAD.encode(input)
    }

    /// Encodes a serializable type.
    fn base64_encode<T: Serialize>(input: &T) -> CbResult<String> {
        let raw =
            serde_json::to_vec(input).map_err(|why| CbError::BadSignature(why.to_string()))?;
        Ok(Self::to_base64(&raw))
    }

    #[inline]
    fn get_alg() -> &'static signature::EcdsaSigningAlgorithm {
        &signature::ECDSA_P256_SHA256_FIXED_SIGNING
    }

    /// Formats a private key into PKCS#8 format and parses it.
    ///
    /// This function takes a private key in PEM format, attempts to format it into PKCS#8 format,
    /// and then parses it. If the key is already in PKCS#8 format, it parses the key directly.
    /// The function supports both PKCS#1 and PKCS#8 PEM-encoded EC keys.
    ///
    /// # Arguments
    ///
    /// * `key`: A byte slice (`&[u8]`) containing the private key in PEM format.
    ///
    /// # Returns
    ///
    /// A `CbResult<Vec<u8>>` with the parsed key data in binary format if successful;
    /// otherwise, an error.
    fn format_key(key: &[u8]) -> CbResult<Vec<u8>> {
        // Check if already in pkcs8 format.
        if let Ok(pkey) = PKey::private_key_from_pem(key) {
            if let Ok(serialized) = pkey.private_key_to_pem_pkcs8() {
                if serialized == key {
                    return Self::parse_key(key);
                }
            }
        }

        // Not in pkcs8 format, attempt conversion.
        let ec_key = EcKey::private_key_from_pem(key)
            .map_err(|why| CbError::BadPrivateKey(why.to_string()))?;
        let pkey =
            PKey::from_ec_key(ec_key).map_err(|why| CbError::BadPrivateKey(why.to_string()))?;

        let new_key = pkey
            .private_key_to_pem_pkcs8()
            .map_err(|why| CbError::BadPrivateKey(why.to_string()))?;

        Self::parse_key(&new_key)
    }

    /// Parses a PEM-encoded private key or a base64-encoded key.
    ///
    /// This function takes a byte slice representing either a PEM-encoded private key
    /// (with or without the "-----BEGIN PRIVATE KEY-----" and "-----END PRIVATE KEY-----" delimiters)
    /// or a base64-encoded key, and returns the raw binary key data.
    ///
    /// # Arguments
    ///
    /// * `api_secret`: A byte slice (`&[u8]`) containing the PEM or base64-encoded private key.
    ///
    /// # Returns
    ///
    /// A `CbResult<Vec<u8>>` which is Ok containing the decoded binary key data if successful,
    /// or an Err with a `CbError::BadPrivateKey` containing the error message if any error occurs.
    fn parse_key(api_secret: &[u8]) -> CbResult<Vec<u8>> {
        let pem_str =
            str::from_utf8(api_secret).map_err(|why| CbError::BadPrivateKey(why.to_string()))?;

        // Checks for the headers and footers to remove them.
        let base64_encoded = if pem_str.starts_with("-----BEGIN") && pem_str.contains("-----END") {
            let start = pem_str
                .find("-----BEGIN")
                .and_then(|s| pem_str[s..].find('\n'))
                .ok_or_else(|| CbError::BadPrivateKey("No BEGIN delimiter".to_string()))?
                + 1;

            let end = pem_str
                .find("-----END")
                .ok_or_else(|| CbError::BadPrivateKey("No END delimiter".to_string()))?;

            // Get the data between the header and footer.
            pem_str[start..end]
                .lines()
                .collect::<String>()
                .replace(['\n', '\r'], "")
        } else {
            pem_str.replace(['\n', '\r'], "")
        };

        // Decode the key.
        STANDARD_NO_PAD
            .decode(base64_encoded)
            .map_err(|why| CbError::BadPrivateKey(why.to_string()))
    }

    /// Signs a message using the pre-initialized ECDSA key pair.
    ///
    /// # Arguments
    ///
    /// * `message`: A byte slice (`&[u8]`) of the message to be signed.
    ///
    /// # Returns
    ///
    /// A `CbResult<String>` with the base64-encoded signature if successful; otherwise, an error.
    fn sign_message(&self, message: &[u8]) -> CbResult<String> {
        let signature = self
            .signing_key
            .sign(&self.rng, message)
            .map_err(|why| CbError::BadSignature(why.to_string()))?;
        Ok(Self::to_base64(signature.as_ref()))
    }

    /// Encodes JWT headers and payload into a signed JWT token.
    ///
    /// # Arguments
    ///
    /// * `uri`: the URI being accessed.
    ///
    /// # Returns
    ///
    /// A `CbResult<String>` with the JWT token if successful; otherwise, an error.
    pub(crate) fn encode(&self, uri: Option<&str>) -> CbResult<String> {
        // Convert the header and payload into base64.
        let header = self.build_header()?.serialize_base64()?;
        let payload = Jwt::base64_encode(&self.build_payload(uri))?;

        // Estimate capacity: header + payload + signature + 2 dots
        // Assuming signature is ~43 characters for ECDSA P-256
        let mut message = String::with_capacity(header.len() + payload.len() + 50);
        message.push_str(&header);
        message.push('.');
        message.push_str(&payload);

        // Sign the message.
        let signature = self.sign_message(message.as_bytes())?;
        message.push('.');
        message.push_str(&signature);

        Ok(message)
    }
}

// Implement serialization for Header to handle base64 encoding
impl<'a> Header<'a> {
    fn serialize_base64(&self) -> CbResult<String> {
        let raw = serde_json::to_vec(self).map_err(|why| CbError::BadSignature(why.to_string()))?;
        Ok(URL_SAFE_NO_PAD.encode(&raw))
    }
}

