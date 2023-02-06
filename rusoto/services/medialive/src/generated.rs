// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

use async_trait::async_trait;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Aac Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AacSettings {
    /// <p>Average bitrate in bits/second. Valid values depend on rate control mode and profile.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    /// <p>Mono, Stereo, or 5.1 channel layout. Valid values depend on rate control mode and profile. The adReceiverMix setting receives a stereo description plus control track and emits a mono AAC encode of the description track, with control data emitted in the PES header as per ETSI TS 101 154 Annex E.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Set to &quot;broadcasterMixedAd&quot; when input contains pre-mixed main audio + AD (narration) as a stereo pair.  The Audio Type field (audioType) will be set to 3, which signals to downstream systems that this stream contains &quot;broadcaster mixed AD&quot;. Note that the input received by the encoder must contain pre-mixed audio; the encoder does not perform the mixing. The values in audioTypeControl and audioType (in AudioDescription) are ignored when set to broadcasterMixedAd.</p>
    ///
    /// <p>Leave set to &quot;normal&quot; when input does not contain pre-mixed audio + AD.</p>
    #[serde(rename = "inputType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    /// <p>AAC Profile.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Rate Control Mode.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Sets LATM / LOAS AAC output for raw containers.</p>
    #[serde(rename = "rawFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<String>,
    /// <p>Sample rate in Hz. Valid values depend on rate control mode and profile.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
    /// <p>Use MPEG-2 AAC audio instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.</p>
    #[serde(rename = "spec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spec: Option<String>,
    /// <p>VBR Quality Level - Only used if rateControlMode is VBR.</p>
    #[serde(rename = "vbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<String>,
}

/// <p>Ac3 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ac3Settings {
    /// <p>Average bitrate in bits/second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    /// <p>Specifies the bitstream mode (bsmod) for the emitted AC-3 stream. See ATSC A/52-2012 for background on these values.</p>
    #[serde(rename = "bitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital coding mode. Determines number of channels.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sets the dialnorm for the output. If excluded and input audio is Dolby Digital, dialnorm will be passed through.</p>
    #[serde(rename = "dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>If set to filmStandard, adds dynamic range compression signaling to the output bitstream as defined in the Dolby Digital specification.</p>
    #[serde(rename = "drcProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_profile: Option<String>,
    /// <p>When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid in codingMode32Lfe mode.</p>
    #[serde(rename = "lfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>When set to &quot;followInput&quot;, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "metadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
}

/// <p>Placeholder documentation for AcceptInputDeviceTransferRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AcceptInputDeviceTransferRequest {
    /// <p>The unique ID of the input device to accept. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
}

/// <p>Placeholder documentation for AcceptInputDeviceTransferResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AcceptInputDeviceTransferResponse {}

/// <p>Ancillary Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AncillarySourceSettings {
    /// <p>Specifies the number (1 to 4) of the captions channel you want to extract from the ancillary captions. If you plan to convert the ancillary captions to another format, complete this field. If you plan to choose Embedded as the captions destination in the output (to pass through all the channels in the ancillary captions), leave this field blank because MediaLive ignores the field.</p>
    #[serde(rename = "sourceAncillaryChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i64>,
}

/// <p>Archive Cdn Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchiveCdnSettings {
    #[serde(rename = "archiveS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_s3_settings: Option<ArchiveS3Settings>,
}

/// <p>Archive Container Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchiveContainerSettings {
    #[serde(rename = "m2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_2ts_settings: Option<M2tsSettings>,
    #[serde(rename = "rawSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_settings: Option<RawSettings>,
}

/// <p>Archive Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchiveGroupSettings {
    /// <p>Parameters that control interactions with the CDN.</p>
    #[serde(rename = "archiveCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_cdn_settings: Option<ArchiveCdnSettings>,
    /// <p>A directory and base filename where archive files should be written.</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>Number of seconds to write to archive file before closing and starting a new one.</p>
    #[serde(rename = "rolloverInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollover_interval: Option<i64>,
}

/// <p>Archive Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchiveOutputSettings {
    /// <p>Settings specific to the container type of the file.</p>
    #[serde(rename = "containerSettings")]
    pub container_settings: ArchiveContainerSettings,
    /// <p>Output file extension. If excluded, this will be auto-selected from the container type.</p>
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// <p>String concatenated to the end of the destination filename.  Required for multiple outputs of the same type.</p>
    #[serde(rename = "nameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

/// <p>Archive S3 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ArchiveS3Settings {
    /// <p>Specify the canned ACL to apply to each S3 request. Defaults to none.</p>
    #[serde(rename = "cannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

/// <p>Arib Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AribDestinationSettings {}

/// <p>Arib Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AribSourceSettings {}

/// <p>Audio Channel Mapping</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioChannelMapping {
    /// <p>Indices and gain values for each input channel that should be remixed into this output channel.</p>
    #[serde(rename = "inputChannelLevels")]
    pub input_channel_levels: Vec<InputChannelLevel>,
    /// <p>The index of the output channel being produced.</p>
    #[serde(rename = "outputChannel")]
    pub output_channel: i64,
}

/// <p>Audio Codec Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioCodecSettings {
    #[serde(rename = "aacSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,
    #[serde(rename = "ac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_3_settings: Option<Ac3Settings>,
    #[serde(rename = "eac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_settings: Option<Eac3Settings>,
    #[serde(rename = "mp2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_2_settings: Option<Mp2Settings>,
    #[serde(rename = "passThroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_through_settings: Option<PassThroughSettings>,
    #[serde(rename = "wavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

/// <p>Audio Description</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioDescription {
    /// <p>Advanced audio normalization settings.</p>
    #[serde(rename = "audioNormalizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,
    /// <p>The name of the AudioSelector used as the source for this AudioDescription.</p>
    #[serde(rename = "audioSelectorName")]
    pub audio_selector_name: String,
    /// <p>Applies only if audioTypeControl is useConfigured. The values for audioType are defined in ISO-IEC 13818-1.</p>
    #[serde(rename = "audioType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<String>,
    /// <p>Determines how audio type is determined.
    /// followInput: If the input contains an ISO 639 audioType, then that value is passed through to the output. If the input contains no ISO 639 audioType, the value in Audio Type is included in the output.
    /// useConfigured: The value in Audio Type is included in the output.
    /// Note that this field and audioType are both ignored if inputType is broadcasterMixedAd.</p>
    #[serde(rename = "audioTypeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<String>,
    /// <p>Audio codec settings.</p>
    #[serde(rename = "codecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<AudioCodecSettings>,
    /// <p>RFC 5646 language code representing the language of the audio output track. Only used if languageControlMode is useConfigured, or there is no ISO 639 language code specified in the input.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Choosing followInput will cause the ISO 639 language code of the output to follow the ISO 639 language code of the input. The languageCode will be used when useConfigured is set, or when followInput is selected but there is no ISO 639 language code specified by the input.</p>
    #[serde(rename = "languageCodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<String>,
    /// <p>The name of this AudioDescription. Outputs will use this name to uniquely identify this AudioDescription.  Description names should be unique within this Live Event.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Settings that control how input audio channels are remixed into the output audio channels.</p>
    #[serde(rename = "remixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    /// <p>Used for MS Smooth and Apple HLS outputs. Indicates the name displayed by the player (eg. English, or Director Commentary).</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Audio Language Selection</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioLanguageSelection {
    /// <p>Selects a specific three-letter language code from within an audio source.</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>When set to &quot;strict&quot;, the transport stream demux strictly identifies audio streams by their language descriptor. If a PMT update occurs such that an audio stream matching the initially selected language is no longer present then mute will be encoded until the language returns. If &quot;loose&quot;, then on a PMT update the demux will choose another audio stream in the program with the same stream type if it can&#39;t find one with the same language.</p>
    #[serde(rename = "languageSelectionPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_selection_policy: Option<String>,
}

/// <p>Audio Normalization Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioNormalizationSettings {
    /// <p>Audio normalization algorithm to use. itu17701 conforms to the CALM Act specification, itu17702 conforms to the EBU R-128 specification.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>When set to correctAudio the output audio is corrected using the chosen algorithm. If set to measureOnly, the audio will be measured but not adjusted.</p>
    #[serde(rename = "algorithmControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<String>,
    /// <p>Target LKFS(loudness) to adjust volume to. If no value is entered, a default value will be used according to the chosen algorithm.  The CALM Act (1770-1) recommends a target of -24 LKFS. The EBU R-128 specification (1770-2) recommends a target of -23 LKFS.</p>
    #[serde(rename = "targetLkfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lkfs: Option<f64>,
}

/// <p>Audio Only Hls Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioOnlyHlsSettings {
    /// <p>Specifies the group to which the audio Rendition belongs.</p>
    #[serde(rename = "audioGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    /// <p>Optional. Specifies the .jpg or .png image to use as the cover art for an audio-only output. We recommend a low bit-size file because the image increases the output audio bandwidth.</p>
    ///
    /// <p>The image is attached to the audio as an ID3 tag, frame type APIC, picture type 0x10, as per the &quot;ID3 tag version 2.4.0 - Native Frames&quot; standard.</p>
    #[serde(rename = "audioOnlyImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_image: Option<InputLocation>,
    /// <p>Four types of audio-only tracks are supported:</p>
    ///
    /// <p>Audio-Only Variant Stream
    /// The client can play back this audio-only stream instead of video in low-bandwidth scenarios. Represented as an EXT-X-STREAM-INF in the HLS manifest.</p>
    ///
    /// <p>Alternate Audio, Auto Select, Default
    /// Alternate rendition that the client should try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=YES, AUTOSELECT=YES</p>
    ///
    /// <p>Alternate Audio, Auto Select, Not Default
    /// Alternate rendition that the client may try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=YES</p>
    ///
    /// <p>Alternate Audio, not Auto Select
    /// Alternate rendition that the client will not try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=NO</p>
    #[serde(rename = "audioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    /// <p>Specifies the segment type.</p>
    #[serde(rename = "segmentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_type: Option<String>,
}

/// <p>Audio Pid Selection</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioPidSelection {
    /// <p>Selects a specific PID from within a source.</p>
    #[serde(rename = "pid")]
    pub pid: i64,
}

/// <p>Audio Selector</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioSelector {
    /// <p>The name of this AudioSelector. AudioDescriptions will use this name to uniquely identify this Selector.  Selector names should be unique per input.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The audio selector settings.</p>
    #[serde(rename = "selectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<AudioSelectorSettings>,
}

/// <p>Audio Selector Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioSelectorSettings {
    #[serde(rename = "audioLanguageSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_language_selection: Option<AudioLanguageSelection>,
    #[serde(rename = "audioPidSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pid_selection: Option<AudioPidSelection>,
    #[serde(rename = "audioTrackSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_selection: Option<AudioTrackSelection>,
}

/// <p>Placeholder documentation for AudioSilenceFailoverSettings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioSilenceFailoverSettings {
    /// <p>The name of the audio selector in the input that MediaLive should monitor to detect silence. Select your most important rendition. If you didn&#39;t create an audio selector in this input, leave blank.</p>
    #[serde(rename = "audioSelectorName")]
    pub audio_selector_name: String,
    /// <p>The amount of time (in milliseconds) that the active input must be silent before automatic input failover occurs. Silence is defined as audio loss or audio quieter than -50 dBFS.</p>
    #[serde(rename = "audioSilenceThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_threshold_msec: Option<i64>,
}

/// <p>Audio Track</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioTrack {
    /// <p>1-based integer value that maps to a specific audio track</p>
    #[serde(rename = "track")]
    pub track: i64,
}

/// <p>Audio Track Selection</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioTrackSelection {
    /// <p>Selects one or more unique audio tracks from within a source.</p>
    #[serde(rename = "tracks")]
    pub tracks: Vec<AudioTrack>,
}

/// <p>The settings for Automatic Input Failover.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutomaticInputFailoverSettings {
    /// <p>This clear time defines the requirement a recovered input must meet to be considered healthy. The input must have no failover conditions for this length of time. Enter a time in milliseconds. This value is particularly important if the input<em>preference for the failover pair is set to PRIMARY</em>INPUT_PREFERRED, because after this time, MediaLive will switch back to the primary input.</p>
    #[serde(rename = "errorClearTimeMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_clear_time_msec: Option<i64>,
    /// <p>A list of failover conditions. If any of these conditions occur, MediaLive will perform a failover to the other input.</p>
    #[serde(rename = "failoverConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_conditions: Option<Vec<FailoverCondition>>,
    /// <p>Input preference when deciding which input to make active when a previously failed input has recovered.</p>
    #[serde(rename = "inputPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_preference: Option<String>,
    /// <p>The input ID of the secondary input in the automatic input failover pair.</p>
    #[serde(rename = "secondaryInputId")]
    pub secondary_input_id: String,
}

/// <p>Avail Blanking</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailBlanking {
    /// <p>Blanking image to be used. Leave empty for solid black. Only bmp and png images are supported.</p>
    #[serde(rename = "availBlankingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking_image: Option<InputLocation>,
    /// <p>When set to enabled, causes video, audio and captions to be blanked when insertion metadata is added.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Avail Configuration</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailConfiguration {
    /// <p>Ad avail settings.</p>
    #[serde(rename = "availSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_settings: Option<AvailSettings>,
}

/// <p>Avail Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailSettings {
    #[serde(rename = "scte35SpliceInsert")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_splice_insert: Option<Scte35SpliceInsert>,
    #[serde(rename = "scte35TimeSignalApos")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_time_signal_apos: Option<Scte35TimeSignalApos>,
}

/// <p>A request to delete resources</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteRequest {
    /// <p>List of channel IDs</p>
    #[serde(rename = "channelIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// <p>List of input IDs</p>
    #[serde(rename = "inputIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_ids: Option<Vec<String>>,
    /// <p>List of input security group IDs</p>
    #[serde(rename = "inputSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_group_ids: Option<Vec<String>>,
    /// <p>List of multiplex IDs</p>
    #[serde(rename = "multiplexIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

/// <p>Placeholder documentation for BatchDeleteResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteResponse {
    /// <p>List of failed operations</p>
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    /// <p>List of successful operations</p>
    #[serde(rename = "successful")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

/// <p>Details from a failed operation</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchFailedResultModel {
    /// <p>ARN of the resource</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Error code for the failed operation</p>
    #[serde(rename = "code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// <p>ID of the resource</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Error message for the failed operation</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>A list of schedule actions to create (in a request) or that have been created (in a response).</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchScheduleActionCreateRequest {
    /// <p>A list of schedule actions to create.</p>
    #[serde(rename = "scheduleActions")]
    pub schedule_actions: Vec<ScheduleAction>,
}

/// <p>List of actions that have been created in the schedule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchScheduleActionCreateResult {
    /// <p>List of actions that have been created in the schedule.</p>
    #[serde(rename = "scheduleActions")]
    pub schedule_actions: Vec<ScheduleAction>,
}

/// <p>A list of schedule actions to delete.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchScheduleActionDeleteRequest {
    /// <p>A list of schedule actions to delete.</p>
    #[serde(rename = "actionNames")]
    pub action_names: Vec<String>,
}

/// <p>List of actions that have been deleted from the schedule.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchScheduleActionDeleteResult {
    /// <p>List of actions that have been deleted from the schedule.</p>
    #[serde(rename = "scheduleActions")]
    pub schedule_actions: Vec<ScheduleAction>,
}

/// <p>A request to start resources</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchStartRequest {
    /// <p>List of channel IDs</p>
    #[serde(rename = "channelIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// <p>List of multiplex IDs</p>
    #[serde(rename = "multiplexIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

/// <p>Placeholder documentation for BatchStartResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchStartResponse {
    /// <p>List of failed operations</p>
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    /// <p>List of successful operations</p>
    #[serde(rename = "successful")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

/// <p>A request to stop resources</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchStopRequest {
    /// <p>List of channel IDs</p>
    #[serde(rename = "channelIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<String>>,
    /// <p>List of multiplex IDs</p>
    #[serde(rename = "multiplexIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_ids: Option<Vec<String>>,
}

/// <p>Placeholder documentation for BatchStopResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchStopResponse {
    /// <p>List of failed operations</p>
    #[serde(rename = "failed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<Vec<BatchFailedResultModel>>,
    /// <p>List of successful operations</p>
    #[serde(rename = "successful")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<Vec<BatchSuccessfulResultModel>>,
}

/// <p>Details from a successful operation</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchSuccessfulResultModel {
    /// <p>ARN of the resource</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>ID of the resource</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Current state of the resource</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>List of actions to create and list of actions to delete.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchUpdateScheduleRequest {
    /// <p>Id of the channel whose schedule is being updated.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// <p>Schedule actions to create in the schedule.</p>
    #[serde(rename = "creates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates: Option<BatchScheduleActionCreateRequest>,
    /// <p>Schedule actions to delete from the schedule.</p>
    #[serde(rename = "deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<BatchScheduleActionDeleteRequest>,
}

/// <p>Placeholder documentation for BatchUpdateScheduleResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchUpdateScheduleResponse {
    /// <p>Schedule actions created in the schedule.</p>
    #[serde(rename = "creates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates: Option<BatchScheduleActionCreateResult>,
    /// <p>Schedule actions deleted from the schedule.</p>
    #[serde(rename = "deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<BatchScheduleActionDeleteResult>,
}

/// <p>Blackout Slate</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BlackoutSlate {
    /// <p>Blackout slate image to be used. Leave empty for solid black. Only bmp and png images are supported.</p>
    #[serde(rename = "blackoutSlateImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate_image: Option<InputLocation>,
    /// <p>Setting to enabled causes the encoder to blackout the video, audio, and captions, and raise the &quot;Network Blackout Image&quot; slate when an SCTE104/35 Network End Segmentation Descriptor is encountered. The blackout will be lifted when the Network Start Segmentation Descriptor is encountered. The Network End and Network Start descriptors must contain a network ID that matches the value entered in &quot;Network ID&quot;.</p>
    #[serde(rename = "networkEndBlackout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout: Option<String>,
    /// <p>Path to local file to use as Network End Blackout image. Image will be scaled to fill the entire output raster.</p>
    #[serde(rename = "networkEndBlackoutImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_end_blackout_image: Option<InputLocation>,
    /// <p>Provides Network ID that matches EIDR ID format (e.g., &quot;10.XXXX/XXXX-XXXX-XXXX-XXXX-XXXX-C&quot;).</p>
    #[serde(rename = "networkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<String>,
    /// <p>When set to enabled, causes video, audio and captions to be blanked when indicated by program metadata.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Burn In Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BurnInDestinationSettings {
    /// <p>If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting &quot;smart&quot; justification will left-justify live subtitles and center-justify pre-recorded subtitles.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent).  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>External font file used for caption burn-in. File extension must be &#39;ttf&#39; or &#39;tte&#39;.  Although the user can select output fonts for many different types of input captions,  embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,
    /// <p>Specifies the color of the burned-in captions.  This option is not valid for source captions that are STL, 608/embedded or teletext.  These source settings are already pre-defined by the caption stream.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>When set to &#39;auto&#39; fontSize will scale depending on the size of the output.  Giving a positive integer will specify the exact font size in points.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter out is equivalent to setting it to 0 (transparent).  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    /// <p>Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.</p>
    #[serde(rename = "teletextGridControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<String>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "xPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "yPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

/// <p>Placeholder documentation for CancelInputDeviceTransferRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelInputDeviceTransferRequest {
    /// <p>The unique ID of the input device to cancel. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
}

/// <p>Placeholder documentation for CancelInputDeviceTransferResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelInputDeviceTransferResponse {}

/// <p>Caption Description</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionDescription {
    /// <p>Specifies which input caption selector to use as a caption source when generating output captions. This field should match a captionSelector name.</p>
    #[serde(rename = "captionSelectorName")]
    pub caption_selector_name: String,
    /// <p>Additional settings for captions destination that depend on the destination type.</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    /// <p>ISO 639-2 three-digit code: http://www.loc.gov/standards/iso639-2/</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Human readable information to indicate captions available for players (eg. English, or Spanish).</p>
    #[serde(rename = "languageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
    /// <p>Name of the caption description.  Used to associate a caption description with an output.  Names must be unique within an event.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Caption Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionDestinationSettings {
    #[serde(rename = "aribDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_destination_settings: Option<AribDestinationSettings>,
    #[serde(rename = "burnInDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burn_in_destination_settings: Option<BurnInDestinationSettings>,
    #[serde(rename = "dvbSubDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    #[serde(rename = "ebuTtDDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebu_tt_d_destination_settings: Option<EbuTtDDestinationSettings>,
    #[serde(rename = "embeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,
    #[serde(rename = "embeddedPlusScte20DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_plus_scte_20_destination_settings: Option<EmbeddedPlusScte20DestinationSettings>,
    #[serde(rename = "rtmpCaptionInfoDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_caption_info_destination_settings: Option<RtmpCaptionInfoDestinationSettings>,
    #[serde(rename = "scte20PlusEmbeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_20_plus_embedded_destination_settings: Option<Scte20PlusEmbeddedDestinationSettings>,
    #[serde(rename = "scte27DestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_27_destination_settings: Option<Scte27DestinationSettings>,
    #[serde(rename = "smpteTtDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte_tt_destination_settings: Option<SmpteTtDestinationSettings>,
    #[serde(rename = "teletextDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    #[serde(rename = "ttmlDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,
    #[serde(rename = "webvttDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt_destination_settings: Option<WebvttDestinationSettings>,
}

/// <p>Maps a caption channel to an ISO 693-2 language code (http://www.loc.gov/standards/iso639-2), with an optional description.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionLanguageMapping {
    /// <p>The closed caption channel being described by this CaptionLanguageMapping.  Each channel mapping must have a unique channel number (maximum of 4)</p>
    #[serde(rename = "captionChannel")]
    pub caption_channel: i64,
    /// <p>Three character ISO 639-2 language code (see http://www.loc.gov/standards/iso639-2)</p>
    #[serde(rename = "languageCode")]
    pub language_code: String,
    /// <p>Textual description of language</p>
    #[serde(rename = "languageDescription")]
    pub language_description: String,
}

/// <p>Caption Rectangle</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionRectangle {
    /// <p>See the description in leftOffset.
    /// For height, specify the entire height of the rectangle as a percentage of the underlying frame height. For example, &quot;80&quot; means the rectangle height is 80% of the underlying frame height. The topOffset and rectangleHeight must add up to 100% or less.
    /// This field corresponds to tts:extent - Y in the TTML standard.</p>
    #[serde(rename = "height")]
    pub height: f64,
    /// <p>Applies only if you plan to convert these source captions to EBU-TT-D or TTML in an output. (Make sure to leave the default if you don&#39;t have either of these formats in the output.) You can define a display rectangle for the captions that is smaller than the underlying video frame. You define the rectangle by specifying the position of the left edge, top edge, bottom edge, and right edge of the rectangle, all within the underlying video frame. The units for the measurements are percentages.
    /// If you specify a value for one of these fields, you must specify a value for all of them.
    /// For leftOffset, specify the position of the left edge of the rectangle, as a percentage of the underlying frame width, and relative to the left edge of the frame. For example, &quot;10&quot; means the measurement is 10% of the underlying frame width. The rectangle left edge starts at that position from the left edge of the frame.
    /// This field corresponds to tts:origin - X in the TTML standard.</p>
    #[serde(rename = "leftOffset")]
    pub left_offset: f64,
    /// <p>See the description in leftOffset.
    /// For topOffset, specify the position of the top edge of the rectangle, as a percentage of the underlying frame height, and relative to the top edge of the frame. For example, &quot;10&quot; means the measurement is 10% of the underlying frame height. The rectangle top edge starts at that position from the top edge of the frame.
    /// This field corresponds to tts:origin - Y in the TTML standard.</p>
    #[serde(rename = "topOffset")]
    pub top_offset: f64,
    /// <p>See the description in leftOffset.
    /// For width, specify the entire width of the rectangle as a percentage of the underlying frame width. For example, &quot;80&quot; means the rectangle width is 80% of the underlying frame width. The leftOffset and rectangleWidth must add up to 100% or less.
    /// This field corresponds to tts:extent - X in the TTML standard.</p>
    #[serde(rename = "width")]
    pub width: f64,
}

/// <p>Output groups for this Live Event. Output groups contain information about where streams should be distributed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionSelector {
    /// <p>When specified this field indicates the three letter language code of the caption track to extract from the source.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Name identifier for a caption selector.  This name is used to associate this caption selector with one or more caption descriptions.  Names must be unique within an event.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Caption selector settings.</p>
    #[serde(rename = "selectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<CaptionSelectorSettings>,
}

/// <p>Caption Selector Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionSelectorSettings {
    #[serde(rename = "ancillarySourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,
    #[serde(rename = "aribSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_source_settings: Option<AribSourceSettings>,
    #[serde(rename = "dvbSubSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    #[serde(rename = "embeddedSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    #[serde(rename = "scte20SourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_20_source_settings: Option<Scte20SourceSettings>,
    #[serde(rename = "scte27SourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_27_source_settings: Option<Scte27SourceSettings>,
    #[serde(rename = "teletextSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
}

/// <p>Placeholder documentation for CdiInputSpecification</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CdiInputSpecification {
    /// <p>Maximum CDI input resolution</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// <p>Placeholder documentation for Channel</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Channel {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Runtime details for the pipelines of a running channel.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Placeholder documentation for ChannelEgressEndpoint</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChannelEgressEndpoint {
    /// <p>Public IP of where a channel&#39;s output comes from</p>
    #[serde(rename = "sourceIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ip: Option<String>,
}

/// <p>Placeholder documentation for ChannelSummary</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChannelSummary {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Passthrough applies no color space conversion to the output</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColorSpacePassthroughSettings {}

/// <p>A request to create a channel</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateChannelRequest {
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level to write to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>Name of channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Unique request ID to be specified. This is needed to prevent retries from
    /// creating multiple resources.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>An optional Amazon Resource Name (ARN) of the role to assume when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettings>,
}

/// <p>Placeholder documentation for CreateChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateChannelResponse {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

/// <p>The name of the input</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInputRequest {
    /// <p>Destination settings for PUSH type inputs.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestinationRequest>>,
    /// <p>Settings for the devices.</p>
    #[serde(rename = "inputDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    /// <p>A list of security groups referenced by IDs to attach to the input.</p>
    #[serde(rename = "inputSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<String>>,
    /// <p>A list of the MediaConnect Flows that you want to use in this input. You can specify as few as one
    /// Flow and presently, as many as two. The only requirement is when you have more than one is that each Flow is in a
    /// separate Availability Zone as this ensures your EML input is redundant to AZ issues.</p>
    #[serde(rename = "mediaConnectFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlowRequest>>,
    /// <p>Name of the input.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Unique identifier of the request to ensure the request is handled
    /// exactly once in case of retries.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role this input assumes during and after creation.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The source URLs for a PULL-type input. Every PULL type input needs
    /// exactly two source URLs for redundancy.
    /// Only specify sources for PULL type Inputs. Leave Destinations empty.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSourceRequest>>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<InputVpcRequest>,
}

/// <p>Placeholder documentation for CreateInputResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInputResponse {
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

/// <p>The IPv4 CIDRs to whitelist for this Input Security Group</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateInputSecurityGroupRequest {
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>List of IPv4 CIDR addresses to whitelist</p>
    #[serde(rename = "whitelistRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
}

/// <p>Placeholder documentation for CreateInputSecurityGroupResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateInputSecurityGroupResponse {
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<InputSecurityGroup>,
}

/// <p>A request to create a program in a multiplex.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMultiplexProgramRequest {
    /// <p>ID of the multiplex where the program is to be created.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The settings for this multiplex program.</p>
    #[serde(rename = "multiplexProgramSettings")]
    pub multiplex_program_settings: MultiplexProgramSettings,
    /// <p>Name of multiplex program.</p>
    #[serde(rename = "programName")]
    pub program_name: String,
    /// <p>Unique request ID. This prevents retries from creating multiple
    /// resources.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
}

/// <p>Placeholder documentation for CreateMultiplexProgramResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMultiplexProgramResponse {
    /// <p>The newly created multiplex program.</p>
    #[serde(rename = "multiplexProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program: Option<MultiplexProgram>,
}

/// <p>A request to create a multiplex.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateMultiplexRequest {
    /// <p>A list of availability zones for the multiplex. You must specify exactly two.</p>
    #[serde(rename = "availabilityZones")]
    pub availability_zones: Vec<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    pub multiplex_settings: MultiplexSettings,
    /// <p>Name of multiplex.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Unique request ID. This prevents retries from creating multiple
    /// resources.</p>
    #[serde(rename = "requestId")]
    pub request_id: String,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for CreateMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateMultiplexResponse {
    /// <p>The newly created multiplex.</p>
    #[serde(rename = "multiplex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<Multiplex>,
}

/// <p>A request to create a partner input</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePartnerInputRequest {
    /// <p>Unique ID of the input.</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
    /// <p>Unique identifier of the request to ensure the request is handled
    /// exactly once in case of retries.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for CreatePartnerInputResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePartnerInputResponse {
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

/// <p>Placeholder documentation for CreateTagsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTagsRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for DeleteChannelRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteChannelRequest {
    /// <p>Unique ID of the channel.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// <p>Placeholder documentation for DeleteChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteChannelResponse {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Runtime details for the pipelines of a running channel.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Placeholder documentation for DeleteInputRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInputRequest {
    /// <p>Unique ID of the input</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
}

/// <p>Placeholder documentation for DeleteInputResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInputResponse {}

/// <p>Placeholder documentation for DeleteInputSecurityGroupRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteInputSecurityGroupRequest {
    /// <p>The Input Security Group to delete</p>
    #[serde(rename = "inputSecurityGroupId")]
    pub input_security_group_id: String,
}

/// <p>Placeholder documentation for DeleteInputSecurityGroupResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteInputSecurityGroupResponse {}

/// <p>Placeholder documentation for DeleteMultiplexProgramRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMultiplexProgramRequest {
    /// <p>The ID of the multiplex that the program belongs to.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The multiplex program name.</p>
    #[serde(rename = "programName")]
    pub program_name: String,
}

/// <p>Placeholder documentation for DeleteMultiplexProgramResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMultiplexProgramResponse {
    /// <p>The MediaLive channel associated with the program.</p>
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The settings for this multiplex program.</p>
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    /// <p>The packet identifier map for this multiplex program.</p>
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    /// <p>Contains information about the current sources for the specified program in the specified multiplex. Keep in mind that each multiplex pipeline connects to both pipelines in a given source channel (the channel identified by the program). But only one of those channel pipelines is ever active at one time.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    /// <p>The name of the multiplex program.</p>
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

/// <p>Placeholder documentation for DeleteMultiplexRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteMultiplexRequest {
    /// <p>The ID of the multiplex.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
}

/// <p>Placeholder documentation for DeleteMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteMultiplexResponse {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of the multiplex output destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for DeleteReservationRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReservationRequest {
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    pub reservation_id: String,
}

/// <p>Placeholder documentation for DeleteReservationResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReservationResponse {
    /// <p>Unique reservation ARN, e.g. &#39;arn:aws:medialive:us-west-2:123456789012:reservation:1234567&#39;</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Number of reserved resources</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. &#39;USD&#39;</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>Lease duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Units for duration, e.g. &#39;MONTHS&#39;</p>
    #[serde(rename = "durationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    /// <p>Reservation UTC end date and time in ISO-8601 format, e.g. &#39;2019-03-01T00:00:00&#39;</p>
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>One-time charge for each reserved resource, e.g. &#39;0.0&#39; for a NO_UPFRONT offering</p>
    #[serde(rename = "fixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>User specified reservation name</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Offering description, e.g. &#39;HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)&#39;</p>
    #[serde(rename = "offeringDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>Offering type, e.g. &#39;NO_UPFRONT&#39;</p>
    #[serde(rename = "offeringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    /// <p>AWS region, e.g. &#39;us-west-2&#39;</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    /// <p>Resource configuration details</p>
    #[serde(rename = "resourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    /// <p>Reservation UTC start date and time in ISO-8601 format, e.g. &#39;2018-03-01T00:00:00&#39;</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>Current state of reservation, e.g. &#39;ACTIVE&#39;</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Recurring usage charge for each reserved resource, e.g. &#39;157.0&#39;</p>
    #[serde(rename = "usagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Placeholder documentation for DeleteScheduleRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteScheduleRequest {
    /// <p>Id of the channel whose schedule is being deleted.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// <p>Placeholder documentation for DeleteScheduleResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteScheduleResponse {}

/// <p>Placeholder documentation for DeleteTagsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>An array of tag keys to delete</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p>Placeholder documentation for DescribeChannelRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeChannelRequest {
    /// <p>channel ID</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// <p>Placeholder documentation for DescribeChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Runtime details for the pipelines of a running channel.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Placeholder documentation for DescribeInputDeviceRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputDeviceRequest {
    /// <p>The unique ID of this input device. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
}

/// <p>Placeholder documentation for DescribeInputDeviceResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInputDeviceResponse {
    /// <p>The unique ARN of the input device.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The state of the connection between the input device and AWS.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The status of the action to synchronize the device configuration. If you change the configuration of the input device (for example, the maximum bitrate), MediaLive sends the new data to the device. The device might not update itself immediately. SYNCED means the device has updated its configuration. SYNCING means that it has not updated its configuration.</p>
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    /// <p>The status of software on the input device.</p>
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    /// <p>Settings that describe an input device that is type HD.</p>
    #[serde(rename = "hdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    /// <p>The unique ID of the input device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The network MAC address of the input device.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>A name that you specify for the input device.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network settings for the input device.</p>
    #[serde(rename = "networkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    /// <p>The unique serial number of the input device.</p>
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The type of the input device.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Settings that describe an input device that is type UHD.</p>
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

/// <p>Placeholder documentation for DescribeInputDeviceThumbnailRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputDeviceThumbnailRequest {
    /// <p>The HTTP Accept header. Indicates the requested type for the thumbnail.</p>
    #[serde(rename = "accept")]
    pub accept: String,
    /// <p>The unique ID of this input device. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
}

/// <p>Placeholder documentation for DescribeInputDeviceThumbnailResponse</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DescribeInputDeviceThumbnailResponse {
    /// <p>The binary data for the thumbnail that the Link device has most recently sent to MediaLive.</p>
    pub body: Option<bytes::Bytes>,
    /// <p>The length of the content.</p>
    pub content_length: Option<i64>,
    /// <p>Specifies the media type of the thumbnail.</p>
    pub content_type: Option<String>,
    /// <p>The unique, cacheable version of this thumbnail.</p>
    pub e_tag: Option<String>,
    /// <p>The date and time the thumbnail was last updated at the device.</p>
    pub last_modified: Option<f64>,
}

/// <p>Placeholder documentation for DescribeInputRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputRequest {
    /// <p>Unique ID of the input</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
}

/// <p>Placeholder documentation for DescribeInputResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInputResponse {
    /// <p>The Unique ARN of the input (generated, immutable).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of channel IDs that that input is attached to (currently an input can only be attached to one channel).</p>
    #[serde(rename = "attachedChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_channels: Option<Vec<String>>,
    /// <p>A list of the destinations of the input (PUSH-type).</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestination>>,
    /// <p>The generated ID of the input (unique for user account, immutable).</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>STANDARD - MediaLive expects two sources to be connected to this input. If the channel is also STANDARD, both sources will be ingested. If the channel is SINGLE<em>PIPELINE, only the first source will be ingested; the second source will always be ignored, even if the first source fails.
    /// SINGLE</em>PIPELINE - You can connect only one source to this input. If the ChannelClass is also  SINGLE_PIPELINE, this value is valid. If the ChannelClass is STANDARD, this value is not valid because the channel requires two sources in the input.</p>
    #[serde(rename = "inputClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_class: Option<String>,
    /// <p>Settings for the input devices.</p>
    #[serde(rename = "inputDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    /// <p>A list of IDs for all Inputs which are partners of this one.</p>
    #[serde(rename = "inputPartnerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_partner_ids: Option<Vec<String>>,
    /// <p>Certain pull input sources can be dynamic, meaning that they can have their URL&#39;s dynamically changes
    /// during input switch actions. Presently, this functionality only works with MP4_FILE inputs.</p>
    #[serde(rename = "inputSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_source_type: Option<String>,
    /// <p>A list of MediaConnect Flows for this input.</p>
    #[serde(rename = "mediaConnectFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlow>>,
    /// <p>The user-assigned name (This is a mutable value).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role this input assumes during and after creation.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A list of IDs for all the Input Security Groups attached to the input.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>A list of the sources of the input (PULL-type).</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSource>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Placeholder documentation for DescribeInputSecurityGroupRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeInputSecurityGroupRequest {
    /// <p>The id of the Input Security Group to describe</p>
    #[serde(rename = "inputSecurityGroupId")]
    pub input_security_group_id: String,
}

/// <p>Placeholder documentation for DescribeInputSecurityGroupResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeInputSecurityGroupResponse {
    /// <p>Unique ARN of Input Security Group</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Id of the Input Security Group</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The list of inputs currently using this Input Security Group.</p>
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// <p>The current state of the Input Security Group.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Whitelist rules and their sync status</p>
    #[serde(rename = "whitelistRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRule>>,
}

/// <p>Placeholder documentation for DescribeMultiplexProgramRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMultiplexProgramRequest {
    /// <p>The ID of the multiplex that the program belongs to.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The name of the program.</p>
    #[serde(rename = "programName")]
    pub program_name: String,
}

/// <p>Placeholder documentation for DescribeMultiplexProgramResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMultiplexProgramResponse {
    /// <p>The MediaLive channel associated with the program.</p>
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The settings for this multiplex program.</p>
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    /// <p>The packet identifier map for this multiplex program.</p>
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    /// <p>Contains information about the current sources for the specified program in the specified multiplex. Keep in mind that each multiplex pipeline connects to both pipelines in a given source channel (the channel identified by the program). But only one of those channel pipelines is ever active at one time.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    /// <p>The name of the multiplex program.</p>
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

/// <p>Placeholder documentation for DescribeMultiplexRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeMultiplexRequest {
    /// <p>The ID of the multiplex.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
}

/// <p>Placeholder documentation for DescribeMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeMultiplexResponse {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of the multiplex output destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for DescribeOfferingRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOfferingRequest {
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    pub offering_id: String,
}

/// <p>Placeholder documentation for DescribeOfferingResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOfferingResponse {
    /// <p>Unique offering ARN, e.g. &#39;arn:aws:medialive:us-west-2:123456789012:offering:87654321&#39;</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. &#39;USD&#39;</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>Lease duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Units for duration, e.g. &#39;MONTHS&#39;</p>
    #[serde(rename = "durationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    /// <p>One-time charge for each reserved resource, e.g. &#39;0.0&#39; for a NO_UPFRONT offering</p>
    #[serde(rename = "fixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>Offering description, e.g. &#39;HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)&#39;</p>
    #[serde(rename = "offeringDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>Offering type, e.g. &#39;NO_UPFRONT&#39;</p>
    #[serde(rename = "offeringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    /// <p>AWS region, e.g. &#39;us-west-2&#39;</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Resource configuration details</p>
    #[serde(rename = "resourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    /// <p>Recurring usage charge for each reserved resource, e.g. &#39;157.0&#39;</p>
    #[serde(rename = "usagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Placeholder documentation for DescribeReservationRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservationRequest {
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    pub reservation_id: String,
}

/// <p>Placeholder documentation for DescribeReservationResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReservationResponse {
    /// <p>Unique reservation ARN, e.g. &#39;arn:aws:medialive:us-west-2:123456789012:reservation:1234567&#39;</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Number of reserved resources</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. &#39;USD&#39;</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>Lease duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Units for duration, e.g. &#39;MONTHS&#39;</p>
    #[serde(rename = "durationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    /// <p>Reservation UTC end date and time in ISO-8601 format, e.g. &#39;2019-03-01T00:00:00&#39;</p>
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>One-time charge for each reserved resource, e.g. &#39;0.0&#39; for a NO_UPFRONT offering</p>
    #[serde(rename = "fixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>User specified reservation name</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Offering description, e.g. &#39;HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)&#39;</p>
    #[serde(rename = "offeringDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>Offering type, e.g. &#39;NO_UPFRONT&#39;</p>
    #[serde(rename = "offeringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    /// <p>AWS region, e.g. &#39;us-west-2&#39;</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    /// <p>Resource configuration details</p>
    #[serde(rename = "resourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    /// <p>Reservation UTC start date and time in ISO-8601 format, e.g. &#39;2018-03-01T00:00:00&#39;</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>Current state of reservation, e.g. &#39;ACTIVE&#39;</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Recurring usage charge for each reserved resource, e.g. &#39;157.0&#39;</p>
    #[serde(rename = "usagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Placeholder documentation for DescribeScheduleRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeScheduleRequest {
    /// <p>Id of the channel whose schedule is being updated.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for DescribeScheduleResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeScheduleResponse {
    /// <p>The next token; for use in pagination.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of actions in the schedule.</p>
    #[serde(rename = "scheduleActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_actions: Option<Vec<ScheduleAction>>,
}

/// <p>DVB Network Information Table (NIT)</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbNitSettings {
    /// <p>The numeric value placed in the Network Information Table (NIT).</p>
    #[serde(rename = "networkId")]
    pub network_id: i64,
    /// <p>The network name text placed in the networkNameDescriptor inside the Network Information Table. Maximum length is 256 characters.</p>
    #[serde(rename = "networkName")]
    pub network_name: String,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "repInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,
}

/// <p>DVB Service Description Table (SDT)</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSdtSettings {
    /// <p>Selects method of inserting SDT information into output stream. The sdtFollow setting copies SDT information from input stream to output stream. The sdtFollowIfPresent setting copies SDT information from input stream to output stream if SDT information is present in the input, otherwise it will fall back on the user-defined values. The sdtManual setting means user will enter the SDT information. The sdtNone setting means output stream will not contain SDT information.</p>
    #[serde(rename = "outputSdt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sdt: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "repInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,
    /// <p>The service name placed in the serviceDescriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The service provider name placed in the serviceDescriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "serviceProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
}

/// <p>Dvb Sub Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSubDestinationSettings {
    /// <p>If no explicit xPosition or yPosition is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. Selecting &quot;smart&quot; justification will left-justify live subtitles and center-justify pre-recorded subtitles.  This option is not valid for source captions that are STL or 608/embedded.  These source settings are already pre-defined by the caption stream.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent).  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>External font file used for caption burn-in. File extension must be &#39;ttf&#39; or &#39;tte&#39;.  Although the user can select output fonts for many different types of input captions, embedded, STL and teletext sources use a strict grid system. Using external fonts with these caption sources could cause unexpected display of proportional fonts.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "font")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<InputLocation>,
    /// <p>Specifies the color of the burned-in captions.  This option is not valid for source captions that are STL, 608/embedded or teletext.  These source settings are already pre-defined by the caption stream.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>When set to auto fontSize will scale depending on the size of the output.  Giving a positive integer will specify the exact font size in points.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent).  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    /// <p>Controls whether a fixed grid size will be used to generate the output subtitles bitmap. Only applicable for Teletext inputs and DVB-Sub/Burn-in outputs.</p>
    #[serde(rename = "teletextGridControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_grid_control: Option<String>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit xPosition is provided, the horizontal caption position will be determined by the alignment parameter.  This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "xPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit yPosition is provided, the caption will be positioned towards the bottom of the output.  This option is not valid for source captions that are STL, 608/embedded or teletext.  These source settings are already pre-defined by the caption stream.  All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "yPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

/// <p>Dvb Sub Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSubSourceSettings {
    /// <p>If you will configure a WebVTT caption description that references this caption selector, use this field to
    /// provide the language to consider when translating the image-based source to text.</p>
    #[serde(rename = "ocrLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<String>,
    /// <p>When using DVB-Sub with Burn-In or SMPTE-TT, use this PID for the source content. Unused for DVB-Sub passthrough. All DVB-Sub content is passed through, regardless of selectors.</p>
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

/// <p>DVB Time and Date Table (SDT)</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbTdtSettings {
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "repInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_interval: Option<i64>,
}

/// <p>Eac3 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Eac3Settings {
    /// <p>When set to attenuate3Db, applies a 3 dB attenuation to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "attenuationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    /// <p>Average bitrate in bits/second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    /// <p>Specifies the bitstream mode (bsmod) for the emitted E-AC-3 stream. See ATSC A/52-2012 (Annex E) for background on these values.</p>
    #[serde(rename = "bitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital Plus coding mode. Determines number of channels.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>When set to enabled, activates a DC highpass filter for all input channels.</p>
    #[serde(rename = "dcFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital Plus, dialnorm will be passed through.</p>
    #[serde(rename = "dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>Sets the Dolby dynamic range compression profile.</p>
    #[serde(rename = "drcLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_line: Option<String>,
    /// <p>Sets the profile for heavy Dolby dynamic range compression, ensures that the instantaneous signal peaks do not exceed specified levels.</p>
    #[serde(rename = "drcRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drc_rf: Option<String>,
    /// <p>When encoding 3/2 audio, setting to lfe enables the LFE channel</p>
    #[serde(rename = "lfeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<String>,
    /// <p>When set to enabled, applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid with codingMode32 coding mode.</p>
    #[serde(rename = "lfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>Left only/Right only center mix level. Only used for 3/2 coding mode.</p>
    #[serde(rename = "loRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Left only/Right only surround mix level. Only used for 3/2 coding mode.</p>
    #[serde(rename = "loRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Left total/Right total center mix level. Only used for 3/2 coding mode.</p>
    #[serde(rename = "ltRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Left total/Right total surround mix level. Only used for 3/2 coding mode.</p>
    #[serde(rename = "ltRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    /// <p>When set to followInput, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "metadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>When set to whenPossible, input DD+ audio will be passed through if it is present on the input. This detection is dynamic over the life of the transcode. Inputs that alternate between DD+ and non-DD+ content will have a consistent DD+ output as the system alternates between passthrough and encoding.</p>
    #[serde(rename = "passthroughControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<String>,
    /// <p>When set to shift90Degrees, applies a 90-degree phase shift to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "phaseControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<String>,
    /// <p>Stereo downmix preference. Only used for 3/2 coding mode.</p>
    #[serde(rename = "stereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    /// <p>When encoding 3/2 audio, sets whether an extra center back surround channel is matrix encoded into the left and right surround channels.</p>
    #[serde(rename = "surroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
    /// <p>When encoding 2/0 audio, sets whether Dolby Surround is matrix encoded into the two channels.</p>
    #[serde(rename = "surroundMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_mode: Option<String>,
}

/// <p>Ebu Tt DDestination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EbuTtDDestinationSettings {
    /// <p>Applies only if you plan to convert these source captions to EBU-TT-D or TTML in an output. Complete this field if you want to include the name of the copyright holder in the copyright metadata tag in the TTML</p>
    #[serde(rename = "copyrightHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright_holder: Option<String>,
    /// <p>Specifies how to handle the gap between the lines (in multi-line captions).</p>
    ///
    /// <ul>
    /// <li>enabled: Fill with the captions background color (as specified in the input captions).</li>
    /// <li>disabled: Leave the gap unfilled.</li>
    /// </ul>
    #[serde(rename = "fillLineGap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_line_gap: Option<String>,
    /// <p>Specifies the font family to include in the font data attached to the EBU-TT captions. Valid only if styleControl is set to include. If you leave this field empty, the font family is set to &quot;monospaced&quot;. (If styleControl is set to exclude, the font family is always set to &quot;monospaced&quot;.)</p>
    ///
    /// <p>You specify only the font family. All other style information (color, bold, position and so on) is copied from the input captions. The size is always set to 100% to allow the downstream player to choose the size.</p>
    ///
    /// <ul>
    /// <li>Enter a list of font families, as a comma-separated list of font names, in order of preference. The name can be a font family (such as “Arial”), or a generic font family (such as “serif”), or “default” (to let the downstream player choose the font).</li>
    /// <li>Leave blank to set the family to “monospace”.</li>
    /// </ul>
    #[serde(rename = "fontFamily")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<String>,
    /// <p>Specifies the style information (font color, font position, and so on) to include in the font data that is attached to the EBU-TT captions.</p>
    ///
    /// <ul>
    /// <li>include: Take the style information (font color, font position, and so on) from the source captions and include that information in the font data attached to the EBU-TT captions. This option is valid only if the source captions are Embedded or Teletext.</li>
    /// <li>exclude: In the font data attached to the EBU-TT captions, set the font family to &quot;monospaced&quot;. Do not include any other style information.</li>
    /// </ul>
    #[serde(rename = "styleControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<String>,
}

/// <p>Embedded Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmbeddedDestinationSettings {}

/// <p>Embedded Plus Scte20 Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmbeddedPlusScte20DestinationSettings {}

/// <p>Embedded Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmbeddedSourceSettings {
    /// <p>If upconvert, 608 data is both passed through via the &quot;608 compatibility bytes&quot; fields of the 708 wrapper as well as translated into 708. 708 data present in the source content will be discarded.</p>
    #[serde(rename = "convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Set to &quot;auto&quot; to handle streams with intermittent and/or non-aligned SCTE-20 and Embedded captions.</p>
    #[serde(rename = "scte20Detection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_20_detection: Option<String>,
    /// <p>Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_channel_number: Option<i64>,
    /// <p>This field is unused and deprecated.</p>
    #[serde(rename = "source608TrackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_track_number: Option<i64>,
}

/// <p>Encoder Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncoderSettings {
    #[serde(rename = "audioDescriptions")]
    pub audio_descriptions: Vec<AudioDescription>,
    /// <p>Settings for ad avail blanking.</p>
    #[serde(rename = "availBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    /// <p>Event-wide configuration settings for ad avail insertion.</p>
    #[serde(rename = "availConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_configuration: Option<AvailConfiguration>,
    /// <p>Settings for blackout slate.</p>
    #[serde(rename = "blackoutSlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blackout_slate: Option<BlackoutSlate>,
    /// <p>Settings for caption decriptions</p>
    #[serde(rename = "captionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,
    /// <p>Feature Activations</p>
    #[serde(rename = "featureActivations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_activations: Option<FeatureActivations>,
    /// <p>Configuration settings that apply to the event as a whole.</p>
    #[serde(rename = "globalConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_configuration: Option<GlobalConfiguration>,
    /// <p>Settings for motion graphics.</p>
    #[serde(rename = "motionGraphicsConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_configuration: Option<MotionGraphicsConfiguration>,
    /// <p>Nielsen configuration settings.</p>
    #[serde(rename = "nielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    #[serde(rename = "outputGroups")]
    pub output_groups: Vec<OutputGroup>,
    /// <p>Contains settings used to acquire and adjust timecode information from inputs.</p>
    #[serde(rename = "timecodeConfig")]
    pub timecode_config: TimecodeConfig,
    #[serde(rename = "videoDescriptions")]
    pub video_descriptions: Vec<VideoDescription>,
}

/// <p>Failover Condition settings. There can be multiple failover conditions inside AutomaticInputFailoverSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FailoverCondition {
    /// <p>Failover condition type-specific settings.</p>
    #[serde(rename = "failoverConditionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_condition_settings: Option<FailoverConditionSettings>,
}

/// <p>Settings for one failover condition.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FailoverConditionSettings {
    /// <p>MediaLive will perform a failover if the specified audio selector is silent for the specified period.</p>
    #[serde(rename = "audioSilenceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_silence_settings: Option<AudioSilenceFailoverSettings>,
    /// <p>MediaLive will perform a failover if content is not detected in this input for the specified period.</p>
    #[serde(rename = "inputLossSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_settings: Option<InputLossFailoverSettings>,
    /// <p>MediaLive will perform a failover if content is considered black for the specified period.</p>
    #[serde(rename = "videoBlackSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_settings: Option<VideoBlackFailoverSettings>,
}

/// <p>Feature Activations</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FeatureActivations {
    /// <p>Enables the Input Prepare feature. You can create Input Prepare actions in the schedule only if this feature is enabled.
    /// If you disable the feature on an existing schedule, make sure that you first delete all input prepare actions from the schedule.</p>
    #[serde(rename = "inputPrepareScheduleActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_prepare_schedule_actions: Option<String>,
}

/// <p>Fec Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FecOutputSettings {
    /// <p>Parameter D from SMPTE 2022-1. The height of the FEC protection matrix.  The number of transport stream packets per column error correction packet. Must be between 4 and 20, inclusive.</p>
    #[serde(rename = "columnDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_depth: Option<i64>,
    /// <p>Enables column only or column and row based FEC</p>
    #[serde(rename = "includeFec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fec: Option<String>,
    /// <p>Parameter L from SMPTE 2022-1. The width of the FEC protection matrix.  Must be between 1 and 20, inclusive. If only Column FEC is used, then larger values increase robustness.  If Row FEC is used, then this is the number of transport stream packets per row error correction packet, and the value must be between 4 and 20, inclusive, if includeFec is columnAndRow. If includeFec is column, this value must be 1 to 20, inclusive.</p>
    #[serde(rename = "rowLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_length: Option<i64>,
}

/// <p>Start time for the action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FixedModeScheduleActionStartSettings {
    /// <p>Start time for the action to start in the channel. (Not the time for the action to be added to the schedule: actions are always added to the schedule immediately.) UTC format: yyyy-mm-ddThh:mm:ss.nnnZ. All the letters are digits (for example, mm might be 01) except for the two constants &quot;T&quot; for time and &quot;Z&quot; for &quot;UTC format&quot;.</p>
    #[serde(rename = "time")]
    pub time: String,
}

/// <p>Fmp4 Hls Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Fmp4HlsSettings {
    /// <p>List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by &#39;,&#39;.</p>
    #[serde(rename = "audioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    /// <p>If set to passthrough, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3_behavior: Option<String>,
    /// <p>When set to passthrough, timed metadata is passed through from input to output.</p>
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
}

/// <p>Settings to specify if an action follows another.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FollowModeScheduleActionStartSettings {
    /// <p>Identifies whether this action starts relative to the start or relative to the end of the reference action.</p>
    #[serde(rename = "followPoint")]
    pub follow_point: String,
    /// <p>The action name of another action that this one refers to.</p>
    #[serde(rename = "referenceActionName")]
    pub reference_action_name: String,
}

/// <p>Frame Capture Cdn Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureCdnSettings {
    #[serde(rename = "frameCaptureS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_s3_settings: Option<FrameCaptureS3Settings>,
}

/// <p>Frame Capture Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureGroupSettings {
    /// <p>The destination for the frame capture files. Either the URI for an Amazon S3 bucket and object, plus a file name prefix (for example, s3ssl://sportsDelivery/highlights/20180820/curling-) or the URI for a MediaStore container, plus a file name prefix (for example, mediastoressl://sportsDelivery/20180820/curling-). The final file names consist of the prefix from the destination field (for example, &quot;curling-&quot;) + name modifier + the counter (5 digits, starting from 00001) + extension (which is always .jpg).  For example, curling-low.00001.jpg</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>Parameters that control interactions with the CDN.</p>
    #[serde(rename = "frameCaptureCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_cdn_settings: Option<FrameCaptureCdnSettings>,
}

/// <p>Frame Capture Hls Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureHlsSettings {}

/// <p>Frame Capture Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureOutputSettings {
    /// <p>Required if the output group contains more than one output. This modifier forms part of the output file name.</p>
    #[serde(rename = "nameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

/// <p>Frame Capture S3 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureS3Settings {
    /// <p>Specify the canned ACL to apply to each S3 request. Defaults to none.</p>
    #[serde(rename = "cannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

/// <p>Frame Capture Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureSettings {
    /// <p>The frequency at which to capture frames for inclusion in the output. May be specified in either seconds or milliseconds, as specified by captureIntervalUnits.</p>
    #[serde(rename = "captureInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval: Option<i64>,
    /// <p>Unit for the frame capture interval.</p>
    #[serde(rename = "captureIntervalUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_interval_units: Option<String>,
}

/// <p>Global Configuration</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct GlobalConfiguration {
    /// <p>Value to set the initial audio gain for the Live Event.</p>
    #[serde(rename = "initialAudioGain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_audio_gain: Option<i64>,
    /// <p>Indicates the action to take when the current input completes (e.g. end-of-file). When switchAndLoopInputs is configured the encoder will restart at the beginning of the first input.  When &quot;none&quot; is configured the encoder will transcode either black, a solid color, or a user specified slate images per the &quot;Input Loss Behavior&quot; configuration until the next input switch occurs (which is controlled through the Channel Schedule API).</p>
    #[serde(rename = "inputEndAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_end_action: Option<String>,
    /// <p>Settings for system actions when input is lost.</p>
    #[serde(rename = "inputLossBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_behavior: Option<InputLossBehavior>,
    /// <p>Indicates how MediaLive pipelines are synchronized.</p>
    ///
    /// <p>PIPELINE<em>LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the other.
    /// EPOCH</em>LOCKING - MediaLive will attempt to synchronize the output of each pipeline to the Unix epoch.</p>
    #[serde(rename = "outputLockingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_locking_mode: Option<String>,
    /// <p>Indicates whether the rate of frames emitted by the Live encoder should be paced by its system clock (which optionally may be locked to another source via NTP) or should be locked to the clock of the source that is providing the input stream.</p>
    #[serde(rename = "outputTimingSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_timing_source: Option<String>,
    /// <p>Adjusts video input buffer for streams with very low video framerates. This is commonly set to enabled for music channels with less than one video frame per second.</p>
    #[serde(rename = "supportLowFramerateInputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_low_framerate_inputs: Option<String>,
}

/// <p>H264 Color Space Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H264ColorSpaceSettings {
    #[serde(rename = "colorSpacePassthroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "rec601Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_601_settings: Option<Rec601Settings>,
    #[serde(rename = "rec709Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_709_settings: Option<Rec709Settings>,
}

/// <p>H264 Filter Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H264FilterSettings {
    #[serde(rename = "temporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

/// <p>H264 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H264Settings {
    /// <p>Enables or disables adaptive quantization, which is a technique MediaLive can apply to video on a frame-by-frame basis to produce more compression without losing quality. There are three types of adaptive quantization: flicker, spatial, and temporal. Set the field in one of these ways: Set to Auto. Recommended. For each type of AQ, MediaLive will determine if AQ is needed, and if so, the appropriate strength. Set a strength (a value other than Auto or Disable). This strength will apply to any of the AQ fields that you choose to enable. Set to Disabled to disable all types of adaptive quantization.</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Indicates that AFD values will be written into the output stream.  If afdSignaling is &quot;auto&quot;, the system will try to preserve the input AFD value (in cases where multiple AFD values are valid). If set to &quot;fixed&quot;, the AFD value will be the value configured in the fixedAfd parameter.</p>
    #[serde(rename = "afdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>Average bitrate in bits/second. Required when the rate control mode is VBR or CBR. Not used for QVBR. In an MS Smooth output group, each output must have a unique value when its bitrate is rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "bufFillPct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_fill_pct: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits.</p>
    #[serde(rename = "bufSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i64>,
    /// <p>Includes colorspace metadata in the output.</p>
    #[serde(rename = "colorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Color Space settings</p>
    #[serde(rename = "colorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H264ColorSpaceSettings>,
    /// <p>Entropy encoding mode.  Use cabac (must be in Main or High profile) or cavlc.</p>
    #[serde(rename = "entropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    /// <p>Optional filters that you can apply to an encode.</p>
    #[serde(rename = "filterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H264FilterSettings>,
    /// <p>Four bit AFD value to write on all frames of video in the output stream. Only valid when afdSignaling is set to &#39;Fixed&#39;.</p>
    #[serde(rename = "fixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    /// <p>Flicker AQ makes adjustments within each frame to reduce flicker or &#39;pop&#39; on I-frames. The value to enter in this field depends on the value in the Adaptive quantization field: If you have set the Adaptive quantization field to Auto, MediaLive ignores any value in this field. MediaLive will determine if flicker AQ is appropriate and will apply the appropriate strength. If you have set the Adaptive quantization field to a strength, you can set this field to Enabled or Disabled. Enabled: MediaLive will apply flicker AQ using the specified strength. Disabled: MediaLive won&#39;t apply flicker AQ. If you have set the Adaptive quantization to Disabled, MediaLive ignores any value in this field and doesn&#39;t apply flicker AQ.</p>
    #[serde(rename = "flickerAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<String>,
    /// <p>This setting applies only when scan type is &quot;interlaced.&quot; It controls whether coding is performed on a field basis or on a frame basis. (When the video is progressive, the coding is always performed on a frame basis.)
    /// enabled: Force MediaLive to code on a field basis, so that odd and even sets of fields are coded separately.
    /// disabled: Code the two sets of fields separately (on a field basis) or together (on a frame basis using PAFF), depending on what is most appropriate for the content.</p>
    #[serde(rename = "forceFieldPictures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_field_pictures: Option<String>,
    /// <p>This field indicates how the output video frame rate is specified.  If &quot;specified&quot; is selected then the output video frame rate is determined by framerateNumerator and framerateDenominator, else if &quot;initializeFromSource&quot; is selected then the output video frame rate will be set equal to the input video frame rate of the first input.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Framerate denominator.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Documentation update needed</p>
    #[serde(rename = "gopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "gopNumBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_b_frames: Option<i64>,
    /// <p>GOP size (keyframe interval) in units of either frames or seconds per gopSizeUnits.
    /// If gopSizeUnits is frames, gopSize must be an integer and must be greater than or equal to 1.
    /// If gopSizeUnits is seconds, gopSize must be greater than 0, but need not be an integer.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Indicates if the gopSize is specified in frames or seconds. If seconds the system will convert the gopSize into a frame count at run time.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>H.264 Level.</p>
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>Amount of lookahead. A value of low can decrease latency and memory usage, while high can produce better quality for certain content.</p>
    #[serde(rename = "lookAheadRateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<String>,
    /// <p>For QVBR: See the tooltip for Quality level</p>
    ///
    /// <p>For VBR: Set the maximum bitrate in order to accommodate expected spikes in the complexity of the video.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Only meaningful if sceneChangeDetect is set to enabled.  Defaults to 5 if multiplex rate control is used.  Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "minIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of reference frames to use. The encoder may use more than requested if using B-frames and/or interlaced encoding.</p>
    #[serde(rename = "numRefFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_ref_frames: Option<i64>,
    /// <p>This field indicates how the output pixel aspect ratio is specified.  If &quot;specified&quot; is selected then the output video pixel aspect ratio is determined by parNumerator and parDenominator, else if &quot;initializeFromSource&quot; is selected then the output pixsel aspect ratio will be set equal to the input video pixel aspect ratio of the first input.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>H.264 Profile.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Leave as STANDARD<em>QUALITY or choose a different value (which might result in additional costs to run the channel).
    /// - ENHANCED</em>QUALITY: Produces a slightly better video quality without an increase in the bitrate. Has an effect only when the Rate control mode is QVBR or CBR. If this channel is in a MediaLive multiplex, the value must be ENHANCED<em>QUALITY.
    /// - STANDARD</em>QUALITY: Valid for any Rate control mode.</p>
    #[serde(rename = "qualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_level: Option<String>,
    /// <p>Controls the target quality for the video encode. Applies only when the rate control mode is QVBR. You can set a target quality or you can let MediaLive determine the best quality. To set a target quality, enter values in the QVBR quality level field and the Max bitrate field. Enter values that suit your most important viewing devices. Recommended values are:
    /// - Primary screen: Quality level: 8 to 10. Max bitrate: 4M
    /// - PC or tablet: Quality level: 7. Max bitrate: 1.5M to 3M
    /// - Smartphone: Quality level: 6. Max bitrate: 1M to 1.5M
    /// To let MediaLive decide, leave the QVBR quality level field empty, and in Max bitrate enter the maximum rate you want in the video. For more information, see the section called &quot;Video - rate control mode&quot; in the MediaLive user guide</p>
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
    /// <p>Rate control mode.</p>
    ///
    /// <p>QVBR: Quality will match the specified quality level except when it is constrained by the
    /// maximum bitrate.  Recommended if you or your viewers pay for bandwidth.</p>
    ///
    /// <p>VBR: Quality and bitrate vary, depending on the video complexity. Recommended instead of QVBR
    /// if you want to maintain a specific average bitrate over the duration of the channel.</p>
    ///
    /// <p>CBR: Quality varies, depending on the video complexity. Recommended only if you distribute
    /// your assets to devices that cannot handle variable bitrates.</p>
    ///
    /// <p>Multiplex: This rate control mode is only supported (and is required) when the video is being
    /// delivered to a MediaLive Multiplex in which case the rate control configuration is controlled
    /// by the properties within the Multiplex Program.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Sets the scan type of the output to progressive or top-field-first interlaced.</p>
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// <p>Scene change detection.</p>
    ///
    /// <ul>
    /// <li>On: inserts I-frames when scene change is detected.</li>
    /// <li>Off: does not force an I-frame when scene change is detected.</li>
    /// </ul>
    #[serde(rename = "sceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.
    /// This field is optional; when no value is specified the encoder will choose the number of slices based on encode resolution.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Softness. Selects quantizer matrix, larger values reduce high-frequency content in the encoded image.  If not set to zero, must be greater than 15.</p>
    #[serde(rename = "softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>Spatial AQ makes adjustments within each frame based on spatial variation of content complexity. The value to enter in this field depends on the value in the Adaptive quantization field: If you have set the Adaptive quantization field to Auto, MediaLive ignores any value in this field. MediaLive will determine if spatial AQ is appropriate and will apply the appropriate strength. If you have set the Adaptive quantization field to a strength, you can set this field to Enabled or Disabled. Enabled: MediaLive will apply spatial AQ using the specified strength. Disabled: MediaLive won&#39;t apply spatial AQ. If you have set the Adaptive quantization to Disabled, MediaLive ignores any value in this field and doesn&#39;t apply spatial AQ.</p>
    #[serde(rename = "spatialAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_aq: Option<String>,
    /// <p>If set to fixed, use gopNumBFrames B-frames per sub-GOP. If set to dynamic, optimize the number of B-frames used for each sub-GOP to improve visual quality.</p>
    #[serde(rename = "subgopLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<String>,
    /// <p>Produces a bitstream compliant with SMPTE RP-2027.</p>
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// <p>Temporal makes adjustments within each frame based on temporal variation of content complexity. The value to enter in this field depends on the value in the Adaptive quantization field: If you have set the Adaptive quantization field to Auto, MediaLive ignores any value in this field. MediaLive will determine if temporal AQ is appropriate and will apply the appropriate strength. If you have set the Adaptive quantization field to a strength, you can set this field to Enabled or Disabled. Enabled: MediaLive will apply temporal AQ using the specified strength. Disabled: MediaLive won&#39;t apply temporal AQ. If you have set the Adaptive quantization to Disabled, MediaLive ignores any value in this field and doesn&#39;t apply temporal AQ.</p>
    #[serde(rename = "temporalAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_aq: Option<String>,
    /// <p>Determines how timecodes should be inserted into the video elementary stream.
    /// - &#39;disabled&#39;: Do not include timecodes
    /// - &#39;picTimingSei&#39;: Pass through picture timing SEI messages from the source specified in Timecode Config</p>
    #[serde(rename = "timecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

/// <p>H265 Color Space Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H265ColorSpaceSettings {
    #[serde(rename = "colorSpacePassthroughSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_passthrough_settings: Option<ColorSpacePassthroughSettings>,
    #[serde(rename = "hdr10Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_settings: Option<Hdr10Settings>,
    #[serde(rename = "rec601Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_601_settings: Option<Rec601Settings>,
    #[serde(rename = "rec709Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rec_709_settings: Option<Rec709Settings>,
}

/// <p>H265 Filter Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H265FilterSettings {
    #[serde(rename = "temporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

/// <p>H265 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H265Settings {
    /// <p>Adaptive quantization. Allows intra-frame quantizers to vary to improve visual quality.</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Indicates that AFD values will be written into the output stream.  If afdSignaling is &quot;auto&quot;, the system will try to preserve the input AFD value (in cases where multiple AFD values are valid). If set to &quot;fixed&quot;, the AFD value will be the value configured in the fixedAfd parameter.</p>
    #[serde(rename = "afdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>Whether or not EML should insert an Alternative Transfer Function SEI message to support backwards compatibility with non-HDR decoders and displays.</p>
    #[serde(rename = "alternativeTransferFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternative_transfer_function: Option<String>,
    /// <p>Average bitrate in bits/second. Required when the rate control mode is VBR or CBR. Not used for QVBR. In an MS Smooth output group, each output must have a unique value when its bitrate is rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits.</p>
    #[serde(rename = "bufSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buf_size: Option<i64>,
    /// <p>Includes colorspace metadata in the output.</p>
    #[serde(rename = "colorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Color Space settings</p>
    #[serde(rename = "colorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<H265ColorSpaceSettings>,
    /// <p>Optional filters that you can apply to an encode.</p>
    #[serde(rename = "filterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<H265FilterSettings>,
    /// <p>Four bit AFD value to write on all frames of video in the output stream. Only valid when afdSignaling is set to &#39;Fixed&#39;.</p>
    #[serde(rename = "fixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    /// <p>If set to enabled, adjust quantization within each frame to reduce flicker or &#39;pop&#39; on I-frames.</p>
    #[serde(rename = "flickerAq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_aq: Option<String>,
    /// <p>Framerate denominator.</p>
    #[serde(rename = "framerateDenominator")]
    pub framerate_denominator: i64,
    /// <p>Framerate numerator - framerate is a fraction, e.g. 24000 / 1001 = 23.976 fps.</p>
    #[serde(rename = "framerateNumerator")]
    pub framerate_numerator: i64,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP size (keyframe interval) in units of either frames or seconds per gopSizeUnits.
    /// If gopSizeUnits is frames, gopSize must be an integer and must be greater than or equal to 1.
    /// If gopSizeUnits is seconds, gopSize must be greater than 0, but need not be an integer.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Indicates if the gopSize is specified in frames or seconds. If seconds the system will convert the gopSize into a frame count at run time.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>H.265 Level.</p>
    #[serde(rename = "level")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    /// <p>Amount of lookahead. A value of low can decrease latency and memory usage, while high can produce better quality for certain content.</p>
    #[serde(rename = "lookAheadRateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub look_ahead_rate_control: Option<String>,
    /// <p>For QVBR: See the tooltip for Quality level</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Only meaningful if sceneChangeDetect is set to enabled.  Defaults to 5 if multiplex rate control is used.  Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "minIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Pixel Aspect Ratio denominator.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Pixel Aspect Ratio numerator.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>H.265 Profile.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Controls the target quality for the video encode. Applies only when the rate control mode is QVBR. Set values for the QVBR quality level field and Max bitrate field that suit your most important viewing devices. Recommended values are:
    /// - Primary screen: Quality level: 8 to 10. Max bitrate: 4M
    /// - PC or tablet: Quality level: 7. Max bitrate: 1.5M to 3M
    /// - Smartphone: Quality level: 6. Max bitrate: 1M to 1.5M</p>
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
    /// <p>Rate control mode.</p>
    ///
    /// <p>QVBR: Quality will match the specified quality level except when it is constrained by the
    /// maximum bitrate.  Recommended if you or your viewers pay for bandwidth.</p>
    ///
    /// <p>CBR: Quality varies, depending on the video complexity. Recommended only if you distribute
    /// your assets to devices that cannot handle variable bitrates.</p>
    ///
    /// <p>Multiplex: This rate control mode is only supported (and is required) when the video is being
    /// delivered to a MediaLive Multiplex in which case the rate control configuration is controlled
    /// by the properties within the Multiplex Program.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Sets the scan type of the output to progressive or top-field-first interlaced.</p>
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// <p>Scene change detection.</p>
    #[serde(rename = "sceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.
    /// This field is optional; when no value is specified the encoder will choose the number of slices based on encode resolution.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>H.265 Tier.</p>
    #[serde(rename = "tier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    /// <p>Determines how timecodes should be inserted into the video elementary stream.
    /// - &#39;disabled&#39;: Do not include timecodes
    /// - &#39;picTimingSei&#39;: Pass through picture timing SEI messages from the source specified in Timecode Config</p>
    #[serde(rename = "timecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

/// <p>Hdr10 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Hdr10Settings {
    /// <p>Maximum Content Light Level
    /// An integer metadata value defining the maximum light level, in nits,
    /// of any single pixel within an encoded HDR video stream or file.</p>
    #[serde(rename = "maxCll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cll: Option<i64>,
    /// <p>Maximum Frame Average Light Level
    /// An integer metadata value defining the maximum average light level, in nits,
    /// for any single frame within an encoded HDR video stream or file.</p>
    #[serde(rename = "maxFall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fall: Option<i64>,
}

/// <p>Hls Akamai Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsAkamaiSettings {
    /// <p>Number of seconds to wait before retrying connection to the CDN if the connection is lost.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>Size in seconds of file cache for streaming outputs.</p>
    #[serde(rename = "filecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,
    /// <p>Specify whether or not to use chunked transfer encoding to Akamai. User should contact Akamai to enable this feature.</p>
    #[serde(rename = "httpTransferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<String>,
    /// <p>Number of retry attempts that will be made before the Live Event is put into an error state.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
    /// <p>If a streaming output fails, number of seconds to wait until a restart is initiated. A value of 0 means never restart.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
    /// <p>Salt for authenticated Akamai.</p>
    #[serde(rename = "salt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<String>,
    /// <p>Token parameter for authenticated akamai. If not specified, <em>gda</em> is used.</p>
    #[serde(rename = "token")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

/// <p>Hls Basic Put Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsBasicPutSettings {
    /// <p>Number of seconds to wait before retrying connection to the CDN if the connection is lost.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>Size in seconds of file cache for streaming outputs.</p>
    #[serde(rename = "filecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,
    /// <p>Number of retry attempts that will be made before the Live Event is put into an error state.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
    /// <p>If a streaming output fails, number of seconds to wait until a restart is initiated. A value of 0 means never restart.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

/// <p>Hls Cdn Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsCdnSettings {
    #[serde(rename = "hlsAkamaiSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_akamai_settings: Option<HlsAkamaiSettings>,
    #[serde(rename = "hlsBasicPutSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_basic_put_settings: Option<HlsBasicPutSettings>,
    #[serde(rename = "hlsMediaStoreSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_media_store_settings: Option<HlsMediaStoreSettings>,
    #[serde(rename = "hlsS3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_s3_settings: Option<HlsS3Settings>,
    #[serde(rename = "hlsWebdavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_webdav_settings: Option<HlsWebdavSettings>,
}

/// <p>Hls Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsGroupSettings {
    /// <p>Choose one or more ad marker types to pass SCTE35 signals through to this group of Apple HLS outputs.</p>
    #[serde(rename = "adMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    /// <p>A partial URI prefix that will be prepended to each output in the media .m3u8 file. Can be used if base manifest is delivered from a different URL than the main .m3u8 file.</p>
    #[serde(rename = "baseUrlContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content: Option<String>,
    /// <p>Optional. One value per output group.</p>
    ///
    /// <p>This field is required only if you are completing Base URL content A, and the downstream system has notified you that the media files for pipeline 1 of all outputs are in a location different from the media files for pipeline 0.</p>
    #[serde(rename = "baseUrlContent1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_content_1: Option<String>,
    /// <p>A partial URI prefix that will be prepended to each output in the media .m3u8 file. Can be used if base manifest is delivered from a different URL than the main .m3u8 file.</p>
    #[serde(rename = "baseUrlManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest: Option<String>,
    /// <p>Optional. One value per output group.</p>
    ///
    /// <p>Complete this field only if you are completing Base URL manifest A, and the downstream system has notified you that the child manifest files for pipeline 1 of all outputs are in a location different from the child manifest files for pipeline 0.</p>
    #[serde(rename = "baseUrlManifest1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url_manifest_1: Option<String>,
    /// <p>Mapping of up to 4 caption channels to caption languages.  Is only meaningful if captionLanguageSetting is set to &quot;insert&quot;.</p>
    #[serde(rename = "captionLanguageMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<CaptionLanguageMapping>>,
    /// <p>Applies only to 608 Embedded output captions.
    /// insert: Include CLOSED-CAPTIONS lines in the manifest. Specify at least one language in the CC1 Language Code field. One CLOSED-CAPTION line is added for each Language Code you specify. Make sure to specify the languages in the order in which they appear in the original source (if the source is embedded format) or the order of the caption selectors (if the source is other than embedded). Otherwise, languages in the manifest will not match up properly with the output captions.
    /// none: Include CLOSED-CAPTIONS=NONE line in the manifest.
    /// omit: Omit any CLOSED-CAPTIONS line from the manifest.</p>
    #[serde(rename = "captionLanguageSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<String>,
    /// <p>When set to &quot;disabled&quot;, sets the #EXT-X-ALLOW-CACHE:no tag in the manifest, which prevents clients from saving media segments for later replay.</p>
    #[serde(rename = "clientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    /// <p>Specification to use (RFC-6381 or the default RFC-4281) during m3u8 playlist generation.</p>
    #[serde(rename = "codecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>For use with encryptionType. This is a 128-bit, 16-byte hex value represented by a 32-character text string. If ivSource is set to &quot;explicit&quot; then this parameter is required and is used as the IV for encryption.</p>
    #[serde(rename = "constantIv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_iv: Option<String>,
    /// <p>A directory or HTTP destination for the HLS segments, manifest files, and encryption keys (if enabled).</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>Place segments in subdirectories.</p>
    #[serde(rename = "directoryStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<String>,
    /// <p>Specifies whether to insert EXT-X-DISCONTINUITY tags in the HLS child manifests for this output group.
    /// Typically, choose Insert because these tags are required in the manifest (according to the HLS specification) and serve an important purpose.
    /// Choose Never Insert only if the downstream system is doing real-time failover (without using the MediaLive automatic failover feature) and only if that downstream system has advised you to exclude the tags.</p>
    #[serde(rename = "discontinuityTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discontinuity_tags: Option<String>,
    /// <p>Encrypts the segments with the given encryption scheme.  Exclude this parameter if no encryption is desired.</p>
    #[serde(rename = "encryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Parameters that control interactions with the CDN.</p>
    #[serde(rename = "hlsCdnSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_cdn_settings: Option<HlsCdnSettings>,
    /// <p>State of HLS ID3 Segment Tagging</p>
    #[serde(rename = "hlsId3SegmentTagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_id_3_segment_tagging: Option<String>,
    /// <p>DISABLED: Do not create an I-frame-only manifest, but do create the master and media manifests (according to the Output Selection field).</p>
    ///
    /// <p>STANDARD: Create an I-frame-only manifest for each output that contains video, as well as the other manifests (according to the Output Selection field). The I-frame manifest contains a #EXT-X-I-FRAMES-ONLY tag to indicate it is I-frame only, and one or more #EXT-X-BYTERANGE entries identifying the I-frame position. For example, #EXT-X-BYTERANGE:160364@1461888&quot;</p>
    #[serde(rename = "iFrameOnlyPlaylists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_frame_only_playlists: Option<String>,
    /// <p>Specifies whether to include the final (incomplete) segment in the media output when the pipeline stops producing output because of a channel stop, a channel pause or a loss of input to the pipeline.
    /// Auto means that MediaLive decides whether to include the final segment, depending on the channel class and the types of output groups.
    /// Suppress means to never include the incomplete segment. We recommend you choose Auto and let MediaLive control the behavior.</p>
    #[serde(rename = "incompleteSegmentBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_segment_behavior: Option<String>,
    /// <p>Applies only if Mode field is LIVE.</p>
    ///
    /// <p>Specifies the maximum number of segments in the media manifest file. After this maximum, older segments are removed from the media manifest. This number must be smaller than the number in the Keep Segments field.</p>
    #[serde(rename = "indexNSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_n_segments: Option<i64>,
    /// <p>Parameter that control output group behavior on input loss.</p>
    #[serde(rename = "inputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    /// <p>For use with encryptionType. The IV (Initialization Vector) is a 128-bit number used in conjunction with the key for encrypting blocks. If set to &quot;include&quot;, IV is listed in the manifest, otherwise the IV is not in the manifest.</p>
    #[serde(rename = "ivInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_in_manifest: Option<String>,
    /// <p>For use with encryptionType. The IV (Initialization Vector) is a 128-bit number used in conjunction with the key for encrypting blocks. If this setting is &quot;followsSegmentNumber&quot;, it will cause the IV to change every segment (to match the segment number). If this is set to &quot;explicit&quot;, you must enter a constantIv value.</p>
    #[serde(rename = "ivSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iv_source: Option<String>,
    /// <p>Applies only if Mode field is LIVE.</p>
    ///
    /// <p>Specifies the number of media segments to retain in the destination directory. This number should be bigger than indexNSegments (Num segments). We recommend (value = (2 x indexNsegments) + 1).</p>
    ///
    /// <p>If this &quot;keep segments&quot; number is too low, the following might happen: the player is still reading a media manifest file that lists this segment, but that segment has been removed from the destination directory (as directed by indexNSegments). This situation would result in a 404 HTTP error on the player.</p>
    #[serde(rename = "keepSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_segments: Option<i64>,
    /// <p>The value specifies how the key is represented in the resource identified by the URI.  If parameter is absent, an implicit value of &quot;identity&quot; is used.  A reverse DNS string can also be given.</p>
    #[serde(rename = "keyFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    /// <p>Either a single positive integer version value or a slash delimited list of version values (1/2/3).</p>
    #[serde(rename = "keyFormatVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format_versions: Option<String>,
    /// <p>The key provider settings.</p>
    #[serde(rename = "keyProviderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_settings: Option<KeyProviderSettings>,
    /// <p>When set to gzip, compresses HLS playlist.</p>
    #[serde(rename = "manifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    /// <p>Indicates whether the output manifest should use floating point or integer values for segment duration.</p>
    #[serde(rename = "manifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>When set, minimumSegmentLength is enforced by looking ahead and back within the specified range for a nearby avail and extending the segment size if needed.</p>
    #[serde(rename = "minSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_length: Option<i64>,
    /// <p>If &quot;vod&quot;, all segments are indexed and kept permanently in the destination and manifest. If &quot;live&quot;, only the number segments specified in keepSegments and indexNSegments are kept; newer segments replace older segments, which may prevent players from rewinding all the way to the beginning of the event.</p>
    ///
    /// <p>VOD mode uses HLS EXT-X-PLAYLIST-TYPE of EVENT while the channel is running, converting it to a &quot;VOD&quot; type manifest on completion of the stream.</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>MANIFESTS<em>AND</em>SEGMENTS: Generates manifests (master manifest, if applicable, and media manifests) for this output group.</p>
    ///
    /// <p>VARIANT<em>MANIFESTS</em>AND_SEGMENTS: Generates media manifests for this output group, but not a master manifest.</p>
    ///
    /// <p>SEGMENTS_ONLY: Does not generate any manifests for this output group.</p>
    #[serde(rename = "outputSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<String>,
    /// <p>Includes or excludes EXT-X-PROGRAM-DATE-TIME tag in .m3u8 manifest files. The value is calculated as follows: either the program date and time are initialized using the input timecode source, or the time is initialized using the input timecode source and the date is initialized using the timestampOffset.</p>
    #[serde(rename = "programDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<String>,
    /// <p>Period of insertion of EXT-X-PROGRAM-DATE-TIME entry, in seconds.</p>
    #[serde(rename = "programDateTimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i64>,
    /// <p>ENABLED: The master manifest (.m3u8 file) for each pipeline includes information about both pipelines: first its own media files, then the media files of the other pipeline. This feature allows playout device that support stale manifest detection to switch from one manifest to the other, when the current manifest seems to be stale. There are still two destinations and two master manifests, but both master manifests reference the media files from both pipelines.</p>
    ///
    /// <p>DISABLED: The master manifest (.m3u8 file) for each pipeline includes information about its own pipeline only.</p>
    ///
    /// <p>For an HLS output group with MediaPackage as the destination, the DISABLED behavior is always followed. MediaPackage regenerates the manifests it serves to players so a redundant manifest from MediaLive is irrelevant.</p>
    #[serde(rename = "redundantManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redundant_manifest: Option<String>,
    /// <p>Length of MPEG-2 Transport Stream segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer.</p>
    #[serde(rename = "segmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>useInputSegmentation has been deprecated. The configured segment size is always used.</p>
    #[serde(rename = "segmentationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<String>,
    /// <p>Number of segments to write to a subdirectory before starting a new one. directoryStructure must be subdirectoryPerStream for this setting to have an effect.</p>
    #[serde(rename = "segmentsPerSubdirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_per_subdirectory: Option<i64>,
    /// <p>Include or exclude RESOLUTION attribute for video in EXT-X-STREAM-INF tag of variant manifest.</p>
    #[serde(rename = "streamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    /// <p>Indicates ID3 frame that has the timecode.</p>
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_frame: Option<String>,
    /// <p>Timed Metadata interval in seconds.</p>
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_period: Option<i64>,
    /// <p>Provides an extra millisecond delta offset to fine tune the timestamps.</p>
    #[serde(rename = "timestampDeltaMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_delta_milliseconds: Option<i64>,
    /// <p>SEGMENTED_FILES: Emit the program as segments - multiple .ts media files.</p>
    ///
    /// <p>SINGLE_FILE: Applies only if Mode field is VOD. Emit the program as a single .ts media file. The media manifest includes #EXT-X-BYTERANGE tags to index segments for playback. A typical use for this value is when sending the output to AWS Elemental MediaConvert, which can accept only a single media file. Playback while the channel is running is not guaranteed due to HTTP server caching.</p>
    #[serde(rename = "tsFileMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ts_file_mode: Option<String>,
}

/// <p>Settings for the action to insert a user-defined ID3 tag in each HLS segment</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsId3SegmentTaggingScheduleActionSettings {
    /// <p>ID3 tag to insert into each segment. Supports special keyword identifiers to substitute in segment-related values.\nSupported keyword identifiers: https://docs.aws.amazon.com/medialive/latest/ug/variable-data-identifiers.html</p>
    #[serde(rename = "tag")]
    pub tag: String,
}

/// <p>Hls Input Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsInputSettings {
    /// <p>When specified the HLS stream with the m3u8 BANDWIDTH that most closely matches this value will be chosen, otherwise the highest bandwidth stream in the m3u8 will be chosen.  The bitrate is specified in bits per second, as in an HLS manifest.</p>
    #[serde(rename = "bandwidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth: Option<i64>,
    /// <p>When specified, reading of the HLS input will begin this many buffer segments from the end (most recently written segment).  When not specified, the HLS input will begin with the first segment specified in the m3u8.</p>
    #[serde(rename = "bufferSegments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_segments: Option<i64>,
    /// <p>The number of consecutive times that attempts to read a manifest or segment must fail before the input is considered unavailable.</p>
    #[serde(rename = "retries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retries: Option<i64>,
    /// <p>The number of seconds between retries when an attempt to read a manifest or segment fails.</p>
    #[serde(rename = "retryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    /// <p>Identifies the source for the SCTE-35 messages that MediaLive will ingest. Messages can be ingested from the content segments (in the stream) or from tags in the playlist (the HLS manifest). MediaLive ignores SCTE-35 information in the source that is not selected.</p>
    #[serde(rename = "scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
}

/// <p>Hls Media Store Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsMediaStoreSettings {
    /// <p>Number of seconds to wait before retrying connection to the CDN if the connection is lost.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>Size in seconds of file cache for streaming outputs.</p>
    #[serde(rename = "filecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,
    /// <p>When set to temporal, output files are stored in non-persistent memory for faster reading and writing.</p>
    #[serde(rename = "mediaStoreStorageClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_store_storage_class: Option<String>,
    /// <p>Number of retry attempts that will be made before the Live Event is put into an error state.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
    /// <p>If a streaming output fails, number of seconds to wait until a restart is initiated. A value of 0 means never restart.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

/// <p>Hls Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsOutputSettings {
    /// <p>Only applicable when this output is referencing an H.265 video description.
    /// Specifies whether MP4 segments should be packaged as HEV1 or HVC1.</p>
    #[serde(rename = "h265PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<String>,
    /// <p>Settings regarding the underlying stream. These settings are different for audio-only outputs.</p>
    #[serde(rename = "hlsSettings")]
    pub hls_settings: HlsSettings,
    /// <p>String concatenated to the end of the destination filename. Accepts &quot;Format Identifiers&quot;:#formatIdentifierParameters.</p>
    #[serde(rename = "nameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
    /// <p>String concatenated to end of segment filenames.</p>
    #[serde(rename = "segmentModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_modifier: Option<String>,
}

/// <p>Hls S3 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsS3Settings {
    /// <p>Specify the canned ACL to apply to each S3 request. Defaults to none.</p>
    #[serde(rename = "cannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

/// <p>Hls Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsSettings {
    #[serde(rename = "audioOnlyHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_hls_settings: Option<AudioOnlyHlsSettings>,
    #[serde(rename = "fmp4HlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmp_4_hls_settings: Option<Fmp4HlsSettings>,
    #[serde(rename = "frameCaptureHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_hls_settings: Option<FrameCaptureHlsSettings>,
    #[serde(rename = "standardHlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standard_hls_settings: Option<StandardHlsSettings>,
}

/// <p>Settings for the action to emit HLS metadata</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsTimedMetadataScheduleActionSettings {
    /// <p>Base64 string formatted according to the ID3 specification: http://id3.org/id3v2.4.0-structure</p>
    #[serde(rename = "id3")]
    pub id_3: String,
}

/// <p>Hls Webdav Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsWebdavSettings {
    /// <p>Number of seconds to wait before retrying connection to the CDN if the connection is lost.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>Size in seconds of file cache for streaming outputs.</p>
    #[serde(rename = "filecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,
    /// <p>Specify whether or not to use chunked transfer encoding to WebDAV.</p>
    #[serde(rename = "httpTransferMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_transfer_mode: Option<String>,
    /// <p>Number of retry attempts that will be made before the Live Event is put into an error state.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
    /// <p>If a streaming output fails, number of seconds to wait until a restart is initiated. A value of 0 means never restart.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

/// <p>Html Motion Graphics Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HtmlMotionGraphicsSettings {}

/// <p>Settings to configure an action so that it occurs as soon as possible.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImmediateModeScheduleActionStartSettings {}

/// <p>Placeholder documentation for Input</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Input {
    /// <p>The Unique ARN of the input (generated, immutable).</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of channel IDs that that input is attached to (currently an input can only be attached to one channel).</p>
    #[serde(rename = "attachedChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attached_channels: Option<Vec<String>>,
    /// <p>A list of the destinations of the input (PUSH-type).</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestination>>,
    /// <p>The generated ID of the input (unique for user account, immutable).</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>STANDARD - MediaLive expects two sources to be connected to this input. If the channel is also STANDARD, both sources will be ingested. If the channel is SINGLE<em>PIPELINE, only the first source will be ingested; the second source will always be ignored, even if the first source fails.
    /// SINGLE</em>PIPELINE - You can connect only one source to this input. If the ChannelClass is also  SINGLE_PIPELINE, this value is valid. If the ChannelClass is STANDARD, this value is not valid because the channel requires two sources in the input.</p>
    #[serde(rename = "inputClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_class: Option<String>,
    /// <p>Settings for the input devices.</p>
    #[serde(rename = "inputDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSettings>>,
    /// <p>A list of IDs for all Inputs which are partners of this one.</p>
    #[serde(rename = "inputPartnerIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_partner_ids: Option<Vec<String>>,
    /// <p>Certain pull input sources can be dynamic, meaning that they can have their URL&#39;s dynamically changes
    /// during input switch actions. Presently, this functionality only works with MP4_FILE inputs.</p>
    #[serde(rename = "inputSourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_source_type: Option<String>,
    /// <p>A list of MediaConnect Flows for this input.</p>
    #[serde(rename = "mediaConnectFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlow>>,
    /// <p>The user-assigned name (This is a mutable value).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role this input assumes during and after creation.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>A list of IDs for all the Input Security Groups attached to the input.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    /// <p>A list of the sources of the input (PULL-type).</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSource>>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Placeholder documentation for InputAttachment</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputAttachment {
    /// <p>User-specified settings for defining what the conditions are for declaring the input unhealthy and failing over to a different input.</p>
    #[serde(rename = "automaticInputFailoverSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_input_failover_settings: Option<AutomaticInputFailoverSettings>,
    /// <p>User-specified name for the attachment. This is required if the user wants to use this input in an input switch action.</p>
    #[serde(rename = "inputAttachmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name: Option<String>,
    /// <p>The ID of the input</p>
    #[serde(rename = "inputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    /// <p>Settings of an input (caption selector, etc.)</p>
    #[serde(rename = "inputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_settings: Option<InputSettings>,
}

/// <p>Input Channel Level</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputChannelLevel {
    /// <p>Remixing value. Units are in dB and acceptable values are within the range from -60 (mute) and 6 dB.</p>
    #[serde(rename = "gain")]
    pub gain: i64,
    /// <p>The index of the input channel used as a source.</p>
    #[serde(rename = "inputChannel")]
    pub input_channel: i64,
}

/// <p>Settings to let you create a clip of the file input, in order to set up the input to ingest only a portion of the file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputClippingSettings {
    /// <p>The source of the timecodes in the source being clipped.</p>
    #[serde(rename = "inputTimecodeSource")]
    pub input_timecode_source: String,
    /// <p>Settings to identify the start of the clip.</p>
    #[serde(rename = "startTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timecode: Option<StartTimecode>,
    /// <p>Settings to identify the end of the clip.</p>
    #[serde(rename = "stopTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_timecode: Option<StopTimecode>,
}

/// <p>The settings for a PUSH type input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDestination {
    /// <p>The system-generated static IP address of endpoint.
    /// It remains fixed for the lifetime of the input.</p>
    #[serde(rename = "ip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// <p>The port number for the input.</p>
    #[serde(rename = "port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    /// <p>This represents the endpoint that the customer stream will be
    /// pushed to.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<InputDestinationVpc>,
}

/// <p>Endpoint settings for a PUSH type input.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputDestinationRequest {
    /// <p>A unique name for the location the RTMP stream is being pushed
    /// to.</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>The properties for a VPC type input destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDestinationVpc {
    /// <p>The availability zone of the Input destination.</p>
    #[serde(rename = "availabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The network interface ID of the Input destination in the VPC.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
}

/// <p>Configurable settings for the input device.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputDeviceConfigurableSettings {
    /// <p>The input source that you want to use. If the device has a source connected to only one of its input ports, or if you don&#39;t care which source the device sends, specify Auto. If the device has sources connected to both its input ports, and you want to use a specific source, specify the source.</p>
    #[serde(rename = "configuredInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    /// <p>The maximum bitrate in bits per second. Set a value here to throttle the bitrate of the source video.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
}

/// <p>Settings that describe the active source from the input device, and the video characteristics of that source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDeviceHdSettings {
    /// <p>If you specified Auto as the configured input, specifies which of the sources is currently active (SDI or HDMI).</p>
    #[serde(rename = "activeInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input: Option<String>,
    /// <p>The source at the input device that is currently active. You can specify this source.</p>
    #[serde(rename = "configuredInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    /// <p>The state of the input device.</p>
    #[serde(rename = "deviceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_state: Option<String>,
    /// <p>The frame rate of the video source.</p>
    #[serde(rename = "framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f64>,
    /// <p>The height of the video source, in pixels.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The current maximum bitrate for ingesting this source, in bits per second. You can specify this maximum.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The scan type of the video source.</p>
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// <p>The width of the video source, in pixels.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>The network settings for the input device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDeviceNetworkSettings {
    /// <p>The DNS addresses of the input device.</p>
    #[serde(rename = "dnsAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_addresses: Option<Vec<String>>,
    /// <p>The network gateway IP address.</p>
    #[serde(rename = "gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// <p>The IP address of the input device.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>Specifies whether the input device has been configured (outside of MediaLive) to use a dynamic IP address assignment (DHCP) or a static IP address.</p>
    #[serde(rename = "ipScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_scheme: Option<String>,
    /// <p>The subnet mask of the input device.</p>
    #[serde(rename = "subnetMask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_mask: Option<String>,
}

/// <p>Settings for an input device.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputDeviceRequest {
    /// <p>The unique ID for the device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Settings for an input device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDeviceSettings {
    /// <p>The unique ID for the device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// <p>Details of the input device.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDeviceSummary {
    /// <p>The unique ARN of the input device.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The state of the connection between the input device and AWS.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The status of the action to synchronize the device configuration. If you change the configuration of the input device (for example, the maximum bitrate), MediaLive sends the new data to the device. The device might not update itself immediately. SYNCED means the device has updated its configuration. SYNCING means that it has not updated its configuration.</p>
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    /// <p>The status of software on the input device.</p>
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    /// <p>Settings that describe an input device that is type HD.</p>
    #[serde(rename = "hdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    /// <p>The unique ID of the input device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The network MAC address of the input device.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>A name that you specify for the input device.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Network settings for the input device.</p>
    #[serde(rename = "networkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    /// <p>The unique serial number of the input device.</p>
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The type of the input device.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Settings that describe an input device that is type UHD.</p>
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

/// <p>Settings that describe the active source from the input device, and the video characteristics of that source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputDeviceUhdSettings {
    /// <p>If you specified Auto as the configured input, specifies which of the sources is currently active (SDI or HDMI).</p>
    #[serde(rename = "activeInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input: Option<String>,
    /// <p>The source at the input device that is currently active. You can specify this source.</p>
    #[serde(rename = "configuredInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_input: Option<String>,
    /// <p>The state of the input device.</p>
    #[serde(rename = "deviceState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_state: Option<String>,
    /// <p>The frame rate of the video source.</p>
    #[serde(rename = "framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<f64>,
    /// <p>The height of the video source, in pixels.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The current maximum bitrate for ingesting this source, in bits per second. You can specify this maximum.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>The scan type of the video source.</p>
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// <p>The width of the video source, in pixels.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Input Location</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputLocation {
    /// <p>key used to extract the password from EC2 Parameter store</p>
    #[serde(rename = "passwordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    /// <p>Uniform Resource Identifier - This should be a path to a file accessible to the Live system (eg. a http:// URI) depending on the output type. For example, a RTMP destination should have a uri simliar to: &quot;rtmp://fmsserver/live&quot;.</p>
    #[serde(rename = "uri")]
    pub uri: String,
    /// <p>Documentation update needed</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Input Loss Behavior</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputLossBehavior {
    /// <p>Documentation update needed</p>
    #[serde(rename = "blackFrameMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_frame_msec: Option<i64>,
    /// <p>When input loss image type is &quot;color&quot; this field specifies the color to use. Value: 6 hex characters representing the values of RGB.</p>
    #[serde(rename = "inputLossImageColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_color: Option<String>,
    /// <p>When input loss image type is &quot;slate&quot; these fields specify the parameters for accessing the slate.</p>
    #[serde(rename = "inputLossImageSlate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_slate: Option<InputLocation>,
    /// <p>Indicates whether to substitute a solid color or a slate into the output after input loss exceeds blackFrameMsec.</p>
    #[serde(rename = "inputLossImageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_image_type: Option<String>,
    /// <p>Documentation update needed</p>
    #[serde(rename = "repeatFrameMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_frame_msec: Option<i64>,
}

/// <p>MediaLive will perform a failover if content is not detected in this input for the specified period.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputLossFailoverSettings {
    /// <p>The amount of time (in milliseconds) that no input is detected. After that time, an input failover will occur.</p>
    #[serde(rename = "inputLossThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_threshold_msec: Option<i64>,
}

/// <p>Action to prepare an input for a future immediate input switch.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputPrepareScheduleActionSettings {
    /// <p>The name of the input attachment that should be prepared by this action. If no name is provided, the action will stop the most recent prepare (if any) when activated.</p>
    #[serde(rename = "inputAttachmentNameReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachment_name_reference: Option<String>,
    /// <p>Settings to let you create a clip of the file input, in order to set up the input to ingest only a portion of the file.</p>
    #[serde(rename = "inputClippingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clipping_settings: Option<InputClippingSettings>,
    /// <p>The value for the variable portion of the URL for the dynamic input, for this instance of the input. Each time you use the same dynamic input in an input switch action, you can provide a different value, in order to connect the input to a different content source.</p>
    #[serde(rename = "urlPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<Vec<String>>,
}

/// <p>An Input Security Group</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputSecurityGroup {
    /// <p>Unique ARN of Input Security Group</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The Id of the Input Security Group</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The list of inputs currently using this Input Security Group.</p>
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    /// <p>The current state of the Input Security Group.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Whitelist rules and their sync status</p>
    #[serde(rename = "whitelistRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRule>>,
}

/// <p>Live Event input parameters. There can be multiple inputs in a single Live Event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputSettings {
    /// <p>Used to select the audio stream to decode for inputs that have multiple available.</p>
    #[serde(rename = "audioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<Vec<AudioSelector>>,
    /// <p>Used to select the caption input to use for inputs that have multiple available.</p>
    #[serde(rename = "captionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<Vec<CaptionSelector>>,
    /// <p>Enable or disable the deblock filter when filtering.</p>
    #[serde(rename = "deblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    /// <p>Enable or disable the denoise filter when filtering.</p>
    #[serde(rename = "denoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Adjusts the magnitude of filtering from 1 (minimal) to 5 (strongest).</p>
    #[serde(rename = "filterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>Turns on the filter for this input. MPEG-2 inputs have the deblocking filter enabled by default.
    /// 1) auto - filtering will be applied depending on input type/quality
    /// 2) disabled - no filtering will be applied to the input
    /// 3) forced - filtering will be applied regardless of input type</p>
    #[serde(rename = "inputFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_filter: Option<String>,
    /// <p>Input settings.</p>
    #[serde(rename = "networkInputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_input_settings: Option<NetworkInputSettings>,
    /// <p>Specifies whether to extract applicable ancillary data from a SMPTE-2038 source in this input. Applicable data types are captions, timecode, AFD, and SCTE-104 messages.
    /// - PREFER: Extract from SMPTE-2038 if present in this input, otherwise extract from another source (if any).
    /// - IGNORE: Never extract any ancillary data from SMPTE-2038.</p>
    #[serde(rename = "smpte2038DataPreference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smpte_2038_data_preference: Option<String>,
    /// <p>Loop input if it is a file. This allows a file input to be streamed indefinitely.</p>
    #[serde(rename = "sourceEndBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_end_behavior: Option<String>,
    /// <p>Informs which video elementary stream to decode for input types that have multiple available.</p>
    #[serde(rename = "videoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>The settings for a PULL type input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputSource {
    /// <p>The key used to extract the password from EC2 Parameter store.</p>
    #[serde(rename = "passwordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    /// <p>This represents the customer&#39;s source URL where stream is
    /// pulled from.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The username for the input source.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Settings for for a PULL type input.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputSourceRequest {
    /// <p>The key used to extract the password from EC2 Parameter store.</p>
    #[serde(rename = "passwordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    /// <p>This represents the customer&#39;s source URL where stream is
    /// pulled from.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>The username for the input source.</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Placeholder documentation for InputSpecification</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputSpecification {
    /// <p>Input codec</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Maximum input bitrate, categorized coarsely</p>
    #[serde(rename = "maximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    /// <p>Input resolution, categorized coarsely</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
}

/// <p>Settings for the &quot;switch input&quot; action: to switch from ingesting one input to ingesting another input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputSwitchScheduleActionSettings {
    /// <p>The name of the input attachment (not the name of the input!) to switch to. The name is specified in the channel configuration.</p>
    #[serde(rename = "inputAttachmentNameReference")]
    pub input_attachment_name_reference: String,
    /// <p>Settings to let you create a clip of the file input, in order to set up the input to ingest only a portion of the file.</p>
    #[serde(rename = "inputClippingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clipping_settings: Option<InputClippingSettings>,
    /// <p>The value for the variable portion of the URL for the dynamic input, for this instance of the input. Each time you use the same dynamic input in an input switch action, you can provide a different value, in order to connect the input to a different content source.</p>
    #[serde(rename = "urlPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_path: Option<Vec<String>>,
}

/// <p>Settings for a private VPC Input.
/// When this property is specified, the input destination addresses will be created in a VPC rather than with public Internet addresses.
/// This property requires setting the roleArn property on Input creation.
/// Not compatible with the inputSecurityGroups property.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputVpcRequest {
    /// <p>A list of up to 5 EC2 VPC security group IDs to attach to the Input VPC network interfaces.
    /// Requires subnetIds. If none are specified then the VPC default security group will be used.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of 2 VPC subnet IDs from the same VPC.
    /// Subnet IDs must be mapped to two unique availability zones (AZ).</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>Whitelist rule</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InputWhitelistRule {
    /// <p>The IPv4 CIDR that&#39;s whitelisted.</p>
    #[serde(rename = "cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>An IPv4 CIDR to whitelist.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct InputWhitelistRuleCidr {
    /// <p>The IPv4 CIDR to whitelist.</p>
    #[serde(rename = "cidr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cidr: Option<String>,
}

/// <p>Key Provider Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KeyProviderSettings {
    #[serde(rename = "staticKeySettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_settings: Option<StaticKeySettings>,
}

/// <p>Placeholder documentation for ListChannelsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChannelsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListChannelsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChannelsResponse {
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<Vec<ChannelSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputDeviceTransfersRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputDeviceTransfersRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "transferType")]
    pub transfer_type: String,
}

/// <p>Placeholder documentation for ListInputDeviceTransfersResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputDeviceTransfersResponse {
    /// <p>The list of devices that you are transferring or are being transferred to you.</p>
    #[serde(rename = "inputDeviceTransfers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_device_transfers: Option<Vec<TransferringInputDeviceSummary>>,
    /// <p>A token to get additional list results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputDevicesRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputDevicesRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputDevicesResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputDevicesResponse {
    /// <p>The list of input devices.</p>
    #[serde(rename = "inputDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceSummary>>,
    /// <p>A token to get additional list results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputSecurityGroupsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputSecurityGroupsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputSecurityGroupsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputSecurityGroupsResponse {
    /// <p>List of input security groups</p>
    #[serde(rename = "inputSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<InputSecurityGroup>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListInputsRequest {
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListInputsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListInputsResponse {
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListMultiplexProgramsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMultiplexProgramsRequest {
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The ID of the multiplex that the programs belong to.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The token to retrieve the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListMultiplexProgramsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMultiplexProgramsResponse {
    /// <p>List of multiplex programs.</p>
    #[serde(rename = "multiplexPrograms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_programs: Option<Vec<MultiplexProgramSummary>>,
    /// <p>Token for the next ListMultiplexProgram request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListMultiplexesRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListMultiplexesRequest {
    /// <p>The maximum number of items to return.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to retrieve the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListMultiplexesResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListMultiplexesResponse {
    /// <p>List of multiplexes.</p>
    #[serde(rename = "multiplexes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplexes: Option<Vec<MultiplexSummary>>,
    /// <p>Token for the next ListMultiplexes request.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Placeholder documentation for ListOfferingsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListOfferingsRequest {
    /// <p>Filter by channel class, &#39;STANDARD&#39; or &#39;SINGLE_PIPELINE&#39;</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>Filter to offerings that match the configuration of an existing channel, e.g. &#39;2345678&#39; (a channel ID)</p>
    #[serde(rename = "channelConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_configuration: Option<String>,
    /// <p>Filter by codec, &#39;AVC&#39;, &#39;HEVC&#39;, &#39;MPEG2&#39;, &#39;AUDIO&#39;, or &#39;LINK&#39;</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Filter by offering duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Filter by bitrate, &#39;MAX<em>10</em>MBPS&#39;, &#39;MAX<em>20</em>MBPS&#39;, or &#39;MAX<em>50</em>MBPS&#39;</p>
    #[serde(rename = "maximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    /// <p>Filter by framerate, &#39;MAX<em>30</em>FPS&#39; or &#39;MAX<em>60</em>FPS&#39;</p>
    #[serde(rename = "maximumFramerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filter by resolution, &#39;SD&#39;, &#39;HD&#39;, &#39;FHD&#39;, or &#39;UHD&#39;</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// <p>Filter by resource type, &#39;INPUT&#39;, &#39;OUTPUT&#39;, &#39;MULTIPLEX&#39;, or &#39;CHANNEL&#39;</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Filter by special feature, &#39;ADVANCED<em>AUDIO&#39; or &#39;AUDIO</em>NORMALIZATION&#39;</p>
    #[serde(rename = "specialFeature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    /// <p>Filter by video quality, &#39;STANDARD&#39;, &#39;ENHANCED&#39;, or &#39;PREMIUM&#39;</p>
    #[serde(rename = "videoQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

/// <p>Placeholder documentation for ListOfferingsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListOfferingsResponse {
    /// <p>Token to retrieve the next page of results</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of offerings</p>
    #[serde(rename = "offerings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offerings: Option<Vec<Offering>>,
}

/// <p>Placeholder documentation for ListReservationsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListReservationsRequest {
    /// <p>Filter by channel class, &#39;STANDARD&#39; or &#39;SINGLE_PIPELINE&#39;</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>Filter by codec, &#39;AVC&#39;, &#39;HEVC&#39;, &#39;MPEG2&#39;, &#39;AUDIO&#39;, or &#39;LINK&#39;</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Filter by bitrate, &#39;MAX<em>10</em>MBPS&#39;, &#39;MAX<em>20</em>MBPS&#39;, or &#39;MAX<em>50</em>MBPS&#39;</p>
    #[serde(rename = "maximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    /// <p>Filter by framerate, &#39;MAX<em>30</em>FPS&#39; or &#39;MAX<em>60</em>FPS&#39;</p>
    #[serde(rename = "maximumFramerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Filter by resolution, &#39;SD&#39;, &#39;HD&#39;, &#39;FHD&#39;, or &#39;UHD&#39;</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// <p>Filter by resource type, &#39;INPUT&#39;, &#39;OUTPUT&#39;, &#39;MULTIPLEX&#39;, or &#39;CHANNEL&#39;</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Filter by special feature, &#39;ADVANCED<em>AUDIO&#39; or &#39;AUDIO</em>NORMALIZATION&#39;</p>
    #[serde(rename = "specialFeature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    /// <p>Filter by video quality, &#39;STANDARD&#39;, &#39;ENHANCED&#39;, or &#39;PREMIUM&#39;</p>
    #[serde(rename = "videoQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

/// <p>Placeholder documentation for ListReservationsResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListReservationsResponse {
    /// <p>Token to retrieve the next page of results</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of reservations</p>
    #[serde(rename = "reservations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<Reservation>>,
}

/// <p>Placeholder documentation for ListTagsForResourceRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

/// <p>Placeholder documentation for ListTagsForResourceResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>M2ts Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct M2tsSettings {
    /// <p>When set to drop, output audio streams will be removed from the program if the selected input audio stream is removed from the input. This allows the output audio configuration to dynamically change based on input configuration. If this is set to encodeSilence, all output audio streams will output encoded silence when not connected to an active input stream.</p>
    #[serde(rename = "absentInputAudioBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absent_input_audio_behavior: Option<String>,
    /// <p>When set to enabled, uses ARIB-compliant field muxing and removes video descriptor.</p>
    #[serde(rename = "arib")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib: Option<String>,
    /// <p>Packet Identifier (PID) for ARIB Captions in the transport stream. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "aribCaptionsPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid: Option<String>,
    /// <p>If set to auto, pid number used for ARIB Captions will be auto-selected from unused pids.  If set to useConfigured, ARIB Captions will be on the configured pid number.</p>
    #[serde(rename = "aribCaptionsPidControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arib_captions_pid_control: Option<String>,
    /// <p>When set to dvb, uses DVB buffer model for Dolby Digital audio.  When set to atsc, the ATSC model is used.</p>
    #[serde(rename = "audioBufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "audioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary audio stream(s) in the transport stream. Multiple values are accepted, and can be entered in ranges and/or by comma separation. Can be entered as decimal or hexadecimal values. Each PID specified must be in the range of 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "audioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<String>,
    /// <p>When set to atsc, uses stream type = 0x81 for AC3 and stream type = 0x87 for EAC3. When set to dvb, uses stream type = 0x06.</p>
    #[serde(rename = "audioStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_stream_type: Option<String>,
    /// <p>The output bitrate of the transport stream in bits per second. Setting to 0 lets the muxer automatically determine the appropriate bitrate.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Controls the timing accuracy for output network traffic. Leave as MULTIPLEX to ensure accurate network packet timing. Or set to NONE, which might result in lower latency but will result in more variability in output network packet timing. This variability might cause interruptions, jitter, or bursty behavior in your playback or receiving devices.</p>
    #[serde(rename = "bufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<String>,
    /// <p>When set to enabled, generates captionServiceDescriptor in PMT.</p>
    #[serde(rename = "ccDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_descriptor: Option<String>,
    /// <p>Inserts DVB Network Information Table (NIT) at the specified table repetition interval.</p>
    #[serde(rename = "dvbNitSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    /// <p>Inserts DVB Service Description Table (SDT) at the specified table repetition interval.</p>
    #[serde(rename = "dvbSdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    /// <p>Packet Identifier (PID) for input source DVB Subtitle data to this output. Multiple values are accepted, and can be entered in ranges and/or by comma separation. Can be entered as decimal or hexadecimal values.  Each PID specified must be in the range of 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "dvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<String>,
    /// <p>Inserts DVB Time and Date Table (TDT) at the specified table repetition interval.</p>
    #[serde(rename = "dvbTdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    /// <p>Packet Identifier (PID) for input source DVB Teletext data to this output. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "dvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<String>,
    /// <p>If set to passthrough, passes any EBIF data from the input source to this output.</p>
    #[serde(rename = "ebif")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebif: Option<String>,
    /// <p>When videoAndFixedIntervals is selected, audio EBP markers will be added to partitions 3 and 4. The interval between these additional markers will be fixed, and will be slightly shorter than the video EBP marker interval. Only available when EBP Cablelabs segmentation markers are selected.  Partitions 1 and 2 will always follow the video interval.</p>
    #[serde(rename = "ebpAudioInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<String>,
    /// <p>When set, enforces that Encoder Boundary Points do not come within the specified time interval of each other by looking ahead at input video. If another EBP is going to come in within the specified time interval, the current EBP is not emitted, and the segment is &quot;stretched&quot; to the next marker.  The lookahead value does not add latency to the system. The Live Event must be configured elsewhere to create sufficient latency to make the lookahead accurate.</p>
    #[serde(rename = "ebpLookaheadMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_lookahead_ms: Option<i64>,
    /// <p>Controls placement of EBP on Audio PIDs. If set to videoAndAudioPids, EBP markers will be placed on the video PID and all audio PIDs.  If set to videoPid, EBP markers will be placed on only the video PID.</p>
    #[serde(rename = "ebpPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<String>,
    /// <p>This field is unused and deprecated.</p>
    #[serde(rename = "ecmPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<String>,
    /// <p>Include or exclude the ES Rate field in the PES header.</p>
    #[serde(rename = "esRateInPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    /// <p>Packet Identifier (PID) for input source ETV Platform data to this output. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "etvPlatformPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_platform_pid: Option<String>,
    /// <p>Packet Identifier (PID) for input source ETV Signal data to this output. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "etvSignalPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_signal_pid: Option<String>,
    /// <p>The length in seconds of each fragment. Only used with EBP markers.</p>
    #[serde(rename = "fragmentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,
    /// <p>If set to passthrough, passes any KLV data from the input source to this output.</p>
    #[serde(rename = "klv")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv: Option<String>,
    /// <p>Packet Identifier (PID) for input source KLV data to this output. Multiple values are accepted, and can be entered in ranges and/or by comma separation. Can be entered as decimal or hexadecimal values.  Each PID specified must be in the range of 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "klvDataPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<String>,
    /// <p>If set to passthrough, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3_behavior: Option<String>,
    /// <p>Value in bits per second of extra null packets to insert into the transport stream. This can be used if a downstream encryption system requires periodic null packets.</p>
    #[serde(rename = "nullPacketBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_packet_bitrate: Option<f64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.  Valid values are 0, 10..1000.</p>
    #[serde(rename = "patInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    /// <p>When set to pcrEveryPesPacket, a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This parameter is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
    #[serde(rename = "pcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Maximum time in milliseconds between Program Clock Reference (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "pcrPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i64>,
    /// <p>Packet Identifier (PID) of the Program Clock Reference (PCR) in the transport stream. When no value is given, the encoder will assign the same value as the Video PID. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "pcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream. Valid values are 0, 10..1000.</p>
    #[serde(rename = "pmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Packet Identifier (PID) for the Program Map Table (PMT) in the transport stream. Can be entered as a decimal or hexadecimal value. Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "pmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<String>,
    /// <p>The value of the program number field in the Program Map Table.</p>
    #[serde(rename = "programNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i64>,
    /// <p>When vbr, does not insert null packets into transport stream to fill specified bitrate. The bitrate setting acts as the maximum bitrate when vbr is set.</p>
    #[serde(rename = "rateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<String>,
    /// <p>Packet Identifier (PID) for input source SCTE-27 data to this output. Multiple values are accepted, and can be entered in ranges and/or by comma separation. Can be entered as decimal or hexadecimal values.  Each PID specified must be in the range of 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "scte27Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_27_pids: Option<String>,
    /// <p>Optionally pass SCTE-35 signals from the input source to this output.</p>
    #[serde(rename = "scte35Control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_control: Option<String>,
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<String>,
    /// <p>Inserts segmentation markers at each segmentationTime period. raiSegstart sets the Random Access Indicator bit in the adaptation field. raiAdapt sets the RAI bit and adds the current timecode in the private data bytes. psiSegstart inserts PAT and PMT tables at the start of segments. ebp adds Encoder Boundary Point information to the adaptation field as per OpenCable specification OC-SP-EBP-I01-130118. ebpLegacy adds Encoder Boundary Point information to the adaptation field using a legacy proprietary format.</p>
    #[serde(rename = "segmentationMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<String>,
    /// <p>The segmentation style parameter controls how segmentation markers are inserted into the transport stream. With avails, it is possible that segments may be truncated, which can influence where future segmentation markers are inserted.</p>
    ///
    /// <p>When a segmentation style of &quot;resetCadence&quot; is selected and a segment is truncated due to an avail, we will reset the segmentation cadence. This means the subsequent segment will have a duration of $segmentationTime seconds.</p>
    ///
    /// <p>When a segmentation style of &quot;maintainCadence&quot; is selected and a segment is truncated due to an avail, we will not reset the segmentation cadence. This means the subsequent segment will likely be truncated as well. However, all segments after that will have a duration of $segmentationTime seconds. Note that EBP lookahead is a slight exception to this rule.</p>
    #[serde(rename = "segmentationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<String>,
    /// <p>The length in seconds of each segment. Required unless markers is set to <em>none</em>.</p>
    #[serde(rename = "segmentationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,
    /// <p>When set to passthrough, timed metadata will be passed through from input to output.</p>
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
    /// <p>Packet Identifier (PID) of the timed metadata stream in the transport stream. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "timedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<String>,
    /// <p>The value of the transport stream ID field in the Program Map Table.</p>
    #[serde(rename = "transportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary video stream in the transport stream. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "videoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<String>,
}

/// <p>Settings information for the .m3u8 container</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct M3u8Settings {
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "audioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary audio stream(s) in the transport stream. Multiple values are accepted, and can be entered in ranges and/or by comma separation. Can be entered as decimal or hexadecimal values.</p>
    #[serde(rename = "audioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<String>,
    /// <p>This parameter is unused and deprecated.</p>
    #[serde(rename = "ecmPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecm_pid: Option<String>,
    /// <p>If set to passthrough, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "nielsenId3Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3_behavior: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream. A value of &quot;0&quot; writes out the PMT once per segment file.</p>
    #[serde(rename = "patInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    /// <p>When set to pcrEveryPesPacket, a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This parameter is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
    #[serde(rename = "pcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Maximum time in milliseconds between Program Clock References (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "pcrPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_period: Option<i64>,
    /// <p>Packet Identifier (PID) of the Program Clock Reference (PCR) in the transport stream. When no value is given, the encoder will assign the same value as the Video PID. Can be entered as a decimal or hexadecimal value.</p>
    #[serde(rename = "pcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream. A value of &quot;0&quot; writes out the PMT once per segment file.</p>
    #[serde(rename = "pmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Packet Identifier (PID) for the Program Map Table (PMT) in the transport stream. Can be entered as a decimal or hexadecimal value.</p>
    #[serde(rename = "pmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<String>,
    /// <p>The value of the program number field in the Program Map Table.</p>
    #[serde(rename = "programNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_num: Option<i64>,
    /// <p>If set to passthrough, passes any SCTE-35 signals from the input source to this output.</p>
    #[serde(rename = "scte35Behavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_behavior: Option<String>,
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream. Can be entered as a decimal or hexadecimal value.</p>
    #[serde(rename = "scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<String>,
    /// <p>When set to passthrough, timed metadata is passed through from input to output.</p>
    #[serde(rename = "timedMetadataBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_behavior: Option<String>,
    /// <p>Packet Identifier (PID) of the timed metadata stream in the transport stream. Can be entered as a decimal or hexadecimal value.  Valid values are 32 (or 0x20)..8182 (or 0x1ff6).</p>
    #[serde(rename = "timedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<String>,
    /// <p>The value of the transport stream ID field in the Program Map Table.</p>
    #[serde(rename = "transportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary video stream in the transport stream. Can be entered as a decimal or hexadecimal value.</p>
    #[serde(rename = "videoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<String>,
}

/// <p>The settings for a MediaConnect Flow.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MediaConnectFlow {
    /// <p>The unique ARN of the MediaConnect Flow being used as a source.</p>
    #[serde(rename = "flowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>The settings for a MediaConnect Flow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MediaConnectFlowRequest {
    /// <p>The ARN of the MediaConnect Flow that you want to use as a source.</p>
    #[serde(rename = "flowArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_arn: Option<String>,
}

/// <p>Media Package Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MediaPackageGroupSettings {
    /// <p>MediaPackage channel destination.</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
}

/// <p>MediaPackage Output Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MediaPackageOutputDestinationSettings {
    /// <p>ID of the channel in MediaPackage that is the destination for this output group. You do not need to specify the individual inputs in MediaPackage; MediaLive will handle the connection of the two MediaLive pipelines to the two MediaPackage inputs. The MediaPackage channel and MediaLive channel must be in the same region.</p>
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
}

/// <p>Media Package Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MediaPackageOutputSettings {}

/// <p>Settings to specify the rendering of motion graphics into the video stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionGraphicsActivateScheduleActionSettings {
    /// <p>Duration (in milliseconds) that motion graphics should render on to the video stream. Leaving out this property or setting to 0 will result in rendering continuing until a deactivate action is processed.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Key used to extract the password from EC2 Parameter store</p>
    #[serde(rename = "passwordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    /// <p>URI of the HTML5 content to be rendered into the live stream.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>Documentation update needed</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Motion Graphics Configuration</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionGraphicsConfiguration {
    #[serde(rename = "motionGraphicsInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_insertion: Option<String>,
    /// <p>Motion Graphics Settings</p>
    #[serde(rename = "motionGraphicsSettings")]
    pub motion_graphics_settings: MotionGraphicsSettings,
}

/// <p>Settings to specify the ending of rendering motion graphics into the video stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionGraphicsDeactivateScheduleActionSettings {}

/// <p>Motion Graphics Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionGraphicsSettings {
    #[serde(rename = "htmlMotionGraphicsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_motion_graphics_settings: Option<HtmlMotionGraphicsSettings>,
}

/// <p>Mp2 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mp2Settings {
    /// <p>Average bitrate in bits/second.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<f64>,
    /// <p>The MPEG2 Audio coding mode.  Valid values are codingMode10 (for mono) or codingMode20 (for stereo).</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

/// <p>Mpeg2 Filter Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mpeg2FilterSettings {
    #[serde(rename = "temporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<TemporalFilterSettings>,
}

/// <p>Mpeg2 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mpeg2Settings {
    /// <p>Choose Off to disable adaptive quantization. Or choose another value to enable the quantizer and set its strength. The strengths are: Auto, Off, Low, Medium, High. When you enable this field, MediaLive allows intra-frame quantizers to vary, which might improve visual quality.</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Indicates the AFD values that MediaLive will write into the video encode. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose AUTO.
    /// AUTO: MediaLive will try to preserve the input AFD value (in cases where multiple AFD values are valid).
    /// FIXED: MediaLive will use the value you specify in fixedAFD.</p>
    #[serde(rename = "afdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>Specifies whether to include the color space metadata. The metadata describes the color space that applies to the video (the colorSpace field). We recommend that you insert the metadata.</p>
    #[serde(rename = "colorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Choose the type of color space conversion to apply to the output. For detailed information on setting up both the input and the output to obtain the desired color space in the output, see the section on &quot;MediaLive Features - Video - color space&quot; in the MediaLive User Guide.
    /// PASSTHROUGH: Keep the color space of the input content - do not convert it.
    /// AUTO:Convert all content that is SD to rec 601, and convert all content that is HD to rec 709.</p>
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// <p>Sets the pixel aspect ratio for the encode.</p>
    #[serde(rename = "displayAspectRatio")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_aspect_ratio: Option<String>,
    /// <p>Optionally specify a noise reduction filter, which can improve quality of compressed content. If you do not choose a filter, no filter will be applied.
    /// TEMPORAL: This filter is useful for both source content that is noisy (when it has excessive digital artifacts) and source content that is clean.
    /// When the content is noisy, the filter cleans up the source content before the encoding phase, with these two effects: First, it improves the output video quality because the content has been cleaned up. Secondly, it decreases the bandwidth because MediaLive does not waste bits on encoding noise.
    /// When the content is reasonably clean, the filter tends to decrease the bitrate.</p>
    #[serde(rename = "filterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<Mpeg2FilterSettings>,
    /// <p>Complete this field only when afdSignaling is set to FIXED. Enter the AFD value (4 bits) to write on all frames of the video encode.</p>
    #[serde(rename = "fixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<String>,
    /// <p>description&quot;: &quot;The framerate denominator. For example, 1001. The framerate is the numerator divided by the denominator. For example, 24000 / 1001 = 23.976 FPS.</p>
    #[serde(rename = "framerateDenominator")]
    pub framerate_denominator: i64,
    /// <p>The framerate numerator. For example, 24000. The framerate is the numerator divided by the denominator. For example, 24000 / 1001 = 23.976 FPS.</p>
    #[serde(rename = "framerateNumerator")]
    pub framerate_numerator: i64,
    /// <p>MPEG2: default is open GOP.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>Relates to the GOP structure. The number of B-frames between reference frames. If you do not know what a B-frame is, use the default.</p>
    #[serde(rename = "gopNumBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_num_b_frames: Option<i64>,
    /// <p>Relates to the GOP structure. The GOP size (keyframe interval) in the units specified in gopSizeUnits. If you do not know what GOP is, use the default.
    /// If gopSizeUnits is frames, then the gopSize must be an integer and must be greater than or equal to 1.
    /// If gopSizeUnits is seconds, the gopSize must be greater than 0, but does not need to be an integer.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Relates to the GOP structure. Specifies whether the gopSize is specified in frames or seconds. If you do not plan to change the default gopSize, leave the default. If you specify SECONDS, MediaLive will internally convert the gop size to a frame count.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Set the scan type of the output to PROGRESSIVE or INTERLACED (top field first).</p>
    #[serde(rename = "scanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type: Option<String>,
    /// <p>Relates to the GOP structure. If you do not know what GOP is, use the default.
    /// FIXED: Set the number of B-frames in each sub-GOP to the value in gopNumBFrames.
    /// DYNAMIC: Let MediaLive optimize the number of B-frames in each sub-GOP, to improve visual quality.</p>
    #[serde(rename = "subgopLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subgop_length: Option<String>,
    /// <p>Determines how MediaLive inserts timecodes in the output video. For detailed information about setting up the input and the output for a timecode, see the section on &quot;MediaLive Features - Timecode configuration&quot; in the MediaLive User Guide.
    /// DISABLED: do not include timecodes.
    /// GOP_TIMECODE: Include timecode metadata in the GOP header.</p>
    #[serde(rename = "timecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
}

/// <p>Ms Smooth Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsSmoothGroupSettings {
    /// <p>The ID to include in each message in the sparse track. Ignored if sparseTrackType is NONE.</p>
    #[serde(rename = "acquisitionPointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acquisition_point_id: Option<String>,
    /// <p>If set to passthrough for an audio-only MS Smooth output, the fragment absolute time will be set to the current timecode. This option does not write timecodes to the audio elementary stream.</p>
    #[serde(rename = "audioOnlyTimecodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_timecode_control: Option<String>,
    /// <p>If set to verifyAuthenticity, verify the https certificate chain to a trusted Certificate Authority (CA).  This will cause https outputs to self-signed certificates to fail.</p>
    #[serde(rename = "certificateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    /// <p>Number of seconds to wait before retrying connection to the IIS server if the connection is lost. Content will be cached during this time and the cache will be be delivered to the IIS server once the connection is re-established.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>Smooth Streaming publish point on an IIS server. Elemental Live acts as a &quot;Push&quot; encoder to IIS.</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>MS Smooth event ID to be sent to the IIS server.</p>
    ///
    /// <p>Should only be specified if eventIdMode is set to useConfigured.</p>
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// <p>Specifies whether or not to send an event ID to the IIS server. If no event ID is sent and the same Live Event is used without changing the publishing point, clients might see cached video from the previous run.</p>
    ///
    /// <p>Options:
    /// - &quot;useConfigured&quot; - use the value provided in eventId
    /// - &quot;useTimestamp&quot; - generate and send an event ID based on the current timestamp
    /// - &quot;noEventId&quot; - do not send an event ID to the IIS server.</p>
    #[serde(rename = "eventIdMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id_mode: Option<String>,
    /// <p>When set to sendEos, send EOS signal to IIS server when stopping the event</p>
    #[serde(rename = "eventStopBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_stop_behavior: Option<String>,
    /// <p>Size in seconds of file cache for streaming outputs.</p>
    #[serde(rename = "filecacheDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filecache_duration: Option<i64>,
    /// <p>Length of mp4 fragments to generate (in seconds). Fragment length must be compatible with GOP size and framerate.</p>
    #[serde(rename = "fragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Parameter that control output group behavior on input loss.</p>
    #[serde(rename = "inputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    /// <p>Number of retry attempts.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
    /// <p>Number of seconds before initiating a restart due to output failure, due to exhausting the numRetries on one segment, or exceeding filecacheDuration.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
    /// <p>useInputSegmentation has been deprecated. The configured segment size is always used.</p>
    #[serde(rename = "segmentationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_mode: Option<String>,
    /// <p>Number of milliseconds to delay the output from the second pipeline.</p>
    #[serde(rename = "sendDelayMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_delay_ms: Option<i64>,
    /// <p>Identifies the type of data to place in the sparse track:
    /// - SCTE35: Insert SCTE-35 messages from the source content. With each message, insert an IDR frame to start a new segment.
    /// - SCTE35<em>WITHOUT</em>SEGMENTATION: Insert SCTE-35 messages from the source content. With each message, insert an IDR frame but don&#39;t start a new segment.
    /// - NONE: Don&#39;t generate a sparse track for any outputs in this output group.</p>
    #[serde(rename = "sparseTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sparse_track_type: Option<String>,
    /// <p>When set to send, send stream manifest so publishing point doesn&#39;t start until all streams start.</p>
    #[serde(rename = "streamManifestBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_manifest_behavior: Option<String>,
    /// <p>Timestamp offset for the event.  Only used if timestampOffsetMode is set to useConfiguredOffset.</p>
    #[serde(rename = "timestampOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset: Option<String>,
    /// <p>Type of timestamp date offset to use.
    /// - useEventStartDate: Use the date the event was started as the offset
    /// - useConfiguredOffset: Use an explicitly configured date as the offset</p>
    #[serde(rename = "timestampOffsetMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset_mode: Option<String>,
}

/// <p>Ms Smooth Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsSmoothOutputSettings {
    /// <p>Only applicable when this output is referencing an H.265 video description.
    /// Specifies whether MP4 segments should be packaged as HEV1 or HVC1.</p>
    #[serde(rename = "h265PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_packaging_type: Option<String>,
    /// <p>String concatenated to the end of the destination filename.  Required for multiple outputs of the same type.</p>
    #[serde(rename = "nameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
}

/// <p>The multiplex object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Multiplex {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of the multiplex output destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Multiplex Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexGroupSettings {}

/// <p>Multiplex MediaConnect output destination settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexMediaConnectOutputDestinationSettings {
    /// <p>The MediaConnect entitlement ARN available as a Flow source.</p>
    #[serde(rename = "entitlementArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entitlement_arn: Option<String>,
}

/// <p>Multiplex output destination settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexOutputDestination {
    /// <p>Multiplex MediaConnect output destination settings.</p>
    #[serde(rename = "mediaConnectSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_settings: Option<MultiplexMediaConnectOutputDestinationSettings>,
}

/// <p>Multiplex Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexOutputSettings {
    /// <p>Destination is a Multiplex.</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
}

/// <p>The multiplex program object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexProgram {
    /// <p>The MediaLive channel associated with the program.</p>
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The settings for this multiplex program.</p>
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    /// <p>The packet identifier map for this multiplex program.</p>
    #[serde(rename = "packetIdentifiersMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packet_identifiers_map: Option<MultiplexProgramPacketIdentifiersMap>,
    /// <p>Contains information about the current sources for the specified program in the specified multiplex. Keep in mind that each multiplex pipeline connects to both pipelines in a given source channel (the channel identified by the program). But only one of those channel pipelines is ever active at one time.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<MultiplexProgramPipelineDetail>>,
    /// <p>The name of the multiplex program.</p>
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

/// <p>Multiplex Program Input Destination Settings for outputting a Channel to a Multiplex</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexProgramChannelDestinationSettings {
    /// <p>The ID of the Multiplex that the encoder is providing output to. You do not need to specify the individual inputs to the Multiplex; MediaLive will handle the connection of the two MediaLive pipelines to the two Multiplex instances.
    /// The Multiplex must be in the same region as the Channel.</p>
    #[serde(rename = "multiplexId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_id: Option<String>,
    /// <p>The program name of the Multiplex program that the encoder is providing output to.</p>
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

/// <p>Packet identifiers map for a given Multiplex program.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexProgramPacketIdentifiersMap {
    #[serde(rename = "audioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    #[serde(rename = "dvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<Vec<i64>>,
    #[serde(rename = "dvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<i64>,
    #[serde(rename = "etvPlatformPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_platform_pid: Option<i64>,
    #[serde(rename = "etvSignalPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etv_signal_pid: Option<i64>,
    #[serde(rename = "klvDataPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klv_data_pids: Option<Vec<i64>>,
    #[serde(rename = "pcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    #[serde(rename = "pmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    #[serde(rename = "privateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    #[serde(rename = "scte27Pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_27_pids: Option<Vec<i64>>,
    #[serde(rename = "scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    #[serde(rename = "timedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    #[serde(rename = "videoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i64>,
}

/// <p>The current source for one of the pipelines in the multiplex.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexProgramPipelineDetail {
    /// <p>Identifies the channel pipeline that is currently active for the pipeline (identified by PipelineId) in the multiplex.</p>
    #[serde(rename = "activeChannelPipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_channel_pipeline: Option<String>,
    /// <p>Identifies a specific pipeline in the multiplex.</p>
    #[serde(rename = "pipelineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
}

/// <p>Transport stream service descriptor configuration for the Multiplex program.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexProgramServiceDescriptor {
    /// <p>Name of the provider.</p>
    #[serde(rename = "providerName")]
    pub provider_name: String,
    /// <p>Name of the service.</p>
    #[serde(rename = "serviceName")]
    pub service_name: String,
}

/// <p>Multiplex Program settings configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexProgramSettings {
    /// <p>Indicates which pipeline is preferred by the multiplex for program ingest.</p>
    #[serde(rename = "preferredChannelPipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_channel_pipeline: Option<String>,
    /// <p>Unique program number.</p>
    #[serde(rename = "programNumber")]
    pub program_number: i64,
    /// <p>Transport stream service descriptor configuration for the Multiplex program.</p>
    #[serde(rename = "serviceDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_descriptor: Option<MultiplexProgramServiceDescriptor>,
    /// <p>Program video settings configuration.</p>
    #[serde(rename = "videoSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_settings: Option<MultiplexVideoSettings>,
}

/// <p>Placeholder documentation for MultiplexProgramSummary</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexProgramSummary {
    /// <p>The MediaLive Channel associated with the program.</p>
    #[serde(rename = "channelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<String>,
    /// <p>The name of the multiplex program.</p>
    #[serde(rename = "programName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_name: Option<String>,
}

/// <p>Contains configuration for a Multiplex event</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexSettings {
    /// <p>Maximum video buffer delay in milliseconds.</p>
    #[serde(rename = "maximumVideoBufferDelayMilliseconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_video_buffer_delay_milliseconds: Option<i64>,
    /// <p>Transport stream bit rate.</p>
    #[serde(rename = "transportStreamBitrate")]
    pub transport_stream_bitrate: i64,
    /// <p>Transport stream ID.</p>
    #[serde(rename = "transportStreamId")]
    pub transport_stream_id: i64,
    /// <p>Transport stream reserved bit rate.</p>
    #[serde(rename = "transportStreamReservedBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_reserved_bitrate: Option<i64>,
}

/// <p>Contains summary configuration for a Multiplex event.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexSettingsSummary {
    /// <p>Transport stream bit rate.</p>
    #[serde(rename = "transportStreamBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_bitrate: Option<i64>,
}

/// <p>Statmux rate control settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexStatmuxVideoSettings {
    /// <p>Maximum statmux bitrate.</p>
    #[serde(rename = "maximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<i64>,
    /// <p>Minimum statmux bitrate.</p>
    #[serde(rename = "minimumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_bitrate: Option<i64>,
    /// <p>The purpose of the priority is to use a combination of the\nmultiplex rate control algorithm and the QVBR capability of the\nencoder to prioritize the video quality of some channels in a\nmultiplex over others.  Channels that have a higher priority will\nget higher video quality at the expense of the video quality of\nother channels in the multiplex with lower priority.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

/// <p>Placeholder documentation for MultiplexSummary</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MultiplexSummary {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettingsSummary>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The video configuration for each program in a multiplex.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MultiplexVideoSettings {
    /// <p>The constant bitrate configuration for the video encode.
    /// When this field is defined, StatmuxSettings must be undefined.</p>
    #[serde(rename = "constantBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_bitrate: Option<i64>,
    /// <p>Statmux rate control settings.
    /// When this field is defined, ConstantBitrate must be undefined.</p>
    #[serde(rename = "statmuxSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statmux_settings: Option<MultiplexStatmuxVideoSettings>,
}

/// <p>Network source to transcode. Must be accessible to the Elemental Live node that is running the live event through a network connection.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NetworkInputSettings {
    /// <p>Specifies HLS input settings when the uri is for a HLS manifest.</p>
    #[serde(rename = "hlsInputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_input_settings: Option<HlsInputSettings>,
    /// <p>Check HTTPS server certificates. When set to checkCryptographyOnly, cryptography in the certificate will be checked, but not the server&#39;s name. Certain subdomains (notably S3 buckets that use dots in the bucket name) do not strictly match the corresponding certificate&#39;s wildcard pattern and would otherwise cause the event to error. This setting is ignored for protocols that do not use https.</p>
    #[serde(rename = "serverValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_validation: Option<String>,
}

/// <p>Nielsen Configuration</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NielsenConfiguration {
    /// <p>Enter the Distributor ID assigned to your organization by Nielsen.</p>
    #[serde(rename = "distributorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor_id: Option<String>,
    /// <p>Enables Nielsen PCM to ID3 tagging</p>
    #[serde(rename = "nielsenPcmToId3Tagging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_pcm_to_id_3_tagging: Option<String>,
}

/// <p>Reserved resources available for purchase</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Offering {
    /// <p>Unique offering ARN, e.g. &#39;arn:aws:medialive:us-west-2:123456789012:offering:87654321&#39;</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. &#39;USD&#39;</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>Lease duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Units for duration, e.g. &#39;MONTHS&#39;</p>
    #[serde(rename = "durationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    /// <p>One-time charge for each reserved resource, e.g. &#39;0.0&#39; for a NO_UPFRONT offering</p>
    #[serde(rename = "fixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>Offering description, e.g. &#39;HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)&#39;</p>
    #[serde(rename = "offeringDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>Offering type, e.g. &#39;NO_UPFRONT&#39;</p>
    #[serde(rename = "offeringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    /// <p>AWS region, e.g. &#39;us-west-2&#39;</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Resource configuration details</p>
    #[serde(rename = "resourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    /// <p>Recurring usage charge for each reserved resource, e.g. &#39;157.0&#39;</p>
    #[serde(rename = "usagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Output settings. There can be multiple outputs within a group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Output {
    /// <p>The names of the AudioDescriptions used as audio sources for this output.</p>
    #[serde(rename = "audioDescriptionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_names: Option<Vec<String>>,
    /// <p>The names of the CaptionDescriptions used as caption sources for this output.</p>
    #[serde(rename = "captionDescriptionNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_description_names: Option<Vec<String>>,
    /// <p>The name used to identify an output.</p>
    #[serde(rename = "outputName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_name: Option<String>,
    /// <p>Output type-specific settings.</p>
    #[serde(rename = "outputSettings")]
    pub output_settings: OutputSettings,
    /// <p>The name of the VideoDescription used as the source for this output.</p>
    #[serde(rename = "videoDescriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description_name: Option<String>,
}

/// <p>Placeholder documentation for OutputDestination</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputDestination {
    /// <p>User-specified id. This is used in an output group or an output.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Destination settings for a MediaPackage output; one destination for both encoders.</p>
    #[serde(rename = "mediaPackageSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_settings: Option<Vec<MediaPackageOutputDestinationSettings>>,
    /// <p>Destination settings for a Multiplex output; one destination for both encoders.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexProgramChannelDestinationSettings>,
    /// <p>Destination settings for a standard output; one destination for each redundant encoder.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Vec<OutputDestinationSettings>>,
}

/// <p>Placeholder documentation for OutputDestinationSettings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputDestinationSettings {
    /// <p>key used to extract the password from EC2 Parameter store</p>
    #[serde(rename = "passwordParam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_param: Option<String>,
    /// <p>Stream name for RTMP destinations (URLs of type rtmp://)</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>A URL specifying a destination</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// <p>username for destination</p>
    #[serde(rename = "username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Output groups for this Live Event. Output groups contain information about where streams should be distributed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputGroup {
    /// <p>Custom output group name optionally defined by the user.  Only letters, numbers, and the underscore character allowed; only 32 characters allowed.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Settings associated with the output group.</p>
    #[serde(rename = "outputGroupSettings")]
    pub output_group_settings: OutputGroupSettings,
    #[serde(rename = "outputs")]
    pub outputs: Vec<Output>,
}

/// <p>Output Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputGroupSettings {
    #[serde(rename = "archiveGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_group_settings: Option<ArchiveGroupSettings>,
    #[serde(rename = "frameCaptureGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_group_settings: Option<FrameCaptureGroupSettings>,
    #[serde(rename = "hlsGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    #[serde(rename = "mediaPackageGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_group_settings: Option<MediaPackageGroupSettings>,
    #[serde(rename = "msSmoothGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,
    #[serde(rename = "multiplexGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_group_settings: Option<MultiplexGroupSettings>,
    #[serde(rename = "rtmpGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_group_settings: Option<RtmpGroupSettings>,
    #[serde(rename = "udpGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_group_settings: Option<UdpGroupSettings>,
}

/// <p>Reference to an OutputDestination ID defined in the channel</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputLocationRef {
    #[serde(rename = "destinationRefId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_ref_id: Option<String>,
}

/// <p>Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputSettings {
    #[serde(rename = "archiveOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_output_settings: Option<ArchiveOutputSettings>,
    #[serde(rename = "frameCaptureOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_output_settings: Option<FrameCaptureOutputSettings>,
    #[serde(rename = "hlsOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_output_settings: Option<HlsOutputSettings>,
    #[serde(rename = "mediaPackageOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_package_output_settings: Option<MediaPackageOutputSettings>,
    #[serde(rename = "msSmoothOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_output_settings: Option<MsSmoothOutputSettings>,
    #[serde(rename = "multiplexOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_output_settings: Option<MultiplexOutputSettings>,
    #[serde(rename = "rtmpOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtmp_output_settings: Option<RtmpOutputSettings>,
    #[serde(rename = "udpOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udp_output_settings: Option<UdpOutputSettings>,
}

/// <p>Pass Through Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PassThroughSettings {}

/// <p>Settings for the action to set pause state of a channel.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PauseStateScheduleActionSettings {
    #[serde(rename = "pipelines")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<Vec<PipelinePauseStateSettings>>,
}

/// <p>Runtime details of a pipeline when a channel is running.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PipelineDetail {
    /// <p>The name of the active input attachment currently being ingested by this pipeline.</p>
    #[serde(rename = "activeInputAttachmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input_attachment_name: Option<String>,
    /// <p>The name of the input switch schedule action that occurred most recently and that resulted in the switch to the current input attachment for this pipeline.</p>
    #[serde(rename = "activeInputSwitchActionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_input_switch_action_name: Option<String>,
    /// <p>The name of the motion graphics activate action that occurred most recently and that resulted in the current graphics URI for this pipeline.</p>
    #[serde(rename = "activeMotionGraphicsActionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_motion_graphics_action_name: Option<String>,
    /// <p>The current URI being used for HTML5 motion graphics for this pipeline.</p>
    #[serde(rename = "activeMotionGraphicsUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_motion_graphics_uri: Option<String>,
    /// <p>Pipeline ID</p>
    #[serde(rename = "pipelineId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_id: Option<String>,
}

/// <p>Settings for pausing a pipeline.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PipelinePauseStateSettings {
    /// <p>Pipeline ID to pause (&quot;PIPELINE<em>0&quot; or &quot;PIPELINE</em>1&quot;).</p>
    #[serde(rename = "pipelineId")]
    pub pipeline_id: String,
}

/// <p>Placeholder documentation for PurchaseOfferingRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseOfferingRequest {
    /// <p>Number of resources</p>
    #[serde(rename = "count")]
    pub count: i64,
    /// <p>Name for the new reservation</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Offering to purchase, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    pub offering_id: String,
    /// <p>Unique request ID to be specified. This is needed to prevent retries from creating multiple resources.</p>
    #[serde(rename = "requestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// <p>Requested reservation start time (UTC) in ISO-8601 format. The specified time must be between the first day of the current month and one year from now. If no value is given, the default is now.</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>A collection of key-value pairs</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Placeholder documentation for PurchaseOfferingResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PurchaseOfferingResponse {
    #[serde(rename = "reservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

/// <p>Raw Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RawSettings {}

/// <p>Rec601 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Rec601Settings {}

/// <p>Rec709 Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Rec709Settings {}

/// <p>Placeholder documentation for RejectInputDeviceTransferRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RejectInputDeviceTransferRequest {
    /// <p>The unique ID of the input device to reject. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
}

/// <p>Placeholder documentation for RejectInputDeviceTransferResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RejectInputDeviceTransferResponse {}

/// <p>Remix Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RemixSettings {
    /// <p>Mapping of input channels to output channels, with appropriate gain adjustments.</p>
    #[serde(rename = "channelMappings")]
    pub channel_mappings: Vec<AudioChannelMapping>,
    /// <p>Number of input channels to be used.</p>
    #[serde(rename = "channelsIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_in: Option<i64>,
    /// <p>Number of output channels to be produced.
    /// Valid values: 1, 2, 4, 6, 8</p>
    #[serde(rename = "channelsOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_out: Option<i64>,
}

/// <p>Reserved resources available to use</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Reservation {
    /// <p>Unique reservation ARN, e.g. &#39;arn:aws:medialive:us-west-2:123456789012:reservation:1234567&#39;</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Number of reserved resources</p>
    #[serde(rename = "count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Currency code for usagePrice and fixedPrice in ISO-4217 format, e.g. &#39;USD&#39;</p>
    #[serde(rename = "currencyCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    /// <p>Lease duration, e.g. &#39;12&#39;</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Units for duration, e.g. &#39;MONTHS&#39;</p>
    #[serde(rename = "durationUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_units: Option<String>,
    /// <p>Reservation UTC end date and time in ISO-8601 format, e.g. &#39;2019-03-01T00:00:00&#39;</p>
    #[serde(rename = "end")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
    /// <p>One-time charge for each reserved resource, e.g. &#39;0.0&#39; for a NO_UPFRONT offering</p>
    #[serde(rename = "fixedPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    /// <p>User specified reservation name</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Offering description, e.g. &#39;HD AVC output at 10-20 Mbps, 30 fps, and standard VQ in US West (Oregon)&#39;</p>
    #[serde(rename = "offeringDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_description: Option<String>,
    /// <p>Unique offering ID, e.g. &#39;87654321&#39;</p>
    #[serde(rename = "offeringId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_id: Option<String>,
    /// <p>Offering type, e.g. &#39;NO_UPFRONT&#39;</p>
    #[serde(rename = "offeringType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    /// <p>AWS region, e.g. &#39;us-west-2&#39;</p>
    #[serde(rename = "region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_id: Option<String>,
    /// <p>Resource configuration details</p>
    #[serde(rename = "resourceSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_specification: Option<ReservationResourceSpecification>,
    /// <p>Reservation UTC start date and time in ISO-8601 format, e.g. &#39;2018-03-01T00:00:00&#39;</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>Current state of reservation, e.g. &#39;ACTIVE&#39;</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Recurring usage charge for each reserved resource, e.g. &#39;157.0&#39;</p>
    #[serde(rename = "usagePrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

/// <p>Resource configuration (codec, resolution, bitrate, ...)</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationResourceSpecification {
    /// <p>Channel class, e.g. &#39;STANDARD&#39;</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>Codec, e.g. &#39;AVC&#39;</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Maximum bitrate, e.g. &#39;MAX<em>20</em>MBPS&#39;</p>
    #[serde(rename = "maximumBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_bitrate: Option<String>,
    /// <p>Maximum framerate, e.g. &#39;MAX<em>30</em>FPS&#39; (Outputs only)</p>
    #[serde(rename = "maximumFramerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_framerate: Option<String>,
    /// <p>Resolution, e.g. &#39;HD&#39;</p>
    #[serde(rename = "resolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<String>,
    /// <p>Resource type, &#39;INPUT&#39;, &#39;OUTPUT&#39;, &#39;MULTIPLEX&#39;, or &#39;CHANNEL&#39;</p>
    #[serde(rename = "resourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    /// <p>Special feature, e.g. &#39;AUDIO_NORMALIZATION&#39; (Channels only)</p>
    #[serde(rename = "specialFeature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_feature: Option<String>,
    /// <p>Video quality, e.g. &#39;STANDARD&#39; (Outputs only)</p>
    #[serde(rename = "videoQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality: Option<String>,
}

/// <p>Rtmp Caption Info Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RtmpCaptionInfoDestinationSettings {}

/// <p>Rtmp Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RtmpGroupSettings {
    /// <p>Choose the ad marker type for this output group. MediaLive will create a message based on the content of each SCTE-35 message, format it for that marker type, and insert it in the datastream.</p>
    #[serde(rename = "adMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    /// <p>Authentication scheme to use when connecting with CDN</p>
    #[serde(rename = "authenticationScheme")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_scheme: Option<String>,
    /// <p>Controls behavior when content cache fills up. If remote origin server stalls the RTMP connection and does not accept content fast enough the &#39;Media Cache&#39; will fill up. When the cache reaches the duration specified by cacheLength the cache will stop accepting new content. If set to disconnectImmediately, the RTMP output will force a disconnect. Clear the media cache, and reconnect after restartDelay seconds. If set to waitForServer, the RTMP output will wait up to 5 minutes to allow the origin server to begin accepting data again.</p>
    #[serde(rename = "cacheFullBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_full_behavior: Option<String>,
    /// <p>Cache length, in seconds, is used to calculate buffer size.</p>
    #[serde(rename = "cacheLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_length: Option<i64>,
    /// <p>Controls the types of data that passes to onCaptionInfo outputs.  If set to &#39;all&#39; then 608 and 708 carried DTVCC data will be passed.  If set to &#39;field1AndField2608&#39; then DTVCC data will be stripped out, but 608 data from both fields will be passed. If set to &#39;field1608&#39; then only the data carried in 608 from field 1 video will be passed.</p>
    #[serde(rename = "captionData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_data: Option<String>,
    /// <p>Controls the behavior of this RTMP group if input becomes unavailable.</p>
    ///
    /// <ul>
    /// <li>emitOutput: Emit a slate until input returns.</li>
    /// <li>pauseOutput: Stop transmitting data until input returns. This does not close the underlying RTMP connection.</li>
    /// </ul>
    #[serde(rename = "inputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    /// <p>If a streaming output fails, number of seconds to wait until a restart is initiated. A value of 0 means never restart.</p>
    #[serde(rename = "restartDelay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restart_delay: Option<i64>,
}

/// <p>Rtmp Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RtmpOutputSettings {
    /// <p>If set to verifyAuthenticity, verify the tls certificate chain to a trusted Certificate Authority (CA).  This will cause rtmps outputs with self-signed certificates to fail.</p>
    #[serde(rename = "certificateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_mode: Option<String>,
    /// <p>Number of seconds to wait before retrying a connection to the Flash Media server if the connection is lost.</p>
    #[serde(rename = "connectionRetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_retry_interval: Option<i64>,
    /// <p>The RTMP endpoint excluding the stream name (eg. rtmp://host/appname). For connection to Akamai, a username and password must be supplied. URI fields accept format identifiers.</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>Number of retry attempts.</p>
    #[serde(rename = "numRetries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_retries: Option<i64>,
}

/// <p>Contains information on a single schedule action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScheduleAction {
    /// <p>The name of the action, must be unique within the schedule. This name provides the main reference to an action once it is added to the schedule. A name is unique if it is no longer in the schedule. The schedule is automatically cleaned up to remove actions with a start time of more than 1 hour ago (approximately) so at that point a name can be reused.</p>
    #[serde(rename = "actionName")]
    pub action_name: String,
    /// <p>Settings for this schedule action.</p>
    #[serde(rename = "scheduleActionSettings")]
    pub schedule_action_settings: ScheduleActionSettings,
    /// <p>The time for the action to start in the channel.</p>
    #[serde(rename = "scheduleActionStartSettings")]
    pub schedule_action_start_settings: ScheduleActionStartSettings,
}

/// <p>Holds the settings for a single schedule action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScheduleActionSettings {
    /// <p>Action to insert HLS ID3 segment tagging</p>
    #[serde(rename = "hlsId3SegmentTaggingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_id_3_segment_tagging_settings: Option<HlsId3SegmentTaggingScheduleActionSettings>,
    /// <p>Action to insert HLS metadata</p>
    #[serde(rename = "hlsTimedMetadataSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_timed_metadata_settings: Option<HlsTimedMetadataScheduleActionSettings>,
    /// <p>Action to prepare an input for a future immediate input switch</p>
    #[serde(rename = "inputPrepareSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_prepare_settings: Option<InputPrepareScheduleActionSettings>,
    /// <p>Action to switch the input</p>
    #[serde(rename = "inputSwitchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_switch_settings: Option<InputSwitchScheduleActionSettings>,
    /// <p>Action to activate a motion graphics image overlay</p>
    #[serde(rename = "motionGraphicsImageActivateSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_image_activate_settings:
        Option<MotionGraphicsActivateScheduleActionSettings>,
    /// <p>Action to deactivate a motion graphics image overlay</p>
    #[serde(rename = "motionGraphicsImageDeactivateSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_graphics_image_deactivate_settings:
        Option<MotionGraphicsDeactivateScheduleActionSettings>,
    /// <p>Action to pause or unpause one or both channel pipelines</p>
    #[serde(rename = "pauseStateSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_state_settings: Option<PauseStateScheduleActionSettings>,
    /// <p>Action to insert SCTE-35 return<em>to</em>network message</p>
    #[serde(rename = "scte35ReturnToNetworkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_return_to_network_settings: Option<Scte35ReturnToNetworkScheduleActionSettings>,
    /// <p>Action to insert SCTE-35 splice_insert message</p>
    #[serde(rename = "scte35SpliceInsertSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_splice_insert_settings: Option<Scte35SpliceInsertScheduleActionSettings>,
    /// <p>Action to insert SCTE-35 time_signal message</p>
    #[serde(rename = "scte35TimeSignalSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_time_signal_settings: Option<Scte35TimeSignalScheduleActionSettings>,
    /// <p>Action to activate a static image overlay</p>
    #[serde(rename = "staticImageActivateSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_activate_settings: Option<StaticImageActivateScheduleActionSettings>,
    /// <p>Action to deactivate a static image overlay</p>
    #[serde(rename = "staticImageDeactivateSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_image_deactivate_settings: Option<StaticImageDeactivateScheduleActionSettings>,
}

/// <p>Settings to specify when an action should occur. Only one of the options must be selected.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScheduleActionStartSettings {
    /// <p>Option for specifying the start time for an action.</p>
    #[serde(rename = "fixedModeScheduleActionStartSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_mode_schedule_action_start_settings: Option<FixedModeScheduleActionStartSettings>,
    /// <p>Option for specifying an action as relative to another action.</p>
    #[serde(rename = "followModeScheduleActionStartSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow_mode_schedule_action_start_settings: Option<FollowModeScheduleActionStartSettings>,
    /// <p>Option for specifying an action that should be applied immediately.</p>
    #[serde(rename = "immediateModeScheduleActionStartSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immediate_mode_schedule_action_start_settings:
        Option<ImmediateModeScheduleActionStartSettings>,
}

/// <p>Scte20 Plus Embedded Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte20PlusEmbeddedDestinationSettings {}

/// <p>Scte20 Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte20SourceSettings {
    /// <p>If upconvert, 608 data is both passed through via the &quot;608 compatibility bytes&quot; fields of the 708 wrapper as well as translated into 708. 708 data present in the source content will be discarded.</p>
    #[serde(rename = "convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_channel_number: Option<i64>,
}

/// <p>Scte27 Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte27DestinationSettings {}

/// <p>Scte27 Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte27SourceSettings {
    /// <p>If you will configure a WebVTT caption description that references this caption selector, use this field to
    /// provide the language to consider when translating the image-based source to text.</p>
    #[serde(rename = "ocrLanguage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocr_language: Option<String>,
    /// <p>The pid field is used in conjunction with the caption selector languageCode field as follows:
    /// - Specify PID and Language: Extracts captions from that PID; the language is &quot;informational&quot;.
    /// - Specify PID and omit Language: Extracts the specified PID.
    /// - Omit PID and specify Language: Extracts the specified language, whichever PID that happens to be.
    /// - Omit PID and omit Language: Valid only if source is DVB-Sub that is being passed through; all languages will be passed through.</p>
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

/// <p>Corresponds to SCTE-35 delivery<em>not</em>restricted_flag parameter. To declare delivery restrictions, include this element and its four &quot;restriction&quot; flags. To declare that there are no restrictions, omit this element.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35DeliveryRestrictions {
    /// <p>Corresponds to SCTE-35 archive<em>allowed</em>flag.</p>
    #[serde(rename = "archiveAllowedFlag")]
    pub archive_allowed_flag: String,
    /// <p>Corresponds to SCTE-35 device_restrictions parameter.</p>
    #[serde(rename = "deviceRestrictions")]
    pub device_restrictions: String,
    /// <p>Corresponds to SCTE-35 no<em>regional</em>blackout_flag parameter.</p>
    #[serde(rename = "noRegionalBlackoutFlag")]
    pub no_regional_blackout_flag: String,
    /// <p>Corresponds to SCTE-35 web<em>delivery</em>allowed_flag parameter.</p>
    #[serde(rename = "webDeliveryAllowedFlag")]
    pub web_delivery_allowed_flag: String,
}

/// <p>Holds one set of SCTE-35 Descriptor Settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35Descriptor {
    /// <p>SCTE-35 Descriptor Settings.</p>
    #[serde(rename = "scte35DescriptorSettings")]
    pub scte_35_descriptor_settings: Scte35DescriptorSettings,
}

/// <p>SCTE-35 Descriptor settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35DescriptorSettings {
    /// <p>SCTE-35 Segmentation Descriptor.</p>
    #[serde(rename = "segmentationDescriptorScte35DescriptorSettings")]
    pub segmentation_descriptor_scte_35_descriptor_settings: Scte35SegmentationDescriptor,
}

/// <p>Settings for a SCTE-35 return<em>to</em>network message.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35ReturnToNetworkScheduleActionSettings {
    /// <p>The splice<em>event</em>id for the SCTE-35 splice_insert, as defined in SCTE-35.</p>
    #[serde(rename = "spliceEventId")]
    pub splice_event_id: i64,
}

/// <p>Corresponds to SCTE-35 segmentation_descriptor.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35SegmentationDescriptor {
    /// <p>Holds the four SCTE-35 delivery restriction parameters.</p>
    #[serde(rename = "deliveryRestrictions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_restrictions: Option<Scte35DeliveryRestrictions>,
    /// <p>Corresponds to SCTE-35 segment<em>num. A value that is valid for the specified segmentation</em>type_id.</p>
    #[serde(rename = "segmentNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_num: Option<i64>,
    /// <p>Corresponds to SCTE-35 segmentation<em>event</em>cancel_indicator.</p>
    #[serde(rename = "segmentationCancelIndicator")]
    pub segmentation_cancel_indicator: String,
    /// <p>Corresponds to SCTE-35 segmentation<em>duration. Optional. The duration for the time</em>signal, in 90 KHz ticks. To convert seconds to ticks, multiple the seconds by 90,000. Enter time in 90 KHz clock ticks. If you do not enter a duration, the time_signal will continue until you insert a cancellation message.</p>
    #[serde(rename = "segmentationDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_duration: Option<i64>,
    /// <p>Corresponds to SCTE-35 segmentation<em>event</em>id. </p>
    #[serde(rename = "segmentationEventId")]
    pub segmentation_event_id: i64,
    /// <p>Corresponds to SCTE-35 segmentation<em>type</em>id. One of the segmentation<em>type</em>id values listed in the SCTE-35 specification. On the console, enter the ID in decimal (for example, &quot;52&quot;). In the CLI, API, or an SDK, enter the ID in hex (for example, &quot;0x34&quot;) or decimal (for example, &quot;52&quot;).</p>
    #[serde(rename = "segmentationTypeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_type_id: Option<i64>,
    /// <p>Corresponds to SCTE-35 segmentation<em>upid. Enter a string containing the hexadecimal representation of the characters that make up the SCTE-35 segmentation</em>upid value. Must contain an even number of hex characters. Do not include spaces between each hex pair. For example, the ASCII &quot;ADS Information&quot; becomes hex &quot;41445320496e666f726d6174696f6e.</p>
    #[serde(rename = "segmentationUpid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_upid: Option<String>,
    /// <p>Corresponds to SCTE-35 segmentation<em>upid</em>type. On the console, enter one of the types listed in the SCTE-35 specification, converted to a decimal. For example, &quot;0x0C&quot; hex from the specification is &quot;12&quot; in decimal. In the CLI, API, or an SDK, enter one of the types listed in the SCTE-35 specification, in either hex (for example, &quot;0x0C&quot; ) or in decimal (for example, &quot;12&quot;).</p>
    #[serde(rename = "segmentationUpidType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_upid_type: Option<i64>,
    /// <p>Corresponds to SCTE-35 segments<em>expected. A value that is valid for the specified segmentation</em>type_id.</p>
    #[serde(rename = "segmentsExpected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments_expected: Option<i64>,
    /// <p>Corresponds to SCTE-35 sub<em>segment</em>num. A value that is valid for the specified segmentation<em>type</em>id.</p>
    #[serde(rename = "subSegmentNum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_segment_num: Option<i64>,
    /// <p>Corresponds to SCTE-35 sub<em>segments</em>expected. A value that is valid for the specified segmentation<em>type</em>id.</p>
    #[serde(rename = "subSegmentsExpected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_segments_expected: Option<i64>,
}

/// <p>Scte35 Splice Insert</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35SpliceInsert {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time. This only applies to embedded SCTE 104/35 messages and does not apply to OOB messages.</p>
    #[serde(rename = "adAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>When set to ignore, Segment Descriptors with noRegionalBlackoutFlag set to 0 will no longer trigger blackouts or Ad Avail slates</p>
    #[serde(rename = "noRegionalBlackoutFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<String>,
    /// <p>When set to ignore, Segment Descriptors with webDeliveryAllowedFlag set to 0 will no longer trigger blackouts or Ad Avail slates</p>
    #[serde(rename = "webDeliveryAllowedFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<String>,
}

/// <p>Settings for a SCTE-35 splice_insert message.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35SpliceInsertScheduleActionSettings {
    /// <p>Optional, the duration for the splice<em>insert, in 90 KHz ticks. To convert seconds to ticks, multiple the seconds by 90,000. If you enter a duration, there is an expectation that the downstream system can read the duration and cue in at that time. If you do not enter a duration, the splice</em>insert will continue indefinitely and there is an expectation that you will enter a return<em>to</em>network to end the splice_insert at the appropriate time.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The splice<em>event</em>id for the SCTE-35 splice_insert, as defined in SCTE-35.</p>
    #[serde(rename = "spliceEventId")]
    pub splice_event_id: i64,
}

/// <p>Scte35 Time Signal Apos</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35TimeSignalApos {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time. This only applies to embedded SCTE 104/35 messages and does not apply to OOB messages.</p>
    #[serde(rename = "adAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>When set to ignore, Segment Descriptors with noRegionalBlackoutFlag set to 0 will no longer trigger blackouts or Ad Avail slates</p>
    #[serde(rename = "noRegionalBlackoutFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_regional_blackout_flag: Option<String>,
    /// <p>When set to ignore, Segment Descriptors with webDeliveryAllowedFlag set to 0 will no longer trigger blackouts or Ad Avail slates</p>
    #[serde(rename = "webDeliveryAllowedFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_delivery_allowed_flag: Option<String>,
}

/// <p>Settings for a SCTE-35 time_signal.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Scte35TimeSignalScheduleActionSettings {
    /// <p>The list of SCTE-35 descriptors accompanying the SCTE-35 time_signal.</p>
    #[serde(rename = "scte35Descriptors")]
    pub scte_35_descriptors: Vec<Scte35Descriptor>,
}

/// <p>Smpte Tt Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SmpteTtDestinationSettings {}

/// <p>Standard Hls Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StandardHlsSettings {
    /// <p>List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by &#39;,&#39;.</p>
    #[serde(rename = "audioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    #[serde(rename = "m3u8Settings")]
    pub m_3u_8_settings: M3u8Settings,
}

/// <p>Placeholder documentation for StartChannelRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartChannelRequest {
    /// <p>A request to start a channel</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// <p>Placeholder documentation for StartChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartChannelResponse {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Runtime details for the pipelines of a running channel.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Placeholder documentation for StartMultiplexRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMultiplexRequest {
    /// <p>The ID of the multiplex.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
}

/// <p>Placeholder documentation for StartMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartMultiplexResponse {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of the multiplex output destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Settings to identify the start of the clip.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StartTimecode {
    /// <p>The timecode for the frame where you want to start the clip. Optional; if not specified, the clip starts at first frame in the file. Enter the timecode as HH:MM:SS:FF or HH:MM:SS;FF.</p>
    #[serde(rename = "timecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

/// <p>Settings for the action to activate a static image.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StaticImageActivateScheduleActionSettings {
    /// <p>The duration in milliseconds for the image to remain on the video. If omitted or set to 0 the duration is unlimited and the image will remain until it is explicitly deactivated.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The time in milliseconds for the image to fade in. The fade-in starts at the start time of the overlay. Default is 0 (no fade-in).</p>
    #[serde(rename = "fadeIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i64>,
    /// <p>Applies only if a duration is specified. The time in milliseconds for the image to fade out. The fade-out starts when the duration time is hit, so it effectively extends the duration. Default is 0 (no fade-out).</p>
    #[serde(rename = "fadeOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i64>,
    /// <p>The height of the image when inserted into the video, in pixels. The overlay will be scaled up or down to the specified height. Leave blank to use the native height of the overlay.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The location and filename of the image file to overlay on the video. The file must be a 32-bit BMP, PNG, or TGA file, and must not be larger (in pixels) than the input video.</p>
    #[serde(rename = "image")]
    pub image: InputLocation,
    /// <p>Placement of the left edge of the overlay relative to the left edge of the video frame, in pixels. 0 (the default) is the left edge of the frame. If the placement causes the overlay to extend beyond the right edge of the underlying video, then the overlay is cropped on the right.</p>
    #[serde(rename = "imageX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i64>,
    /// <p>Placement of the top edge of the overlay relative to the top edge of the video frame, in pixels. 0 (the default) is the top edge of the frame. If the placement causes the overlay to extend beyond the bottom edge of the underlying video, then the overlay is cropped on the bottom.</p>
    #[serde(rename = "imageY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i64>,
    /// <p>The number of the layer, 0 to 7. There are 8 layers that can be overlaid on the video, each layer with a different image. The layers are in Z order, which means that overlays with higher values of layer are inserted on top of overlays with lower values of layer. Default is 0.</p>
    #[serde(rename = "layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i64>,
    /// <p>Opacity of image where 0 is transparent and 100 is fully opaque. Default is 100.</p>
    #[serde(rename = "opacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i64>,
    /// <p>The width of the image when inserted into the video, in pixels. The overlay will be scaled up or down to the specified width. Leave blank to use the native width of the overlay.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Settings for the action to deactivate the image in a specific layer.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StaticImageDeactivateScheduleActionSettings {
    /// <p>The time in milliseconds for the image to fade out. Default is 0 (no fade-out).</p>
    #[serde(rename = "fadeOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i64>,
    /// <p>The image overlay layer to deactivate, 0 to 7. Default is 0.</p>
    #[serde(rename = "layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i64>,
}

/// <p>Static Key Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StaticKeySettings {
    /// <p>The URL of the license server used for protecting content.</p>
    #[serde(rename = "keyProviderServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_provider_server: Option<InputLocation>,
    /// <p>Static key value as a 32 character hexadecimal string.</p>
    #[serde(rename = "staticKeyValue")]
    pub static_key_value: String,
}

/// <p>Placeholder documentation for StopChannelRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopChannelRequest {
    /// <p>A request to stop a running channel</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
}

/// <p>Placeholder documentation for StopChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopChannelResponse {
    /// <p>The unique arn of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>The class for this channel. STANDARD for a channel with two pipelines or SINGLE_PIPELINE for a channel with one pipeline.</p>
    #[serde(rename = "channelClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_class: Option<String>,
    /// <p>A list of destinations of the channel. For UDP outputs, there is one
    /// destination per output. For other types (HLS, for example), there is
    /// one destination per packager.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The endpoints where outgoing connections initiate from</p>
    #[serde(rename = "egressEndpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub egress_endpoints: Option<Vec<ChannelEgressEndpoint>>,
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    /// <p>The unique id of the channel.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>List of input attachments for channel.</p>
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level being written to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel. (user-mutable)</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Runtime details for the pipelines of a running channel.</p>
    #[serde(rename = "pipelineDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_details: Option<Vec<PipelineDetail>>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the role assumed when running the Channel.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Settings for VPC output</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<VpcOutputSettingsDescription>,
}

/// <p>Placeholder documentation for StopMultiplexRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopMultiplexRequest {
    /// <p>The ID of the multiplex.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
}

/// <p>Placeholder documentation for StopMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopMultiplexResponse {
    /// <p>The unique arn of the multiplex.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>A list of availability zones for the multiplex.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of the multiplex output destinations.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<MultiplexOutputDestination>>,
    /// <p>The unique id of the multiplex.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Configuration for a multiplex event.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>The name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The number of currently healthy pipelines.</p>
    #[serde(rename = "pipelinesRunningCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipelines_running_count: Option<i64>,
    /// <p>The number of programs in the multiplex.</p>
    #[serde(rename = "programCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_count: Option<i64>,
    /// <p>The current state of the multiplex.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Settings to identify the end of the clip.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StopTimecode {
    /// <p>If you specify a StopTimecode in an input (in order to clip the file), you can specify if you want the clip to exclude (the default) or include the frame specified by the timecode.</p>
    #[serde(rename = "lastFrameClippingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_frame_clipping_behavior: Option<String>,
    /// <p>The timecode for the frame where you want to stop the clip. Optional; if not specified, the clip continues to the end of the file. Enter the timecode as HH:MM:SS:FF or HH:MM:SS;FF.</p>
    #[serde(rename = "timecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

/// <p>Teletext Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TeletextDestinationSettings {}

/// <p>Teletext Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TeletextSourceSettings {
    /// <p>Optionally defines a region where TTML style captions will be displayed</p>
    #[serde(rename = "outputRectangle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_rectangle: Option<CaptionRectangle>,
    /// <p>Specifies the teletext page number within the data stream from which to extract captions. Range of 0x100 (256) to 0x8FF (2303). Unused for passthrough. Should be specified as a hexadecimal string with no &quot;0x&quot; prefix.</p>
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

/// <p>Temporal Filter Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TemporalFilterSettings {
    /// <p>If you enable this filter, the results are the following:
    /// - If the source content is noisy (it contains excessive digital artifacts), the filter cleans up the source.
    /// - If the source content is already clean, the filter tends to decrease the bitrate, especially when the rate control mode is QVBR.</p>
    #[serde(rename = "postFilterSharpening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpening: Option<String>,
    /// <p>Choose a filter strength. We recommend a strength of 1 or 2. A higher strength might take out good information, resulting in an image that is overly soft.</p>
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
}

/// <p>Timecode Config</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimecodeConfig {
    /// <p>Identifies the source for the timecode that will be associated with the events outputs.
    /// -Embedded (embedded): Initialize the output timecode with timecode from the the source.  If no embedded timecode is detected in the source, the system falls back to using &quot;Start at 0&quot; (zerobased).
    /// -System Clock (systemclock): Use the UTC time.
    /// -Start at 0 (zerobased): The time of the first frame of the event will be 00:00:00:00.</p>
    #[serde(rename = "source")]
    pub source: String,
    /// <p>Threshold in frames beyond which output timecode is resynchronized to the input timecode. Discrepancies below this threshold are permitted to avoid unnecessary discontinuities in the output timecode. No timecode sync when this is not specified.</p>
    #[serde(rename = "syncThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_threshold: Option<i64>,
}

/// <p>A request to transfer an input device.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TransferInputDeviceRequest {
    /// <p>The unique ID of this input device. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
    /// <p>The AWS account ID (12 digits) for the recipient of the device transfer.</p>
    #[serde(rename = "targetCustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_customer_id: Option<String>,
    /// <p>The target AWS region to transfer the device.</p>
    #[serde(rename = "targetRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_region: Option<String>,
    /// <p>An optional message for the recipient. Maximum 280 characters.</p>
    #[serde(rename = "transferMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_message: Option<String>,
}

/// <p>Placeholder documentation for TransferInputDeviceResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransferInputDeviceResponse {}

/// <p>Details about the input device that is being transferred.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TransferringInputDeviceSummary {
    /// <p>The unique ID of the input device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The optional message that the sender has attached to the transfer.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The AWS account ID for the recipient of the input device transfer.</p>
    #[serde(rename = "targetCustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_customer_id: Option<String>,
    /// <p>The type (direction) of the input device transfer.</p>
    #[serde(rename = "transferType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_type: Option<String>,
}

/// <p>Ttml Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TtmlDestinationSettings {
    /// <p>When set to passthrough, passes through style and position information from a TTML-like input source (TTML, SMPTE-TT, CFF-TT) to the CFF-TT output or TTML output.</p>
    #[serde(rename = "styleControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_control: Option<String>,
}

/// <p>Udp Container Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UdpContainerSettings {
    #[serde(rename = "m2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_2ts_settings: Option<M2tsSettings>,
}

/// <p>Udp Group Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UdpGroupSettings {
    /// <p>Specifies behavior of last resort when input video is lost, and no more backup inputs are available. When dropTs is selected the entire transport stream will stop being emitted.  When dropProgram is selected the program can be dropped from the transport stream (and replaced with null packets to meet the TS bitrate requirement).  Or, when emitProgram is chosen the transport stream will continue to be produced normally with repeat frames, black frames, or slate frames substituted for the absent input video.</p>
    #[serde(rename = "inputLossAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_loss_action: Option<String>,
    /// <p>Indicates ID3 frame that has the timecode.</p>
    #[serde(rename = "timedMetadataId3Frame")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_frame: Option<String>,
    /// <p>Timed Metadata interval in seconds.</p>
    #[serde(rename = "timedMetadataId3Period")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_id_3_period: Option<i64>,
}

/// <p>Udp Output Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct UdpOutputSettings {
    /// <p>UDP output buffering in milliseconds. Larger values increase latency through the transcoder but simultaneously assist the transcoder in maintaining a constant, low-jitter UDP/RTP output while accommodating clock recovery, input switching, input disruptions, picture reordering, etc.</p>
    #[serde(rename = "bufferMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_msec: Option<i64>,
    #[serde(rename = "containerSettings")]
    pub container_settings: UdpContainerSettings,
    /// <p>Destination address and port number for RTP or UDP packets. Can be unicast or multicast RTP or UDP (eg. rtp://239.10.10.10:5001 or udp://10.100.100.100:5002).</p>
    #[serde(rename = "destination")]
    pub destination: OutputLocationRef,
    /// <p>Settings for enabling and adjusting Forward Error Correction on UDP outputs.</p>
    #[serde(rename = "fecOutputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fec_output_settings: Option<FecOutputSettings>,
}

/// <p>Channel class that the channel should be updated to.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateChannelClassRequest {
    /// <p>The channel class that you wish to update this channel to use.</p>
    #[serde(rename = "channelClass")]
    pub channel_class: String,
    /// <p>Channel Id of the channel whose class should be updated.</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// <p>A list of output destinations for this channel.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
}

/// <p>Placeholder documentation for UpdateChannelClassResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChannelClassResponse {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

/// <p>A request to update a channel.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateChannelRequest {
    /// <p>Specification of CDI inputs for this channel</p>
    #[serde(rename = "cdiInputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdi_input_specification: Option<CdiInputSpecification>,
    /// <p>channel ID</p>
    #[serde(rename = "channelId")]
    pub channel_id: String,
    /// <p>A list of output destinations for this channel.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<OutputDestination>>,
    /// <p>The encoder settings for this channel.</p>
    #[serde(rename = "encoderSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoder_settings: Option<EncoderSettings>,
    #[serde(rename = "inputAttachments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_attachments: Option<Vec<InputAttachment>>,
    /// <p>Specification of network and file inputs for this channel</p>
    #[serde(rename = "inputSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_specification: Option<InputSpecification>,
    /// <p>The log level to write to CloudWatch Logs.</p>
    #[serde(rename = "logLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_level: Option<String>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An optional Amazon Resource Name (ARN) of the role to assume when running the Channel. If you do not specify this on an update call but the role was previously set that role will be removed.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Placeholder documentation for UpdateChannelResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateChannelResponse {
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
}

/// <p>A request to update an input device.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInputDeviceRequest {
    /// <p>The settings that you want to apply to the HD input device.</p>
    #[serde(rename = "hdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceConfigurableSettings>,
    /// <p>The unique ID of the input device. For example, hd-123456789abcdef.</p>
    #[serde(rename = "inputDeviceId")]
    pub input_device_id: String,
    /// <p>The name that you assigned to this input device (not the unique ID).</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The settings that you want to apply to the UHD input device.</p>
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceConfigurableSettings>,
}

/// <p>Placeholder documentation for UpdateInputDeviceResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInputDeviceResponse {
    /// <p>The unique ARN of the input device.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The state of the connection between the input device and AWS.</p>
    #[serde(rename = "connectionState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    /// <p>The status of the action to synchronize the device configuration. If you change the configuration of the input device (for example, the maximum bitrate), MediaLive sends the new data to the device. The device might not update itself immediately. SYNCED means the device has updated its configuration. SYNCING means that it has not updated its configuration.</p>
    #[serde(rename = "deviceSettingsSyncState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_settings_sync_state: Option<String>,
    /// <p>The status of software on the input device.</p>
    #[serde(rename = "deviceUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_update_status: Option<String>,
    /// <p>Settings that describe an input device that is type HD.</p>
    #[serde(rename = "hdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hd_device_settings: Option<InputDeviceHdSettings>,
    /// <p>The unique ID of the input device.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The network MAC address of the input device.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>A name that you specify for the input device.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The network settings for the input device.</p>
    #[serde(rename = "networkSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_settings: Option<InputDeviceNetworkSettings>,
    /// <p>The unique serial number of the input device.</p>
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// <p>The type of the input device.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>Settings that describe an input device that is type UHD.</p>
    #[serde(rename = "uhdDeviceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uhd_device_settings: Option<InputDeviceUhdSettings>,
}

/// <p>A request to update an input.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInputRequest {
    /// <p>Destination settings for PUSH type inputs.</p>
    #[serde(rename = "destinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<InputDestinationRequest>>,
    /// <p>Settings for the devices.</p>
    #[serde(rename = "inputDevices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_devices: Option<Vec<InputDeviceRequest>>,
    /// <p>Unique ID of the input.</p>
    #[serde(rename = "inputId")]
    pub input_id: String,
    /// <p>A list of security groups referenced by IDs to attach to the input.</p>
    #[serde(rename = "inputSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_security_groups: Option<Vec<String>>,
    /// <p>A list of the MediaConnect Flow ARNs that you want to use as the source of the input. You can specify as few as one
    /// Flow and presently, as many as two. The only requirement is when you have more than one is that each Flow is in a
    /// separate Availability Zone as this ensures your EML input is redundant to AZ issues.</p>
    #[serde(rename = "mediaConnectFlows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_connect_flows: Option<Vec<MediaConnectFlowRequest>>,
    /// <p>Name of the input.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the role this input assumes during and after creation.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The source URLs for a PULL-type input. Every PULL type input needs
    /// exactly two source URLs for redundancy.
    /// Only specify sources for PULL type Inputs. Leave Destinations empty.</p>
    #[serde(rename = "sources")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<InputSourceRequest>>,
}

/// <p>Placeholder documentation for UpdateInputResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInputResponse {
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<Input>,
}

/// <p>The request to update some combination of the Input Security Group name and the IPv4 CIDRs the Input Security Group should allow.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateInputSecurityGroupRequest {
    /// <p>The id of the Input Security Group to update.</p>
    #[serde(rename = "inputSecurityGroupId")]
    pub input_security_group_id: String,
    /// <p>A collection of key-value pairs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>List of IPv4 CIDR addresses to whitelist</p>
    #[serde(rename = "whitelistRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist_rules: Option<Vec<InputWhitelistRuleCidr>>,
}

/// <p>Placeholder documentation for UpdateInputSecurityGroupResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateInputSecurityGroupResponse {
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<InputSecurityGroup>,
}

/// <p>A request to update a program in a multiplex.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMultiplexProgramRequest {
    /// <p>The ID of the multiplex of the program to update.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The new settings for a multiplex program.</p>
    #[serde(rename = "multiplexProgramSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program_settings: Option<MultiplexProgramSettings>,
    /// <p>The name of the program to update.</p>
    #[serde(rename = "programName")]
    pub program_name: String,
}

/// <p>Placeholder documentation for UpdateMultiplexProgramResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMultiplexProgramResponse {
    /// <p>The updated multiplex program.</p>
    #[serde(rename = "multiplexProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_program: Option<MultiplexProgram>,
}

/// <p>A request to update a multiplex.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateMultiplexRequest {
    /// <p>ID of the multiplex to update.</p>
    #[serde(rename = "multiplexId")]
    pub multiplex_id: String,
    /// <p>The new settings for a multiplex.</p>
    #[serde(rename = "multiplexSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex_settings: Option<MultiplexSettings>,
    /// <p>Name of the multiplex.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p>Placeholder documentation for UpdateMultiplexResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateMultiplexResponse {
    /// <p>The updated multiplex.</p>
    #[serde(rename = "multiplex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplex: Option<Multiplex>,
}

/// <p>Request to update a reservation</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateReservationRequest {
    /// <p>Name of the reservation</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Unique reservation ID, e.g. &#39;1234567&#39;</p>
    #[serde(rename = "reservationId")]
    pub reservation_id: String,
}

/// <p>Placeholder documentation for UpdateReservationResponse</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateReservationResponse {
    #[serde(rename = "reservation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation: Option<Reservation>,
}

/// <p>Placeholder documentation for ValidationError</p>
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValidationError {
    /// <p>Path to the source of the error.</p>
    pub element_path: Option<String>,
    /// <p>The error message.</p>
    pub error_message: Option<String>,
}

/// <p>Placeholder documentation for VideoBlackFailoverSettings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoBlackFailoverSettings {
    /// <p>A value used in calculating the threshold below which MediaLive considers a pixel to be &#39;black&#39;. For the input to be considered black, every pixel in a frame must be below this threshold. The threshold is calculated as a percentage (expressed as a decimal) of white. Therefore .1 means 10% white (or 90% black). Note how the formula works for any color depth. For example, if you set this field to 0.1 in 10-bit color depth: (1023<em>0.1=102.3), which means a pixel value of 102 or less is &#39;black&#39;. If you set this field to .1 in an 8-bit color depth: (255</em>0.1=25.5), which means a pixel value of 25 or less is &#39;black&#39;. The range is 0.0 to 1.0, with any number of decimal places.</p>
    #[serde(rename = "blackDetectThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_detect_threshold: Option<f64>,
    /// <p>The amount of time (in milliseconds) that the active input must be black before automatic input failover occurs.</p>
    #[serde(rename = "videoBlackThresholdMsec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_black_threshold_msec: Option<i64>,
}

/// <p>Video Codec Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoCodecSettings {
    #[serde(rename = "frameCaptureSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    #[serde(rename = "h264Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,
    #[serde(rename = "h265Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,
    #[serde(rename = "mpeg2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_settings: Option<Mpeg2Settings>,
}

/// <p>Video settings for this stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoDescription {
    /// <p>Video codec settings.</p>
    #[serde(rename = "codecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<VideoCodecSettings>,
    /// <p>Output video height, in pixels. Must be an even number. For most codecs, you can leave this field and width blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>The name of this VideoDescription. Outputs will use this name to uniquely identify this Description.  Description names should be unique within this Live Event.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Indicates how MediaLive will respond to the AFD values that might be in the input video. If you do not know what AFD signaling is, or if your downstream system has not given you guidance, choose PASSTHROUGH.
    /// RESPOND: MediaLive clips the input video using a formula that uses the AFD values (configured in afdSignaling ), the input display aspect ratio, and the output display aspect ratio. MediaLive also includes the AFD values in the output, unless the codec for this encode is FRAME_CAPTURE.
    /// PASSTHROUGH: MediaLive ignores the AFD values and does not clip the video. But MediaLive does include the values in the output.
    /// NONE: MediaLive does not clip the input video and does not include the AFD values in the output</p>
    #[serde(rename = "respondToAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<String>,
    /// <p>STRETCH<em>TO</em>OUTPUT configures the output position to stretch the video to the specified output resolution (height and width). This option will override any position value. DEFAULT may insert black boxes (pillar boxes or letter boxes) around the video to provide the specified output resolution.</p>
    #[serde(rename = "scalingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<String>,
    /// <p>Changes the strength of the anti-alias filter used for scaling. 0 is the softest setting, 100 is the sharpest. A setting of 50 is recommended for most content.</p>
    #[serde(rename = "sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i64>,
    /// <p>Output video width, in pixels. Must be an even number. For most codecs, you can leave this field and height blank in order to use the height and width (resolution) from the source. Note, however, that leaving blank is not recommended. For the Frame Capture codec, height and width are required.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Specifies a particular video stream within an input source. An input may have only a single video selector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelector {
    /// <p>Specifies the color space of an input. This setting works in tandem with colorSpaceUsage and a video description&#39;s colorSpaceSettingsChoice to determine if any conversion will be performed.</p>
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// <p>Color space settings</p>
    #[serde(rename = "colorSpaceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_settings: Option<VideoSelectorColorSpaceSettings>,
    /// <p>Applies only if colorSpace is a value other than follow. This field controls how the value in the colorSpace field will be used. fallback means that when the input does include color space data, that data will be used, but when the input has no color space data, the value in colorSpace will be used. Choose fallback if your input is sometimes missing color space data, but when it does have color space data, that data is correct. force means to always use the value in colorSpace. Choose force if your input usually has no color space data or might have unreliable color space data.</p>
    #[serde(rename = "colorSpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<String>,
    /// <p>The video selector settings.</p>
    #[serde(rename = "selectorSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_settings: Option<VideoSelectorSettings>,
}

/// <p>Video Selector Color Space Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelectorColorSpaceSettings {
    #[serde(rename = "hdr10Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_settings: Option<Hdr10Settings>,
}

/// <p>Video Selector Pid</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelectorPid {
    /// <p>Selects a specific PID from within a video source.</p>
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

/// <p>Video Selector Program Id</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelectorProgramId {
    /// <p>Selects a specific program from within a multi-program transport stream. If the program doesn&#39;t exist, the first program within the transport stream will be selected by default.</p>
    #[serde(rename = "programId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_id: Option<i64>,
}

/// <p>Video Selector Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelectorSettings {
    #[serde(rename = "videoSelectorPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_pid: Option<VideoSelectorPid>,
    #[serde(rename = "videoSelectorProgramId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector_program_id: Option<VideoSelectorProgramId>,
}

/// <p>The properties for a private VPC Output
/// When this property is specified, the output egress addresses will be created in a user specified VPC</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcOutputSettings {
    /// <p>List of public address allocation ids to associate with ENIs that will be created in Output VPC.
    /// Must specify one for SINGLE_PIPELINE, two for STANDARD channels</p>
    #[serde(rename = "publicAddressAllocationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_address_allocation_ids: Option<Vec<String>>,
    /// <p>A list of up to 5 EC2 VPC security group IDs to attach to the Output VPC network interfaces.
    /// If none are specified then the VPC default security group will be used</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs from the same VPC.
    /// If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).</p>
    #[serde(rename = "subnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>The properties for a private VPC Output</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcOutputSettingsDescription {
    /// <p>The Availability Zones where the vpc subnets are located.
    /// The first Availability Zone applies to the first subnet in the list of subnets.
    /// The second Availability Zone applies to the second subnet.</p>
    #[serde(rename = "availabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>A list of Elastic Network Interfaces created by MediaLive in the customer&#39;s VPC</p>
    #[serde(rename = "networkInterfaceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    /// <p>A list of up EC2 VPC security group IDs attached to the Output VPC network interfaces.</p>
    #[serde(rename = "securityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs from the same VPC.
    /// If STANDARD channel, subnet IDs must be mapped to two unique availability zones (AZ).</p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>Wav Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WavSettings {
    /// <p>Bits per sample.</p>
    #[serde(rename = "bitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<f64>,
    /// <p>The audio coding mode for the WAV audio. The mode determines the number of channels in the audio.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<f64>,
}

/// <p>Webvtt Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebvttDestinationSettings {}

/// Errors returned by AcceptInputDeviceTransfer
#[derive(Debug, PartialEq)]
pub enum AcceptInputDeviceTransferError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl AcceptInputDeviceTransferError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AcceptInputDeviceTransferError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::BadGateway(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        AcceptInputDeviceTransferError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AcceptInputDeviceTransferError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        AcceptInputDeviceTransferError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AcceptInputDeviceTransferError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AcceptInputDeviceTransferError::BadGateway(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::BadRequest(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::Conflict(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::Forbidden(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            AcceptInputDeviceTransferError::NotFound(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            AcceptInputDeviceTransferError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AcceptInputDeviceTransferError {}
/// Errors returned by BatchDelete
#[derive(Debug, PartialEq)]
pub enum BatchDeleteError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl BatchDeleteError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(BatchDeleteError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(BatchDeleteError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(BatchDeleteError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchDeleteError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(BatchDeleteError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BatchDeleteError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchDeleteError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(BatchDeleteError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchDeleteError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteError::BadGateway(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::Conflict(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchDeleteError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteError {}
/// Errors returned by BatchStart
#[derive(Debug, PartialEq)]
pub enum BatchStartError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl BatchStartError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchStartError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(BatchStartError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(BatchStartError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(BatchStartError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchStartError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(BatchStartError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BatchStartError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchStartError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(BatchStartError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchStartError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchStartError::BadGateway(ref cause) => write!(f, "{}", cause),
            BatchStartError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchStartError::Conflict(ref cause) => write!(f, "{}", cause),
            BatchStartError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchStartError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            BatchStartError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BatchStartError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchStartError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchStartError {}
/// Errors returned by BatchStop
#[derive(Debug, PartialEq)]
pub enum BatchStopError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl BatchStopError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchStopError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(BatchStopError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(BatchStopError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(BatchStopError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchStopError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(BatchStopError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BatchStopError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchStopError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(BatchStopError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchStopError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchStopError::BadGateway(ref cause) => write!(f, "{}", cause),
            BatchStopError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchStopError::Conflict(ref cause) => write!(f, "{}", cause),
            BatchStopError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchStopError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            BatchStopError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BatchStopError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchStopError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchStopError {}
/// Errors returned by BatchUpdateSchedule
#[derive(Debug, PartialEq)]
pub enum BatchUpdateScheduleError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl BatchUpdateScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchUpdateScheduleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(BatchUpdateScheduleError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for BatchUpdateScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchUpdateScheduleError::BadGateway(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::BadRequest(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::Forbidden(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::NotFound(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            BatchUpdateScheduleError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchUpdateScheduleError {}
/// Errors returned by CancelInputDeviceTransfer
#[derive(Debug, PartialEq)]
pub enum CancelInputDeviceTransferError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl CancelInputDeviceTransferError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelInputDeviceTransferError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::BadGateway(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CancelInputDeviceTransferError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CancelInputDeviceTransferError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        CancelInputDeviceTransferError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelInputDeviceTransferError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelInputDeviceTransferError::BadGateway(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::BadRequest(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::Conflict(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::Forbidden(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelInputDeviceTransferError::NotFound(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CancelInputDeviceTransferError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CancelInputDeviceTransferError {}
/// Errors returned by CreateChannel
#[derive(Debug, PartialEq)]
pub enum CreateChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl CreateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreateChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateChannelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateChannelError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreateChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateChannelError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateChannelError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreateChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateChannelError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreateChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateChannelError {}
/// Errors returned by CreateInput
#[derive(Debug, PartialEq)]
pub enum CreateInputError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl CreateInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreateInputError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateInputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateInputError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreateInputError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateInputError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateInputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInputError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreateInputError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateInputError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateInputError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreateInputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateInputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInputError {}
/// Errors returned by CreateInputSecurityGroup
#[derive(Debug, PartialEq)]
pub enum CreateInputSecurityGroupError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl CreateInputSecurityGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateInputSecurityGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreateInputSecurityGroupError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateInputSecurityGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateInputSecurityGroupError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreateInputSecurityGroupError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        CreateInputSecurityGroupError::InternalServerError(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateInputSecurityGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateInputSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateInputSecurityGroupError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreateInputSecurityGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateInputSecurityGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateInputSecurityGroupError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreateInputSecurityGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateInputSecurityGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateInputSecurityGroupError {}
/// Errors returned by CreateMultiplex
#[derive(Debug, PartialEq)]
pub enum CreateMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl CreateMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreateMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateMultiplexError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateMultiplexError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreateMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateMultiplexError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateMultiplexError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateMultiplexError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateMultiplexError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMultiplexError {}
/// Errors returned by CreateMultiplexProgram
#[derive(Debug, PartialEq)]
pub enum CreateMultiplexProgramError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl CreateMultiplexProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateMultiplexProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(CreateMultiplexProgramError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateMultiplexProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateMultiplexProgramError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            CreateMultiplexProgramError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateMultiplexProgramError {}
/// Errors returned by CreatePartnerInput
#[derive(Debug, PartialEq)]
pub enum CreatePartnerInputError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl CreatePartnerInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePartnerInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(CreatePartnerInputError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(CreatePartnerInputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePartnerInputError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(CreatePartnerInputError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreatePartnerInputError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePartnerInputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePartnerInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePartnerInputError::BadGateway(ref cause) => write!(f, "{}", cause),
            CreatePartnerInputError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePartnerInputError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePartnerInputError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            CreatePartnerInputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreatePartnerInputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePartnerInputError {}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateTagsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateTagsError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateTagsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTagsError {}
/// Errors returned by DeleteChannel
#[derive(Debug, PartialEq)]
pub enum DeleteChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteChannelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteChannelError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteChannelError {}
/// Errors returned by DeleteInput
#[derive(Debug, PartialEq)]
pub enum DeleteInputError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteInputError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteInputError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteInputError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteInputError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteInputError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteInputError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInputError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteInputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInputError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteInputError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteInputError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteInputError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteInputError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteInputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteInputError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteInputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInputError {}
/// Errors returned by DeleteInputSecurityGroup
#[derive(Debug, PartialEq)]
pub enum DeleteInputSecurityGroupError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteInputSecurityGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteInputSecurityGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DeleteInputSecurityGroupError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteInputSecurityGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteInputSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteInputSecurityGroupError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteInputSecurityGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteInputSecurityGroupError {}
/// Errors returned by DeleteMultiplex
#[derive(Debug, PartialEq)]
pub enum DeleteMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMultiplexError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteMultiplexError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteMultiplexError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMultiplexError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteMultiplexError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMultiplexError {}
/// Errors returned by DeleteMultiplexProgram
#[derive(Debug, PartialEq)]
pub enum DeleteMultiplexProgramError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteMultiplexProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteMultiplexProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteMultiplexProgramError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteMultiplexProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteMultiplexProgramError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteMultiplexProgramError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteMultiplexProgramError {}
/// Errors returned by DeleteReservation
#[derive(Debug, PartialEq)]
pub enum DeleteReservationError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteReservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReservationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteReservationError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteReservationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteReservationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteReservationError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteReservationError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteReservationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteReservationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteReservationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReservationError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteReservationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReservationError {}
/// Errors returned by DeleteSchedule
#[derive(Debug, PartialEq)]
pub enum DeleteScheduleError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DeleteScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteScheduleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DeleteScheduleError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DeleteScheduleError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteScheduleError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DeleteScheduleError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteScheduleError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteScheduleError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteScheduleError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteScheduleError::BadGateway(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteScheduleError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteScheduleError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteTagsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteTagsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteTagsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DescribeChannel
#[derive(Debug, PartialEq)]
pub enum DescribeChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeChannelError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeChannelError {}
/// Errors returned by DescribeInput
#[derive(Debug, PartialEq)]
pub enum DescribeInputError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeInputError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeInputError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeInputError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeInputError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeInputError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeInputError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeInputError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInputError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeInputError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeInputError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeInputError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeInputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeInputError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeInputError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInputError {}
/// Errors returned by DescribeInputDevice
#[derive(Debug, PartialEq)]
pub enum DescribeInputDeviceError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeInputDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeInputDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeInputDeviceError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeInputDeviceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeInputDeviceError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeInputDeviceError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeInputDeviceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeInputDeviceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeInputDeviceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInputDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInputDeviceError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInputDeviceError {}
/// Errors returned by DescribeInputDeviceThumbnail
#[derive(Debug, PartialEq)]
pub enum DescribeInputDeviceThumbnailError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeInputDeviceThumbnailError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInputDeviceThumbnailError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeInputDeviceThumbnailError::BadGateway(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeInputDeviceThumbnailError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeInputDeviceThumbnailError::Forbidden(
                        err.msg,
                    ))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeInputDeviceThumbnailError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeInputDeviceThumbnailError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeInputDeviceThumbnailError::NotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DescribeInputDeviceThumbnailError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInputDeviceThumbnailError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInputDeviceThumbnailError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceThumbnailError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceThumbnailError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceThumbnailError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceThumbnailError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInputDeviceThumbnailError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeInputDeviceThumbnailError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInputDeviceThumbnailError {}
/// Errors returned by DescribeInputSecurityGroup
#[derive(Debug, PartialEq)]
pub enum DescribeInputSecurityGroupError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeInputSecurityGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeInputSecurityGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::BadGateway(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::BadRequest(
                        err.msg,
                    ))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::Forbidden(
                        err.msg,
                    ))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeInputSecurityGroupError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeInputSecurityGroupError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeInputSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeInputSecurityGroupError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeInputSecurityGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeInputSecurityGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeInputSecurityGroupError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeInputSecurityGroupError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeInputSecurityGroupError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeInputSecurityGroupError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeInputSecurityGroupError {}
/// Errors returned by DescribeMultiplex
#[derive(Debug, PartialEq)]
pub enum DescribeMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeMultiplexError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeMultiplexError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeMultiplexError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeMultiplexError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMultiplexError {}
/// Errors returned by DescribeMultiplexProgram
#[derive(Debug, PartialEq)]
pub enum DescribeMultiplexProgramError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeMultiplexProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeMultiplexProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        DescribeMultiplexProgramError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeMultiplexProgramError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeMultiplexProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeMultiplexProgramError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeMultiplexProgramError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeMultiplexProgramError {}
/// Errors returned by DescribeOffering
#[derive(Debug, PartialEq)]
pub enum DescribeOfferingError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeOfferingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeOfferingError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeOfferingError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeOfferingError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeOfferingError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeOfferingError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeOfferingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeOfferingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeOfferingError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeOfferingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeOfferingError {}
/// Errors returned by DescribeReservation
#[derive(Debug, PartialEq)]
pub enum DescribeReservationError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeReservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReservationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeReservationError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeReservationError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeReservationError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeReservationError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeReservationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeReservationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeReservationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservationError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeReservationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReservationError {}
/// Errors returned by DescribeSchedule
#[derive(Debug, PartialEq)]
pub enum DescribeScheduleError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl DescribeScheduleError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeScheduleError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(DescribeScheduleError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(DescribeScheduleError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeScheduleError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(DescribeScheduleError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeScheduleError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeScheduleError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeScheduleError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeScheduleError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeScheduleError::BadGateway(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeScheduleError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeScheduleError {}
/// Errors returned by ListChannels
#[derive(Debug, PartialEq)]
pub enum ListChannelsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListChannelsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListChannelsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListChannelsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListChannelsError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListChannelsError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListChannelsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListChannelsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListChannelsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListChannelsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListChannelsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListChannelsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListChannelsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListChannelsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChannelsError {}
/// Errors returned by ListInputDeviceTransfers
#[derive(Debug, PartialEq)]
pub enum ListInputDeviceTransfersError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl ListInputDeviceTransfersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputDeviceTransfersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListInputDeviceTransfersError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListInputDeviceTransfersError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListInputDeviceTransfersError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListInputDeviceTransfersError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        ListInputDeviceTransfersError::InternalServerError(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListInputDeviceTransfersError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        ListInputDeviceTransfersError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputDeviceTransfersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputDeviceTransfersError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            ListInputDeviceTransfersError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputDeviceTransfersError {}
/// Errors returned by ListInputDevices
#[derive(Debug, PartialEq)]
pub enum ListInputDevicesError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListInputDevicesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputDevicesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListInputDevicesError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListInputDevicesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListInputDevicesError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListInputDevicesError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListInputDevicesError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListInputDevicesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputDevicesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputDevicesError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListInputDevicesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListInputDevicesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListInputDevicesError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListInputDevicesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListInputDevicesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputDevicesError {}
/// Errors returned by ListInputSecurityGroups
#[derive(Debug, PartialEq)]
pub enum ListInputSecurityGroupsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListInputSecurityGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputSecurityGroupsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListInputSecurityGroupsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputSecurityGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputSecurityGroupsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListInputSecurityGroupsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListInputSecurityGroupsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListInputSecurityGroupsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListInputSecurityGroupsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListInputSecurityGroupsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputSecurityGroupsError {}
/// Errors returned by ListInputs
#[derive(Debug, PartialEq)]
pub enum ListInputsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListInputsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListInputsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListInputsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListInputsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListInputsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListInputsError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListInputsError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListInputsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListInputsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListInputsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListInputsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListInputsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListInputsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListInputsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListInputsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListInputsError {}
/// Errors returned by ListMultiplexPrograms
#[derive(Debug, PartialEq)]
pub enum ListMultiplexProgramsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListMultiplexProgramsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMultiplexProgramsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListMultiplexProgramsError::TooManyRequests(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMultiplexProgramsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMultiplexProgramsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListMultiplexProgramsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMultiplexProgramsError {}
/// Errors returned by ListMultiplexes
#[derive(Debug, PartialEq)]
pub enum ListMultiplexesError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListMultiplexesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListMultiplexesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListMultiplexesError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListMultiplexesError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListMultiplexesError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListMultiplexesError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListMultiplexesError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListMultiplexesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListMultiplexesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListMultiplexesError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListMultiplexesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListMultiplexesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListMultiplexesError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListMultiplexesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListMultiplexesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListMultiplexesError {}
/// Errors returned by ListOfferings
#[derive(Debug, PartialEq)]
pub enum ListOfferingsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListOfferingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListOfferingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListOfferingsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListOfferingsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListOfferingsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListOfferingsError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListOfferingsError::InternalServerError(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListOfferingsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListOfferingsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListOfferingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListOfferingsError {}
/// Errors returned by ListReservations
#[derive(Debug, PartialEq)]
pub enum ListReservationsError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl ListReservationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListReservationsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(ListReservationsError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(ListReservationsError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListReservationsError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(ListReservationsError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListReservationsError::InternalServerError(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListReservationsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListReservationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListReservationsError::BadGateway(ref cause) => write!(f, "{}", cause),
            ListReservationsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListReservationsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListReservationsError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            ListReservationsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListReservationsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListReservationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListTagsForResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by PurchaseOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseOfferingError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl PurchaseOfferingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PurchaseOfferingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(PurchaseOfferingError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(PurchaseOfferingError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(PurchaseOfferingError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(PurchaseOfferingError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(PurchaseOfferingError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(PurchaseOfferingError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(PurchaseOfferingError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PurchaseOfferingError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PurchaseOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseOfferingError::BadGateway(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::BadRequest(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::Conflict(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::Forbidden(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::InternalServerError(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::NotFound(ref cause) => write!(f, "{}", cause),
            PurchaseOfferingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PurchaseOfferingError {}
/// Errors returned by RejectInputDeviceTransfer
#[derive(Debug, PartialEq)]
pub enum RejectInputDeviceTransferError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl RejectInputDeviceTransferError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RejectInputDeviceTransferError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::BadGateway(
                        err.msg,
                    ))
                }
                "BadRequestException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::BadRequest(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        RejectInputDeviceTransferError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RejectInputDeviceTransferError::TooManyRequests(
                        err.msg,
                    ))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(
                        RejectInputDeviceTransferError::UnprocessableEntity(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RejectInputDeviceTransferError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RejectInputDeviceTransferError::BadGateway(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::BadRequest(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::Conflict(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::Forbidden(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::InternalServerError(ref cause) => {
                write!(f, "{}", cause)
            }
            RejectInputDeviceTransferError::NotFound(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            RejectInputDeviceTransferError::UnprocessableEntity(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RejectInputDeviceTransferError {}
/// Errors returned by StartChannel
#[derive(Debug, PartialEq)]
pub enum StartChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl StartChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(StartChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(StartChannelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartChannelError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(StartChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            StartChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartChannelError::Conflict(ref cause) => write!(f, "{}", cause),
            StartChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            StartChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            StartChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartChannelError {}
/// Errors returned by StartMultiplex
#[derive(Debug, PartialEq)]
pub enum StartMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl StartMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(StartMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(StartMultiplexError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartMultiplexError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StartMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(StartMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StartMultiplexError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StartMultiplexError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StartMultiplexError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::Conflict(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::NotFound(ref cause) => write!(f, "{}", cause),
            StartMultiplexError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMultiplexError {}
/// Errors returned by StopChannel
#[derive(Debug, PartialEq)]
pub enum StopChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl StopChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(StopChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(StopChannelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StopChannelError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StopChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(StopChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopChannelError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopChannelError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopChannelError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            StopChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopChannelError::Conflict(ref cause) => write!(f, "{}", cause),
            StopChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            StopChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            StopChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopChannelError::NotFound(ref cause) => write!(f, "{}", cause),
            StopChannelError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopChannelError {}
/// Errors returned by StopMultiplex
#[derive(Debug, PartialEq)]
pub enum StopMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl StopMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(StopMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(StopMultiplexError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(StopMultiplexError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(StopMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(StopMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(StopMultiplexError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(StopMultiplexError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(StopMultiplexError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::Conflict(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::NotFound(ref cause) => write!(f, "{}", cause),
            StopMultiplexError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopMultiplexError {}
/// Errors returned by TransferInputDevice
#[derive(Debug, PartialEq)]
pub enum TransferInputDeviceError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl TransferInputDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TransferInputDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(TransferInputDeviceError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(TransferInputDeviceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TransferInputDeviceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(TransferInputDeviceError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(TransferInputDeviceError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TransferInputDeviceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TransferInputDeviceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TransferInputDeviceError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(TransferInputDeviceError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TransferInputDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TransferInputDeviceError::BadGateway(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::Conflict(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::Forbidden(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            TransferInputDeviceError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TransferInputDeviceError {}
/// Errors returned by UpdateChannel
#[derive(Debug, PartialEq)]
pub enum UpdateChannelError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl UpdateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateChannelError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateChannelError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateChannelError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateChannelError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateChannelError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateChannelError::InternalServerError(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateChannelError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateChannelError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateChannelError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateChannelError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateChannelError {}
/// Errors returned by UpdateChannelClass
#[derive(Debug, PartialEq)]
pub enum UpdateChannelClassError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl UpdateChannelClassError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelClassError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateChannelClassError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateChannelClassError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateChannelClassError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateChannelClassError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateChannelClassError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateChannelClassError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateChannelClassError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateChannelClassError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateChannelClassError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateChannelClassError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateChannelClassError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateChannelClassError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateChannelClassError {}
/// Errors returned by UpdateInput
#[derive(Debug, PartialEq)]
pub enum UpdateInputError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
}

impl UpdateInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInputError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateInputError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateInputError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateInputError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateInputError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateInputError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateInputError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateInputError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInputError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInputError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateInputError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateInputError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateInputError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateInputError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateInputError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateInputError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInputError {}
/// Errors returned by UpdateInputDevice
#[derive(Debug, PartialEq)]
pub enum UpdateInputDeviceError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl UpdateInputDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInputDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateInputDeviceError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateInputDeviceError::BadRequest(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateInputDeviceError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateInputDeviceError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateInputDeviceError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateInputDeviceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateInputDeviceError::TooManyRequests(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateInputDeviceError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInputDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInputDeviceError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            UpdateInputDeviceError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInputDeviceError {}
/// Errors returned by UpdateInputSecurityGroup
#[derive(Debug, PartialEq)]
pub enum UpdateInputSecurityGroupError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
}

impl UpdateInputSecurityGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateInputSecurityGroupError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(
                        UpdateInputSecurityGroupError::InternalServerError(err.msg),
                    )
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateInputSecurityGroupError::NotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateInputSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateInputSecurityGroupError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateInputSecurityGroupError::NotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateInputSecurityGroupError {}
/// Errors returned by UpdateMultiplex
#[derive(Debug, PartialEq)]
pub enum UpdateMultiplexError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl UpdateMultiplexError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMultiplexError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateMultiplexError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMultiplexError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMultiplexError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateMultiplexError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateMultiplexError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateMultiplexError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMultiplexError::NotFound(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateMultiplexError::UnprocessableEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMultiplexError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMultiplexError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMultiplexError {}
/// Errors returned by UpdateMultiplexProgram
#[derive(Debug, PartialEq)]
pub enum UpdateMultiplexProgramError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for UnprocessableEntityException</p>
    UnprocessableEntity(String),
}

impl UpdateMultiplexProgramError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateMultiplexProgramError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::GatewayTimeout(
                        err.msg,
                    ))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::NotFound(err.msg))
                }
                "UnprocessableEntityException" => {
                    return RusotoError::Service(UpdateMultiplexProgramError::UnprocessableEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateMultiplexProgramError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateMultiplexProgramError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateMultiplexProgramError::UnprocessableEntity(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateMultiplexProgramError {}
/// Errors returned by UpdateReservation
#[derive(Debug, PartialEq)]
pub enum UpdateReservationError {
    /// <p>Placeholder documentation for BadGatewayException</p>
    BadGateway(String),
    /// <p>Placeholder documentation for BadRequestException</p>
    BadRequest(String),
    /// <p>Placeholder documentation for ConflictException</p>
    Conflict(String),
    /// <p>Placeholder documentation for ForbiddenException</p>
    Forbidden(String),
    /// <p>Placeholder documentation for GatewayTimeoutException</p>
    GatewayTimeout(String),
    /// <p>Placeholder documentation for InternalServerErrorException</p>
    InternalServerError(String),
    /// <p>Placeholder documentation for NotFoundException</p>
    NotFound(String),
    /// <p>Placeholder documentation for TooManyRequestsException</p>
    TooManyRequests(String),
}

impl UpdateReservationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateReservationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadGatewayException" => {
                    return RusotoError::Service(UpdateReservationError::BadGateway(err.msg))
                }
                "BadRequestException" => {
                    return RusotoError::Service(UpdateReservationError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateReservationError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateReservationError::Forbidden(err.msg))
                }
                "GatewayTimeoutException" => {
                    return RusotoError::Service(UpdateReservationError::GatewayTimeout(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateReservationError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateReservationError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateReservationError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateReservationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateReservationError::BadGateway(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::GatewayTimeout(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateReservationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateReservationError {}
/// Trait representing the capabilities of the MediaLive API. MediaLive clients implement this trait.
#[async_trait]
pub trait MediaLive {
    /// <p>Accept an incoming input device transfer. The ownership of the device will transfer to your AWS account.</p>
    async fn accept_input_device_transfer(
        &self,
        input: AcceptInputDeviceTransferRequest,
    ) -> Result<AcceptInputDeviceTransferResponse, RusotoError<AcceptInputDeviceTransferError>>;

    /// <p>Starts delete of resources.</p>
    async fn batch_delete(
        &self,
        input: BatchDeleteRequest,
    ) -> Result<BatchDeleteResponse, RusotoError<BatchDeleteError>>;

    /// <p>Starts existing resources</p>
    async fn batch_start(
        &self,
        input: BatchStartRequest,
    ) -> Result<BatchStartResponse, RusotoError<BatchStartError>>;

    /// <p>Stops running resources</p>
    async fn batch_stop(
        &self,
        input: BatchStopRequest,
    ) -> Result<BatchStopResponse, RusotoError<BatchStopError>>;

    /// <p>Update a channel schedule</p>
    async fn batch_update_schedule(
        &self,
        input: BatchUpdateScheduleRequest,
    ) -> Result<BatchUpdateScheduleResponse, RusotoError<BatchUpdateScheduleError>>;

    /// <p>Cancel an input device transfer that you have requested.</p>
    async fn cancel_input_device_transfer(
        &self,
        input: CancelInputDeviceTransferRequest,
    ) -> Result<CancelInputDeviceTransferResponse, RusotoError<CancelInputDeviceTransferError>>;

    /// <p>Creates a new channel</p>
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>>;

    /// <p>Create an input</p>
    async fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Result<CreateInputResponse, RusotoError<CreateInputError>>;

    /// <p>Creates a Input Security Group</p>
    async fn create_input_security_group(
        &self,
        input: CreateInputSecurityGroupRequest,
    ) -> Result<CreateInputSecurityGroupResponse, RusotoError<CreateInputSecurityGroupError>>;

    /// <p>Create a new multiplex.</p>
    async fn create_multiplex(
        &self,
        input: CreateMultiplexRequest,
    ) -> Result<CreateMultiplexResponse, RusotoError<CreateMultiplexError>>;

    /// <p>Create a new program in the multiplex.</p>
    async fn create_multiplex_program(
        &self,
        input: CreateMultiplexProgramRequest,
    ) -> Result<CreateMultiplexProgramResponse, RusotoError<CreateMultiplexProgramError>>;

    /// <p>Create a partner input</p>
    async fn create_partner_input(
        &self,
        input: CreatePartnerInputRequest,
    ) -> Result<CreatePartnerInputResponse, RusotoError<CreatePartnerInputError>>;

    /// <p>Create tags for a resource</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<(), RusotoError<CreateTagsError>>;

    /// <p>Starts deletion of channel. The associated outputs are also deleted.</p>
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>>;

    /// <p>Deletes the input end point</p>
    async fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Result<DeleteInputResponse, RusotoError<DeleteInputError>>;

    /// <p>Deletes an Input Security Group</p>
    async fn delete_input_security_group(
        &self,
        input: DeleteInputSecurityGroupRequest,
    ) -> Result<DeleteInputSecurityGroupResponse, RusotoError<DeleteInputSecurityGroupError>>;

    /// <p>Delete a multiplex. The multiplex must be idle.</p>
    async fn delete_multiplex(
        &self,
        input: DeleteMultiplexRequest,
    ) -> Result<DeleteMultiplexResponse, RusotoError<DeleteMultiplexError>>;

    /// <p>Delete a program from a multiplex.</p>
    async fn delete_multiplex_program(
        &self,
        input: DeleteMultiplexProgramRequest,
    ) -> Result<DeleteMultiplexProgramResponse, RusotoError<DeleteMultiplexProgramError>>;

    /// <p>Delete an expired reservation.</p>
    async fn delete_reservation(
        &self,
        input: DeleteReservationRequest,
    ) -> Result<DeleteReservationResponse, RusotoError<DeleteReservationError>>;

    /// <p>Delete all schedule actions on a channel.</p>
    async fn delete_schedule(
        &self,
        input: DeleteScheduleRequest,
    ) -> Result<DeleteScheduleResponse, RusotoError<DeleteScheduleError>>;

    /// <p>Removes tags for a resource</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<(), RusotoError<DeleteTagsError>>;

    /// <p>Gets details about a channel</p>
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>>;

    /// <p>Produces details about an input</p>
    async fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Result<DescribeInputResponse, RusotoError<DescribeInputError>>;

    /// <p>Gets the details for the input device</p>
    async fn describe_input_device(
        &self,
        input: DescribeInputDeviceRequest,
    ) -> Result<DescribeInputDeviceResponse, RusotoError<DescribeInputDeviceError>>;

    /// <p>Get the latest thumbnail data for the input device.</p>
    async fn describe_input_device_thumbnail(
        &self,
        input: DescribeInputDeviceThumbnailRequest,
    ) -> Result<DescribeInputDeviceThumbnailResponse, RusotoError<DescribeInputDeviceThumbnailError>>;

    /// <p>Produces a summary of an Input Security Group</p>
    async fn describe_input_security_group(
        &self,
        input: DescribeInputSecurityGroupRequest,
    ) -> Result<DescribeInputSecurityGroupResponse, RusotoError<DescribeInputSecurityGroupError>>;

    /// <p>Gets details about a multiplex.</p>
    async fn describe_multiplex(
        &self,
        input: DescribeMultiplexRequest,
    ) -> Result<DescribeMultiplexResponse, RusotoError<DescribeMultiplexError>>;

    /// <p>Get the details for a program in a multiplex.</p>
    async fn describe_multiplex_program(
        &self,
        input: DescribeMultiplexProgramRequest,
    ) -> Result<DescribeMultiplexProgramResponse, RusotoError<DescribeMultiplexProgramError>>;

    /// <p>Get details for an offering.</p>
    async fn describe_offering(
        &self,
        input: DescribeOfferingRequest,
    ) -> Result<DescribeOfferingResponse, RusotoError<DescribeOfferingError>>;

    /// <p>Get details for a reservation.</p>
    async fn describe_reservation(
        &self,
        input: DescribeReservationRequest,
    ) -> Result<DescribeReservationResponse, RusotoError<DescribeReservationError>>;

    /// <p>Get a channel schedule</p>
    async fn describe_schedule(
        &self,
        input: DescribeScheduleRequest,
    ) -> Result<DescribeScheduleResponse, RusotoError<DescribeScheduleError>>;

    /// <p>Produces list of channels that have been created</p>
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>>;

    /// <p>List input devices that are currently being transferred. List input devices that you are transferring from your AWS account or input devices that another AWS account is transferring to you.</p>
    async fn list_input_device_transfers(
        &self,
        input: ListInputDeviceTransfersRequest,
    ) -> Result<ListInputDeviceTransfersResponse, RusotoError<ListInputDeviceTransfersError>>;

    /// <p>List input devices</p>
    async fn list_input_devices(
        &self,
        input: ListInputDevicesRequest,
    ) -> Result<ListInputDevicesResponse, RusotoError<ListInputDevicesError>>;

    /// <p>Produces a list of Input Security Groups for an account</p>
    async fn list_input_security_groups(
        &self,
        input: ListInputSecurityGroupsRequest,
    ) -> Result<ListInputSecurityGroupsResponse, RusotoError<ListInputSecurityGroupsError>>;

    /// <p>Produces list of inputs that have been created</p>
    async fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Result<ListInputsResponse, RusotoError<ListInputsError>>;

    /// <p>List the programs that currently exist for a specific multiplex.</p>
    async fn list_multiplex_programs(
        &self,
        input: ListMultiplexProgramsRequest,
    ) -> Result<ListMultiplexProgramsResponse, RusotoError<ListMultiplexProgramsError>>;

    /// <p>Retrieve a list of the existing multiplexes.</p>
    async fn list_multiplexes(
        &self,
        input: ListMultiplexesRequest,
    ) -> Result<ListMultiplexesResponse, RusotoError<ListMultiplexesError>>;

    /// <p>List offerings available for purchase.</p>
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResponse, RusotoError<ListOfferingsError>>;

    /// <p>List purchased reservations.</p>
    async fn list_reservations(
        &self,
        input: ListReservationsRequest,
    ) -> Result<ListReservationsResponse, RusotoError<ListReservationsError>>;

    /// <p>Produces list of tags that have been created for a resource</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Purchase an offering and create a reservation.</p>
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResponse, RusotoError<PurchaseOfferingError>>;

    /// <p>Reject the transfer of the specified input device to your AWS account.</p>
    async fn reject_input_device_transfer(
        &self,
        input: RejectInputDeviceTransferRequest,
    ) -> Result<RejectInputDeviceTransferResponse, RusotoError<RejectInputDeviceTransferError>>;

    /// <p>Starts an existing channel</p>
    async fn start_channel(
        &self,
        input: StartChannelRequest,
    ) -> Result<StartChannelResponse, RusotoError<StartChannelError>>;

    /// <p>Start (run) the multiplex. Starting the multiplex does not start the channels. You must explicitly start each channel.</p>
    async fn start_multiplex(
        &self,
        input: StartMultiplexRequest,
    ) -> Result<StartMultiplexResponse, RusotoError<StartMultiplexError>>;

    /// <p>Stops a running channel</p>
    async fn stop_channel(
        &self,
        input: StopChannelRequest,
    ) -> Result<StopChannelResponse, RusotoError<StopChannelError>>;

    /// <p>Stops a running multiplex. If the multiplex isn&#39;t running, this action has no effect.</p>
    async fn stop_multiplex(
        &self,
        input: StopMultiplexRequest,
    ) -> Result<StopMultiplexResponse, RusotoError<StopMultiplexError>>;

    /// <p>Start an input device transfer to another AWS account. After you make the request, the other account must accept or reject the transfer.</p>
    async fn transfer_input_device(
        &self,
        input: TransferInputDeviceRequest,
    ) -> Result<TransferInputDeviceResponse, RusotoError<TransferInputDeviceError>>;

    /// <p>Updates a channel.</p>
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>>;

    /// <p>Changes the class of the channel.</p>
    async fn update_channel_class(
        &self,
        input: UpdateChannelClassRequest,
    ) -> Result<UpdateChannelClassResponse, RusotoError<UpdateChannelClassError>>;

    /// <p>Updates an input.</p>
    async fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Result<UpdateInputResponse, RusotoError<UpdateInputError>>;

    /// <p>Updates the parameters for the input device.</p>
    async fn update_input_device(
        &self,
        input: UpdateInputDeviceRequest,
    ) -> Result<UpdateInputDeviceResponse, RusotoError<UpdateInputDeviceError>>;

    /// <p>Update an Input Security Group&#39;s Whilelists.</p>
    async fn update_input_security_group(
        &self,
        input: UpdateInputSecurityGroupRequest,
    ) -> Result<UpdateInputSecurityGroupResponse, RusotoError<UpdateInputSecurityGroupError>>;

    /// <p>Updates a multiplex.</p>
    async fn update_multiplex(
        &self,
        input: UpdateMultiplexRequest,
    ) -> Result<UpdateMultiplexResponse, RusotoError<UpdateMultiplexError>>;

    /// <p>Update a program in a multiplex.</p>
    async fn update_multiplex_program(
        &self,
        input: UpdateMultiplexProgramRequest,
    ) -> Result<UpdateMultiplexProgramResponse, RusotoError<UpdateMultiplexProgramError>>;

    /// <p>Update reservation.</p>
    async fn update_reservation(
        &self,
        input: UpdateReservationRequest,
    ) -> Result<UpdateReservationResponse, RusotoError<UpdateReservationError>>;
}
/// A client for the MediaLive API.
#[derive(Clone)]
pub struct MediaLiveClient {
    client: Client,
    region: region::Region,
}

impl MediaLiveClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaLiveClient {
        MediaLiveClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaLiveClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaLiveClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaLiveClient {
        MediaLiveClient { client, region }
    }
}

#[async_trait]
impl MediaLive for MediaLiveClient {
    /// <p>Accept an incoming input device transfer. The ownership of the device will transfer to your AWS account.</p>
    #[allow(unused_mut)]
    async fn accept_input_device_transfer(
        &self,
        input: AcceptInputDeviceTransferRequest,
    ) -> Result<AcceptInputDeviceTransferResponse, RusotoError<AcceptInputDeviceTransferError>>
    {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}/accept",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AcceptInputDeviceTransferResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AcceptInputDeviceTransferError::from_response(response))
        }
    }

    /// <p>Starts delete of resources.</p>
    #[allow(unused_mut)]
    async fn batch_delete(
        &self,
        input: BatchDeleteRequest,
    ) -> Result<BatchDeleteResponse, RusotoError<BatchDeleteError>> {
        let request_uri = "/prod/batch/delete";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDeleteResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeleteError::from_response(response))
        }
    }

    /// <p>Starts existing resources</p>
    #[allow(unused_mut)]
    async fn batch_start(
        &self,
        input: BatchStartRequest,
    ) -> Result<BatchStartResponse, RusotoError<BatchStartError>> {
        let request_uri = "/prod/batch/start";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchStartResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchStartError::from_response(response))
        }
    }

    /// <p>Stops running resources</p>
    #[allow(unused_mut)]
    async fn batch_stop(
        &self,
        input: BatchStopRequest,
    ) -> Result<BatchStopResponse, RusotoError<BatchStopError>> {
        let request_uri = "/prod/batch/stop";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchStopResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchStopError::from_response(response))
        }
    }

    /// <p>Update a channel schedule</p>
    #[allow(unused_mut)]
    async fn batch_update_schedule(
        &self,
        input: BatchUpdateScheduleRequest,
    ) -> Result<BatchUpdateScheduleResponse, RusotoError<BatchUpdateScheduleError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/schedule",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchUpdateScheduleResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BatchUpdateScheduleError::from_response(response))
        }
    }

    /// <p>Cancel an input device transfer that you have requested.</p>
    #[allow(unused_mut)]
    async fn cancel_input_device_transfer(
        &self,
        input: CancelInputDeviceTransferRequest,
    ) -> Result<CancelInputDeviceTransferResponse, RusotoError<CancelInputDeviceTransferError>>
    {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}/cancel",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelInputDeviceTransferResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelInputDeviceTransferError::from_response(response))
        }
    }

    /// <p>Creates a new channel</p>
    #[allow(unused_mut)]
    async fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> Result<CreateChannelResponse, RusotoError<CreateChannelError>> {
        let request_uri = "/prod/channels";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateChannelError::from_response(response))
        }
    }

    /// <p>Create an input</p>
    #[allow(unused_mut)]
    async fn create_input(
        &self,
        input: CreateInputRequest,
    ) -> Result<CreateInputResponse, RusotoError<CreateInputError>> {
        let request_uri = "/prod/inputs";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInputError::from_response(response))
        }
    }

    /// <p>Creates a Input Security Group</p>
    #[allow(unused_mut)]
    async fn create_input_security_group(
        &self,
        input: CreateInputSecurityGroupRequest,
    ) -> Result<CreateInputSecurityGroupResponse, RusotoError<CreateInputSecurityGroupError>> {
        let request_uri = "/prod/inputSecurityGroups";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateInputSecurityGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateInputSecurityGroupError::from_response(response))
        }
    }

    /// <p>Create a new multiplex.</p>
    #[allow(unused_mut)]
    async fn create_multiplex(
        &self,
        input: CreateMultiplexRequest,
    ) -> Result<CreateMultiplexResponse, RusotoError<CreateMultiplexError>> {
        let request_uri = "/prod/multiplexes";

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMultiplexError::from_response(response))
        }
    }

    /// <p>Create a new program in the multiplex.</p>
    #[allow(unused_mut)]
    async fn create_multiplex_program(
        &self,
        input: CreateMultiplexProgramRequest,
    ) -> Result<CreateMultiplexProgramResponse, RusotoError<CreateMultiplexProgramError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/programs",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateMultiplexProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateMultiplexProgramError::from_response(response))
        }
    }

    /// <p>Create a partner input</p>
    #[allow(unused_mut)]
    async fn create_partner_input(
        &self,
        input: CreatePartnerInputRequest,
    ) -> Result<CreatePartnerInputResponse, RusotoError<CreatePartnerInputError>> {
        let request_uri = format!(
            "/prod/inputs/{input_id}/partners",
            input_id = input.input_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreatePartnerInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePartnerInputError::from_response(response))
        }
    }

    /// <p>Create tags for a resource</p>
    #[allow(unused_mut)]
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<(), RusotoError<CreateTagsError>> {
        let request_uri = format!(
            "/prod/tags/{resource_arn}",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTagsError::from_response(response))
        }
    }

    /// <p>Starts deletion of channel. The associated outputs are also deleted.</p>
    #[allow(unused_mut)]
    async fn delete_channel(
        &self,
        input: DeleteChannelRequest,
    ) -> Result<DeleteChannelResponse, RusotoError<DeleteChannelError>> {
        let request_uri = format!("/prod/channels/{channel_id}", channel_id = input.channel_id);

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteChannelError::from_response(response))
        }
    }

    /// <p>Deletes the input end point</p>
    #[allow(unused_mut)]
    async fn delete_input(
        &self,
        input: DeleteInputRequest,
    ) -> Result<DeleteInputResponse, RusotoError<DeleteInputError>> {
        let request_uri = format!("/prod/inputs/{input_id}", input_id = input.input_id);

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInputError::from_response(response))
        }
    }

    /// <p>Deletes an Input Security Group</p>
    #[allow(unused_mut)]
    async fn delete_input_security_group(
        &self,
        input: DeleteInputSecurityGroupRequest,
    ) -> Result<DeleteInputSecurityGroupResponse, RusotoError<DeleteInputSecurityGroupError>> {
        let request_uri = format!(
            "/prod/inputSecurityGroups/{input_security_group_id}",
            input_security_group_id = input.input_security_group_id
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteInputSecurityGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteInputSecurityGroupError::from_response(response))
        }
    }

    /// <p>Delete a multiplex. The multiplex must be idle.</p>
    #[allow(unused_mut)]
    async fn delete_multiplex(
        &self,
        input: DeleteMultiplexRequest,
    ) -> Result<DeleteMultiplexResponse, RusotoError<DeleteMultiplexError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMultiplexError::from_response(response))
        }
    }

    /// <p>Delete a program from a multiplex.</p>
    #[allow(unused_mut)]
    async fn delete_multiplex_program(
        &self,
        input: DeleteMultiplexProgramRequest,
    ) -> Result<DeleteMultiplexProgramResponse, RusotoError<DeleteMultiplexProgramError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/programs/{program_name}",
            multiplex_id = input.multiplex_id,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteMultiplexProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteMultiplexProgramError::from_response(response))
        }
    }

    /// <p>Delete an expired reservation.</p>
    #[allow(unused_mut)]
    async fn delete_reservation(
        &self,
        input: DeleteReservationRequest,
    ) -> Result<DeleteReservationResponse, RusotoError<DeleteReservationError>> {
        let request_uri = format!(
            "/prod/reservations/{reservation_id}",
            reservation_id = input.reservation_id
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteReservationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteReservationError::from_response(response))
        }
    }

    /// <p>Delete all schedule actions on a channel.</p>
    #[allow(unused_mut)]
    async fn delete_schedule(
        &self,
        input: DeleteScheduleRequest,
    ) -> Result<DeleteScheduleResponse, RusotoError<DeleteScheduleError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/schedule",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteScheduleResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteScheduleError::from_response(response))
        }
    }

    /// <p>Removes tags for a resource</p>
    #[allow(unused_mut)]
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<(), RusotoError<DeleteTagsError>> {
        let request_uri = format!(
            "/prod/tags/{resource_arn}",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("DELETE", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Gets details about a channel</p>
    #[allow(unused_mut)]
    async fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> Result<DescribeChannelResponse, RusotoError<DescribeChannelError>> {
        let request_uri = format!("/prod/channels/{channel_id}", channel_id = input.channel_id);

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeChannelError::from_response(response))
        }
    }

    /// <p>Produces details about an input</p>
    #[allow(unused_mut)]
    async fn describe_input(
        &self,
        input: DescribeInputRequest,
    ) -> Result<DescribeInputResponse, RusotoError<DescribeInputError>> {
        let request_uri = format!("/prod/inputs/{input_id}", input_id = input.input_id);

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInputError::from_response(response))
        }
    }

    /// <p>Gets the details for the input device</p>
    #[allow(unused_mut)]
    async fn describe_input_device(
        &self,
        input: DescribeInputDeviceRequest,
    ) -> Result<DescribeInputDeviceResponse, RusotoError<DescribeInputDeviceError>> {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInputDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInputDeviceError::from_response(response))
        }
    }

    /// <p>Get the latest thumbnail data for the input device.</p>
    #[allow(unused_mut)]
    async fn describe_input_device_thumbnail(
        &self,
        input: DescribeInputDeviceThumbnailRequest,
    ) -> Result<DescribeInputDeviceThumbnailResponse, RusotoError<DescribeInputDeviceThumbnailError>>
    {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}/thumbnailData",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header("accept", &input.accept.to_string());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = DescribeInputDeviceThumbnailResponse::default();
            result.body = Some(response.body);

            result.content_length = response
                .headers
                .remove("Content-Length")
                .map(|value| value.parse::<i64>().unwrap());
            result.content_type = response.headers.remove("Content-Type");
            result.e_tag = response.headers.remove("ETag");
            result.last_modified = response
                .headers
                .remove("Last-Modified")
                .map(|value| value.parse::<f64>().unwrap());

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInputDeviceThumbnailError::from_response(response))
        }
    }

    /// <p>Produces a summary of an Input Security Group</p>
    #[allow(unused_mut)]
    async fn describe_input_security_group(
        &self,
        input: DescribeInputSecurityGroupRequest,
    ) -> Result<DescribeInputSecurityGroupResponse, RusotoError<DescribeInputSecurityGroupError>>
    {
        let request_uri = format!(
            "/prod/inputSecurityGroups/{input_security_group_id}",
            input_security_group_id = input.input_security_group_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeInputSecurityGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeInputSecurityGroupError::from_response(response))
        }
    }

    /// <p>Gets details about a multiplex.</p>
    #[allow(unused_mut)]
    async fn describe_multiplex(
        &self,
        input: DescribeMultiplexRequest,
    ) -> Result<DescribeMultiplexResponse, RusotoError<DescribeMultiplexError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMultiplexError::from_response(response))
        }
    }

    /// <p>Get the details for a program in a multiplex.</p>
    #[allow(unused_mut)]
    async fn describe_multiplex_program(
        &self,
        input: DescribeMultiplexProgramRequest,
    ) -> Result<DescribeMultiplexProgramResponse, RusotoError<DescribeMultiplexProgramError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/programs/{program_name}",
            multiplex_id = input.multiplex_id,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeMultiplexProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeMultiplexProgramError::from_response(response))
        }
    }

    /// <p>Get details for an offering.</p>
    #[allow(unused_mut)]
    async fn describe_offering(
        &self,
        input: DescribeOfferingRequest,
    ) -> Result<DescribeOfferingResponse, RusotoError<DescribeOfferingError>> {
        let request_uri = format!(
            "/prod/offerings/{offering_id}",
            offering_id = input.offering_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeOfferingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeOfferingError::from_response(response))
        }
    }

    /// <p>Get details for a reservation.</p>
    #[allow(unused_mut)]
    async fn describe_reservation(
        &self,
        input: DescribeReservationRequest,
    ) -> Result<DescribeReservationResponse, RusotoError<DescribeReservationError>> {
        let request_uri = format!(
            "/prod/reservations/{reservation_id}",
            reservation_id = input.reservation_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeReservationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeReservationError::from_response(response))
        }
    }

    /// <p>Get a channel schedule</p>
    #[allow(unused_mut)]
    async fn describe_schedule(
        &self,
        input: DescribeScheduleRequest,
    ) -> Result<DescribeScheduleResponse, RusotoError<DescribeScheduleError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/schedule",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeScheduleResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeScheduleError::from_response(response))
        }
    }

    /// <p>Produces list of channels that have been created</p>
    #[allow(unused_mut)]
    async fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> Result<ListChannelsResponse, RusotoError<ListChannelsError>> {
        let request_uri = "/prod/channels";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListChannelsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListChannelsError::from_response(response))
        }
    }

    /// <p>List input devices that are currently being transferred. List input devices that you are transferring from your AWS account or input devices that another AWS account is transferring to you.</p>
    #[allow(unused_mut)]
    async fn list_input_device_transfers(
        &self,
        input: ListInputDeviceTransfersRequest,
    ) -> Result<ListInputDeviceTransfersResponse, RusotoError<ListInputDeviceTransfersError>> {
        let request_uri = "/prod/inputDeviceTransfers";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        params.put("transferType", &input.transfer_type);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputDeviceTransfersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputDeviceTransfersError::from_response(response))
        }
    }

    /// <p>List input devices</p>
    #[allow(unused_mut)]
    async fn list_input_devices(
        &self,
        input: ListInputDevicesRequest,
    ) -> Result<ListInputDevicesResponse, RusotoError<ListInputDevicesError>> {
        let request_uri = "/prod/inputDevices";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputDevicesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputDevicesError::from_response(response))
        }
    }

    /// <p>Produces a list of Input Security Groups for an account</p>
    #[allow(unused_mut)]
    async fn list_input_security_groups(
        &self,
        input: ListInputSecurityGroupsRequest,
    ) -> Result<ListInputSecurityGroupsResponse, RusotoError<ListInputSecurityGroupsError>> {
        let request_uri = "/prod/inputSecurityGroups";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputSecurityGroupsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputSecurityGroupsError::from_response(response))
        }
    }

    /// <p>Produces list of inputs that have been created</p>
    #[allow(unused_mut)]
    async fn list_inputs(
        &self,
        input: ListInputsRequest,
    ) -> Result<ListInputsResponse, RusotoError<ListInputsError>> {
        let request_uri = "/prod/inputs";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListInputsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListInputsError::from_response(response))
        }
    }

    /// <p>List the programs that currently exist for a specific multiplex.</p>
    #[allow(unused_mut)]
    async fn list_multiplex_programs(
        &self,
        input: ListMultiplexProgramsRequest,
    ) -> Result<ListMultiplexProgramsResponse, RusotoError<ListMultiplexProgramsError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/programs",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMultiplexProgramsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMultiplexProgramsError::from_response(response))
        }
    }

    /// <p>Retrieve a list of the existing multiplexes.</p>
    #[allow(unused_mut)]
    async fn list_multiplexes(
        &self,
        input: ListMultiplexesRequest,
    ) -> Result<ListMultiplexesResponse, RusotoError<ListMultiplexesError>> {
        let request_uri = "/prod/multiplexes";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListMultiplexesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListMultiplexesError::from_response(response))
        }
    }

    /// <p>List offerings available for purchase.</p>
    #[allow(unused_mut)]
    async fn list_offerings(
        &self,
        input: ListOfferingsRequest,
    ) -> Result<ListOfferingsResponse, RusotoError<ListOfferingsError>> {
        let request_uri = "/prod/offerings";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.channel_class {
            params.put("channelClass", x);
        }
        if let Some(ref x) = input.channel_configuration {
            params.put("channelConfiguration", x);
        }
        if let Some(ref x) = input.codec {
            params.put("codec", x);
        }
        if let Some(ref x) = input.duration {
            params.put("duration", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.maximum_bitrate {
            params.put("maximumBitrate", x);
        }
        if let Some(ref x) = input.maximum_framerate {
            params.put("maximumFramerate", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.resolution {
            params.put("resolution", x);
        }
        if let Some(ref x) = input.resource_type {
            params.put("resourceType", x);
        }
        if let Some(ref x) = input.special_feature {
            params.put("specialFeature", x);
        }
        if let Some(ref x) = input.video_quality {
            params.put("videoQuality", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListOfferingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListOfferingsError::from_response(response))
        }
    }

    /// <p>List purchased reservations.</p>
    #[allow(unused_mut)]
    async fn list_reservations(
        &self,
        input: ListReservationsRequest,
    ) -> Result<ListReservationsResponse, RusotoError<ListReservationsError>> {
        let request_uri = "/prod/reservations";

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.channel_class {
            params.put("channelClass", x);
        }
        if let Some(ref x) = input.codec {
            params.put("codec", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.maximum_bitrate {
            params.put("maximumBitrate", x);
        }
        if let Some(ref x) = input.maximum_framerate {
            params.put("maximumFramerate", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.resolution {
            params.put("resolution", x);
        }
        if let Some(ref x) = input.resource_type {
            params.put("resourceType", x);
        }
        if let Some(ref x) = input.special_feature {
            params.put("specialFeature", x);
        }
        if let Some(ref x) = input.video_quality {
            params.put("videoQuality", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListReservationsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListReservationsError::from_response(response))
        }
    }

    /// <p>Produces list of tags that have been created for a resource</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!(
            "/prod/tags/{resource_arn}",
            resource_arn = input.resource_arn
        );

        let mut request = SignedRequest::new("GET", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Purchase an offering and create a reservation.</p>
    #[allow(unused_mut)]
    async fn purchase_offering(
        &self,
        input: PurchaseOfferingRequest,
    ) -> Result<PurchaseOfferingResponse, RusotoError<PurchaseOfferingError>> {
        let request_uri = format!(
            "/prod/offerings/{offering_id}/purchase",
            offering_id = input.offering_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PurchaseOfferingResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PurchaseOfferingError::from_response(response))
        }
    }

    /// <p>Reject the transfer of the specified input device to your AWS account.</p>
    #[allow(unused_mut)]
    async fn reject_input_device_transfer(
        &self,
        input: RejectInputDeviceTransferRequest,
    ) -> Result<RejectInputDeviceTransferResponse, RusotoError<RejectInputDeviceTransferError>>
    {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}/reject",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RejectInputDeviceTransferResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RejectInputDeviceTransferError::from_response(response))
        }
    }

    /// <p>Starts an existing channel</p>
    #[allow(unused_mut)]
    async fn start_channel(
        &self,
        input: StartChannelRequest,
    ) -> Result<StartChannelResponse, RusotoError<StartChannelError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/start",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartChannelError::from_response(response))
        }
    }

    /// <p>Start (run) the multiplex. Starting the multiplex does not start the channels. You must explicitly start each channel.</p>
    #[allow(unused_mut)]
    async fn start_multiplex(
        &self,
        input: StartMultiplexRequest,
    ) -> Result<StartMultiplexResponse, RusotoError<StartMultiplexError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/start",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartMultiplexError::from_response(response))
        }
    }

    /// <p>Stops a running channel</p>
    #[allow(unused_mut)]
    async fn stop_channel(
        &self,
        input: StopChannelRequest,
    ) -> Result<StopChannelResponse, RusotoError<StopChannelError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/stop",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopChannelError::from_response(response))
        }
    }

    /// <p>Stops a running multiplex. If the multiplex isn&#39;t running, this action has no effect.</p>
    #[allow(unused_mut)]
    async fn stop_multiplex(
        &self,
        input: StopMultiplexRequest,
    ) -> Result<StopMultiplexResponse, RusotoError<StopMultiplexError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/stop",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StopMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StopMultiplexError::from_response(response))
        }
    }

    /// <p>Start an input device transfer to another AWS account. After you make the request, the other account must accept or reject the transfer.</p>
    #[allow(unused_mut)]
    async fn transfer_input_device(
        &self,
        input: TransferInputDeviceRequest,
    ) -> Result<TransferInputDeviceResponse, RusotoError<TransferInputDeviceError>> {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}/transfer",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("POST", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TransferInputDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TransferInputDeviceError::from_response(response))
        }
    }

    /// <p>Updates a channel.</p>
    #[allow(unused_mut)]
    async fn update_channel(
        &self,
        input: UpdateChannelRequest,
    ) -> Result<UpdateChannelResponse, RusotoError<UpdateChannelError>> {
        let request_uri = format!("/prod/channels/{channel_id}", channel_id = input.channel_id);

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateChannelResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateChannelError::from_response(response))
        }
    }

    /// <p>Changes the class of the channel.</p>
    #[allow(unused_mut)]
    async fn update_channel_class(
        &self,
        input: UpdateChannelClassRequest,
    ) -> Result<UpdateChannelClassResponse, RusotoError<UpdateChannelClassError>> {
        let request_uri = format!(
            "/prod/channels/{channel_id}/channelClass",
            channel_id = input.channel_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateChannelClassResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateChannelClassError::from_response(response))
        }
    }

    /// <p>Updates an input.</p>
    #[allow(unused_mut)]
    async fn update_input(
        &self,
        input: UpdateInputRequest,
    ) -> Result<UpdateInputResponse, RusotoError<UpdateInputError>> {
        let request_uri = format!("/prod/inputs/{input_id}", input_id = input.input_id);

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInputResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInputError::from_response(response))
        }
    }

    /// <p>Updates the parameters for the input device.</p>
    #[allow(unused_mut)]
    async fn update_input_device(
        &self,
        input: UpdateInputDeviceRequest,
    ) -> Result<UpdateInputDeviceResponse, RusotoError<UpdateInputDeviceError>> {
        let request_uri = format!(
            "/prod/inputDevices/{input_device_id}",
            input_device_id = input.input_device_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInputDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInputDeviceError::from_response(response))
        }
    }

    /// <p>Update an Input Security Group&#39;s Whilelists.</p>
    #[allow(unused_mut)]
    async fn update_input_security_group(
        &self,
        input: UpdateInputSecurityGroupRequest,
    ) -> Result<UpdateInputSecurityGroupResponse, RusotoError<UpdateInputSecurityGroupError>> {
        let request_uri = format!(
            "/prod/inputSecurityGroups/{input_security_group_id}",
            input_security_group_id = input.input_security_group_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateInputSecurityGroupResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateInputSecurityGroupError::from_response(response))
        }
    }

    /// <p>Updates a multiplex.</p>
    #[allow(unused_mut)]
    async fn update_multiplex(
        &self,
        input: UpdateMultiplexRequest,
    ) -> Result<UpdateMultiplexResponse, RusotoError<UpdateMultiplexError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}",
            multiplex_id = input.multiplex_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateMultiplexResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMultiplexError::from_response(response))
        }
    }

    /// <p>Update a program in a multiplex.</p>
    #[allow(unused_mut)]
    async fn update_multiplex_program(
        &self,
        input: UpdateMultiplexProgramRequest,
    ) -> Result<UpdateMultiplexProgramResponse, RusotoError<UpdateMultiplexProgramError>> {
        let request_uri = format!(
            "/prod/multiplexes/{multiplex_id}/programs/{program_name}",
            multiplex_id = input.multiplex_id,
            program_name = input.program_name
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateMultiplexProgramResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateMultiplexProgramError::from_response(response))
        }
    }

    /// <p>Update reservation.</p>
    #[allow(unused_mut)]
    async fn update_reservation(
        &self,
        input: UpdateReservationRequest,
    ) -> Result<UpdateReservationResponse, RusotoError<UpdateReservationError>> {
        let request_uri = format!(
            "/prod/reservations/{reservation_id}",
            reservation_id = input.reservation_id
        );

        let mut request = SignedRequest::new("PUT", "medialive", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateReservationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateReservationError::from_response(response))
        }
    }
}
