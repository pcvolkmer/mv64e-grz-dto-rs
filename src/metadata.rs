use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// General metadata schema for submissions to the GRZ
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    /// List of donors including the index patient.
    pub donors: Vec<Donor>,

    pub submission: Submission,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Donor {
    /// A unique identifier given by the Leistungserbringer for each donor of a single, duo or
    /// trio sequencing; the donorPseudonym needs to be identifiable by the Leistungserbringer in
    /// case of changes to the consents by one of the donors. For Index patient use index.
    pub donor_pseudonym: String,

    /// Gender of the donor.
    pub gender: Gender,

    /// Lab data related to the donor.
    pub lab_data: Vec<LabDatum>,

    pub mv_consent: MvConsent,

    /// Relationship of the donor in respect to the index patient, e.g. 'index', 'brother',
    /// 'mother', etc.
    pub relation: Relation,

    /// Research consents. Multiple declarations of consent are possible! Must be assigned to the
    /// respective data sets.
    pub research_consents: Vec<ResearchConsent>,
}

/// Gender of the donor.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct LabDatum {
    /// The barcode used or 'na'
    pub barcode: String,

    /// Name/version of the enrichment kit
    pub enrichment_kit_description: String,

    /// Manufacturer of the enrichment kit
    pub enrichment_kit_manufacturer: EnrichmentKitManufacturer,

    /// Fragmentation method
    pub fragmentation_method: FragmentationMethod,

    /// Sequencing kit manufacturer
    pub kit_manufacturer: String,

    /// Name/version of the sequencing kit
    pub kit_name: String,

    /// Name/ID of the biospecimen e.g. 'Blut DNA normal'
    pub lab_data_name: String,

    /// Name/version of the library prepkit
    pub library_prep_kit: String,

    /// Library prep kit manufacturer
    pub library_prep_kit_manufacturer: String,

    /// Library type
    pub library_type: LibraryType,

    /// Sample conservation
    pub sample_conservation: SampleConservation,

    /// Date of sample in ISO 8601 format YYYY-MM-DD
    pub sample_date: String,

    /// Sequence data generated from the wet lab experiment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_data: Option<SequenceData>,

    /// Subtype of sequence (germline, somatic, etc.)
    pub sequence_subtype: SequenceSubtype,

    /// Type of sequence (DNA or RNA)
    pub sequence_type: SequenceType,

    /// Sequencer manufacturer
    pub sequencer_manufacturer: String,

    /// Name/version of the sequencer model
    pub sequencer_model: String,

    /// The sequencing layout, aka the end type of sequencing.
    pub sequencing_layout: SequencingLayout,

    pub tissue_ontology: TissueOntology,

    /// Tissue ID according to the ontology in use.
    pub tissue_type_id: String,

    /// Tissue name according to the ontology in use.
    pub tissue_type_name: String,

    /// Tuple of tumor cell counts and how they were determined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_count: Option<Vec<TumorCellCount>>,
}

/// Manufacturer of the enrichment kit
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum EnrichmentKitManufacturer {
    Agilent,

    Illumina,

    #[serde(rename = "NEB")]
    Neb,

    #[serde(rename = "none")]
    None,

    #[serde(rename = "other")]
    Other,

    Twist,

    #[serde(rename = "unknown")]
    Unknown,
}

/// Fragmentation method
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum FragmentationMethod {
    Enzymatic,

    None,

    Other,

    Sonication,

    Unknown,
}

/// Library type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum LibraryType {
    Other,

    Panel,

    #[serde(rename = "panel_lr")]
    PanelLr,

    Unknown,

    Wes,

    #[serde(rename = "wes_lr")]
    WesLr,

    Wgs,

    #[serde(rename = "wgs_lr")]
    WgsLr,

    Wxs,

    #[serde(rename = "wxs_lr")]
    WxsLr,
}

/// Sample conservation
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum SampleConservation {
    #[serde(rename = "cryo-frozen")]
    CryoFrozen,

    Ffpe,

    #[serde(rename = "fresh-tissue")]
    FreshTissue,

    Other,

    Unknown,
}

/// Sequence data generated from the wet lab experiment.
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct SequenceData {
    /// Name of the bioinformatics pipeline used
    pub bioinformatics_pipeline_name: String,

    /// Version or commit hash of the bioinformatics pipeline
    pub bioinformatics_pipeline_version: String,

    /// Caller that is used in the pipeline
    pub caller_used: Vec<CallerUsed>,

    /// List of files generated and required in this analysis.
    pub files: Vec<File>,

    /// Mean depth of coverage
    pub mean_depth_of_coverage: f64,

    /// Minimum coverage
    pub min_coverage: f64,

    /// The analysis includes non-coding variants -> true or false
    pub non_coding_variants: bool,

    /// Percentage of bases with a specified minimum quality threshold, according to
    /// https://www.bfarm.de/SharedDocs/Downloads/DE/Forschung/modellvorhaben-genomsequenzierung/Qs-durch-GRZ.pdf?__blob=publicationFile
    pub percent_bases_above_quality_threshold: PercentBasesAboveQualityThreshold,

    /// Reference genome used according to the Genome Reference Consortium
    /// (https://www.ncbi.nlm.nih.gov/grc)
    pub reference_genome: ReferenceGenome,

    /// Fraction of targeted regions that are above minimum coverage
    pub targeted_regions_above_min_coverage: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CallerUsed {
    /// Name of the caller used
    pub name: String,

    /// Version of the caller used
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct File {
    /// Type of checksum algorithm used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_type: Option<ChecksumType>,

    /// checksum of the file
    pub file_checksum: String,

    /// Path relative to the submission files directory, e.g.:
    /// 'patient_001/patient_001_dna.fastq.gz' if the file is located in <submission
    /// root>/files/patient_001/patient_001_dna.fastq.gz
    pub file_path: String,

    /// Size of the file in bytes
    pub file_size_in_bytes: f64,

    /// Type of the file; if BED file is submitted, only 1 file is allowed.
    pub file_type: FileType,

    /// Indicates the flow cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flowcell_id: Option<String>,

    /// Indicates the lane
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lane_id: Option<String>,

    /// The read length; in the case of long-read sequencing it is the rounded average read
    /// length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_length: Option<i64>,

    /// Indicates the read order for paired-end reads.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_order: Option<ReadOrder>,
}

/// Type of checksum algorithm used
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumType {
    Sha256,
}

/// Type of the file; if BED file is submitted, only 1 file is allowed.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    Bam,

    Bed,

    Fastq,

    Vcf,
}

/// Indicates the read order for paired-end reads.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ReadOrder {
    R1,

    R2,
}

/// Percentage of bases with a specified minimum quality threshold, according to
/// https://www.bfarm.de/SharedDocs/Downloads/DE/Forschung/modellvorhaben-genomsequenzierung/Qs-durch-GRZ.pdf?__blob=publicationFile
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct PercentBasesAboveQualityThreshold {
    /// The minimum quality score threshold
    pub minimum_quality: f64,

    /// Percentage of bases that meet or exceed the minimum quality score
    pub percent: f64,
}

/// Reference genome used according to the Genome Reference Consortium
/// (https://www.ncbi.nlm.nih.gov/grc)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum ReferenceGenome {
    #[serde(rename = "GRCh37")]
    GrCh37,

    #[serde(rename = "GRCh38")]
    GrCh38,
}

/// Subtype of sequence (germline, somatic, etc.)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum SequenceSubtype {
    Germline,

    Other,

    Somatic,

    Unknown,
}

/// Type of sequence (DNA or RNA)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum SequenceType {
    Dna,

    Rna,
}

/// The sequencing layout, aka the end type of sequencing.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum SequencingLayout {
    Other,

    #[serde(rename = "paired-end")]
    PairedEnd,

    Reverse,

    #[serde(rename = "single-end")]
    SingleEnd,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TissueOntology {
    /// Name of the tissue ontology
    pub name: String,

    /// Version of the tissue ontology
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TumorCellCount {
    /// Tumor cell count in %
    pub count: f64,

    /// Method used to determine cell count.
    pub method: Method,
}

/// Method used to determine cell count.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Method {
    Bioinformatics,

    Other,

    Pathology,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct MvConsent {
    /// Date of delivery. Date (in ISO 8601 format YYYY-MM-DD) on which the Model Project
    /// Declaration of Participation was presented to the patient, unless identical to the date
    /// of signature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_date: Option<String>,

    /// Modules of the consent to MV: must have at least a permit of mvSequencing
    pub scope: Vec<Scope>,

    /// Version of the declaration of participation. Name and version of the declaration of
    /// participation in the MV GenomSeq, e.g.: 'Patient Info TE Consent MVGenomSeq vers01'
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Scope {
    /// Date of signature of the pilot projects consent; in ISO 8601 format YYYY-MM-DD.
    pub date: String,

    /// Scope of consent or revocation.
    pub domain: Domain,

    /// Consent or refusal to participate and consent, must be indicated for each option listed
    /// in the scope of consent.
    #[serde(rename = "type")]
    pub scope_type: Type,
}

/// Scope of consent or revocation.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum Domain {
    #[serde(rename = "caseIdentification")]
    CaseIdentification,

    #[serde(rename = "mvSequencing")]
    MvSequencing,

    #[serde(rename = "reIdentification")]
    ReIdentification,
}

/// Consent or refusal to participate and consent, must be indicated for each option listed
/// in the scope of consent.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Deny,

    Permit,
}

/// Relationship of the donor in respect to the index patient, e.g. 'index', 'brother',
/// 'mother', etc.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    Brother,

    Child,

    Father,

    Index,

    Mother,

    Other,

    Sister,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct ResearchConsent {
    /// Justification if no scope object is present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_scope_justification: Option<NoScopeJustification>,

    /// Date of the delivery of the research consent in ISO 8601 format (YYYY-MM-DD)
    pub presentation_date: String,

    /// Schema version of de.medizininformatikinitiative.kerndatensatz.consent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<SchemaVersion>,

    /// Scope of the research consent in JSON format following the MII IG Consent v2025 FHIR
    /// schema. See
    /// 'https://www.medizininformatik-initiative.de/Kerndatensatz/KDS_Consent_V2025/MII-IG-Modul-Consent.html'
    /// and
    /// 'https://packages2.fhir.org/packages/de.medizininformatikinitiative.kerndatensatz.consent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<HashMap<String, Option<serde_json::Value>>>,
}

/// Justification if no scope object is present.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum NoScopeJustification {
    #[serde(rename = "consent information cannot be submitted by LE due to technical reason")]
    TechnicalReason,

    #[serde(rename = "consent is not implemented at LE due to organizational issues")]
    OrganizationalIssues,

    #[serde(rename = "other patient-related reason")]
    OtherPatientRelatedReason,

    #[serde(rename = "patient did not return consent documents")]
    PatientDidNotReturnConsentDocuments,

    #[serde(rename = "patient refuses to sign consent")]
    PatientRefusesToSignConsent,

    #[serde(rename = "patient unable to consent")]
    PatientUnableToConsent,
}

/// Schema version of de.medizininformatikinitiative.kerndatensatz.consent
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum SchemaVersion {
    #[serde(rename = "2025.0.1")]
    Version202501,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub struct Submission {
    /// ID of the clinical data node in the format KDKXXXnnn.
    pub clinical_data_node_id: String,

    /// "GKV" gesetzliche Krankenversicherung, "PKV" private Krankenversicherung, "BG"
    /// Berufsgenossenschaft, "SEL" Selbstzahler, "SOZ" Sozialamt, "GPV" gesetzliche
    /// Pflegeversicherung, "PPV" private Pflegeversicherung, "BEI" Beihilfe, "SKT" Sonstige
    /// Kostenträger, "UNK" Unbekannt
    pub coverage_type: CoverageType,

    /// Type of the disease
    pub disease_type: DiseaseType,

    /// ID of the genomic data center in the format GRZXXXnnn.
    pub genomic_data_center_id: String,

    /// whether tumor and/or germ-line are tested
    pub genomic_study_subtype: GenomicStudySubtype,

    /// whether additional persons are tested as well
    pub genomic_study_type: GenomicStudyType,

    /// Name of the sequencing lab.
    pub lab_name: String,

    /// A local case identifier of the Leistungserbringer to be able to track multiple
    /// submissions referring to the same index patient
    pub local_case_id: String,

    /// Date of submission in ISO 8601 format YYYY-MM-DD
    pub submission_date: String,

    /// The options are: 'initial' for first submission, 'followup' is for followup submissions,
    /// 'addition' for additional submission, 'correction' for correction
    pub submission_type: SubmissionType,

    /// Institutional ID of the submitter according to §293 SGB V.
    pub submitter_id: String,

    /// The VNg of the genomic data of the index patient that will be reimbursed --> a unique
    /// 32-length byte code represented in a hex string of length 64.
    pub tan_g: String,
}

/// "GKV" gesetzliche Krankenversicherung, "PKV" private Krankenversicherung, "BG"
/// Berufsgenossenschaft, "SEL" Selbstzahler, "SOZ" Sozialamt, "GPV" gesetzliche
/// Pflegeversicherung, "PPV" private Pflegeversicherung, "BEI" Beihilfe, "SKT" Sonstige
/// Kostenträger, "UNK" Unbekannt
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub enum CoverageType {
    #[serde(rename = "BEI")]
    Bei,

    #[serde(rename = "BG")]
    Bg,

    #[serde(rename = "GKV")]
    Gkv,

    #[serde(rename = "GPV")]
    Gpv,

    #[serde(rename = "PKV")]
    Pkv,

    #[serde(rename = "PPV")]
    Ppv,

    #[serde(rename = "SEL")]
    Sel,

    #[serde(rename = "SKT")]
    Skt,

    #[serde(rename = "SOZ")]
    Soz,

    #[serde(rename = "UNK")]
    Unk,
}

/// Type of the disease
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum DiseaseType {
    Hereditary,

    Oncological,

    Rare,
}

/// whether tumor and/or germ-line are tested
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub enum GenomicStudySubtype {
    #[serde(rename = "germline-only")]
    GermlineOnly,

    #[serde(rename = "tumor+germline")]
    TumorGermline,

    #[serde(rename = "tumor-only")]
    TumorOnly,
}

/// whether additional persons are tested as well
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum GenomicStudyType {
    Duo,

    Single,

    Trio,
}

/// The options are: 'initial' for first submission, 'followup' is for followup submissions,
/// 'addition' for additional submission, 'correction' for correction
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "snake_case")]
pub enum SubmissionType {
    Addition,

    Correction,

    Followup,

    Initial,

    Test,
}
