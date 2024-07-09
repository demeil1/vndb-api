use reqwest::Client;

use crate::format::error::VndbApiError;
use crate::request::query::{
    CharacterQuery, ProducerQuery, Query, ReleaseQuery, StaffQuery, TagQuery, TraitQuery,
    UListQuery, VnQuery,
};
use crate::request::response::Response;

use crate::format::auth::AuthInfo;
use crate::format::stats::VndbStats;
use crate::format::user::{UserSearch, UserSearchFields};

use crate::format::character::Character;
use crate::format::producer::Producer;
use crate::format::release::Release;
use crate::format::rlist::RListPatch;
use crate::format::staff::Staff;
use crate::format::tag::Tag;
use crate::format::traits::Trait;
use crate::format::ulist::{UList, UListLabels, UListLabelsFieldChoices, UListPatch};
use crate::format::vn::VisualNovel;

const API_ENDPOINT: &str = "https://api.vndb.org/kana";

pub struct VndbApiClient {
    client: Client,
    access_token: String,
}

impl VndbApiClient {
    pub fn new(token: &String) -> Self {
        VndbApiClient {
            client: Client::new(),
            access_token: token.clone(),
        }
    }

    pub async fn get_stats(&self) -> Result<VndbStats, VndbApiError> {
        let url = format!("{}/stats", API_ENDPOINT);
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let result = response.json::<VndbStats>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn get_auth_info(&self) -> Result<AuthInfo, VndbApiError> {
        let url = format!("{}/authinfo", API_ENDPOINT);
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            let result = response.json::<AuthInfo>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn get_user(
        &self,
        q: &Vec<String>,
        fields: &UserSearchFields,
    ) -> Result<UserSearch, VndbApiError> {
        let url = format!(
            "{}/user?q={}&fields={}",
            API_ENDPOINT,
            q.join("&q="),
            fields.to_csv()
        );
        let response = self.client.get(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<UserSearch>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn vn_search(
        &self,
        q: &Query<VnQuery>,
    ) -> Result<Response<VisualNovel>, VndbApiError> {
        let url = format!("{}/vn", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<VisualNovel>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn release_search(
        &self,
        q: &Query<ReleaseQuery>,
    ) -> Result<Response<Release>, VndbApiError> {
        let url = format!("{}/release", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Release>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn producer_search(
        &self,
        q: &Query<ProducerQuery>,
    ) -> Result<Response<Producer>, VndbApiError> {
        let url = format!("{}/producer", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Producer>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn character_search(
        &self,
        q: &Query<CharacterQuery>,
    ) -> Result<Response<Character>, VndbApiError> {
        let url = format!("{}/character", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Character>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn staff_search(
        &self,
        q: &Query<StaffQuery>,
    ) -> Result<Response<Staff>, VndbApiError> {
        let url = format!("{}/staff", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Staff>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn tag_search(&self, q: &Query<TagQuery>) -> Result<Response<Tag>, VndbApiError> {
        let url = format!("{}/tag", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Tag>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn trait_search(
        &self,
        q: &Query<TraitQuery>,
    ) -> Result<Response<Trait>, VndbApiError> {
        let url = format!("{}/trait", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<Trait>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn ulist(&self, q: &Query<UListQuery>) -> Result<Response<UList>, VndbApiError> {
        let url = format!("{}/ulist", API_ENDPOINT);
        let response = self.client.post(&url).json(q).send().await?;

        if response.status().is_success() {
            let result = response.json::<Response<UList>>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn get_ulist_labels(
        &self,
        user: &String,
        fields: &UListLabelsFieldChoices,
    ) -> Result<UListLabels, VndbApiError> {
        let url = format!(
            "{}/ulist_labels?user={}&fields={}",
            API_ENDPOINT,
            user.clone(),
            fields.to_csv()
        );
        let response = self.client.get(&url).send().await?;

        if response.status().is_success() {
            let result = response.json::<UListLabels>().await?;
            Ok(result)
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn ulist_patch(
        &self,
        vn_id: &String,
        patch: &UListPatch,
    ) -> Result<(), VndbApiError> {
        let url = format!("{}/ulist/{}", API_ENDPOINT, vn_id);
        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(patch)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn rlist_patch(&self, r_id: &String, patch: &RListPatch) -> Result<(), VndbApiError> {
        let url = format!("{}/rlist/{}", API_ENDPOINT, r_id);
        let response = self
            .client
            .patch(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .header("Content-Type", "application/json")
            .json(patch)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn ulist_remove(&self, vn_id: &String) -> Result<(), VndbApiError> {
        let url = format!("{}/ulist/{}", API_ENDPOINT, vn_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }

    pub async fn rlist_remove(&self, r_id: &String) -> Result<(), VndbApiError> {
        let url = format!("{}/rlist/{}", API_ENDPOINT, r_id);
        let response = self
            .client
            .delete(&url)
            .header("Authorization", format!("token {}", self.access_token))
            .send()
            .await?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(VndbApiError::new(response.status(), response.text().await?))
        }
    }
}
