use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub struct VnQuery {}
pub struct ReleaseQuery {}
pub struct ProducerQuery {}
pub struct CharacterQuery {}
pub struct StaffQuery {}
pub struct TagQuery {}
pub struct TraitQuery {}
pub struct UListQuery {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Query<T> {
    /// Used to determine which database items to fetch
    filters: Option<Vec<String>>,
    /// List of fields to fetch for each database item
    fields: Option<String>,
    /// Field to sort on
    sort: Option<SortField>,
    /// Set to true to sort in descending order
    reverse: Option<bool>,
    /// Number of results per page max 100
    /// can also be set to 0 if you’re not interested in the results at all
    results: Option<u8>,
    /// Page number to request starting from 1
    page: Option<u8>,
    /// User ID
    user: Option<String>,
    /// Whether the response should include the count field
    count: Option<bool>,
    /// Whether the response should include the compact_filters field
    compact_filters: Option<bool>,
    /// Whether the response should include the normalized_filters field
    normalized_filters: Option<bool>,
    #[serde(skip)]
    _phantom: Option<PhantomData<T>>,
}

#[derive(Debug)]
pub struct QueryBuilder<T> {
    pub filters: Option<Vec<String>>,
    pub fields: Option<String>,
    pub sort: Option<SortField>,
    pub reverse: Option<bool>,
    pub results: Option<u8>,
    pub page: Option<u8>,
    pub user: Option<String>,
    pub count: Option<bool>,
    pub compact_filters: Option<bool>,
    pub normalized_filters: Option<bool>,
    _phantom: Option<PhantomData<T>>,
}

impl<T> QueryBuilder<T> {
    pub fn new() -> Self {
        Self {
            filters: None,
            fields: None,
            sort: Some(SortField::Id),
            reverse: Some(false),
            results: Some(10),
            page: Some(1),
            user: None,
            count: Some(false),
            compact_filters: Some(false),
            normalized_filters: Some(false),
            _phantom: None,
        }
    }

    pub fn filters(mut self, filters: Vec<String>) -> Self {
        self.filters = Some(filters.clone());
        self
    }

    pub fn reverse(mut self) -> Self {
        self.reverse = Some(true);
        self
    }

    pub fn results(mut self, num_results: u8) -> Self {
        self.results = Some(num_results.clamp(0, 100));
        self
    }

    pub fn page(mut self, page_num: u8) -> Self {
        self.page = Some(page_num);
        self
    }

    pub fn user(mut self, user_id: &String) -> Self {
        self.user = Some(user_id.clone());
        self
    }

    pub fn enable_compact_filters(mut self) -> Self {
        self.compact_filters = Some(true);
        self
    }

    pub fn enable_normalized_filters(mut self) -> Self {
        self.normalized_filters = Some(true);
        self
    }

    pub fn build(self) -> Query<T> {
        Query::<T> {
            filters: self.filters,
            fields: self.fields,
            sort: self.sort,
            reverse: self.reverse,
            results: self.results,
            page: self.page,
            user: self.user,
            count: self.count,
            compact_filters: self.compact_filters,
            normalized_filters: self.normalized_filters,
            _phantom: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortField {
    Id,
    Title,
    Released,
    Rating,
    Votecount,
    Searchrank,
    Name,
    VnCount,
    CharCount,
    Voted,
    Vote,
    Added,
    Lastmod,
    Started,
    Finished,
}

pub struct VnFieldChoices(pub Vec<VnField>);

#[derive(Serialize, Debug, EnumIter)]
pub enum VnField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "alttitle")]
    Alttitle,
    #[serde(rename = "titles.lang")]
    TitlesLang,
    #[serde(rename = "titles.title")]
    TitlesTitle,
    #[serde(rename = "titles.latin")]
    TitlesLatin,
    #[serde(rename = "titles.official")]
    TitlesOfficial,
    #[serde(rename = "titles.main")]
    TitlesMain,
    #[serde(rename = "aliases")]
    Aliases,
    #[serde(rename = "olang")]
    Olang,
    #[serde(rename = "devstatus")]
    DevStatus,
    #[serde(rename = "released")]
    Released,
    #[serde(rename = "languages")]
    Languages,
    #[serde(rename = "platforms")]
    Platforms,
    #[serde(rename = "image.id")]
    ImageId,
    #[serde(rename = "image.url")]
    ImageUrl,
    #[serde(rename = "image.dims")]
    ImageDims,
    #[serde(rename = "image.sexual")]
    ImageSexual,
    #[serde(rename = "image.violence")]
    ImageViolence,
    #[serde(rename = "image.votecount")]
    ImageVoteCount,
    #[serde(rename = "image.thumbnail")]
    ImageThumbnail,
    #[serde(rename = "image.thumbnail_dims")]
    ImageThumbnailDims,
    #[serde(rename = "length")]
    Length,
    #[serde(rename = "length_minutes")]
    LengthMinutes,
    #[serde(rename = "length_votes")]
    LengthVotes,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "average")]
    Average,
    #[serde(rename = "rating")]
    Rating,
    #[serde(rename = "votecount")]
    VoteCount,
    #[serde(rename = "screenshots.id")]
    ScreenshotsId,
    #[serde(rename = "screenshots.url")]
    ScreenshotsUrl,
    #[serde(rename = "screenshots.dims")]
    ScreenshotsDims,
    #[serde(rename = "screenshots.sexual")]
    ScreenshotsSexual,
    #[serde(rename = "screenshots.violence")]
    ScreenshotsViolence,
    #[serde(rename = "screenshots.votecount")]
    ScreenshotsVoteCount,
    #[serde(rename = "screenshots.thumbnail")]
    ScreenshotsThumbnail,
    #[serde(rename = "screenshots.thumbnail_dims")]
    ScreenshotsThumbnailDims,
    #[serde(rename = "screenshots.release.id")]
    ScreenshotsReleaseId,
    #[serde(rename = "screenshots.release.title")]
    ScreenshotsReleaseTitle,
    #[serde(rename = "relations.relation")]
    RelationsRelation,
    #[serde(rename = "relations.relation_official")]
    RelationsRelationOfficial,
    #[serde(rename = "relations.id")]
    RelationsId,
    #[serde(rename = "relations.title")]
    RelationsTitle,
    #[serde(rename = "tags.rating")]
    TagsRating,
    #[serde(rename = "tags.spoiler")]
    TagsSpoiler,
    #[serde(rename = "tags.lie")]
    TagsLie,
    #[serde(rename = "tags.id")]
    TagsId,
    #[serde(rename = "tags.name")]
    TagsName,
    #[serde(rename = "tags.aliases")]
    TagsAliases,
    #[serde(rename = "tags.description")]
    TagsDescription,
    #[serde(rename = "tags.category")]
    TagsCategory,
    #[serde(rename = "tags.searchable")]
    TagsSearchable,
    #[serde(rename = "tags.applicable")]
    TagsApplicable,
    #[serde(rename = "developers.id")]
    DevelopersId,
    #[serde(rename = "developers.name")]
    DevelopersName,
    #[serde(rename = "developers.original")]
    DevelopersOriginal,
    #[serde(rename = "developers.aliases")]
    DevelopersAliases,
    #[serde(rename = "developers.lang")]
    DevelopersLang,
    #[serde(rename = "developers.type")]
    DevelopersType,
    #[serde(rename = "developers.description")]
    DevelopersDescription,
    #[serde(rename = "editions.eid")]
    EditionsEid,
    #[serde(rename = "editions.lang")]
    EditionsLang,
    #[serde(rename = "editions.name")]
    EditionsName,
    #[serde(rename = "editions.official")]
    EditionsOfficial,
    #[serde(rename = "staff.eid")]
    StaffEid,
    #[serde(rename = "staff.role")]
    StaffRole,
    #[serde(rename = "staff.note")]
    StaffNote,
    #[serde(rename = "staff.id")]
    StaffId,
    #[serde(rename = "staff.aid")]
    StaffAid,
    #[serde(rename = "staff.ismain")]
    StaffIsmain,
    #[serde(rename = "staff.name")]
    StaffName,
    #[serde(rename = "staff.lang")]
    StaffLang,
    #[serde(rename = "staff.gender")]
    StaffGender,
    #[serde(rename = "staff.description")]
    StaffDescription,
    #[serde(rename = "staff.aliases.aid")]
    StaffAliasesAid,
    #[serde(rename = "staff.aliases.name")]
    StaffAliasesName,
    #[serde(rename = "staff.aliases.latin")]
    StaffAliasesLatin,
    #[serde(rename = "staff.aliases.ismain")]
    StaffAliasesIsmain,
    #[serde(rename = "va.note")]
    VaNote,
    #[serde(rename = "va.staff.id")]
    VaStaffId,
    #[serde(rename = "va.staff.aid")]
    VaStaffAid,
    #[serde(rename = "va.staff.ismain")]
    VaStaffIsmain,
    #[serde(rename = "va.staff.name")]
    VaStaffName,
    #[serde(rename = "va.staff.lang")]
    VaStaffLang,
    #[serde(rename = "va.staff.gender")]
    VaStaffGender,
    #[serde(rename = "va.staff.description")]
    VaStaffDescription,
    #[serde(rename = "va.character.id")]
    VaCharacterId,
    #[serde(rename = "va.character.name")]
    VaCharacterName,
    #[serde(rename = "va.staff.extlinks.url")]
    VaStaffExtlinksUrl,
    #[serde(rename = "va.staff.extlinks.label")]
    VaStaffExtlinksLabel,
    #[serde(rename = "va.staff.extlinks.name")]
    VaStaffExtlinksName,
    #[serde(rename = "va.staff.extlinks.id")]
    VaStaffExtlinksId,
    #[serde(rename = "va.staff.aliases.aid")]
    VaStaffAliasesAid,
    #[serde(rename = "va.staff.aliases.name")]
    VaStaffAliasesName,
    #[serde(rename = "va.staff.aliases.latin")]
    VaStaffAliasesLatin,
    #[serde(rename = "va.staff.aliases.ismain")]
    VaStaffAliasesIsmain,
}

impl VnFieldChoices {
    pub fn new() -> Self {
        VnFieldChoices(vec![])
    }

    pub fn from(vec: Vec<VnField>) -> Self {
        VnFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(VnField::iter().len());
        for variant in VnField::iter() {
            vec.push(variant);
        }
        VnFieldChoices(vec)
    }
}

impl QueryBuilder<VnQuery> {
    pub fn fields(mut self, fields: VnFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id
            | SortField::Title
            | SortField::Released
            | SortField::Rating
            | SortField::Votecount
            | SortField::Searchrank => self.sort = Some(sort_field),
            _ => return self,
        }
        self
    }
}

pub struct ReleaseFieldChoices(pub Vec<ReleaseField>);

#[derive(Serialize, Debug, EnumIter)]
pub enum ReleaseField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "alttitle")]
    Alttitle,
    #[serde(rename = "languages.lang")]
    LanguagesLang,
    #[serde(rename = "languages.title")]
    LanguagesTitle,
    #[serde(rename = "languages.latin")]
    LanguagesLatin,
    #[serde(rename = "languages.mtl")]
    LaguagesMtl,
    #[serde(rename = "languages.main")]
    LanguagesMain,
    #[serde(rename = "platforms")]
    Platforms,
    #[serde(rename = "media.medium")]
    MediaMedium,
    #[serde(rename = "media.qty")]
    MediaQty,
    #[serde(rename = "vns.rtype")]
    VnsRtype,
    #[serde(rename = "vns.id")]
    VnsId,
    #[serde(rename = "vns.title")]
    VnsTitle,
    #[serde(rename = "producers.developer")]
    ProducersDeveloper,
    #[serde(rename = "producers.publisher")]
    ProducersPublisher,
    #[serde(rename = "producers.id")]
    ProducersId,
    #[serde(rename = "producers.name")]
    ProducersName,
    #[serde(rename = "producers.original")]
    ProducersOriginal,
    #[serde(rename = "producers.aliases")]
    ProducersAliases,
    #[serde(rename = "producers.lang")]
    ProducersLang,
    #[serde(rename = "producers.type")]
    ProducersType,
    #[serde(rename = "producers.description")]
    ProducersDescription,
    #[serde(rename = "released")]
    Released,
    #[serde(rename = "minage")]
    MinAge,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "freeware")]
    Freeware,
    #[serde(rename = "uncensored")]
    Uncensored,
    #[serde(rename = "official")]
    Official,
    #[serde(rename = "has_ero")]
    HasEro,
    #[serde(rename = "resolution")]
    Resolution,
    #[serde(rename = "engine")]
    Engine,
    #[serde(rename = "voiced")]
    Voiced,
    #[serde(rename = "notes")]
    Notes,
    #[serde(rename = "gtin")]
    Gtin,
    #[serde(rename = "catalog")]
    Catalog,
    #[serde(rename = "extlinks.url")]
    ExtlinksUrl,
    #[serde(rename = "extlinks.label")]
    ExtlinksLabel,
    #[serde(rename = "extlinks.name")]
    ExtlinksName,
    #[serde(rename = "extlinks.id")]
    ExtlinksId,
}

impl ReleaseFieldChoices {
    pub fn new() -> Self {
        ReleaseFieldChoices(vec![])
    }

    pub fn from(vec: Vec<ReleaseField>) -> Self {
        ReleaseFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(ReleaseField::iter().len());
        for variant in ReleaseField::iter() {
            vec.push(variant);
        }
        ReleaseFieldChoices(vec)
    }
}

impl QueryBuilder<ReleaseQuery> {
    pub fn fields(mut self, fields: ReleaseFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Title | SortField::Released | SortField::Searchrank => {
                self.sort = Some(sort_field)
            }
            _ => return self,
        }
        self
    }
}

pub struct ProducerFieldChoices(pub Vec<ProducerField>);

#[derive(Serialize, Debug, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum ProducerField {
    Id,
    Name,
    Original,
    Aliases,
    Lang,
    Type,
    Description,
}

impl ProducerFieldChoices {
    pub fn new() -> Self {
        ProducerFieldChoices(vec![])
    }

    pub fn from(vec: Vec<ProducerField>) -> Self {
        ProducerFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(ProducerField::iter().len());
        for variant in ProducerField::iter() {
            vec.push(variant);
        }
        ProducerFieldChoices(vec)
    }
}

impl QueryBuilder<ProducerQuery> {
    pub fn fields(mut self, fields: ProducerFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Name | SortField::Searchrank => self.sort = Some(sort_field),
            _ => return self,
        }
        self
    }
}

pub struct CharacterFieldChoices(pub Vec<CharacterField>);

#[derive(Serialize, Debug, EnumIter)]
pub enum CharacterField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "aliases")]
    Aliases,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "image.id")]
    ImageId,
    #[serde(rename = "image.url")]
    ImageUrl,
    #[serde(rename = "image.dims")]
    ImageDims,
    #[serde(rename = "image.sexual")]
    ImageSexual,
    #[serde(rename = "image.violence")]
    ImageViolence,
    #[serde(rename = "image.votecount")]
    ImageVoteCount,
    #[serde(rename = "blood_type")]
    BloodType,
    #[serde(rename = "height")]
    Height,
    #[serde(rename = "weight")]
    Weight,
    #[serde(rename = "bust")]
    Bust,
    #[serde(rename = "waist")]
    Waist,
    #[serde(rename = "hips")]
    Hips,
    #[serde(rename = "cup")]
    Cup,
    #[serde(rename = "age")]
    Age,
    #[serde(rename = "birthday")]
    Birthday,
    #[serde(rename = "sex")]
    Sex,
    #[serde(rename = "vns.spoiler")]
    VnsSpoiler,
    #[serde(rename = "vns.role")]
    VnsRole,
    #[serde(rename = "vns.id")]
    VnsId,
    #[serde(rename = "vns.title")]
    VnsTitle,
    #[serde(rename = "vns.release.id")]
    VnsReleaseId,
    #[serde(rename = "vns.release.title")]
    VnsReleaseTitle,
    #[serde(rename = "traits.spoiler")]
    TraitsSpoiler,
    #[serde(rename = "traits.lie")]
    TraitsLie,
    #[serde(rename = "traits.id")]
    TraitsId,
    #[serde(rename = "traits.name")]
    TraitsName,
    #[serde(rename = "traits.aliases")]
    TraitsAliases,
    #[serde(rename = "traits.description")]
    TraitsDescription,
    #[serde(rename = "traits.searchable")]
    TraitsSearchable,
    #[serde(rename = "traits.applicable")]
    TraitsApplicable,
    #[serde(rename = "traits.group_id")]
    TraitsGroupId,
    #[serde(rename = "traits.group_name")]
    TraitsGroupName,
    #[serde(rename = "traits.char_count")]
    TraitsCharCount,
}

impl CharacterFieldChoices {
    pub fn new() -> Self {
        CharacterFieldChoices(vec![])
    }

    pub fn from(vec: Vec<CharacterField>) -> Self {
        CharacterFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(CharacterField::iter().len());
        for variant in CharacterField::iter() {
            vec.push(variant);
        }
        CharacterFieldChoices(vec)
    }
}

impl QueryBuilder<CharacterQuery> {
    pub fn fields(mut self, fields: CharacterFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Name | SortField::Searchrank => self.sort = Some(sort_field),
            _ => return self,
        }
        self
    }
}

pub struct StaffFieldChoices(pub Vec<StaffField>);

#[derive(Serialize, Debug, EnumIter)]
pub enum StaffField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "aid")]
    Aid,
    #[serde(rename = "ismain")]
    Ismain,
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "lang")]
    Lang,
    #[serde(rename = "gender")]
    Gender,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "extlinks.url")]
    ExtlinksUrl,
    #[serde(rename = "extlinks.label")]
    ExtlinksLabel,
    #[serde(rename = "extlinks.name")]
    ExtlinksName,
    #[serde(rename = "extlinks.id")]
    ExtlinksId,
    #[serde(rename = "aliases.aid")]
    AliasesAid,
    #[serde(rename = "aliases.name")]
    AliasesName,
    #[serde(rename = "aliases.latin")]
    AliasesLatin,
    #[serde(rename = "aliases.ismain")]
    AliasesIsmain,
}

impl StaffFieldChoices {
    pub fn new() -> Self {
        StaffFieldChoices(vec![])
    }

    pub fn from(vec: Vec<StaffField>) -> Self {
        StaffFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(StaffField::iter().len());
        for variant in StaffField::iter() {
            vec.push(variant);
        }
        StaffFieldChoices(vec)
    }
}

impl QueryBuilder<StaffQuery> {
    pub fn fields(mut self, fields: StaffFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Name | SortField::Searchrank => self.sort = Some(sort_field),
            _ => return self,
        }
        self
    }
}

pub struct TagFieldChoices(pub Vec<TagField>);

#[derive(Serialize, Debug, EnumIter)]
#[serde(rename_all = "lowercase")]
pub enum TagField {
    Id,
    Name,
    Aliases,
    Description,
    Category,
    Searchable,
    Applicable,
}

impl TagFieldChoices {
    pub fn new() -> Self {
        TagFieldChoices(vec![])
    }

    pub fn from(vec: Vec<TagField>) -> Self {
        TagFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(TagField::iter().len());
        for variant in TagField::iter() {
            vec.push(variant);
        }
        TagFieldChoices(vec)
    }
}

impl QueryBuilder<TagQuery> {
    pub fn fields(mut self, fields: TagFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Name | SortField::VnCount | SortField::Searchrank => {
                self.sort = Some(sort_field)
            }
            _ => return self,
        }
        self
    }
}

pub struct TraitFieldChoices(pub Vec<TraitField>);

#[derive(Serialize, Debug, EnumIter)]
#[serde(rename_all = "snake_case")]
pub enum TraitField {
    Id,
    Name,
    Aliases,
    Description,
    Searchable,
    Applicable,
    GroupId,
    GroupName,
    CharCount,
}

impl TraitFieldChoices {
    pub fn new() -> Self {
        TraitFieldChoices(vec![])
    }

    pub fn from(vec: Vec<TraitField>) -> Self {
        TraitFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(TraitField::iter().len());
        for variant in TraitField::iter() {
            vec.push(variant);
        }
        TraitFieldChoices(vec)
    }
}

impl QueryBuilder<TraitQuery> {
    pub fn fields(mut self, fields: TraitFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn enable_count(mut self) -> Self {
        self.count = Some(true);
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id | SortField::Name | SortField::CharCount | SortField::Searchrank => {
                self.sort = Some(sort_field)
            }
            _ => return self,
        }
        self
    }
}

pub struct UListFieldChoices(pub Vec<UListField>);

#[derive(Serialize, Debug, EnumIter)]
pub enum UListField {
    #[serde(rename = "id")]
    Id,
    #[serde(rename = "added")]
    Added,
    #[serde(rename = "voted")]
    Voted,
    #[serde(rename = "lastmod")]
    LastMod,
    #[serde(rename = "vote")]
    Vote,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "notes")]
    Notes,
    #[serde(rename = "labels.id")]
    LabelsId,
    #[serde(rename = "labels.label")]
    LabelsLabel,
    /// All /vn fields apply
    #[serde(rename = "vn.id")]
    VnId,
    #[serde(rename = "vn.title")]
    VnTitle,
    #[serde(rename = "vn.alttitle")]
    VnAlttitle,
    #[serde(rename = "vn.titles.lang")]
    VnTitlesLang,
    #[serde(rename = "vn.titles.title")]
    VnTitlesTitle,
    #[serde(rename = "vn.titles.latin")]
    VnTitlesLatin,
    #[serde(rename = "vn.titles.official")]
    VnTitlesOfficial,
    #[serde(rename = "vn.titles.main")]
    VnTitlesMain,
    #[serde(rename = "vn.aliases")]
    VnAliases,
    #[serde(rename = "vn.olang")]
    VnOlang,
    #[serde(rename = "vn.devstatus")]
    VnDevStatus,
    #[serde(rename = "vn.released")]
    VnReleased,
    #[serde(rename = "vn.languages")]
    VnLanguages,
    #[serde(rename = "vn.platforms")]
    VnPlatforms,
    #[serde(rename = "vn.image.id")]
    VnImageId,
    #[serde(rename = "vn.image.url")]
    VnImageUrl,
    #[serde(rename = "vn.image.dims")]
    VnImageDims,
    #[serde(rename = "vn.image.sexual")]
    VnImageSexual,
    #[serde(rename = "vn.image.violence")]
    VnImageViolence,
    #[serde(rename = "vn.image.votecount")]
    VnImageVoteCount,
    #[serde(rename = "vn.image.thumbnail")]
    VnImageThumbnail,
    #[serde(rename = "vn.image.thumbnail_dims")]
    VnImageThumbnailDims,
    #[serde(rename = "vn.length")]
    VnLength,
    #[serde(rename = "vn.length_minutes")]
    VnLengthMinutes,
    #[serde(rename = "vn.length_votes")]
    VnLengthVotes,
    #[serde(rename = "vn.description")]
    VnDescription,
    #[serde(rename = "vn.average")]
    VnAverage,
    #[serde(rename = "vn.rating")]
    VnRating,
    #[serde(rename = "vn.votecount")]
    VnVoteCount,
    #[serde(rename = "vn.screenshots.id")]
    VnScreenshotsId,
    #[serde(rename = "vn.screenshots.url")]
    VnScreenshotsUrl,
    #[serde(rename = "vn.screenshots.dims")]
    VnScreenshotsDims,
    #[serde(rename = "vn.screenshots.sexual")]
    VnScreenshotsSexual,
    #[serde(rename = "vn.screenshots.violence")]
    VnScreenshotsViolence,
    #[serde(rename = "vn.screenshots.votecount")]
    VnScreenshotsVoteCount,
    #[serde(rename = "vn.screenshots.thumbnail")]
    VnScreenshotsThumbnail,
    #[serde(rename = "vn.screenshots.thumbnail_dims")]
    VnScreenshotsThumbnailDims,
    #[serde(rename = "vn.screenshots.release.id")]
    VnScreenshotsReleaseId,
    #[serde(rename = "vn.screenshots.release.title")]
    VnScreenshotsReleaseTitle,
    #[serde(rename = "vn.relations.relation")]
    VnRelationsRelation,
    #[serde(rename = "vn.relations.relation_official")]
    VnRelationsRelationOfficial,
    #[serde(rename = "vn.relations.id")]
    VnRelationsId,
    #[serde(rename = "vn.relations.title")]
    VnRelationsTitle,
    #[serde(rename = "vn.tags.rating")]
    VnTagsRating,
    #[serde(rename = "vn.tags.spoiler")]
    VnTagsSpoiler,
    #[serde(rename = "vn.tags.lie")]
    VnTagsLie,
    #[serde(rename = "vn.tags.id")]
    VnTagsId,
    #[serde(rename = "vn.tags.name")]
    VnTagsName,
    #[serde(rename = "vn.tags.aliases")]
    VnTagsAliases,
    #[serde(rename = "vn.tags.description")]
    VnTagsDescription,
    #[serde(rename = "vn.tags.category")]
    VnTagsCategory,
    #[serde(rename = "vn.tags.searchable")]
    VnTagsSearchable,
    #[serde(rename = "vn.tags.applicable")]
    VnTagsApplicable,
    #[serde(rename = "vn.developers.id")]
    VnDevelopersId,
    #[serde(rename = "vn.developers.name")]
    VnDevelopersName,
    #[serde(rename = "vn.developers.original")]
    VnDevelopersOriginal,
    #[serde(rename = "vn.developers.aliases")]
    VnDevelopersAliases,
    #[serde(rename = "vn.developers.lang")]
    VnDevelopersLang,
    #[serde(rename = "vn.developers.type")]
    VnDevelopersType,
    #[serde(rename = "vn.developers.description")]
    VnDevelopersDescription,
    #[serde(rename = "vn.editions.eid")]
    VnEditionsEid,
    #[serde(rename = "vn.editions.lang")]
    VnEditionsLang,
    #[serde(rename = "vn.editions.name")]
    VnEditionsName,
    #[serde(rename = "vn.editions.official")]
    VnEditionsOfficial,
    #[serde(rename = "vn.staff.eid")]
    VnStaffEid,
    #[serde(rename = "vn.staff.role")]
    VnStaffRole,
    #[serde(rename = "vn.staff.note")]
    VnStaffNote,
    #[serde(rename = "vn.staff.id")]
    VnStaffId,
    #[serde(rename = "vn.staff.aid")]
    VnStaffAid,
    #[serde(rename = "vn.staff.ismain")]
    VnStaffIsmain,
    #[serde(rename = "vn.staff.name")]
    VnStaffName,
    #[serde(rename = "vn.staff.lang")]
    VnStaffLang,
    #[serde(rename = "vn.staff.gender")]
    VnStaffGender,
    #[serde(rename = "vn.staff.description")]
    VnStaffDescription,
    #[serde(rename = "vn.staff.aliases.aid")]
    VnStaffAliasesAid,
    #[serde(rename = "vn.staff.aliases.name")]
    VnStaffAliasesName,
    #[serde(rename = "vn.staff.aliases.latin")]
    VnStaffAliasesLatin,
    #[serde(rename = "vn.staff.aliases.ismain")]
    VnStaffAliasesIsmain,
    #[serde(rename = "vn.va.note")]
    VnVaNote,
    #[serde(rename = "vn.va.staff.id")]
    VnVaStaffId,
    #[serde(rename = "vn.va.staff.aid")]
    VnVaStaffAid,
    #[serde(rename = "vn.va.staff.ismain")]
    VnVaStaffIsmain,
    #[serde(rename = "vn.va.staff.name")]
    VnVaStaffName,
    #[serde(rename = "vn.va.staff.lang")]
    VnVaStaffLang,
    #[serde(rename = "vn.va.staff.gender")]
    VnVaStaffGender,
    #[serde(rename = "vn.va.staff.description")]
    VnVaStaffDescription,
    #[serde(rename = "vn.va.character.id")]
    VnVaCharacterId,
    #[serde(rename = "vn.va.character.name")]
    VnVaCharacterName,
    #[serde(rename = "vn.va.staff.extlinks.url")]
    VnVaStaffExtlinksUrl,
    #[serde(rename = "vn.va.staff.extlinks.label")]
    VnVaStaffExtlinksLabel,
    #[serde(rename = "vn.va.staff.extlinks.name")]
    VnVaStaffExtlinksName,
    #[serde(rename = "vn.va.staff.extlinks.id")]
    VnVaStaffExtlinksId,
    #[serde(rename = "vn.va.staff.aliases.aid")]
    VnVaStaffAliasesAid,
    #[serde(rename = "vn.va.staff.aliases.name")]
    VnVaStaffAliasesName,
    #[serde(rename = "vn.va.staff.aliases.latin")]
    VnVaStaffAliasesLatin,
    #[serde(rename = "vn.va.staff.aliases.ismain")]
    VnVaStaffAliasesIsmain,
    /// All /release fields apply
    #[serde(rename = "releases.list_status")]
    ReleasesListStatus,
    #[serde(rename = "releases.id")]
    ReleasesId,
    #[serde(rename = "releases.title")]
    ReleasesTitle,
    #[serde(rename = "releases.alttitle")]
    ReleasesAlttitle,
    #[serde(rename = "releases.languages.lang")]
    ReleasesLanguagesLang,
    #[serde(rename = "releases.languages.title")]
    ReleasesLanguagesTitle,
    #[serde(rename = "releases.languages.latin")]
    ReleasesLanguagesLatin,
    #[serde(rename = "releases.languages.mtl")]
    ReleasesLaguagesMtl,
    #[serde(rename = "releases.languages.main")]
    ReleasesLanguagesMain,
    #[serde(rename = "releases.platforms")]
    ReleasesPlatforms,
    #[serde(rename = "releases.media.medium")]
    ReleasesMediaMedium,
    #[serde(rename = "releases.media.qty")]
    ReleasesMediaQty,
    #[serde(rename = "releases.vns.rtype")]
    ReleasesVnsRtype,
    #[serde(rename = "releases.vns.id")]
    ReleasesVnsId,
    #[serde(rename = "releases.vns.title")]
    ReleasesVnsTitle,
    #[serde(rename = "releases.producers.developer")]
    ReleasesProducersDeveloper,
    #[serde(rename = "releases.producers.publisher")]
    ReleasesProducersPublisher,
    #[serde(rename = "releases.producers.id")]
    ReleasesProducersId,
    #[serde(rename = "releases.producers.name")]
    ReleasesProducersName,
    #[serde(rename = "releases.producers.original")]
    ReleasesProducersOriginal,
    #[serde(rename = "releases.producers.aliases")]
    ReleasesProducersAliases,
    #[serde(rename = "releases.producers.lang")]
    ReleasesProducersLang,
    #[serde(rename = "releases.producers.type")]
    ReleasesProducersType,
    #[serde(rename = "releases.producers.description")]
    ReleasesProducersDescription,
    #[serde(rename = "releases.released")]
    ReleasesReleased,
    #[serde(rename = "releases.minage")]
    ReleasesMinAge,
    #[serde(rename = "releases.patch")]
    ReleasesPatch,
    #[serde(rename = "releases.freeware")]
    ReleasesFreeware,
    #[serde(rename = "releases.uncensored")]
    ReleasesUncensored,
    #[serde(rename = "releases.official")]
    ReleasesOfficial,
    #[serde(rename = "releases.has_ero")]
    ReleasesHasEro,
    #[serde(rename = "releases.resolution")]
    ReleasesResolution,
    #[serde(rename = "releases.engine")]
    ReleasesEngine,
    #[serde(rename = "releases.voiced")]
    ReleasesVoiced,
    #[serde(rename = "releases.notes")]
    ReleasesNotes,
    #[serde(rename = "releases.gtin")]
    ReleasesGtin,
    #[serde(rename = "releases.catalog")]
    ReleasesCatalog,
    #[serde(rename = "releases.extlinks.url")]
    ReleasesExtlinksUrl,
    #[serde(rename = "releases.extlinks.label")]
    ReleasesExtlinksLabel,
    #[serde(rename = "releases.extlinks.name")]
    ReleasesExtlinksName,
    #[serde(rename = "releases.extlinks.id")]
    ReleasesExtlinksId,
}

impl UListFieldChoices {
    pub fn new() -> Self {
        UListFieldChoices(vec![])
    }

    pub fn from(vec: Vec<UListField>) -> Self {
        UListFieldChoices(vec)
    }

    pub fn all() -> Self {
        let mut vec = Vec::with_capacity(UListField::iter().len());
        for variant in UListField::iter() {
            vec.push(variant);
        }
        UListFieldChoices(vec)
    }
}

impl QueryBuilder<UListQuery> {
    pub fn fields(mut self, fields: UListFieldChoices) -> Self {
        self.fields = Some(
            fields
                .0
                .iter()
                .map(|field| serde_json::to_string(&field).unwrap().replace("\"", ""))
                .collect::<Vec<String>>()
                .join(","),
        );
        self
    }

    pub fn sort(mut self, sort_field: SortField) -> Self {
        match sort_field {
            SortField::Id
            | SortField::Title
            | SortField::Released
            | SortField::Rating
            | SortField::Votecount
            | SortField::Voted
            | SortField::Vote
            | SortField::Added
            | SortField::Lastmod
            | SortField::Started
            | SortField::Finished
            | SortField::Searchrank => self.sort = Some(sort_field),
            _ => return self,
        }
        self
    }
}
