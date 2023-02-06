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
/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AAC. The service accepts one of two mutually exclusive groups of AAC settings--VBR and CBR. To select one of these modes, set the value of Bitrate control mode (rateControlMode) to &quot;VBR&quot; or &quot;CBR&quot;.  In VBR mode, you control the audio quality with the setting VBR quality (vbrQuality). In CBR mode, you use the setting Bitrate (bitrate). Defaults and valid values depend on the rate control mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AacSettings {
    /// <p>Choose BROADCASTER<em>MIXED</em>AD when the input contains pre-mixed main audio + audio description (AD) as a stereo pair. The value for AudioType will be set to 3, which signals to downstream systems that this stream contains &quot;broadcaster mixed AD&quot;. Note that the input received by the encoder must contain pre-mixed audio; the encoder does not perform the mixing. When you choose BROADCASTER<em>MIXED</em>AD, the encoder ignores any values you provide in AudioType and  FollowInputAudioType. Choose NORMAL when the input does not contain pre-mixed audio + audio description (AD). In this case, the encoder will use any values you provide for AudioType and FollowInputAudioType.</p>
    #[serde(rename = "audioDescriptionBroadcasterMix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_description_broadcaster_mix: Option<String>,
    /// <p>Specify the average bitrate in bits per second. The set of valid values for this setting is: 6000, 8000, 10000, 12000, 14000, 16000, 20000, 24000, 28000, 32000, 40000, 48000, 56000, 64000, 80000, 96000, 112000, 128000, 160000, 192000, 224000, 256000, 288000, 320000, 384000, 448000, 512000, 576000, 640000, 768000, 896000, 1024000. The value you set is also constrained by the values that you choose for Profile (codecProfile), Bitrate control mode (codingMode), and Sample rate (sampleRate). Default values depend on Bitrate control mode and Profile.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>AAC Profile.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Mono (Audio Description), Mono, Stereo, or 5.1 channel layout. Valid values depend on rate control mode and profile. &quot;1.0 - Audio Description (Receiver Mix)&quot; setting receives a stereo description plus control track and emits a mono AAC encode of the description track, with control data emitted in the PES header as per ETSI TS 101 154 Annex E.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Rate Control Mode.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Enables LATM/LOAS AAC output. Note that if you use LATM/LOAS AAC in an output, you must choose &quot;No container&quot; for the output container.</p>
    #[serde(rename = "rawFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_format: Option<String>,
    /// <p>Sample rate in Hz. Valid values depend on rate control mode and profile.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Use MPEG-2 AAC instead of MPEG-4 AAC audio for raw or MPEG-2 Transport Stream containers.</p>
    #[serde(rename = "specification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    /// <p>VBR Quality Level - Only used if rate<em>control</em>mode is VBR.</p>
    #[serde(rename = "vbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AC3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Ac3Settings {
    /// <p>Specify the average bitrate in bits per second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the AC-3 stream that the encoder emits. For more information about the AC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "bitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital coding mode. Determines number of channels.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital, dialnorm will be passed through.</p>
    #[serde(rename = "dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>Choose the Dolby Digital dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby Digital stream for the line operating mode. Related setting: When you use this setting, MediaConvert ignores any value you provide for Dynamic range compression profile (DynamicRangeCompressionProfile). For information about the Dolby Digital DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    /// <p>When you want to add Dolby dynamic range compression (DRC) signaling to your output stream, we recommend that you use the mode-specific settings instead of Dynamic range compression profile (DynamicRangeCompressionProfile). The mode-specific settings are Dynamic range compression profile, line mode (dynamicRangeCompressionLine) and Dynamic range compression profile, RF mode (dynamicRangeCompressionRf). Note that when you specify values for all three settings, MediaConvert ignores the value of this setting in favor of the mode-specific settings. If you do use this setting instead of the mode-specific settings, choose None (NONE) to leave out DRC signaling. Keep the default Film standard (FILM_STANDARD) to set the profile to Dolby&#39;s film standard profile for all operating modes.</p>
    #[serde(rename = "dynamicRangeCompressionProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_profile: Option<String>,
    /// <p>Choose the Dolby Digital dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby Digital stream for the RF operating mode. Related setting: When you use this setting, MediaConvert ignores any value you provide for Dynamic range compression profile (DynamicRangeCompressionProfile). For information about the Dolby Digital DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    /// <p>Applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid with 3<em>2</em>LFE coding mode.</p>
    #[serde(rename = "lfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>When set to FOLLOW_INPUT, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "metadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AccelerationSettings {
    /// <p>Specify the conditions when the service will run your job with accelerated transcoding.</p>
    #[serde(rename = "mode")]
    pub mode: String,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AIFF.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AiffSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "bitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Valid values are 1 and even numbers up to 64. For example, 1, 2, 4, 6, and so on, up to 64.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Settings for ancillary captions source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AncillarySourceSettings {
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
    #[serde(rename = "convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Specifies the 608 channel number in the ancillary data track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "sourceAncillaryChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ancillary_channel_number: Option<i64>,
    /// <p>By default, the service terminates any unterminated captions at the end of each input. If you want the caption to continue onto your next input, disable this setting.</p>
    #[serde(rename = "terminateCaptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_captions: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateCertificateRequest {
    /// <p>The ARN of the ACM certificate that you want to associate with your MediaConvert resource.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateCertificateResponse {}

/// <p>When you mimic a multi-channel audio layout with multiple mono-channel tracks, you can tag each channel layout manually. For example, you would tag the tracks that contain your left, right, and center audio with Left (L), Right (R), and Center (C), respectively. When you don&#39;t specify a value, MediaConvert labels your track as Center (C) by default. To use audio layout tagging, your output must be in a QuickTime (.mov) container; your audio codec must be AAC, WAV, or AIFF; and you must set up your audio track to have only one channel.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioChannelTaggingSettings {
    /// <p>You can add a tag for this mono-channel audio track to mimic its placement in a multi-channel layout.  For example, if this track is the left surround channel, choose Left surround (LS).</p>
    #[serde(rename = "channelTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_tag: Option<String>,
}

/// <p>Settings related to audio encoding. The settings in this group vary depending on the value that you choose for your audio codec.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioCodecSettings {
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AAC. The service accepts one of two mutually exclusive groups of AAC settings--VBR and CBR. To select one of these modes, set the value of Bitrate control mode (rateControlMode) to &quot;VBR&quot; or &quot;CBR&quot;.  In VBR mode, you control the audio quality with the setting VBR quality (vbrQuality). In CBR mode, you use the setting Bitrate (bitrate). Defaults and valid values depend on the rate control mode.</p>
    #[serde(rename = "aacSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aac_settings: Option<AacSettings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AC3.</p>
    #[serde(rename = "ac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ac_3_settings: Option<Ac3Settings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value AIFF.</p>
    #[serde(rename = "aiffSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aiff_settings: Option<AiffSettings>,
    /// <p>Choose the audio codec for this output. Note that the option Dolby Digital passthrough (PASSTHROUGH) applies only to Dolby Digital and Dolby Digital Plus audio inputs. Make sure that you choose a codec that&#39;s supported with your output container: https://docs.aws.amazon.com/mediaconvert/latest/ug/reference-codecs-containers.html#reference-codecs-containers-output-audio For audio-only outputs, make sure that both your input audio codec and your output audio codec are supported for audio-only workflows. For more information, see: https://docs.aws.amazon.com/mediaconvert/latest/ug/reference-codecs-containers-input.html#reference-codecs-containers-input-audio-only and https://docs.aws.amazon.com/mediaconvert/latest/ug/reference-codecs-containers.html#audio-only-output</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3_ATMOS.</p>
    #[serde(rename = "eac3AtmosSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_atmos_settings: Option<Eac3AtmosSettings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3.</p>
    #[serde(rename = "eac3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eac_3_settings: Option<Eac3Settings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value MP2.</p>
    #[serde(rename = "mp2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_2_settings: Option<Mp2Settings>,
    /// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value MP3.</p>
    #[serde(rename = "mp3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_3_settings: Option<Mp3Settings>,
    /// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value OPUS.</p>
    #[serde(rename = "opusSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opus_settings: Option<OpusSettings>,
    /// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value Vorbis.</p>
    #[serde(rename = "vorbisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vorbis_settings: Option<VorbisSettings>,
    /// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value WAV.</p>
    #[serde(rename = "wavSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_settings: Option<WavSettings>,
}

/// <p>Settings related to one audio tab on the MediaConvert console. In your job JSON, an instance of AudioDescription is equivalent to one audio tab in the console. Usually, one audio tab corresponds to one output audio track. Depending on how you set up your input audio selectors and whether you use audio selector groups, one audio tab can correspond to a group of output audio tracks.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioDescription {
    /// <p>When you mimic a multi-channel audio layout with multiple mono-channel tracks, you can tag each channel layout manually. For example, you would tag the tracks that contain your left, right, and center audio with Left (L), Right (R), and Center (C), respectively. When you don&#39;t specify a value, MediaConvert labels your track as Center (C) by default. To use audio layout tagging, your output must be in a QuickTime (.mov) container; your audio codec must be AAC, WAV, or AIFF; and you must set up your audio track to have only one channel.</p>
    #[serde(rename = "audioChannelTaggingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_tagging_settings: Option<AudioChannelTaggingSettings>,
    /// <p>Advanced audio normalization settings. Ignore these settings unless you need to comply with a loudness standard.</p>
    #[serde(rename = "audioNormalizationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_normalization_settings: Option<AudioNormalizationSettings>,
    /// <p>Specifies which audio data to use from each input. In the simplest case, specify an &quot;Audio Selector&quot;:#inputs-audio<em>selector by name based on its order within each input. For example if you specify &quot;Audio Selector 3&quot;, then the third audio selector will be used from each input. If an input does not have an &quot;Audio Selector 3&quot;, then the audio selector marked as &quot;default&quot; in that input will be used. If there is no audio selector marked as &quot;default&quot;, silence will be inserted for the duration of that input. Alternatively, an &quot;Audio Selector Group&quot;:#inputs-audio</em>selector<em>group name may be specified, with similar default/silence behavior. If no audio</em>source_name is specified, then &quot;Audio Selector 1&quot; will be chosen automatically.</p>
    #[serde(rename = "audioSourceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_source_name: Option<String>,
    /// <p>Applies only if Follow Input Audio Type is unchecked (false). A number between 0 and 255. The following are defined in ISO-IEC 13818-1: 0 = Undefined, 1 = Clean Effects, 2 = Hearing Impaired, 3 = Visually Impaired Commentary, 4-255 = Reserved.</p>
    #[serde(rename = "audioType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type: Option<i64>,
    /// <p>When set to FOLLOW<em>INPUT, if the input contains an ISO 639 audio</em>type, then that value is passed through to the output. If the input contains no ISO 639 audio<em>type, the value in Audio Type is included in the output. Otherwise the value in Audio Type is included in the output. Note that this field and audioType are both ignored if audioDescriptionBroadcasterMix is set to BROADCASTER</em>MIXED_AD.</p>
    #[serde(rename = "audioTypeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_type_control: Option<String>,
    /// <p>Settings related to audio encoding. The settings in this group vary depending on the value that you choose for your audio codec.</p>
    #[serde(rename = "codecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<AudioCodecSettings>,
    /// <p>Specify the language for this audio output track. The service puts this language code into your output audio track when you set Language code control (AudioLanguageCodeControl) to Use configured (USE<em>CONFIGURED). The service also uses your specified custom language code when you set Language code control (AudioLanguageCodeControl) to Follow input (FOLLOW</em>INPUT), but your input file doesn&#39;t specify a language code. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Indicates the language of the audio output track. The ISO 639 language specified in the &#39;Language Code&#39; drop down will be used when &#39;Follow Input Language Code&#39; is not selected or when &#39;Follow Input Language Code&#39; is selected but there is no ISO 639 language code specified by the input.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify which source for language code takes precedence for this audio track. When you choose Follow input (FOLLOW<em>INPUT), the service uses the language code from the input track if it&#39;s present. If there&#39;s no languge code on the input track, the service uses the code that you specify in the setting Language code (languageCode or customLanguageCode). When you choose Use configured (USE</em>CONFIGURED), the service uses the language code that you specify.</p>
    #[serde(rename = "languageCodeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code_control: Option<String>,
    /// <p>Advanced audio remixing settings.</p>
    #[serde(rename = "remixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    /// <p>Specify a label for this output audio stream. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "streamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
}

/// <p>Advanced audio normalization settings. Ignore these settings unless you need to comply with a loudness standard.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioNormalizationSettings {
    /// <p>Choose one of the following audio normalization algorithms: ITU-R BS.1770-1: Ungated loudness. A measurement of ungated average loudness for an entire piece of content, suitable for measurement of short-form content under ATSC recommendation A/85. Supports up to 5.1 audio channels. ITU-R BS.1770-2: Gated loudness. A measurement of gated average loudness compliant with the requirements of EBU-R128. Supports up to 5.1 audio channels. ITU-R BS.1770-3: Modified peak. The same loudness measurement algorithm as 1770-2, with an updated true peak measurement. ITU-R BS.1770-4: Higher channel count. Allows for more audio channels than the other algorithms, including configurations such as 7.1.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <p>When enabled the output audio is corrected using the chosen algorithm. If disabled, the audio will be measured but not adjusted.</p>
    #[serde(rename = "algorithmControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm_control: Option<String>,
    /// <p>Content measuring above this level will be corrected to the target level. Content measuring below this level will not be corrected.</p>
    #[serde(rename = "correctionGateLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_gate_level: Option<i64>,
    /// <p>If set to LOG, log each output&#39;s audio track loudness to a CSV file.</p>
    #[serde(rename = "loudnessLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness_logging: Option<String>,
    /// <p>If set to TRUE_PEAK, calculate and log the TruePeak for each output&#39;s audio track loudness.</p>
    #[serde(rename = "peakCalculation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peak_calculation: Option<String>,
    /// <p>When you use Audio normalization (AudioNormalizationSettings), optionally use this setting to specify a target loudness. If you don&#39;t specify a value here, the encoder chooses a value for you, based on the algorithm that you choose for Algorithm (algorithm). If you choose algorithm 1770-1, the encoder will choose -24 LKFS; otherwise, the encoder will choose -23 LKFS.</p>
    #[serde(rename = "targetLkfs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lkfs: Option<f64>,
}

/// <p>Use Audio selectors (AudioSelectors) to specify a track or set of tracks from the input that you will use in your outputs. You can use multiple Audio selectors per input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioSelector {
    /// <p>Selects a specific language code from within an audio source, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Enable this setting on one audio selector to set it as the default for the job. The service uses this default for outputs where it can&#39;t find the specified input audio. If you don&#39;t set a default, those outputs have no audio.</p>
    #[serde(rename = "defaultSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_selection: Option<String>,
    /// <p>Specifies audio data from an external file source.</p>
    #[serde(rename = "externalAudioFileInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_audio_file_input: Option<String>,
    /// <p>Settings specific to audio sources in an HLS alternate rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique audio track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the properties provided, the job fails. If no properties in hlsRenditionGroupSettings are specified, the default audio track within the video segment is chosen. If there is no audio within video segment, the alternative audio with DEFAULT=YES is chosen instead.</p>
    #[serde(rename = "hlsRenditionGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_rendition_group_settings: Option<HlsRenditionGroupSettings>,
    /// <p>Selects a specific language code from within an audio source.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specifies a time delta in milliseconds to offset the audio from the input video.</p>
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// <p>Selects a specific PID from within an audio source (e.g. 257 selects PID 0x101).</p>
    #[serde(rename = "pids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pids: Option<Vec<i64>>,
    /// <p>Use this setting for input streams that contain Dolby E, to have the service extract specific program data from the track. To select multiple programs, create multiple selectors with the same Track and different Program numbers. In the console, this setting is visible when you set Selector type to Track. Choose the program number from the dropdown list. If you are sending a JSON file, provide the program ID, which is part of the audio metadata. If your input file has incorrect metadata, you can choose All channels instead of a program number to have the service ignore the program IDs and include all the programs in the track.</p>
    #[serde(rename = "programSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_selection: Option<i64>,
    /// <p>Use these settings to reorder the audio channels of one input to match those of another input. This allows you to combine the two files into a single output, one after the other.</p>
    #[serde(rename = "remixSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remix_settings: Option<RemixSettings>,
    /// <p>Specifies the type of the audio selector.</p>
    #[serde(rename = "selectorType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector_type: Option<String>,
    /// <p>Identify a track from the input audio to include in this selector by entering the track index number. To include several tracks in a single audio selector, specify multiple tracks as follows. Using the console, enter a comma-separated list. For examle, type &quot;1,2,3&quot; to include tracks 1 through 3. Specifying directly in your JSON job file, provide the track numbers in an array. For example, &quot;tracks&quot;: [1,2,3].</p>
    #[serde(rename = "tracks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracks: Option<Vec<i64>>,
}

/// <p>Use audio selector groups to combine multiple sidecar audio inputs so that you can assign them to a single output audio tab (AudioDescription). Note that, if you&#39;re working with embedded audio, it&#39;s simpler to assign multiple input tracks into a single audio selector rather than use an audio selector group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AudioSelectorGroup {
    /// <p>Name of an Audio Selector within the same input to include in the group.  Audio selector names are standardized, based on their order within the input (e.g., &quot;Audio Selector 1&quot;). The audio selector name parameter can be repeated to add any number of audio selectors to the group.</p>
    #[serde(rename = "audioSelectorNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_names: Option<Vec<String>>,
}

/// <p>Use automated ABR to have MediaConvert set up the renditions in your ABR package for you automatically, based on characteristics of your input video. This feature optimizes video quality while minimizing the overall size of your ABR package.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutomatedAbrSettings {
    /// <p>Optional. The maximum target bit rate used in your automated ABR stack. Use this value to set an upper limit on the bandwidth consumed by the highest-quality rendition. This is the rendition that is delivered to viewers with the fastest internet connections. If you don&#39;t specify a value, MediaConvert uses 8,000,000 (8 mb/s) by default.</p>
    #[serde(rename = "maxAbrBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_abr_bitrate: Option<i64>,
    /// <p>Optional. The maximum number of renditions that MediaConvert will create in your automated ABR stack. The number of renditions is determined automatically, based on analysis of each job, but will never exceed this limit. When you set this to Auto in the console, which is equivalent to excluding it from your JSON job specification, MediaConvert defaults to a limit of 15.</p>
    #[serde(rename = "maxRenditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_renditions: Option<i64>,
    /// <p>Optional. The minimum target bitrate used in your automated ABR stack. Use this value to set a lower limit on the bitrate of video delivered to viewers with slow internet connections. If you don&#39;t specify a value, MediaConvert uses 600,000 (600 kb/s) by default.</p>
    #[serde(rename = "minAbrBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_abr_bitrate: Option<i64>,
}

/// <p>Use automated encoding to have MediaConvert choose your encoding settings for you, based on characteristics of your input video.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AutomatedEncodingSettings {
    /// <p>Use automated ABR to have MediaConvert set up the renditions in your ABR package for you automatically, based on characteristics of your input video. This feature optimizes video quality while minimizing the overall size of your ABR package.</p>
    #[serde(rename = "abrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abr_settings: Option<AutomatedAbrSettings>,
}

/// <p>Settings for quality-defined variable bitrate encoding with the AV1 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Av1QvbrSettings {
    /// <p>Required when you use QVBR rate control mode. That is, when you specify qvbrSettings within av1Settings. Specify the general target quality level for this output, from 1 to 10. Use higher numbers for greater quality. Level 10 results in nearly lossless compression. The quality level for most broadcast-quality transcodes is between 6 and 9. Optionally, to specify a value between whole numbers, also provide a value for the setting qvbrQualityLevelFineTune. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33.</p>
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
    /// <p>Optional. Specify a value here to set the QVBR quality to a level that is between whole numbers. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33. MediaConvert rounds your QVBR quality level to the nearest third of a whole number. For example, if you set qvbrQualityLevel to 7 and you set qvbrQualityLevelFineTune to .25, your actual QVBR quality level is 7.33.</p>
    #[serde(rename = "qvbrQualityLevelFineTune")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level_fine_tune: Option<f64>,
}

/// <p>Required when you set Codec, under VideoDescription&gt;CodecSettings to the value AV1.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Av1Settings {
    /// <p>Specify the strength of any adaptive quantization filters that you enable. The value that you choose here applies to Spatial adaptive quantization (spatialAdaptiveQuantization).</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Specify the GOP length (keyframe interval) in frames. With AV1, MediaConvert doesn&#39;t support GOP length in seconds. This value must be greater than zero and preferably equal to 1 + ((numberBFrames + 1) * x), where x is an integer value.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. Required when Rate control mode is QVBR.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Specify from the number of B-frames, in the range of 0-15. For AV1 encoding, we recommend using 7 or 15. Choose a larger number for a lower bitrate and smaller file size; choose a smaller number for better video quality.</p>
    #[serde(rename = "numberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Settings for quality-defined variable bitrate encoding with the AV1 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
    #[serde(rename = "qvbrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_settings: Option<Av1QvbrSettings>,
    /// <p>&#39;With AV1 outputs, for rate control mode, MediaConvert supports only quality-defined variable bitrate (QVBR). You can&#39;&#39;t use CBR or VBR.&#39;</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Specify the number of slices per picture. This value must be 1, 2, 4, 8, 16, or 32. For progressive pictures, this value must be less than or equal to the number of macroblock rows. For interlaced pictures, this value must be less than or equal to half the number of macroblock rows.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on spatial variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas that can sustain more distortion with no noticeable visual degradation and uses more bits on areas where any small distortion will be noticeable. For example, complex textured blocks are encoded with fewer bits and smooth textured blocks are encoded with more bits. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen with a lot of complex texture, you might choose to disable this feature. Related setting: When you enable spatial adaptive quantization, set the value for Adaptive quantization (adaptiveQuantization) depending on your content. For homogeneous content, such as cartoons and video games, set it to Low. For content with a wider variety of textures, set it to High or Higher.</p>
    #[serde(rename = "spatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
}

/// <p>Use ad avail blanking settings to specify your output content during SCTE-35 triggered ad avails. You can blank your video or overlay it with an image. MediaConvert also removes any audio and embedded captions during the ad avail. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/ad-avail-blanking.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvailBlanking {
    /// <p>Blanking image to be used. Leave empty for solid black. Only bmp and png images are supported.</p>
    #[serde(rename = "availBlankingImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking_image: Option<String>,
}

/// <p>Required when you choose AVC-Intra for your output video codec. For more information about the AVC-Intra settings, see the relevant specification. For detailed information about SD and HD in AVC-Intra, see https://ieeexplore.ieee.org/document/7290936. For information about 4K/2K in AVC-Intra, see https://pro-av.panasonic.net/en/avc-ultra/AVC-ULTRAoverview.pdf.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvcIntraSettings {
    /// <p>Specify the AVC-Intra class of your output. The AVC-Intra class selection determines the output video bit rate depending on the frame rate of the output. Outputs with higher class values have higher bitrates and improved image quality. Note that for Class 4K/2K, MediaConvert supports only 4:2:2 chroma subsampling.</p>
    #[serde(rename = "avcIntraClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avc_intra_class: Option<String>,
    /// <p>Optional when you set AVC-Intra class (avcIntraClass) to Class 4K/2K (CLASS<em>4K</em>2K). When you set AVC-Intra class to a different value, this object isn&#39;t allowed.</p>
    #[serde(rename = "avcIntraUhdSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avc_intra_uhd_settings: Option<AvcIntraUhdSettings>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output. When you enable slow PAL, MediaConvert relabels the video frames to 25 fps and resamples your audio to keep it synchronized with the video. Note that enabling this setting will slightly reduce the duration of your video. Required settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>When you do frame rate conversion from 23.976 frames per second (fps) to 29.97 fps, and your output scan type is interlaced, you can optionally enable hard telecine (HARD) to create a smoother picture. When you keep the default value, None (NONE), MediaConvert does a standard frame rate conversion to 29.97 without doing anything with the field polarity to create a smoother picture.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
}

/// <p>Optional when you set AVC-Intra class (avcIntraClass) to Class 4K/2K (CLASS<em>4K</em>2K). When you set AVC-Intra class to a different value, this object isn&#39;t allowed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AvcIntraUhdSettings {
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how many transcoding passes MediaConvert does with your video. When you choose Multi-pass (MULTI<em>PASS), your video quality is better and your output bitrate is more accurate. That is, the actual bitrate of your output is closer to the target bitrate defined in the specification. When you choose Single-pass (SINGLE</em>PASS), your encoding time is faster. The default behavior is Single-pass (SINGLE_PASS).</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
}

/// <p>Settings related to burn-in captions. Set up burn-in captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/burn-in-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to BURN_IN.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BurninDestinationSettings {
    /// <p>If no explicit x<em>position or y</em>position is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>Provide the font script, using an ISO 15924 script code, if the LanguageCode is not sufficient for determining the script type. Where LanguageCode or CustomLanguageCode is sufficient, use &quot;AUTOMATIC&quot; or leave unset. This is used to help determine the appropriate font for rendering burn-in captions.</p>
    #[serde(rename = "fontScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_script: Option<String>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    /// <p>Only applies to jobs with input captions in Teletext or STL formats. Specify whether the spacing between letters in your captions is set by the captions grid or varies depending on letter width. Choose fixed grid to conform to the spacing specified in the captions file more accurately. Choose proportional to make the text easier to read if the captions are closed caption.</p>
    #[serde(rename = "teletextSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_spacing: Option<String>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit x_position is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "xPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit y_position is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "yPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRequest {
    /// <p>The Job ID of the job to be cancelled.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobResponse {}

/// <p>This object holds groups of settings related to captions for one output. For each output that has captions, include one instance of CaptionDescriptions.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionDescription {
    /// <p>Specifies which &quot;Caption Selector&quot;:#inputs-caption_selector to use from each input when generating captions. The name should be of the format &quot;Caption Selector <N>&quot;, which denotes that the Nth Caption Selector will be used from each input.</p>
    #[serde(rename = "captionSelectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selector_name: Option<String>,
    /// <p>Specify the language for this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information when automatically selecting the font script for rendering the captions text. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Settings related to one captions tab on the MediaConvert console. In your job JSON, an instance of captions DestinationSettings is equivalent to one captions tab in the console. Usually, one captions tab corresponds to one output captions track. Depending on your output captions format, one tab might correspond to a set of output captions tracks. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/including-captions.html.</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    /// <p>Specify the language of this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information to choose the font language for rendering the captions text.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify a label for this set of output captions. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "languageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Caption Description for preset</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionDescriptionPreset {
    /// <p>Specify the language for this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information when automatically selecting the font script for rendering the captions text. For all outputs, you can use an ISO 639-2 or ISO 639-3 code. For streaming outputs, you can also use any other code in the full RFC-5646 specification. Streaming outputs are those that are in one of the following output groups: CMAF, DASH ISO, Apple HLS, or Microsoft Smooth Streaming.</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Settings related to one captions tab on the MediaConvert console. In your job JSON, an instance of captions DestinationSettings is equivalent to one captions tab in the console. Usually, one captions tab corresponds to one output captions track. Depending on your output captions format, one tab might correspond to a set of output captions tracks. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/including-captions.html.</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<CaptionDestinationSettings>,
    /// <p>Specify the language of this captions output track. For most captions output formats, the encoder puts this language information in the output captions metadata. If your output captions format is DVB-Sub or Burn in, the encoder uses this language information to choose the font language for rendering the captions text.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Specify a label for this set of output captions. For example, &quot;English&quot;, &quot;Director commentary&quot;, or &quot;track_2&quot;. For streaming outputs, MediaConvert passes this information into destination manifests for display on the end-viewer&#39;s player device. For outputs in other output groups, the service ignores this setting.</p>
    #[serde(rename = "languageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Settings related to one captions tab on the MediaConvert console. In your job JSON, an instance of captions DestinationSettings is equivalent to one captions tab in the console. Usually, one captions tab corresponds to one output captions track. Depending on your output captions format, one tab might correspond to a set of output captions tracks. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/including-captions.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionDestinationSettings {
    /// <p>Settings related to burn-in captions. Set up burn-in captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/burn-in-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to BURN_IN.</p>
    #[serde(rename = "burninDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub burnin_destination_settings: Option<BurninDestinationSettings>,
    /// <p>Specify the format for this set of captions on this output. The default format is embedded without SCTE-20. Note that your choice of video output container constrains your choice of output captions format. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/captions-support-tables.html. If you are using SCTE-20 and you want to create an output that complies with the SCTE-43 spec, choose SCTE-20 plus embedded (SCTE20<em>PLUS</em>EMBEDDED). To create a non-compliant output where the embedded captions come first, choose Embedded plus SCTE-20 (EMBEDDED<em>PLUS</em>SCTE20).</p>
    #[serde(rename = "destinationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
    /// <p>Settings related to DVB-Sub captions. Set up DVB-Sub captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/dvb-sub-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to DVB_SUB.</p>
    #[serde(rename = "dvbSubDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_destination_settings: Option<DvbSubDestinationSettings>,
    /// <p>Settings related to CEA/EIA-608 and CEA/EIA-708 (also called embedded or ancillary) captions. Set up embedded captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/embedded-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to EMBEDDED, EMBEDDED<em>PLUS</em>SCTE20, or SCTE20<em>PLUS</em>EMBEDDED.</p>
    #[serde(rename = "embeddedDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_destination_settings: Option<EmbeddedDestinationSettings>,
    /// <p>Settings related to IMSC captions. IMSC is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/ttml-and-webvtt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to IMSC.</p>
    #[serde(rename = "imscDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsc_destination_settings: Option<ImscDestinationSettings>,
    /// <p>Settings related to SCC captions. SCC is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/scc-srt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to SCC.</p>
    #[serde(rename = "sccDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_destination_settings: Option<SccDestinationSettings>,
    /// <p>Settings related to teletext captions. Set up teletext captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/teletext-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to TELETEXT.</p>
    #[serde(rename = "teletextDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_destination_settings: Option<TeletextDestinationSettings>,
    /// <p>Settings related to TTML captions. TTML is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/ttml-and-webvtt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to TTML.</p>
    #[serde(rename = "ttmlDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttml_destination_settings: Option<TtmlDestinationSettings>,
    /// <p>WEBVTT Destination Settings</p>
    #[serde(rename = "webvttDestinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt_destination_settings: Option<WebvttDestinationSettings>,
}

/// <p>Use captions selectors to specify the captions data from your input that you use in your outputs. You can use up to 20 captions selectors per input.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionSelector {
    /// <p>The specific language to extract from source, using the ISO 639-2 or ISO 639-3 three-letter language code. If input is SCTE-27, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub and output is Burn-in or SMPTE-TT, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub that is being passed through, omit this field (and PID field); there is no way to extract a specific language with pass-through captions.</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>The specific language to extract from source. If input is SCTE-27, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub and output is Burn-in or SMPTE-TT, complete this field and/or PID to select the caption language to extract. If input is DVB-Sub that is being passed through, omit this field (and PID field); there is no way to extract a specific language with pass-through captions.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>If your input captions are SCC, TTML, STL, SMI, SRT, or IMSC in an xml file, specify the URI of the input captions source file. If your input captions are IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
    #[serde(rename = "sourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_settings: Option<CaptionSourceSettings>,
}

/// <p>Ignore this setting unless your input captions format is SCC. To have the service compensate for differing frame rates between your input captions and input video, specify the frame rate of the captions file. Specify this value as a fraction, using the settings Framerate numerator (framerateNumerator) and Framerate denominator (framerateDenominator). For example, you might specify 24 / 1 for 24 fps, 25 / 1 for 25 fps, 24000 / 1001 for 23.976 fps, or 30000 / 1001 for 29.97 fps.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionSourceFramerate {
    /// <p>Specify the denominator of the fraction that represents the frame rate for the setting Caption source frame rate (CaptionSourceFramerate). Use this setting along with the setting Framerate numerator (framerateNumerator).</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Specify the numerator of the fraction that represents the frame rate for the setting Caption source frame rate (CaptionSourceFramerate). Use this setting along with the setting Framerate denominator (framerateDenominator).</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
}

/// <p>If your input captions are SCC, TTML, STL, SMI, SRT, or IMSC in an xml file, specify the URI of the input captions source file. If your input captions are IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CaptionSourceSettings {
    /// <p>Settings for ancillary captions source.</p>
    #[serde(rename = "ancillarySourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancillary_source_settings: Option<AncillarySourceSettings>,
    /// <p>DVB Sub Source Settings</p>
    #[serde(rename = "dvbSubSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_source_settings: Option<DvbSubSourceSettings>,
    /// <p>Settings for embedded captions Source</p>
    #[serde(rename = "embeddedSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_source_settings: Option<EmbeddedSourceSettings>,
    /// <p>If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
    #[serde(rename = "fileSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_source_settings: Option<FileSourceSettings>,
    /// <p>Use Source (SourceType) to identify the format of your input captions.  The service cannot auto-detect caption format.</p>
    #[serde(rename = "sourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>Settings specific to Teletext caption sources, including Page number.</p>
    #[serde(rename = "teletextSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_source_settings: Option<TeletextSourceSettings>,
    /// <p>Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.</p>
    #[serde(rename = "trackSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_source_settings: Option<TrackSourceSettings>,
    /// <p>Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.</p>
    #[serde(rename = "webvttHlsSourceSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webvtt_hls_source_settings: Option<WebvttHlsSourceSettings>,
}

/// <p>Channel mapping (ChannelMapping) contains the group of fields that hold the remixing value for each channel, in dB. Specify remix values to indicate how much of the content from your input audio channel you want in your output audio channels. Each instance of the InputChannels or InputChannelsFineTune array specifies these values for one output channel. Use one instance of this array for each output channel. In the console, each array corresponds to a column in the graphical depiction of the mapping matrix. The rows of the graphical matrix correspond to input channels. Valid values are within the range from -60 (mute) through 6. A setting of 0 passes the input channel unchanged to the output channel (no attenuation or amplification). Use InputChannels or InputChannelsFineTune to specify your remix values. Don&#39;t use both.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ChannelMapping {
    /// <p>In your JSON job specification, include one child of OutputChannels for each audio channel that you want in your output. Each child should contain one instance of InputChannels or InputChannelsFineTune.</p>
    #[serde(rename = "outputChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_channels: Option<Vec<OutputChannelMapping>>,
}

/// <p>Specify the details for each pair of HLS and DASH additional manifests that you want the service to generate for this CMAF output group. Each pair of manifests can reference a different subset of outputs in the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CmafAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your HLS group is film-name.m3u8. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.m3u8. For HLS output groups, specify a manifestNameModifier that is different from the nameModifier of the output. The service uses the output name modifier to create unique names for the individual variant manifests.</p>
    #[serde(rename = "manifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "selectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Settings for CMAF encryption</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CmafEncryptionSettings {
    /// <p>This is a 128-bit, 16-byte hex value represented by a 32-character text string. If this parameter is not set then the Initialization Vector will follow the segment number by default.</p>
    #[serde(rename = "constantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>Specify the encryption scheme that you want the service to use when encrypting your CMAF segments. Choose AES-CBC subsample (SAMPLE-AES) or AES_CTR (AES-CTR).</p>
    #[serde(rename = "encryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>When you use DRM with CMAF outputs, choose whether the service writes the 128-bit encryption initialization vector in the HLS and DASH manifests.</p>
    #[serde(rename = "initializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    /// <p>If your output group type is CMAF, use these settings when doing DRM encryption with a SPEKE-compliant key provider. If your output group type is HLS, DASH, or Microsoft Smooth, use the SpekeKeyProvider settings instead.</p>
    #[serde(rename = "spekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProviderCmaf>,
    /// <p>Use these settings to set up encryption with a static key provider.</p>
    #[serde(rename = "staticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    /// <p>Specify whether your DRM encryption key is static or from a key provider that follows the SPEKE standard. For more information about SPEKE, see https://docs.aws.amazon.com/speke/latest/documentation/what-is-speke.html.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Settings related to your CMAF output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to CMAF<em>GROUP</em>SETTINGS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CmafGroupSettings {
    /// <p>By default, the service creates one top-level .m3u8 HLS manifest and one top -level .mpd DASH manifest for each CMAF output group in your job. These default manifests reference every output in the output group. To create additional top-level manifests that reference a subset of the outputs in the output group, specify a list of them here. For each additional manifest that you specify, the service creates one HLS manifest and one DASH manifest.</p>
    #[serde(rename = "additionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<CmafAdditionalManifest>>,
    /// <p>A partial URI prefix that will be put in the manifest file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Disable this setting only when your workflow requires the #EXT-X-ALLOW-CACHE:no tag. Otherwise, keep the default value Enabled (ENABLED) and control caching in your video distribution set up. For example, use the Cache-Control http header.</p>
    #[serde(rename = "clientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    /// <p>Specification to use (RFC-6381 or the default RFC-4281) during m3u8 playlist generation.</p>
    #[serde(rename = "codecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>DRM settings.</p>
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<CmafEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "fragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Specify whether MediaConvert generates images for trick play. Keep the default value, None (NONE), to not generate any images. Choose Thumbnail (THUMBNAIL) to generate tiled thumbnails. Choose Thumbnail and full frame (THUMBNAIL<em>AND</em>FULLFRAME) to generate tiled thumbnails and full-resolution images of single frames. When you enable Write HLS manifest (WriteHlsManifest), MediaConvert creates a child manifest for each set of images that you generate and adds corresponding entries to the parent manifest. When you enable Write DASH manifest (WriteDashManifest), MediaConvert adds an entry in the .mpd manifest for each set of images that you generate. A common application for these images is Roku trick mode. The thumbnails and full-frame images that MediaConvert creates with this feature are compatible with this Roku specification: https://developer.roku.com/docs/developer-program/media-playback/trick-mode/hls-and-dash.md</p>
    #[serde(rename = "imageBasedTrickPlay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_based_trick_play: Option<String>,
    /// <p>When set to GZIP, compresses HLS playlist.</p>
    #[serde(rename = "manifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    /// <p>Indicates whether the output manifest should use floating point values for segment duration.</p>
    #[serde(rename = "manifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "minBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    /// <p>Keep this setting at the default value of 0, unless you are troubleshooting a problem with how devices play back the end of your video asset. If you know that player devices are hanging on the final segment of your video because the length of your final segment is too short, use this setting to specify a minimum final segment length, in seconds. Choose a value that is greater than or equal to 1 and less than your segment length. When you specify a value for this setting, the encoder will combine any final segment that is shorter than the length that you specify with the previous segment. For example, your segment length is 3 seconds and your final segment is .5 seconds without a minimum final segment length; when you set the minimum final segment length to 1, your final segment is 3.5 seconds.</p>
    #[serde(rename = "minFinalSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_final_segment_length: Option<f64>,
    /// <p>Specify whether your DASH profile is on-demand or main. When you choose Main profile (MAIN<em>PROFILE), the service signals  urn:mpeg:dash:profile:isoff-main:2011 in your .mpd DASH manifest. When you choose On-demand (ON</em>DEMAND<em>PROFILE), the service signals urn:mpeg:dash:profile:isoff-on-demand:2011 in your .mpd. When you choose On-demand, you must also set the output group setting Segment control (SegmentControl) to Single file (SINGLE</em>FILE).</p>
    #[serde(rename = "mpdProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_profile: Option<String>,
    /// <p>Use this setting only when your output video stream has B-frames, which causes the initial presentation time stamp (PTS) to be offset from the initial decode time stamp (DTS). Specify how MediaConvert handles PTS when writing time stamps in output DASH manifests. Choose Match initial PTS (MATCH<em>INITIAL</em>PTS) when you want MediaConvert to use the initial PTS as the first time stamp in the manifest. Choose Zero-based (ZERO_BASED) to have MediaConvert ignore the initial PTS in the video stream and instead write the initial time stamp as zero in the manifest. For outputs that don&#39;t have B-frames, the time stamps in your DASH manifests start at zero regardless of your choice here.</p>
    #[serde(rename = "ptsOffsetHandlingForBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts_offset_handling_for_b_frames: Option<String>,
    /// <p>When set to SINGLE<em>FILE, a single output file is generated, which is internally segmented using the Fragment Length and Segment Length. When set to SEGMENTED</em>FILES, separate segment files will be created.</p>
    #[serde(rename = "segmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Use this setting to specify the length, in seconds, of each individual CMAF segment. This value applies to the whole package; that is, to every output in the output group. Note that segments end on the first keyframe after this number of seconds, so the actual segment length might be slightly longer. If you set Segment control (CmafSegmentControl) to single file, the service puts the content of each output in a single file that has metadata that marks these segments. If you set it to segmented files, the service creates multiple files for each output, each with the content of one segment.</p>
    #[serde(rename = "segmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>Include or exclude RESOLUTION attribute for video in EXT-X-STREAM-INF tag of variant manifest.</p>
    #[serde(rename = "streamInfResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_inf_resolution: Option<String>,
    /// <p>When set to ENABLED, a DASH MPD manifest will be generated for this output.</p>
    #[serde(rename = "writeDashManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_dash_manifest: Option<String>,
    /// <p>When set to ENABLED, an Apple HLS manifest will be generated for this output.</p>
    #[serde(rename = "writeHlsManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_hls_manifest: Option<String>,
    /// <p>When you enable Precise segment duration in DASH manifests (writeSegmentTimelineInRepresentation), your DASH manifest shows precise segment durations. The segment duration information appears inside the SegmentTimeline element, inside SegmentTemplate at the Representation level. When this feature isn&#39;t enabled, the segment durations in your DASH manifest are approximate. The segment duration information appears in the duration attribute of the SegmentTemplate element.</p>
    #[serde(rename = "writeSegmentTimelineInRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_segment_timeline_in_representation: Option<String>,
}

/// <p>These settings relate to the fragmented MP4 container for the segments in your CMAF outputs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CmfcSettings {
    /// <p>Specify this setting only when your output will be consumed by a downstream repackaging workflow that is sensitive to very small duration differences between video and audio. For this situation, choose Match video duration (MATCH<em>VIDEO</em>DURATION). In all other cases, keep the default value, Default codec duration (DEFAULT<em>CODEC</em>DURATION). When you choose Match video duration, MediaConvert pads the output audio streams with silence or trims them to ensure that the total duration of each audio stream is at least as long as the total duration of the video stream. After padding or trimming, the audio stream duration is no more than one frame longer than the video stream. MediaConvert applies audio padding or trimming only to the end of the last segment of the output. For unsegmented outputs, MediaConvert adds padding only to the end of the file. When you keep the default value, any minor discrepancies between audio and video duration will depend on your output audio codec.</p>
    #[serde(rename = "audioDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<String>,
    /// <p>Specify the audio rendition group for this audio rendition. Specify up to one value for each audio output in your output group. This value appears in your HLS parent manifest in the EXT-X-MEDIA tag of TYPE=AUDIO, as the value for the GROUP-ID attribute. For example, if you specify &quot;audio<em>aac</em>1&quot; for Audio group ID, it appears in your manifest like this: #EXT-X-MEDIA:TYPE=AUDIO,GROUP-ID=&quot;audio<em>aac</em>1&quot;. Related setting: To associate the rendition group that this audio track belongs to with a video rendition, include the same value that you provide here for that video output&#39;s setting Audio rendition sets (audioRenditionSets).</p>
    #[serde(rename = "audioGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    /// <p>List the audio rendition groups that you want included with this video rendition. Use a comma-separated list. For example, say you want to include the audio rendition groups that have the audio group IDs &quot;audio<em>aac</em>1&quot; and &quot;audio<em>dolby&quot;. Then you would specify this value: &quot;audio</em>aac<em>1, audio</em>dolby&quot;. Related setting: The rendition groups that you include in your comma-separated list should all match values that you specify in the setting Audio group ID (AudioGroupId) for audio renditions in the same output group as this video rendition. Default behavior: If you don&#39;t specify anything here and for Audio group ID, MediaConvert puts each audio variant in its own audio rendition group and associates it with every video variant. Each value in your list appears in your HLS parent manifest in the EXT-X-STREAM-INF tag as the value for the AUDIO attribute. To continue the previous example, say that the file name for the child manifest for your video rendition is &quot;amazing<em>video</em>1.m3u8&quot;. Then, in your parent manifest, each value will appear on separate lines, like this: #EXT-X-STREAM-INF:AUDIO=&quot;audio<em>aac</em>1&quot;... amazing<em>video</em>1.m3u8 #EXT-X-STREAM-INF:AUDIO=&quot;audio<em>dolby&quot;... amazing</em>video_1.m3u8</p>
    #[serde(rename = "audioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    /// <p>Use this setting to control the values that MediaConvert puts in your HLS parent playlist to control how the client player selects which audio track to play. The other options for this setting determine the values that MediaConvert writes for the DEFAULT and AUTOSELECT attributes of the EXT-X-MEDIA entry for the audio variant. For more information about these attributes, see the Apple documentation article https://developer.apple.com/documentation/http<em>live</em>streaming/example<em>playlists</em>for<em>http</em>live<em>streaming/adding</em>alternate<em>media</em>to<em>a</em>playlist. Choose Alternate audio, auto select, default (ALTERNATE<em>AUDIO</em>AUTO<em>SELECT</em>DEFAULT) to set DEFAULT=YES and AUTOSELECT=YES. Choose this value for only one variant in your output group. Choose Alternate audio, auto select, not default (ALTERNATE<em>AUDIO</em>AUTO_SELECT) to set DEFAULT=NO and AUTOSELECT=YES. Choose Alternate Audio, Not Auto Select to set DEFAULT=NO and AUTOSELECT=NO. When you don&#39;t specify a value for this setting, MediaConvert defaults to Alternate audio, auto select, default. When there is more than one variant in your output group, you must explicitly choose a value for this setting.</p>
    #[serde(rename = "audioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    /// <p>Specify whether to flag this audio track as descriptive video service (DVS) in your HLS parent manifest. When you choose Flag (FLAG), MediaConvert includes the parameter CHARACTERISTICS=&quot;public.accessibility.describes-video&quot; in the EXT-X-MEDIA entry for this track. When you keep the default choice, Don&#39;t flag (DONT_FLAG), MediaConvert leaves this parameter out. The DVS flag can help with accessibility on Apple devices. For more information, see the Apple documentation.</p>
    #[serde(rename = "descriptiveVideoServiceFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_video_service_flag: Option<String>,
    /// <p>Choose Include (INCLUDE) to have MediaConvert generate an HLS child manifest that lists only the I-frames for this rendition, in addition to your regular manifest for this rendition. You might use this manifest as part of a workflow that creates preview functions for your video. MediaConvert adds both the I-frame only child manifest and the regular child manifest to the parent manifest. When you don&#39;t need the I-frame only child manifest, keep the default value Exclude (EXCLUDE).</p>
    #[serde(rename = "iFrameOnlyManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_frame_only_manifest: Option<String>,
    /// <p>Use this setting only when you specify SCTE-35 markers from ESAM. Choose INSERT to put SCTE-35 markers in this output at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<String>,
    /// <p>Ignore this setting unless you have SCTE-35 markers in your input video file. Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want those SCTE-35 markers in this output.</p>
    #[serde(rename = "scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
}

/// <p>Settings for color correction.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ColorCorrector {
    /// <p>Brightness level.</p>
    #[serde(rename = "brightness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<i64>,
    /// <p>Specify the color space you want for this output. The service supports conversion between HDR formats, between SDR formats, from SDR to HDR, and from HDR to SDR. SDR to HDR conversion doesn&#39;t upgrade the dynamic range. The converted video has an HDR format, but visually appears the same as an unconverted output. HDR to SDR conversion uses Elemental tone mapping technology to approximate the outcome of manually regrading from HDR to SDR.</p>
    #[serde(rename = "colorSpaceConversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_conversion: Option<String>,
    /// <p>Contrast level.</p>
    #[serde(rename = "contrast")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contrast: Option<i64>,
    /// <p>Use these settings when you convert to the HDR 10 color space. Specify the SMPTE ST 2086 Mastering Display Color Volume static metadata that you want signaled in the output. These values don&#39;t affect the pixel values that are encoded in the video stream. They are intended to help the downstream video player display content in a way that reflects the intentions of the the content creator. When you set Color space conversion (ColorSpaceConversion) to HDR 10 (FORCE_HDR10), these settings are required. You must set values for Max frame average light level (maxFrameAverageLightLevel) and Max content light level (maxContentLightLevel); these settings don&#39;t have a default value. The default values for the other HDR 10 metadata settings are defined by the P3D65 color space. For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
    #[serde(rename = "hdr10Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_metadata: Option<Hdr10Metadata>,
    /// <p>Hue in degrees.</p>
    #[serde(rename = "hue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<i64>,
    /// <p>Specify the video color sample range for this output. To create a full range output, you must start with a full range YUV input and keep the default value, None (NONE). To create a limited range output from a full range input, choose Limited range (LIMITED<em>RANGE</em>SQUEEZE). With RGB inputs, your output is always limited range, regardless of your choice here. When you create a limited range output from a full range input, MediaConvert limits the active pixel values in a way that depends on the output&#39;s bit depth: 8-bit outputs contain only values from 16 through 235 and 10-bit outputs contain only values from 64 through 940. With this conversion, MediaConvert also changes the output metadata to note the limited range.</p>
    #[serde(rename = "sampleRangeConversion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_range_conversion: Option<String>,
    /// <p>Saturation level.</p>
    #[serde(rename = "saturation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturation: Option<i64>,
}

/// <p>Container specific settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerSettings {
    /// <p>These settings relate to the fragmented MP4 container for the segments in your CMAF outputs.</p>
    #[serde(rename = "cmfcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmfc_settings: Option<CmfcSettings>,
    /// <p>Container for this output. Some containers require a container settings object. If not specified, the default object will be created.</p>
    #[serde(rename = "container")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    /// <p>Settings for F4v container</p>
    #[serde(rename = "f4vSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_4v_settings: Option<F4vSettings>,
    /// <p>MPEG-2 TS container settings. These apply to outputs in a File output group when the output&#39;s container (ContainerType) is MPEG-2 Transport Stream (M2TS). In these assets, data is organized by the program map table (PMT). Each transport stream program contains subsets of data, including audio, video, and metadata. Each of these subsets of data has a numerical label called a packet identifier (PID). Each transport stream program corresponds to one MediaConvert output. The PMT lists the types of data in a program along with their PID. Downstream systems and players use the program map table to look up the PID for each type of data it accesses and then uses the PIDs to locate specific data within the asset.</p>
    #[serde(rename = "m2tsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_2ts_settings: Option<M2tsSettings>,
    /// <p>These settings relate to the MPEG-2 transport stream (MPEG2-TS) container for the MPEG2-TS segments in your HLS outputs.</p>
    #[serde(rename = "m3u8Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub m_3u_8_settings: Option<M3u8Settings>,
    /// <p>These settings relate to your QuickTime MOV output container.</p>
    #[serde(rename = "movSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mov_settings: Option<MovSettings>,
    /// <p>These settings relate to your MP4 output container. You can create audio only outputs with this container. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/supported-codecs-containers-audio-only.html#output-codecs-and-containers-supported-for-audio-only.</p>
    #[serde(rename = "mp4Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_settings: Option<Mp4Settings>,
    /// <p>These settings relate to the fragmented MP4 container for the segments in your DASH outputs.</p>
    #[serde(rename = "mpdSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_settings: Option<MpdSettings>,
    /// <p>These settings relate to your MXF output container.</p>
    #[serde(rename = "mxfSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mxf_settings: Option<MxfSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobRequest {
    /// <p>Optional. Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "accelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>Optional. Choose a tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up. Any transcoding outputs that don&#39;t have an associated tag will appear in your billing report unsorted. If you don&#39;t choose a valid value for this field, your job outputs will appear on the billing report unsorted.</p>
    #[serde(rename = "billingTagsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tags_source: Option<String>,
    /// <p>Optional. Idempotency token for CreateJob operation.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "hopDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// <p>Optional. When you create a job, you can either specify a job template or specify the transcoding settings individually.</p>
    #[serde(rename = "jobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>Optional. Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>Required. The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html.</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p>JobSettings contains all the transcode settings for a job.</p>
    #[serde(rename = "settings")]
    pub settings: JobSettings,
    /// <p>Optional. Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.</p>
    #[serde(rename = "simulateReservedQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulate_reserved_queue: Option<String>,
    /// <p>Optional. Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "statusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>Optional. The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.  Use standard AWS tags on your job for automatic integration with AWS services and for custom integrations and workflows.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>Optional. User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.  Use only for existing integrations or workflows that rely on job metadata tags. Otherwise, we recommend that you use standard AWS tags.</p>
    #[serde(rename = "userMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobResponse {
    /// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateJobTemplateRequest {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "accelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>Optional. A category for the job template you are creating</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. A description of the job template you are creating.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Optional. Use queue hopping to avoid overly long waits in the backlog of the queue that you submit your job to. Specify an alternate queue and the maximum time that your job will wait in the initial queue before hopping. For more information about this feature, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "hopDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// <p>The name of the job template you are creating.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "settings")]
    pub settings: JobTemplateSettings,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "statusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "jobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreatePresetRequest {
    /// <p>Optional. A category for the preset you are creating.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. A description of the preset you are creating.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the preset you are creating.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Settings for preset</p>
    #[serde(rename = "settings")]
    pub settings: PresetSettings,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreatePresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateQueueRequest {
    /// <p>Optional. A description of the queue that you are creating.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue that you are creating.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment. When you use the API to create a queue, the default is on-demand.</p>
    #[serde(rename = "pricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,
    /// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
    #[serde(rename = "reservationPlanSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan_settings: Option<ReservationPlanSettings>,
    /// <p>Initial state of the queue. If you create a paused queue, then jobs in that queue won&#39;t begin.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Specify the details for each additional DASH manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your DASH group is film-name.mpd. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.mpd.</p>
    #[serde(rename = "manifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "selectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Specifies DRM settings for DASH outputs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashIsoEncryptionSettings {
    /// <p>This setting can improve the compatibility of your output with video players on obsolete devices. It applies only to DASH H.264 outputs with DRM encryption. Choose Unencrypted SEI (UNENCRYPTED<em>SEI) only to correct problems with playback on older devices. Otherwise, keep the default setting CENC v1 (CENC</em>V1). If you choose Unencrypted SEI, for that output, the service will exclude the access unit delimiter and will leave the SEI NAL units unencrypted.</p>
    #[serde(rename = "playbackDeviceCompatibility")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback_device_compatibility: Option<String>,
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "spekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
}

/// <p>Settings related to your DASH output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to DASH<em>ISO</em>GROUP_SETTINGS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DashIsoGroupSettings {
    /// <p>By default, the service creates one .mpd DASH manifest for each DASH ISO output group in your job. This default manifest references every output in the output group. To create additional DASH manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "additionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<DashAdditionalManifest>>,
    /// <p>Use this setting only when your audio codec is a Dolby one (AC3, EAC3, or Atmos) and your downstream workflow requires that your DASH manifest use the Dolby channel configuration tag, rather than the MPEG one. For example, you might need to use this to make dynamic ad insertion work. Specify which audio channel configuration scheme ID URI MediaConvert writes in your DASH manifest. Keep the default value, MPEG channel configuration (MPEG<em>CHANNEL</em>CONFIGURATION), to have MediaConvert write this: urn:mpeg:mpegB:cicp:ChannelConfiguration. Choose Dolby channel configuration (DOLBY<em>CHANNEL</em>CONFIGURATION) to have MediaConvert write this instead: tag:dolby.com,2014:dash:audio<em>channel</em>configuration:2011.</p>
    #[serde(rename = "audioChannelConfigSchemeIdUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channel_config_scheme_id_uri: Option<String>,
    /// <p>A partial URI prefix that will be put in the manifest (.mpd) file at the top level BaseURL element. Can be used if streams are delivered from a different URL than the manifest file.</p>
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>DRM settings.</p>
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<DashIsoEncryptionSettings>,
    /// <p>Length of fragments to generate (in seconds). Fragment length must be compatible with GOP size and Framerate. Note that fragments will end on the next keyframe after this number of seconds, so actual fragment length may be longer. When Emit Single File is checked, the fragmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "fragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Supports HbbTV specification as indicated</p>
    #[serde(rename = "hbbtvCompliance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hbbtv_compliance: Option<String>,
    /// <p>Specify whether MediaConvert generates images for trick play. Keep the default value, None (NONE), to not generate any images. Choose Thumbnail (THUMBNAIL) to generate tiled thumbnails. Choose Thumbnail and full frame (THUMBNAIL<em>AND</em>FULLFRAME) to generate tiled thumbnails and full-resolution images of single frames. MediaConvert adds an entry in the .mpd manifest for each set of images that you generate. A common application for these images is Roku trick mode. The thumbnails and full-frame images that MediaConvert creates with this feature are compatible with this Roku specification: https://developer.roku.com/docs/developer-program/media-playback/trick-mode/hls-and-dash.md</p>
    #[serde(rename = "imageBasedTrickPlay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_based_trick_play: Option<String>,
    /// <p>Minimum time of initially buffered media that is needed to ensure smooth playout.</p>
    #[serde(rename = "minBufferTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_buffer_time: Option<i64>,
    /// <p>Keep this setting at the default value of 0, unless you are troubleshooting a problem with how devices play back the end of your video asset. If you know that player devices are hanging on the final segment of your video because the length of your final segment is too short, use this setting to specify a minimum final segment length, in seconds. Choose a value that is greater than or equal to 1 and less than your segment length. When you specify a value for this setting, the encoder will combine any final segment that is shorter than the length that you specify with the previous segment. For example, your segment length is 3 seconds and your final segment is .5 seconds without a minimum final segment length; when you set the minimum final segment length to 1, your final segment is 3.5 seconds.</p>
    #[serde(rename = "minFinalSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_final_segment_length: Option<f64>,
    /// <p>Specify whether your DASH profile is on-demand or main. When you choose Main profile (MAIN<em>PROFILE), the service signals  urn:mpeg:dash:profile:isoff-main:2011 in your .mpd DASH manifest. When you choose On-demand (ON</em>DEMAND<em>PROFILE), the service signals urn:mpeg:dash:profile:isoff-on-demand:2011 in your .mpd. When you choose On-demand, you must also set the output group setting Segment control (SegmentControl) to Single file (SINGLE</em>FILE).</p>
    #[serde(rename = "mpdProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpd_profile: Option<String>,
    /// <p>Use this setting only when your output video stream has B-frames, which causes the initial presentation time stamp (PTS) to be offset from the initial decode time stamp (DTS). Specify how MediaConvert handles PTS when writing time stamps in output DASH manifests. Choose Match initial PTS (MATCH<em>INITIAL</em>PTS) when you want MediaConvert to use the initial PTS as the first time stamp in the manifest. Choose Zero-based (ZERO_BASED) to have MediaConvert ignore the initial PTS in the video stream and instead write the initial time stamp as zero in the manifest. For outputs that don&#39;t have B-frames, the time stamps in your DASH manifests start at zero regardless of your choice here.</p>
    #[serde(rename = "ptsOffsetHandlingForBFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pts_offset_handling_for_b_frames: Option<String>,
    /// <p>When set to SINGLE<em>FILE, a single output file is generated, which is internally segmented using the Fragment Length and Segment Length. When set to SEGMENTED</em>FILES, separate segment files will be created.</p>
    #[serde(rename = "segmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of mpd segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer. When Emit Single File is checked, the segmentation is internal to a single output file and it does not cause the creation of many output files as in other output types.</p>
    #[serde(rename = "segmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>If you get an HTTP error in the 400 range when you play back your DASH output, enable this setting and run your transcoding job again. When you enable this setting, the service writes precise segment durations in the DASH manifest. The segment duration information appears inside the SegmentTimeline element, inside SegmentTemplate at the Representation level. When you don&#39;t enable this setting, the service writes approximate segment durations in your DASH manifest.</p>
    #[serde(rename = "writeSegmentTimelineInRepresentation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_segment_timeline_in_representation: Option<String>,
}

/// <p>Settings for deinterlacer</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Deinterlacer {
    /// <p>Only applies when you set Deinterlacer (DeinterlaceMode) to Deinterlace (DEINTERLACE) or Adaptive (ADAPTIVE). Motion adaptive interpolate (INTERPOLATE) produces sharper pictures, while blend (BLEND) produces smoother motion. Use (INTERPOLATE<em>TICKER) OR (BLEND</em>TICKER) if your source file includes a ticker, such as a scrolling headline at the bottom of the frame.</p>
    #[serde(rename = "algorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// <ul>
    /// <li>When set to NORMAL (default), the deinterlacer does not convert frames that are tagged  in metadata as progressive. It will only convert those that are tagged as some other type. - When set to FORCE<em>ALL</em>FRAMES, the deinterlacer converts every frame to progressive - even those that are already tagged as progressive. Turn Force mode on only if there is  a good chance that the metadata has tagged frames as progressive when they are not  progressive. Do not turn on otherwise; processing frames that are already progressive  into progressive will probably result in lower quality video.</li>
    /// </ul>
    #[serde(rename = "control")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<String>,
    /// <p>Use Deinterlacer (DeinterlaceMode) to choose how the service will do deinterlacing. Default is Deinterlace. - Deinterlace converts interlaced to progressive. - Inverse telecine converts Hard Telecine 29.97i to progressive 23.976p. - Adaptive auto-detects and converts to progressive.</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteJobTemplateRequest {
    /// <p>The name of the job template to be deleted.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteJobTemplateResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeletePresetRequest {
    /// <p>The name of the preset to be deleted.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeletePresetResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteQueueRequest {
    /// <p>The name of the queue that you want to delete.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteQueueResponse {}

/// <p>DescribeEndpointsRequest</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointsRequest {
    /// <p>Optional. Max number of endpoints, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Optional field, defaults to DEFAULT. Specify DEFAULT for this operation to return your endpoints if any exist, or to create an endpoint for you and return it if one doesn&#39;t already exist. Specify GET_ONLY to return your endpoints if any exist, or an empty list if none exist.</p>
    #[serde(rename = "mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of endpoints.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>List of endpoints</p>
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// <p>Use this string to request the next batch of endpoints.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Settings associated with the destination. Will vary based on the type of destination</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DestinationSettings {
    /// <p>Settings associated with S3 destination</p>
    #[serde(rename = "s3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3DestinationSettings>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateCertificateRequest {
    /// <p>The ARN of the ACM certificate that you want to disassociate from your MediaConvert resource.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateCertificateResponse {}

/// <p>With AWS Elemental MediaConvert, you can create profile 5 Dolby Vision outputs from MXF and IMF sources that contain mastering information as frame-interleaved Dolby Vision metadata.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DolbyVision {
    /// <p>Use these settings when you set DolbyVisionLevel6Mode to SPECIFY to override the MaxCLL and MaxFALL values in your input with new values.</p>
    #[serde(rename = "l6Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l6_metadata: Option<DolbyVisionLevel6Metadata>,
    /// <p>Use Dolby Vision Mode to choose how the service will handle Dolby Vision MaxCLL and MaxFALL properies.</p>
    #[serde(rename = "l6Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l6_mode: Option<String>,
    /// <p>In the current MediaConvert implementation, the Dolby Vision profile is always 5 (PROFILE_5). Therefore, all of your inputs must contain Dolby Vision frame interleaved data.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}

/// <p>Use these settings when you set DolbyVisionLevel6Mode to SPECIFY to override the MaxCLL and MaxFALL values in your input with new values.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DolbyVisionLevel6Metadata {
    /// <p>Maximum Content Light Level. Static HDR metadata that corresponds to the brightest pixel in the entire stream. Measured in nits.</p>
    #[serde(rename = "maxCll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cll: Option<i64>,
    /// <p>Maximum Frame-Average Light Level. Static HDR metadata that corresponds to the highest frame-average brightness in the entire stream. Measured in nits.</p>
    #[serde(rename = "maxFall")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_fall: Option<i64>,
}

/// <p>Use these settings to insert a DVB Network Information Table (NIT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbNitSettings {
    /// <p>The numeric value placed in the Network Information Table (NIT).</p>
    #[serde(rename = "networkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_id: Option<i64>,
    /// <p>The network name text placed in the network<em>name</em>descriptor inside the Network Information Table. Maximum length is 256 characters.</p>
    #[serde(rename = "networkName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_name: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "nitInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nit_interval: Option<i64>,
}

/// <p>Use these settings to insert a DVB Service Description Table (SDT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSdtSettings {
    /// <p>Selects method of inserting SDT information into output stream.  &quot;Follow input SDT&quot; copies SDT information from input stream to  output stream. &quot;Follow input SDT if present&quot; copies SDT information from  input stream to output stream if SDT information is present in the input, otherwise it will fall back on the user-defined values. Enter &quot;SDT  Manually&quot; means user will enter the SDT information. &quot;No SDT&quot; means output  stream will not contain SDT information.</p>
    #[serde(rename = "outputSdt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_sdt: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "sdtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdt_interval: Option<i64>,
    /// <p>The service name placed in the service_descriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "serviceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
    /// <p>The service provider name placed in the service_descriptor in the Service Description Table. Maximum length is 256 characters.</p>
    #[serde(rename = "serviceProviderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_provider_name: Option<String>,
}

/// <p>Settings related to DVB-Sub captions. Set up DVB-Sub captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/dvb-sub-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to DVB_SUB.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSubDestinationSettings {
    /// <p>If no explicit x<em>position or y</em>position is provided, setting alignment to centered will place the captions at the bottom center of the output. Similarly, setting a left alignment will align captions to the bottom left of the output. If x and y positions are given in conjunction with the alignment parameter, the font will be justified (either left or centered) relative to those coordinates. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "alignment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<String>,
    /// <p>Specifies the color of the rectangle behind the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// <p>Specifies the opacity of the background rectangle. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "backgroundOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_opacity: Option<i64>,
    /// <p>Specify how MediaConvert handles the display definition segment (DDS). Keep the default, None (NONE), to exclude the DDS from this set of captions. Choose No display window (NO<em>DISPLAY</em>WINDOW) to have MediaConvert include the DDS but not include display window data. In this case, MediaConvert writes that information to the page composition segment (PCS) instead. Choose Specify (SPECIFIED) to have MediaConvert set up the display window based on the values that you specify in related job settings. For video resolutions that are 576 pixels or smaller in height, MediaConvert doesn&#39;t include the DDS, regardless of the value you choose for DDS handling (ddsHandling). In this case, it doesn&#39;t write the display window data to the PCS either. Related settings: Use the settings DDS x-coordinate (ddsXCoordinate) and DDS y-coordinate (ddsYCoordinate) to specify the offset between the top left corner of the display window and the top left corner of the video frame. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ddsHandling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dds_handling: Option<String>,
    /// <p>Use this setting, along with DDS y-coordinate (ddsYCoordinate), to specify the upper left corner of the display definition segment (DDS) display window. With this setting, specify the distance, in pixels, between the left side of the frame and the left side of the DDS display window. Keep the default value, 0, to have MediaConvert automatically choose this offset. Related setting: When you use this setting, you must set DDS handling (ddsHandling) to a value other than None (NONE). MediaConvert uses these values to determine whether to write page position data to the DDS or to the page composition segment (PCS). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ddsXCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dds_x_coordinate: Option<i64>,
    /// <p>Use this setting, along with DDS x-coordinate (ddsXCoordinate), to specify the upper left corner of the display definition segment (DDS) display window. With this setting, specify the distance, in pixels, between the top of the frame and the top of the DDS display window. Keep the default value, 0, to have MediaConvert automatically choose this offset. Related setting: When you use this setting, you must set DDS handling (ddsHandling) to a value other than None (NONE). MediaConvert uses these values to determine whether to write page position data to the DDS or to the page composition segment (PCS). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "ddsYCoordinate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dds_y_coordinate: Option<i64>,
    /// <p>Specifies the color of the burned-in captions. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// <p>Specifies the opacity of the burned-in captions. 255 is opaque; 0 is transparent.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_opacity: Option<i64>,
    /// <p>Font resolution in DPI (dots per inch); default is 96 dpi.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontResolution")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_resolution: Option<i64>,
    /// <p>Provide the font script, using an ISO 15924 script code, if the LanguageCode is not sufficient for determining the script type. Where LanguageCode or CustomLanguageCode is sufficient, use &quot;AUTOMATIC&quot; or leave unset. This is used to help determine the appropriate font for rendering DVB-Sub captions.</p>
    #[serde(rename = "fontScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_script: Option<String>,
    /// <p>A positive integer indicates the exact font size in points. Set to 0 for automatic font size selection. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// <p>Specify the height, in pixels, of this set of DVB-Sub captions. The default value is 576 pixels. Related setting: When you use this setting, you must set DDS handling (ddsHandling) to a value other than None (NONE). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Specifies font outline color. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_color: Option<String>,
    /// <p>Specifies font outline size in pixels. This option is not valid for source captions that are either 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "outlineSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outline_size: Option<i64>,
    /// <p>Specifies the color of the shadow cast by the captions.
    /// All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowColor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_color: Option<String>,
    /// <p>Specifies the opacity of the shadow. 255 is opaque; 0 is transparent. Leaving this parameter blank is equivalent to setting it to 0 (transparent). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowOpacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_opacity: Option<i64>,
    /// <p>Specifies the horizontal offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels to the left. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowXOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_x_offset: Option<i64>,
    /// <p>Specifies the vertical offset of the shadow relative to the captions in pixels. A value of -2 would result in a shadow offset 2 pixels above the text. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "shadowYOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_y_offset: Option<i64>,
    /// <p>Specify whether your DVB subtitles are standard or for hearing impaired. Choose hearing impaired if your subtitles include audio descriptions and dialogue. Choose standard if your subtitles include only dialogue.</p>
    #[serde(rename = "subtitlingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitling_type: Option<String>,
    /// <p>Only applies to jobs with input captions in Teletext or STL formats. Specify whether the spacing between letters in your captions is set by the captions grid or varies depending on letter width. Choose fixed grid to conform to the spacing specified in the captions file more accurately. Choose proportional to make the text easier to read if the captions are closed caption.</p>
    #[serde(rename = "teletextSpacing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub teletext_spacing: Option<String>,
    /// <p>Specify the width, in pixels, of this set of DVB-Sub captions. The default value is 720 pixels. Related setting: When you use this setting, you must set DDS handling (ddsHandling) to a value other than None (NONE). All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// <p>Specifies the horizontal position of the caption relative to the left side of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the left of the output. If no explicit x_position is provided, the horizontal caption position will be determined by the alignment parameter. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "xPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x_position: Option<i64>,
    /// <p>Specifies the vertical position of the caption relative to the top of the output in pixels. A value of 10 would result in the captions starting 10 pixels from the top of the output. If no explicit y_position is provided, the caption will be positioned towards the bottom of the output. This option is not valid for source captions that are STL, 608/embedded or teletext. These source settings are already pre-defined by the caption stream. All burn-in and DVB-Sub font settings must match.</p>
    #[serde(rename = "yPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y_position: Option<i64>,
}

/// <p>DVB Sub Source Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbSubSourceSettings {
    /// <p>When using DVB-Sub with Burn-In or SMPTE-TT, use this PID for the source content. Unused for DVB-Sub passthrough. All DVB-Sub content is passed through, regardless of selectors.</p>
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
}

/// <p>Use these settings to insert a DVB Time and Date Table (TDT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DvbTdtSettings {
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "tdtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tdt_interval: Option<i64>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3_ATMOS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Eac3AtmosSettings {
    /// <p>Specify the average bitrate for this output in bits per second. Valid values: 384k, 448k, 576k, 640k, 768k, 1024k Default value: 448k Note that MediaConvert supports 384k only with channel-based immersive (CBI) 7.1.4 and 5.1.4 inputs. For CBI 9.1.6 and other input types, MediaConvert automatically increases your output bitrate to 448k.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the E-AC-3 stream that the encoder emits. For more information about the EAC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "bitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>The coding mode for Dolby Digital Plus JOC (Atmos).</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Enable Dolby Dialogue Intelligence to adjust loudness based on dialogue analysis.</p>
    #[serde(rename = "dialogueIntelligence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialogue_intelligence: Option<String>,
    /// <p>Specify whether MediaConvert should use any downmix metadata from your input file. Keep the default value, Custom (SPECIFIED) to provide downmix values in your job settings. Choose Follow source (INITIALIZE<em>FROM</em>SOURCE) to use the metadata from your input. Related settings--Use these settings to specify your downmix values: Left only/Right only surround (LoRoSurroundMixLevel), Left total/Right total surround (LtRtSurroundMixLevel), Left total/Right total center (LtRtCenterMixLevel), Left only/Right only center (LoRoCenterMixLevel),  and Stereo downmix (StereoDownmix). When you keep Custom (SPECIFIED) for Downmix control (DownmixControl) and you don&#39;t specify values for the related settings, MediaConvert uses default values for those settings.</p>
    #[serde(rename = "downmixControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub downmix_control: Option<String>,
    /// <p>Choose the Dolby dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby stream for the line operating mode. Default value: Film light (ATMOS<em>STORAGE</em>DDP<em>COMPR</em>FILM_LIGHT) Related setting: To have MediaConvert use the value you specify here, keep the default value, Custom (SPECIFIED) for the setting Dynamic range control (DynamicRangeControl). Otherwise, MediaConvert ignores Dynamic range compression line (DynamicRangeCompressionLine). For information about the Dolby DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    /// <p>Choose the Dolby dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby stream for the RF operating mode. Default value: Film light (ATMOS<em>STORAGE</em>DDP<em>COMPR</em>FILM_LIGHT) Related setting: To have MediaConvert use the value you specify here, keep the default value, Custom (SPECIFIED) for the setting Dynamic range control (DynamicRangeControl). Otherwise, MediaConvert ignores Dynamic range compression RF (DynamicRangeCompressionRf). For information about the Dolby DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    /// <p>Specify whether MediaConvert should use any dynamic range control metadata from your input file. Keep the default value, Custom (SPECIFIED), to provide dynamic range control values in your job settings. Choose Follow source (INITIALIZE<em>FROM</em>SOURCE) to use the metadata from your input. Related settings--Use these settings to specify your dynamic range control values: Dynamic range compression line (DynamicRangeCompressionLine) and Dynamic range compression RF (DynamicRangeCompressionRf). When you keep the value Custom (SPECIFIED) for Dynamic range control (DynamicRangeControl) and you don&#39;t specify values for the related settings, MediaConvert uses default values for those settings.</p>
    #[serde(rename = "dynamicRangeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_control: Option<String>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left only/Right only center mix (Lo/Ro center). MediaConvert uses this value for downmixing. Default value: -3 dB (ATMOS<em>STORAGE</em>DDP<em>MIXLEV</em>MINUS<em>3</em>DB). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, and -6.0. Related setting: How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Related setting: To have MediaConvert use this value, keep the default value, Custom (SPECIFIED) for the setting Downmix control (DownmixControl). Otherwise, MediaConvert ignores Left only/Right only center (LoRoCenterMixLevel).</p>
    #[serde(rename = "loRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left only/Right only (Lo/Ro surround). MediaConvert uses this value for downmixing. Default value: -3 dB (ATMOS<em>STORAGE</em>DDP<em>MIXLEV</em>MINUS<em>3</em>DB). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. Related setting: How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Related setting: To have MediaConvert use this value, keep the default value, Custom (SPECIFIED) for the setting Downmix control (DownmixControl). Otherwise, MediaConvert ignores Left only/Right only surround (LoRoSurroundMixLevel).</p>
    #[serde(rename = "loRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left total/Right total center mix (Lt/Rt center). MediaConvert uses this value for downmixing. Default value: -3 dB (ATMOS<em>STORAGE</em>DDP<em>MIXLEV</em>MINUS<em>3</em>DB) Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, and -6.0. Related setting: How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Related setting: To have MediaConvert use this value, keep the default value, Custom (SPECIFIED) for the setting Downmix control (DownmixControl). Otherwise, MediaConvert ignores Left total/Right total center (LtRtCenterMixLevel).</p>
    #[serde(rename = "ltRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Atmos setting: Left total/Right total surround mix (Lt/Rt surround). MediaConvert uses this value for downmixing. Default value: -3 dB (ATMOS<em>STORAGE</em>DDP<em>MIXLEV</em>MINUS<em>3</em>DB) Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. Related setting: How the service uses this value depends on the value that you choose for Stereo downmix (Eac3AtmosStereoDownmix). Related setting: To have MediaConvert use this value, keep the default value, Custom (SPECIFIED) for the setting Downmix control (DownmixControl). Otherwise, the service ignores Left total/Right total surround (LtRtSurroundMixLevel).</p>
    #[serde(rename = "ltRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    /// <p>Choose how the service meters the loudness of your audio.</p>
    #[serde(rename = "meteringMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metering_mode: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Specify the percentage of audio content, from 0% to 100%, that must be speech in order for the encoder to use the measured speech loudness as the overall program loudness. Default value: 15%</p>
    #[serde(rename = "speechThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_threshold: Option<i64>,
    /// <p>Choose how the service does stereo downmixing. Default value: Not indicated (ATMOS<em>STORAGE</em>DDP<em>DMIXMOD</em>NOT_INDICATED) Related setting: To have MediaConvert use this value, keep the default value, Custom (SPECIFIED) for the setting Downmix control (DownmixControl). Otherwise, MediaConvert ignores Stereo downmix (StereoDownmix).</p>
    #[serde(rename = "stereoDownmix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stereo_downmix: Option<String>,
    /// <p>Specify whether your input audio has an additional center rear surround channel matrix encoded into your left and right surround channels.</p>
    #[serde(rename = "surroundExMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround_ex_mode: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value EAC3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Eac3Settings {
    /// <p>If set to ATTENUATE<em>3</em>DB, applies a 3 dB attenuation to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "attenuationControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attenuation_control: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Valid bitrates depend on the coding mode.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the bitstream mode for the E-AC-3 stream that the encoder emits. For more information about the EAC3 bitstream mode, see ATSC A/52-2012 (Annex E).</p>
    #[serde(rename = "bitstreamMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitstream_mode: Option<String>,
    /// <p>Dolby Digital Plus coding mode. Determines number of channels.</p>
    #[serde(rename = "codingMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coding_mode: Option<String>,
    /// <p>Activates a DC highpass filter for all input channels.</p>
    #[serde(rename = "dcFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dc_filter: Option<String>,
    /// <p>Sets the dialnorm for the output. If blank and input audio is Dolby Digital Plus, dialnorm will be passed through.</p>
    #[serde(rename = "dialnorm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialnorm: Option<i64>,
    /// <p>Choose the Dolby Digital dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby Digital stream for the line operating mode. Related setting: When you use this setting, MediaConvert ignores any value you provide for Dynamic range compression profile (DynamicRangeCompressionProfile). For information about the Dolby Digital DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionLine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_line: Option<String>,
    /// <p>Choose the Dolby Digital dynamic range control (DRC) profile that MediaConvert uses when encoding the metadata in the Dolby Digital stream for the RF operating mode. Related setting: When you use this setting, MediaConvert ignores any value you provide for Dynamic range compression profile (DynamicRangeCompressionProfile). For information about the Dolby Digital DRC operating modes and profiles, see the Dynamic Range Control chapter of the Dolby Metadata Guide at https://developer.dolby.com/globalassets/professional/documents/dolby-metadata-guide.pdf.</p>
    #[serde(rename = "dynamicRangeCompressionRf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_range_compression_rf: Option<String>,
    /// <p>When encoding 3/2 audio, controls whether the LFE channel is enabled</p>
    #[serde(rename = "lfeControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_control: Option<String>,
    /// <p>Applies a 120Hz lowpass filter to the LFE channel prior to encoding. Only valid with 3<em>2</em>LFE coding mode.</p>
    #[serde(rename = "lfeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lfe_filter: Option<String>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left only/Right only center mix (Lo/Ro center). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left only/Right only center (loRoCenterMixLevel).</p>
    #[serde(rename = "loRoCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left only/Right only (Lo/Ro surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left only/Right only surround (loRoSurroundMixLevel).</p>
    #[serde(rename = "loRoSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lo_ro_surround_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left total/Right total center mix (Lt/Rt center). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: 3.0, 1.5, 0.0, -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left total/Right total center (ltRtCenterMixLevel).</p>
    #[serde(rename = "ltRtCenterMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_center_mix_level: Option<f64>,
    /// <p>Specify a value for the following Dolby Digital Plus setting: Left total/Right total surround mix (Lt/Rt surround). MediaConvert uses this value for downmixing. How the service uses this value depends on the value that you choose for Stereo downmix (Eac3StereoDownmix). Valid values: -1.5, -3.0, -4.5, -6.0, and -60. The value -60 mutes the channel. This setting applies only if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Left total/Right total surround (ltRtSurroundMixLevel).</p>
    #[serde(rename = "ltRtSurroundMixLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_rt_surround_mix_level: Option<f64>,
    /// <p>When set to FOLLOW_INPUT, encoder metadata will be sourced from the DD, DD+, or DolbyE decoder that supplied this audio data. If audio was not supplied from one of these streams, then the static metadata settings will be used.</p>
    #[serde(rename = "metadataControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_control: Option<String>,
    /// <p>When set to WHEN_POSSIBLE, input DD+ audio will be passed through if it is present on the input. this detection is dynamic over the life of the transcode. Inputs that alternate between DD+ and non-DD+ content will have a consistent DD+ output as the system alternates between passthrough and encoding.</p>
    #[serde(rename = "passthroughControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passthrough_control: Option<String>,
    /// <p>Controls the amount of phase-shift applied to the surround channels. Only used for 3/2 coding mode.</p>
    #[serde(rename = "phaseControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_control: Option<String>,
    /// <p>This value is always 48000. It represents the sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Choose how the service does stereo downmixing. This setting only applies if you keep the default value of 3/2 - L, R, C, Ls, Rs (CODING<em>MODE</em>3_2) for the setting Coding mode (Eac3CodingMode). If you choose a different value for Coding mode, the service ignores Stereo downmix (Eac3StereoDownmix).</p>
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

/// <p>Settings related to CEA/EIA-608 and CEA/EIA-708 (also called embedded or ancillary) captions. Set up embedded captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/embedded-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to EMBEDDED, EMBEDDED<em>PLUS</em>SCTE20, or SCTE20<em>PLUS</em>EMBEDDED.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmbeddedDestinationSettings {
    /// <p>Ignore this setting unless your input captions are SCC format and your output captions are embedded in the video stream. Specify a CC number for each captions channel in this output. If you have two channels, choose CC numbers that aren&#39;t in the same field. For example, choose 1 and 3. For more information, see https://docs.aws.amazon.com/console/mediaconvert/dual-scc-to-embedded.</p>
    #[serde(rename = "destination608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_608_channel_number: Option<i64>,
    /// <p>Ignore this setting unless your input captions are SCC format and you want both 608 and 708 captions embedded in your output stream. Optionally, specify the 708 service number for each output captions channel. Choose a different number for each channel. To use this setting, also set Force 608 to 708 upconvert (Convert608To708) to Upconvert (UPCONVERT) in your input captions selector settings. If you choose to upconvert but don&#39;t specify a 708 service number, MediaConvert uses the number that you specify for CC channel number (destination608ChannelNumber) for the 708 service number. For more information, see https://docs.aws.amazon.com/console/mediaconvert/dual-scc-to-embedded.</p>
    #[serde(rename = "destination708ServiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_708_service_number: Option<i64>,
}

/// <p>Settings for embedded captions Source</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EmbeddedSourceSettings {
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
    #[serde(rename = "convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Specifies the 608/708 channel number within the video track from which to extract captions. Unused for passthrough.</p>
    #[serde(rename = "source608ChannelNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_channel_number: Option<i64>,
    /// <p>Specifies the video track index used for extracting captions. The system only supports one input video track, so this should always be set to &#39;1&#39;.</p>
    #[serde(rename = "source608TrackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_608_track_number: Option<i64>,
    /// <p>By default, the service terminates any unterminated captions at the end of each input. If you want the caption to continue onto your next input, disable this setting.</p>
    #[serde(rename = "terminateCaptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_captions: Option<String>,
}

/// <p>Describes an account-specific API endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>URL of endpoint</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>ESAM ManifestConfirmConditionNotification defined by OC-SP-ESAM-API-I03-131025.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EsamManifestConfirmConditionNotification {
    /// <p>Provide your ESAM ManifestConfirmConditionNotification XML document inside your JSON job settings. Form the XML document as per OC-SP-ESAM-API-I03-131025. The transcoder will use the Manifest Conditioning instructions in the message that you supply.</p>
    #[serde(rename = "mccXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc_xml: Option<String>,
}

/// <p>Settings for Event Signaling And Messaging (ESAM). If you don&#39;t do ad insertion, you can ignore these settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EsamSettings {
    /// <p>Specifies an ESAM ManifestConfirmConditionNotification XML as per OC-SP-ESAM-API-I03-131025. The transcoder uses the manifest conditioning instructions that you provide in the setting MCC XML (mccXml).</p>
    #[serde(rename = "manifestConfirmConditionNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_confirm_condition_notification: Option<EsamManifestConfirmConditionNotification>,
    /// <p>Specifies the stream distance, in milliseconds, between the SCTE 35 messages that the transcoder places and the splice points that they refer to. If the time between the start of the asset and the SCTE-35 message is less than this value, then the transcoder places the SCTE-35 marker at the beginning of the stream.</p>
    #[serde(rename = "responseSignalPreroll")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_signal_preroll: Option<i64>,
    /// <p>Specifies an ESAM SignalProcessingNotification XML as per OC-SP-ESAM-API-I03-131025. The transcoder uses the signal processing instructions that you provide in the setting SCC XML (sccXml).</p>
    #[serde(rename = "signalProcessingNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signal_processing_notification: Option<EsamSignalProcessingNotification>,
}

/// <p>ESAM SignalProcessingNotification data defined by OC-SP-ESAM-API-I03-131025.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EsamSignalProcessingNotification {
    /// <p>Provide your ESAM SignalProcessingNotification XML document inside your JSON job settings. Form the XML document as per OC-SP-ESAM-API-I03-131025. The transcoder will use the signal processing instructions in the message that you supply. Provide your ESAM SignalProcessingNotification XML document inside your JSON job settings. For your MPEG2-TS file outputs, if you want the service to place SCTE-35 markers at the insertion points you specify in the XML document, you must also enable SCTE-35 ESAM (scte35Esam). Note that you can either specify an ESAM XML document or enable SCTE-35 passthrough. You can&#39;t do both.</p>
    #[serde(rename = "sccXml")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scc_xml: Option<String>,
}

/// <p>Settings for F4v container</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct F4vSettings {
    /// <p>If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.</p>
    #[serde(rename = "moovPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moov_placement: Option<String>,
}

/// <p>Settings related to your File output group. MediaConvert uses this group of settings to generate a single standalone file, rather than a streaming package. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to FILE<em>GROUP</em>SETTINGS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FileGroupSettings {
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
}

/// <p>If your input captions are SCC, SMI, SRT, STL, TTML, WebVTT, or IMSC 1.1 in an xml file, specify the URI of the input caption source file. If your caption source is IMSC in an IMF package, use TrackSourceSettings instead of FileSoureSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FileSourceSettings {
    /// <p>Specify whether this set of input captions appears in your outputs in both 608 and 708 format. If you choose Upconvert (UPCONVERT), MediaConvert includes the captions data in two ways: it passes the 608 data through using the 608 compatibility bytes fields of the 708 wrapper, and it also translates the 608 data into 708.</p>
    #[serde(rename = "convert608To708")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_608_to_708: Option<String>,
    /// <p>Ignore this setting unless your input captions format is SCC. To have the service compensate for differing frame rates between your input captions and input video, specify the frame rate of the captions file. Specify this value as a fraction, using the settings Framerate numerator (framerateNumerator) and Framerate denominator (framerateDenominator). For example, you might specify 24 / 1 for 24 fps, 25 / 1 for 25 fps, 24000 / 1001 for 23.976 fps, or 30000 / 1001 for 29.97 fps.</p>
    #[serde(rename = "framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<CaptionSourceFramerate>,
    /// <p>External caption file used for loading captions. Accepted file extensions are &#39;scc&#39;, &#39;ttml&#39;, &#39;dfxp&#39;, &#39;stl&#39;, &#39;srt&#39;, &#39;xml&#39;, &#39;smi&#39;, &#39;webvtt&#39;, and &#39;vtt&#39;.</p>
    #[serde(rename = "sourceFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_file: Option<String>,
    /// <p>Specifies a time delta in seconds to offset the captions from the source file.</p>
    #[serde(rename = "timeDelta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_delta: Option<i64>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value FRAME_CAPTURE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FrameCaptureSettings {
    /// <p>Frame capture will encode the first frame of the output stream, then one frame every framerateDenominator/framerateNumerator seconds. For example, settings of framerateNumerator = 1 and framerateDenominator = 3 (a rate of 1/3 frame per second) will capture the first frame, then 1 frame every 3s. Files will be named as filename.n.jpg where n is the 0-based sequence number of each Capture.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>Frame capture will encode the first frame of the output stream, then one frame every framerateDenominator/framerateNumerator seconds. For example, settings of framerateNumerator = 1 and framerateDenominator = 3 (a rate of 1/3 frame per second) will capture the first frame, then 1 frame every 3s. Files will be named as filename.NNNNNNN.jpg where N is the 0-based frame sequence number zero padded to 7 decimal places.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Maximum number of captures (encoded jpg output files).</p>
    #[serde(rename = "maxCaptures")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_captures: Option<i64>,
    /// <p>JPEG Quality - a higher value equals higher quality.</p>
    #[serde(rename = "quality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobRequest {
    /// <p>the job ID of the job.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobResponse {
    /// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "job")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<Job>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetJobTemplateRequest {
    /// <p>The name of the job template.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "jobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPresetRequest {
    /// <p>The name of the preset.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetPresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetQueueRequest {
    /// <p>The name of the queue that you want information about.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Settings for quality-defined variable bitrate encoding with the H.264 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H264QvbrSettings {
    /// <p>Use this setting only when Rate control mode is QVBR and Quality tuning level is Multi-pass HQ. For Max average bitrate values suited to the complexity of your input video, the service limits the average bitrate of the video part of this output to the value that you choose. That is, the total size of the video element is less than or equal to the value you set multiplied by the number of seconds of encoded output.</p>
    #[serde(rename = "maxAverageBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_average_bitrate: Option<i64>,
    /// <p>Required when you use QVBR rate control mode. That is, when you specify qvbrSettings within h264Settings. Specify the general target quality level for this output, from 1 to 10. Use higher numbers for greater quality. Level 10 results in nearly lossless compression. The quality level for most broadcast-quality transcodes is between 6 and 9. Optionally, to specify a value between whole numbers, also provide a value for the setting qvbrQualityLevelFineTune. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33.</p>
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
    /// <p>Optional. Specify a value here to set the QVBR quality to a level that is between whole numbers. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33. MediaConvert rounds your QVBR quality level to the nearest third of a whole number. For example, if you set qvbrQualityLevel to 7 and you set qvbrQualityLevelFineTune to .25, your actual QVBR quality level is 7.33.</p>
    #[serde(rename = "qvbrQualityLevelFineTune")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level_fine_tune: Option<f64>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value H_264.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H264Settings {
    /// <p>Keep the default value, Auto (AUTO), for this setting to have MediaConvert automatically apply the best types of quantization for your video content. When you want to apply your quantization settings manually, you must set H264AdaptiveQuantization to a value other than Auto (AUTO). Use this setting to specify the strength of any adaptive quantization filters that you enable. If you don&#39;t want MediaConvert to do any adaptive quantization in this transcode, set Adaptive quantization (H264AdaptiveQuantization) to Off (OFF). Related settings: The value that you choose here applies to the following settings: H264FlickerAdaptiveQuantization, H264SpatialAdaptiveQuantization, and H264TemporalAdaptiveQuantization.</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify an H.264 level that is consistent with your output video settings. If you aren&#39;t sure what level to specify, choose Auto (AUTO).</p>
    #[serde(rename = "codecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>H.264 Profile. High 4:2:2 and 10-bit profiles are only available with the AVC-I License.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "dynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>Entropy encoding mode. Use CABAC (must be in Main or High profile) or CAVLC.</p>
    #[serde(rename = "entropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    /// <p>Keep the default value, PAFF, to have MediaConvert use PAFF encoding for interlaced outputs. Choose Force field (FORCE_FIELD) to disable PAFF encoding and create separate interlaced fields.</p>
    #[serde(rename = "fieldEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_encoding: Option<String>,
    /// <p>Only use this setting when you change the default value, AUTO, for the setting H264AdaptiveQuantization. When you keep all defaults, excluding H264AdaptiveQuantization and all other adaptive quantization from your JSON job specification, MediaConvert automatically applies the best types of quantization for your video content. When you set H264AdaptiveQuantization to a value other than AUTO, the default value for H264FlickerAdaptiveQuantization is Disabled (DISABLED). Change this value to Enabled (ENABLED) to reduce I-frame pop. I-frame pop appears as a visual flicker that can arise when the encoder saves bits by copying some macroblocks many times from frame to frame, and then refreshes them at the I-frame. When you enable this setting, the encoder updates these macroblocks slightly more often to smooth out the flicker. To manually enable or disable H264FlickerAdaptiveQuantization, you must set Adaptive quantization (H264AdaptiveQuantization) to a value other than AUTO.</p>
    #[serde(rename = "flickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>If enable, use reference B frames for GOP structures that have B frames &gt; 1.</p>
    #[serde(rename = "gopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Indicates if the GOP Size in H264 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "hrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. Required when Rate control mode is QVBR.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "minIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "numberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Number of reference frames to use. The encoder may use more than requested if using B-frames and/or interlaced encoding.</p>
    #[serde(rename = "numberReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_reference_frames: Option<i64>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio (PAR) for this output. The default behavior, Follow source (INITIALIZE<em>FROM</em>SOURCE), uses the PAR from your input video for your output. To specify a different PAR in the console, choose any value other than Follow source. To specify a different PAR by editing the JSON job specification, choose SPECIFIED. When you choose SPECIFIED for this setting, you must also specify values for the parNumerator and parDenominator settings.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, single-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Settings for quality-defined variable bitrate encoding with the H.264 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
    #[serde(rename = "qvbrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_settings: Option<H264QvbrSettings>,
    /// <p>Use this setting to specify whether this output has a variable bitrate (VBR), constant bitrate (CBR) or quality-defined variable bitrate (QVBR).</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Places a PPS header on each encoded picture, even if repeated.</p>
    #[serde(rename = "repeatPps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_pps: Option<String>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default. If this output uses QVBR, choose Transition detection (TRANSITION_DETECTION) for further video quality improvement. For more information about QVBR, see https://docs.aws.amazon.com/console/mediaconvert/cbr-vbr-qvbr.</p>
    #[serde(rename = "sceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output. When you enable slow PAL, MediaConvert relabels the video frames to 25 fps and resamples your audio to keep it synchronized with the video. Note that enabling this setting will slightly reduce the duration of your video. Required settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Ignore this setting unless you need to comply with a specification that requires a specific value. If you don&#39;t have a specification requirement, we recommend that you adjust the softness of your output by using a lower value for the setting Sharpness (sharpness) or by enabling a noise reducer filter (noiseReducerFilter). The Softness (softness) setting specifies the quantization matrices that the encoder uses. Keep the default value, 0, for flat quantization. Choose the value 1 or 16 to use the default JVT softening quantization matricies from the H.264 specification. Choose a value from 17 to 128 to use planar interpolation. Increasing values from 17 to 128 result in increasing reduction of high-frequency data. The value 128 results in the softest video.</p>
    #[serde(rename = "softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>Only use this setting when you change the default value, Auto (AUTO), for the setting H264AdaptiveQuantization. When you keep all defaults, excluding H264AdaptiveQuantization and all other adaptive quantization from your JSON job specification, MediaConvert automatically applies the best types of quantization for your video content. When you set H264AdaptiveQuantization to a value other than AUTO, the default value for H264SpatialAdaptiveQuantization is Enabled (ENABLED). Keep this default value to adjust quantization within each frame based on spatial variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas that can sustain more distortion with no noticeable visual degradation and uses more bits on areas where any small distortion will be noticeable. For example, complex textured blocks are encoded with fewer bits and smooth textured blocks are encoded with more bits. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen with a lot of complex texture, you might choose to set H264SpatialAdaptiveQuantization to Disabled (DISABLED). Related setting: When you enable spatial adaptive quantization, set the value for Adaptive quantization (H264AdaptiveQuantization) depending on your content. For homogeneous content, such as cartoons and video games, set it to Low. For content with a wider variety of textures, set it to High or Higher. To manually enable or disable H264SpatialAdaptiveQuantization, you must set Adaptive quantization (H264AdaptiveQuantization) to a value other than AUTO.</p>
    #[serde(rename = "spatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>Produces a bitstream compliant with SMPTE RP-2027.</p>
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// <p>When you do frame rate conversion from 23.976 frames per second (fps) to 29.97 fps, and your output scan type is interlaced, you can optionally enable hard or soft telecine to create a smoother picture. Hard telecine (HARD) produces a 29.97i output. Soft telecine (SOFT) produces an output with a 23.976 output that signals to the video player device to do the conversion during play back. When you keep the default value, None (NONE), MediaConvert does a standard frame rate conversion to 29.97 without doing anything with the field polarity to create a smoother picture.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Only use this setting when you change the default value, AUTO, for the setting H264AdaptiveQuantization. When you keep all defaults, excluding H264AdaptiveQuantization and all other adaptive quantization from your JSON job specification, MediaConvert automatically applies the best types of quantization for your video content. When you set H264AdaptiveQuantization to a value other than AUTO, the default value for H264TemporalAdaptiveQuantization is Enabled (ENABLED). Keep this default value to adjust quantization within each frame based on temporal variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas of the frame that aren&#39;t moving and uses more bits on complex objects with sharp edges that move a lot. For example, this feature improves the readability of text tickers on newscasts and scoreboards on sports matches. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen that doesn&#39;t have moving objects with sharp edges, such as sports athletes&#39; faces, you might choose to set H264TemporalAdaptiveQuantization to Disabled (DISABLED). Related setting: When you enable temporal quantization, adjust the strength of the filter with the setting Adaptive quantization (adaptiveQuantization). To manually enable or disable H264TemporalAdaptiveQuantization, you must set Adaptive quantization (H264AdaptiveQuantization) to a value other than AUTO.</p>
    #[serde(rename = "temporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    /// <p>Inserts timecode for each frame as 4 bytes of an unregistered SEI message.</p>
    #[serde(rename = "unregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
}

/// <p>Settings for quality-defined variable bitrate encoding with the H.265 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H265QvbrSettings {
    /// <p>Use this setting only when Rate control mode is QVBR and Quality tuning level is Multi-pass HQ. For Max average bitrate values suited to the complexity of your input video, the service limits the average bitrate of the video part of this output to the value that you choose. That is, the total size of the video element is less than or equal to the value you set multiplied by the number of seconds of encoded output.</p>
    #[serde(rename = "maxAverageBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_average_bitrate: Option<i64>,
    /// <p>Required when you use QVBR rate control mode. That is, when you specify qvbrSettings within h265Settings. Specify the general target quality level for this output, from 1 to 10. Use higher numbers for greater quality. Level 10 results in nearly lossless compression. The quality level for most broadcast-quality transcodes is between 6 and 9. Optionally, to specify a value between whole numbers, also provide a value for the setting qvbrQualityLevelFineTune. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33.</p>
    #[serde(rename = "qvbrQualityLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level: Option<i64>,
    /// <p>Optional. Specify a value here to set the QVBR quality to a level that is between whole numbers. For example, if you want your QVBR quality level to be 7.33, set qvbrQualityLevel to 7 and set qvbrQualityLevelFineTune to .33. MediaConvert rounds your QVBR quality level to the nearest third of a whole number. For example, if you set qvbrQualityLevel to 7 and you set qvbrQualityLevelFineTune to .25, your actual QVBR quality level is 7.33.</p>
    #[serde(rename = "qvbrQualityLevelFineTune")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_quality_level_fine_tune: Option<f64>,
}

/// <p>Settings for H265 codec</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct H265Settings {
    /// <p>Specify the strength of any adaptive quantization filters that you enable. The value that you choose here applies to the following settings: Flicker adaptive quantization (flickerAdaptiveQuantization), Spatial adaptive quantization (spatialAdaptiveQuantization), and Temporal adaptive quantization (temporalAdaptiveQuantization).</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Enables Alternate Transfer Function SEI message for outputs using Hybrid Log Gamma (HLG) Electro-Optical Transfer Function (EOTF).</p>
    #[serde(rename = "alternateTransferFunctionSei")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_transfer_function_sei: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>H.265 Level.</p>
    #[serde(rename = "codecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>Represents the Profile and Tier, per the HEVC (H.265) specification. Selections are grouped as [Profile] / [Tier], so &quot;Main/High&quot; represents Main Profile with High Tier. 4:2:2 profiles are only available with the HEVC 4:2:2 License.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "dynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>Enable this setting to have the encoder reduce I-frame pop. I-frame pop appears as a visual flicker that can arise when the encoder saves bits by copying some macroblocks many times from frame to frame, and then refreshes them at the I-frame. When you enable this setting, the encoder updates these macroblocks slightly more often to smooth out the flicker. This setting is disabled by default. Related setting: In addition to enabling this setting, you must also set adaptiveQuantization to a value other than Off (OFF).</p>
    #[serde(rename = "flickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>If enable, use reference B frames for GOP structures that have B frames &gt; 1.</p>
    #[serde(rename = "gopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Indicates if the GOP Size in H265 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "hrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. Required when Rate control mode is QVBR.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "minIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "numberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Number of reference frames to use. The encoder may use more than requested if using B-frames and/or interlaced encoding.</p>
    #[serde(rename = "numberReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_reference_frames: Option<i64>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio (PAR) for this output. The default behavior, Follow source (INITIALIZE<em>FROM</em>SOURCE), uses the PAR from your input video for your output. To specify a different PAR in the console, choose any value other than Follow source. To specify a different PAR by editing the JSON job specification, choose SPECIFIED. When you choose SPECIFIED for this setting, you must also specify values for the parNumerator and parDenominator settings.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, single-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Settings for quality-defined variable bitrate encoding with the H.265 codec. Required when you set Rate control mode to QVBR. Not valid when you set Rate control mode to a value other than QVBR, or when you don&#39;t define Rate control mode.</p>
    #[serde(rename = "qvbrSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qvbr_settings: Option<H265QvbrSettings>,
    /// <p>Use this setting to specify whether this output has a variable bitrate (VBR), constant bitrate (CBR) or quality-defined variable bitrate (QVBR).</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Specify Sample Adaptive Offset (SAO) filter strength.  Adaptive mode dynamically selects best strength based on content</p>
    #[serde(rename = "sampleAdaptiveOffsetFilterMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_adaptive_offset_filter_mode: Option<String>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default. If this output uses QVBR, choose Transition detection (TRANSITION_DETECTION) for further video quality improvement. For more information about QVBR, see https://docs.aws.amazon.com/console/mediaconvert/cbr-vbr-qvbr.</p>
    #[serde(rename = "sceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output. When you enable slow PAL, MediaConvert relabels the video frames to 25 fps and resamples your audio to keep it synchronized with the video. Note that enabling this setting will slightly reduce the duration of your video. Required settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on spatial variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas that can sustain more distortion with no noticeable visual degradation and uses more bits on areas where any small distortion will be noticeable. For example, complex textured blocks are encoded with fewer bits and smooth textured blocks are encoded with more bits. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen with a lot of complex texture, you might choose to disable this feature. Related setting: When you enable spatial adaptive quantization, set the value for Adaptive quantization (adaptiveQuantization) depending on your content. For homogeneous content, such as cartoons and video games, set it to Low. For content with a wider variety of textures, set it to High or Higher.</p>
    #[serde(rename = "spatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>This field applies only if the Streams &gt; Advanced &gt; Framerate (framerate) field  is set to 29.970. This field works with the Streams &gt; Advanced &gt; Preprocessors &gt; Deinterlacer  field (deinterlace<em>mode) and the Streams &gt; Advanced &gt; Interlaced Mode field (interlace</em>mode)  to identify the scan type for the output: Progressive, Interlaced, Hard Telecine or Soft Telecine. - Hard: produces 29.97i output from 23.976 input. - Soft: produces 23.976; the player converts this output to 29.97i.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on temporal variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas of the frame that aren&#39;t moving and uses more bits on complex objects with sharp edges that move a lot. For example, this feature improves the readability of text tickers on newscasts and scoreboards on sports matches. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen that doesn&#39;t have moving objects with sharp edges, such as sports athletes&#39; faces, you might choose to disable this feature. Related setting: When you enable temporal quantization, adjust the strength of the filter with the setting Adaptive quantization (adaptiveQuantization).</p>
    #[serde(rename = "temporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    /// <p>Enables temporal layer identifiers in the encoded bitstream. Up to 3 layers are supported depending on GOP structure: I- and P-frames form one layer, reference B-frames can form a second layer and non-reference b-frames can form a third layer. Decoders can optionally decode only the lower temporal layers to generate a lower frame rate output. For example, given a bitstream with temporal IDs and with b-frames = 1 (i.e. IbPbPb display order), a decoder could decode all the frames for full frame rate output or only the I and P frames (lowest temporal layer) for a half frame rate output.</p>
    #[serde(rename = "temporalIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_ids: Option<String>,
    /// <p>Enable use of tiles, allowing horizontal as well as vertical subdivision of the encoded pictures.</p>
    #[serde(rename = "tiles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiles: Option<String>,
    /// <p>Inserts timecode for each frame as 4 bytes of an unregistered SEI message.</p>
    #[serde(rename = "unregisteredSeiTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unregistered_sei_timecode: Option<String>,
    /// <p>If the location of parameter set NAL units doesn&#39;t matter in your workflow, ignore this setting. Use this setting only with CMAF or DASH outputs, or with standalone file outputs in an MPEG-4 container (MP4 outputs). Choose HVC1 to mark your output as HVC1. This makes your output compliant with the following specification: ISO IECJTC1 SC29 N13798 Text ISO/IEC FDIS 14496-15 3rd Edition. For these outputs, the service stores parameter set NAL units in the sample headers but not in the samples directly. For MP4 outputs, when you choose HVC1, your output video might not work properly with some downstream systems and video players. The service defaults to marking your output as HEV1. For these outputs, the service writes parameter set NAL units directly into the samples.</p>
    #[serde(rename = "writeMp4PackagingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_mp_4_packaging_type: Option<String>,
}

/// <p>Use these settings to specify static color calibration metadata, as defined by SMPTE ST 2086. These values don&#39;t affect the pixel values that are encoded in the video stream. They are intended to help the downstream video player display content in a way that reflects the intentions of the the content creator.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Hdr10Metadata {
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "bluePrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "bluePrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_primary_y: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "greenPrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "greenPrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_primary_y: Option<i64>,
    /// <p>Maximum light level among all samples in the coded video sequence, in units of candelas per square meter.  This setting doesn&#39;t have a default value; you must specify a value that is suitable for the content.</p>
    #[serde(rename = "maxContentLightLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_content_light_level: Option<i64>,
    /// <p>Maximum average light level of any frame in the coded video sequence, in units of candelas per square meter. This setting doesn&#39;t have a default value; you must specify a value that is suitable for the content.</p>
    #[serde(rename = "maxFrameAverageLightLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_frame_average_light_level: Option<i64>,
    /// <p>Nominal maximum mastering display luminance in units of of 0.0001 candelas per square meter.</p>
    #[serde(rename = "maxLuminance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_luminance: Option<i64>,
    /// <p>Nominal minimum mastering display luminance in units of of 0.0001 candelas per square meter</p>
    #[serde(rename = "minLuminance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_luminance: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "redPrimaryX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_primary_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "redPrimaryY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_primary_y: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "whitePointX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_point_x: Option<i64>,
    /// <p>HDR Master Display Information must be provided by a color grader, using color grading tools. Range is 0 to 50,000, each increment represents 0.00002 in CIE1931 color coordinate. Note that this setting is not for color correction.</p>
    #[serde(rename = "whitePointY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_point_y: Option<i64>,
}

/// <p>Setting for HDR10+ metadata insertion</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Hdr10Plus {
    /// <p>Specify the HDR10+ mastering display normalized peak luminance, in nits. This is the normalized actual peak luminance of the mastering display, as defined by ST 2094-40.</p>
    #[serde(rename = "masteringMonitorNits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastering_monitor_nits: Option<i64>,
    /// <p>Specify the HDR10+ target display nominal peak luminance, in nits. This is the nominal maximum luminance of the target display as defined by ST 2094-40.</p>
    #[serde(rename = "targetMonitorNits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_monitor_nits: Option<i64>,
}

/// <p>Specify the details for each additional HLS manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your HLS group is film-name.m3u8. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.m3u8. For HLS output groups, specify a manifestNameModifier that is different from the nameModifier of the output. The service uses the output name modifier to create unique names for the individual variant manifests.</p>
    #[serde(rename = "manifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "selectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>Caption Language Mapping</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsCaptionLanguageMapping {
    /// <p>Caption channel.</p>
    #[serde(rename = "captionChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_channel: Option<i64>,
    /// <p>Specify the language for this captions channel, using the ISO 639-2 or ISO 639-3 three-letter language code</p>
    #[serde(rename = "customLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_language_code: Option<String>,
    /// <p>Specify the language, using the ISO 639-2 three-letter code listed at https://www.loc.gov/standards/iso639-2/php/code_list.php.</p>
    #[serde(rename = "languageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// <p>Caption language description.</p>
    #[serde(rename = "languageDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_description: Option<String>,
}

/// <p>Settings for HLS encryption</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsEncryptionSettings {
    /// <p>This is a 128-bit, 16-byte hex value represented by a 32-character text string. If this parameter is not set then the Initialization Vector will follow the segment number by default.</p>
    #[serde(rename = "constantInitializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constant_initialization_vector: Option<String>,
    /// <p>Encrypts the segments with the given encryption scheme. Leave blank to disable. Selecting &#39;Disabled&#39; in the web interface also disables encryption.</p>
    #[serde(rename = "encryptionMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_method: Option<String>,
    /// <p>The Initialization Vector is a 128-bit number used in conjunction with the key for encrypting blocks. If set to INCLUDE, Initialization Vector is listed in the manifest. Otherwise Initialization Vector is not in the manifest.</p>
    #[serde(rename = "initializationVectorInManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector_in_manifest: Option<String>,
    /// <p>Enable this setting to insert the EXT-X-SESSION-KEY element into the master playlist. This allows for offline Apple HLS FairPlay content protection.</p>
    #[serde(rename = "offlineEncrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline_encrypted: Option<String>,
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "spekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
    /// <p>Use these settings to set up encryption with a static key provider.</p>
    #[serde(rename = "staticKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_provider: Option<StaticKeyProvider>,
    /// <p>Specify whether your DRM encryption key is static or from a key provider that follows the SPEKE standard. For more information about SPEKE, see https://docs.aws.amazon.com/speke/latest/documentation/what-is-speke.html.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Settings related to your HLS output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to HLS<em>GROUP</em>SETTINGS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsGroupSettings {
    /// <p>Choose one or more ad marker types to decorate your Apple HLS manifest. This setting does not determine whether SCTE-35 markers appear in the outputs themselves.</p>
    #[serde(rename = "adMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_markers: Option<Vec<String>>,
    /// <p>By default, the service creates one top-level .m3u8 HLS manifest for each HLS output group in your job. This default manifest references every output in the output group. To create additional top-level manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "additionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<HlsAdditionalManifest>>,
    /// <p>Ignore this setting unless you are using FairPlay DRM with Verimatrix and you encounter playback issues. Keep the default value, Include (INCLUDE), to output audio-only headers. Choose Exclude (EXCLUDE) to remove the audio-only headers from your audio segments.</p>
    #[serde(rename = "audioOnlyHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_header: Option<String>,
    /// <p>A partial URI prefix that will be prepended to each output in the media .m3u8 file. Can be used if base manifest is delivered from a different URL than the main .m3u8 file.</p>
    #[serde(rename = "baseUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// <p>Language to be used on Caption outputs</p>
    #[serde(rename = "captionLanguageMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_mappings: Option<Vec<HlsCaptionLanguageMapping>>,
    /// <p>Applies only to 608 Embedded output captions. Insert: Include CLOSED-CAPTIONS lines in the manifest. Specify at least one language in the CC1 Language Code field. One CLOSED-CAPTION line is added for each Language Code you specify. Make sure to specify the languages in the order in which they appear in the original source (if the source is embedded format) or the order of the caption selectors (if the source is other than embedded). Otherwise, languages in the manifest will not match up properly with the output captions. None: Include CLOSED-CAPTIONS=NONE line in the manifest. Omit: Omit any CLOSED-CAPTIONS line from the manifest.</p>
    #[serde(rename = "captionLanguageSetting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_language_setting: Option<String>,
    /// <p>Disable this setting only when your workflow requires the #EXT-X-ALLOW-CACHE:no tag. Otherwise, keep the default value Enabled (ENABLED) and control caching in your video distribution set up. For example, use the Cache-Control http header.</p>
    #[serde(rename = "clientCache")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_cache: Option<String>,
    /// <p>Specification to use (RFC-6381 or the default RFC-4281) during m3u8 playlist generation.</p>
    #[serde(rename = "codecSpecification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_specification: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>Indicates whether segments should be placed in subdirectories.</p>
    #[serde(rename = "directoryStructure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_structure: Option<String>,
    /// <p>DRM settings.</p>
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<HlsEncryptionSettings>,
    /// <p>Specify whether MediaConvert generates images for trick play. Keep the default value, None (NONE), to not generate any images. Choose Thumbnail (THUMBNAIL) to generate tiled thumbnails. Choose Thumbnail and full frame (THUMBNAIL<em>AND</em>FULLFRAME) to generate tiled thumbnails and full-resolution images of single frames. MediaConvert creates a child manifest for each set of images that you generate and adds corresponding entries to the parent manifest. A common application for these images is Roku trick mode. The thumbnails and full-frame images that MediaConvert creates with this feature are compatible with this Roku specification: https://developer.roku.com/docs/developer-program/media-playback/trick-mode/hls-and-dash.md</p>
    #[serde(rename = "imageBasedTrickPlay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_based_trick_play: Option<String>,
    /// <p>When set to GZIP, compresses HLS playlist.</p>
    #[serde(rename = "manifestCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_compression: Option<String>,
    /// <p>Indicates whether the output manifest should use floating point values for segment duration.</p>
    #[serde(rename = "manifestDurationFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_duration_format: Option<String>,
    /// <p>Keep this setting at the default value of 0, unless you are troubleshooting a problem with how devices play back the end of your video asset. If you know that player devices are hanging on the final segment of your video because the length of your final segment is too short, use this setting to specify a minimum final segment length, in seconds. Choose a value that is greater than or equal to 1 and less than your segment length. When you specify a value for this setting, the encoder will combine any final segment that is shorter than the length that you specify with the previous segment. For example, your segment length is 3 seconds and your final segment is .5 seconds without a minimum final segment length; when you set the minimum final segment length to 1, your final segment is 3.5 seconds.</p>
    #[serde(rename = "minFinalSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_final_segment_length: Option<f64>,
    /// <p>When set, Minimum Segment Size is enforced by looking ahead and back within the specified range for a nearby avail and extending the segment size if needed.</p>
    #[serde(rename = "minSegmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_segment_length: Option<i64>,
    /// <p>Indicates whether the .m3u8 manifest file should be generated for this HLS output group.</p>
    #[serde(rename = "outputSelection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_selection: Option<String>,
    /// <p>Includes or excludes EXT-X-PROGRAM-DATE-TIME tag in .m3u8 manifest files. The value is calculated as follows: either the program date and time are initialized using the input timecode source, or the time is initialized using the input timecode source and the date is initialized using the timestamp_offset.</p>
    #[serde(rename = "programDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time: Option<String>,
    /// <p>Period of insertion of EXT-X-PROGRAM-DATE-TIME entry, in seconds.</p>
    #[serde(rename = "programDateTimePeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_date_time_period: Option<i64>,
    /// <p>When set to SINGLE_FILE, emits program as a single media resource (.ts) file, uses #EXT-X-BYTERANGE tags to index segment for playback.</p>
    #[serde(rename = "segmentControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_control: Option<String>,
    /// <p>Length of MPEG-2 Transport Stream segments to create (in seconds). Note that segments will end on the next keyframe after this number of seconds, so actual segment length may be longer.</p>
    #[serde(rename = "segmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_length: Option<i64>,
    /// <p>Number of segments to write to a subdirectory before starting a new one. directoryStructure must be SINGLE_DIRECTORY for this setting to have an effect.</p>
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
}

/// <p>Settings specific to audio sources in an HLS alternate rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique audio track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the properties provided, the job fails. If no properties in hlsRenditionGroupSettings are specified, the default audio track within the video segment is chosen. If there is no audio within video segment, the alternative audio with DEFAULT=YES is chosen instead.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsRenditionGroupSettings {
    /// <p>Optional. Specify alternative group ID</p>
    #[serde(rename = "renditionGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_group_id: Option<String>,
    /// <p>Optional. Specify ISO 639-2 or ISO 639-3 code in the language property</p>
    #[serde(rename = "renditionLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_language_code: Option<String>,
    /// <p>Optional. Specify media name</p>
    #[serde(rename = "renditionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_name: Option<String>,
}

/// <p>Settings for HLS output groups</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HlsSettings {
    /// <p>Specifies the group to which the audio rendition belongs.</p>
    #[serde(rename = "audioGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_group_id: Option<String>,
    /// <p>Use this setting only in audio-only outputs. Choose MPEG-2 Transport Stream (M2TS) to create a file in an MPEG2-TS container. Keep the default value Automatic (AUTOMATIC) to create an audio-only file in a raw container. Regardless of the value that you specify here, if this output has video, the service will place the output into an MPEG2-TS container.</p>
    #[serde(rename = "audioOnlyContainer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_only_container: Option<String>,
    /// <p>List all the audio groups that are used with the video output stream. Input all the audio GROUP-IDs that are associated to the video, separate by &#39;,&#39;.</p>
    #[serde(rename = "audioRenditionSets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_rendition_sets: Option<String>,
    /// <p>Four types of audio-only tracks are supported: Audio-Only Variant Stream The client can play back this audio-only stream instead of video in low-bandwidth scenarios. Represented as an EXT-X-STREAM-INF in the HLS manifest. Alternate Audio, Auto Select, Default Alternate rendition that the client should try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=YES, AUTOSELECT=YES Alternate Audio, Auto Select, Not Default Alternate rendition that the client may try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=YES Alternate Audio, not Auto Select Alternate rendition that the client will not try to play back by default. Represented as an EXT-X-MEDIA in the HLS manifest with DEFAULT=NO, AUTOSELECT=NO</p>
    #[serde(rename = "audioTrackType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_type: Option<String>,
    /// <p>Specify whether to flag this audio track as descriptive video service (DVS) in your HLS parent manifest. When you choose Flag (FLAG), MediaConvert includes the parameter CHARACTERISTICS=&quot;public.accessibility.describes-video&quot; in the EXT-X-MEDIA entry for this track. When you keep the default choice, Don&#39;t flag (DONT_FLAG), MediaConvert leaves this parameter out. The DVS flag can help with accessibility on Apple devices. For more information, see the Apple documentation.</p>
    #[serde(rename = "descriptiveVideoServiceFlag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_video_service_flag: Option<String>,
    /// <p>Choose Include (INCLUDE) to have MediaConvert generate a child manifest that lists only the I-frames for this rendition, in addition to your regular manifest for this rendition. You might use this manifest as part of a workflow that creates preview functions for your video. MediaConvert adds both the I-frame only child manifest and the regular child manifest to the parent manifest. When you don&#39;t need the I-frame only child manifest, keep the default value Exclude (EXCLUDE).</p>
    #[serde(rename = "iFrameOnlyManifest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_frame_only_manifest: Option<String>,
    /// <p>Use this setting to add an identifying string to the filename of each segment. The service adds this string between the name modifier and segment index number. You can use format identifiers in the string. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/using-variables-in-your-job-settings.html</p>
    #[serde(rename = "segmentModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segment_modifier: Option<String>,
}

/// <p>Optional. Configuration for a destination queue to which the job can hop once a customer-defined minimum wait time has passed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HopDestination {
    /// <p>Optional. When you set up a job to use queue hopping, you can specify a different relative priority for the job in the destination queue. If you don&#39;t specify, the relative priority will remain the same as in the previous queue.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional unless the job is submitted on the default queue. When you set up a job to use queue hopping, you can specify a destination queue. This queue cannot be the original queue to which the job is submitted. If the original queue isn&#39;t the default queue and you don&#39;t specify the destination queue, the job will move to the default queue.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>Required for setting up a job to use queue hopping. Minimum wait time in minutes until the job can hop to the destination queue. Valid range is 1 to 1440 minutes, inclusive.</p>
    #[serde(rename = "waitMinutes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_minutes: Option<i64>,
}

/// <p>To insert ID3 tags in your output, specify two values. Use ID3 tag (Id3) to specify the base 64 encoded string and use Timecode (TimeCode) to specify the time when the tag should be inserted. To insert multiple ID3 tags in your output, create multiple instances of ID3 insertion (Id3Insertion).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Id3Insertion {
    /// <p>Use ID3 tag (Id3) to provide a tag value in base64-encode format.</p>
    #[serde(rename = "id3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_3: Option<String>,
    /// <p>Provide a Timecode (TimeCode) in HH:MM:SS:FF or HH:MM:SS;FF format.</p>
    #[serde(rename = "timecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode: Option<String>,
}

/// <p>Use the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input or output individually. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/graphic-overlay.html. This setting is disabled by default.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImageInserter {
    /// <p>Specify the images that you want to overlay on your video. The images must be PNG or TGA files.</p>
    #[serde(rename = "insertableImages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertable_images: Option<Vec<InsertableImage>>,
}

/// <p>Settings related to IMSC captions. IMSC is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/ttml-and-webvtt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to IMSC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImscDestinationSettings {
    /// <p>Keep this setting enabled to have MediaConvert use the font style and position information from the captions source in the output. This option is available only when your input captions are IMSC, SMPTE-TT, or TTML. Disable this setting for simplified output captions.</p>
    #[serde(rename = "stylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
}

/// <p>Use inputs to define the source files used in your transcoding job. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/specify-input-settings.html. You can use multiple video inputs to do input stitching. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/assembling-multiple-inputs-and-input-clips.html</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Input {
    /// <p>Use audio selector groups to combine multiple sidecar audio inputs so that you can assign them to a single output audio tab (AudioDescription). Note that, if you&#39;re working with embedded audio, it&#39;s simpler to assign multiple input tracks into a single audio selector rather than use an audio selector group.</p>
    #[serde(rename = "audioSelectorGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_groups: Option<::std::collections::HashMap<String, AudioSelectorGroup>>,
    /// <p>Use Audio selectors (AudioSelectors) to specify a track or set of tracks from the input that you will use in your outputs. You can use multiple Audio selectors per input.</p>
    #[serde(rename = "audioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<::std::collections::HashMap<String, AudioSelector>>,
    /// <p>Use captions selectors to specify the captions data from your input that you use in your outputs. You can use up to 20 captions selectors per input.</p>
    #[serde(rename = "captionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<::std::collections::HashMap<String, CaptionSelector>>,
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame. If you specify a value here, it will override any value that you specify in the output setting Cropping selection (crop).</p>
    #[serde(rename = "crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Enable Deblock (InputDeblockFilter) to produce smoother motion in the output. Default is disabled. Only manually controllable for MPEG2 and uncompressed video inputs.</p>
    #[serde(rename = "deblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    /// <p>Settings for decrypting any input files that you encrypt before you upload them to Amazon S3. MediaConvert can decrypt files only when you use AWS Key Management Service (KMS) to encrypt the data key that you use to encrypt your content.</p>
    #[serde(rename = "decryptionSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_settings: Option<InputDecryptionSettings>,
    /// <p>Enable Denoise (InputDenoiseFilter) to filter noise from the input.  Default is disabled. Only applicable to MPEG2, H.264, H.265, and uncompressed video inputs.</p>
    #[serde(rename = "denoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Specify the source file for your transcoding job. You can use multiple inputs in a single job. The service concatenates these inputs, in the order that you specify them in the job, to create the outputs. If your input format is IMF, specify your input by providing the path to your CPL. For example, &quot;s3://bucket/vf/cpl.xml&quot;. If the CPL is in an incomplete IMP, make sure to use <em>Supplemental IMPs</em> (SupplementalImps) to specify any supplemental IMPs that contain assets referenced by the CPL.</p>
    #[serde(rename = "fileInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_input: Option<String>,
    /// <p>Specify how the transcoding service applies the denoise and deblock filters. You must also enable the filters separately, with Denoise (InputDenoiseFilter) and Deblock (InputDeblockFilter). * Auto - The transcoding service determines whether to apply filtering, depending on input type and quality. * Disable - The input is not filtered. This is true even if you use the API to enable them in (InputDeblockFilter) and (InputDeblockFilter). * Force - The input is filtered regardless of input type.</p>
    #[serde(rename = "filterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "filterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>Enable the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input individually. This setting is disabled by default.</p>
    #[serde(rename = "imageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "inputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>When you have a progressive segmented frame (PsF) input, use this setting to flag the input as PsF. MediaConvert doesn&#39;t automatically detect PsF. Therefore, flagging your input as PsF results in better preservation of video quality when you do deinterlacing and frame rate conversion. If you don&#39;t specify, the default value is Auto (AUTO). Auto is the correct setting for all inputs that are not PsF. Don&#39;t set this value to PsF when your input is interlaced. Doing so creates horizontal interlacing artifacts.</p>
    #[serde(rename = "inputScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_scan_type: Option<String>,
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black. If you specify a value here, it will override any value that you specify in the output setting Selection placement (position). If you specify a value here, this will override any AFD values in your input, even if you set Respond to AFD (RespondToAfd) to Respond (RESPOND). If you specify a value here, this will ignore anything that you specify for the setting Scaling Behavior (scalingBehavior).</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "programNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Set PSI control (InputPsiControl) for transport stream inputs to specify which data the demux process to scans. * Ignore PSI - Scan all PIDs for audio and video. * Use PSI - Scan only PSI data.</p>
    #[serde(rename = "psiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    /// <p>Provide a list of any necessary supplemental IMPs. You need supplemental IMPs if the CPL that you&#39;re using for your input is in an incomplete IMP. Specify either the supplemental IMP directories with a trailing slash or the ASSETMAP.xml files. For example [&quot;s3://bucket/ov/&quot;, &quot;s3://bucket/vf2/ASSETMAP.xml&quot;]. You don&#39;t need to specify the IMP that contains your input CPL, because the service automatically detects it.</p>
    #[serde(rename = "supplementalImps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplemental_imps: Option<Vec<String>>,
    /// <p>Use this Timecode source setting, located under the input settings (InputTimecodeSource), to specify how the service counts input video frames. This input frame count affects only the behavior of features that apply to a single input at a time, such as input clipping and synchronizing some captions formats. Choose Embedded (EMBEDDED) to use the timecodes in your input video. Choose Start at zero (ZEROBASED) to start the first frame at zero. Choose Specified start (SPECIFIEDSTART) to start the first frame at the timecode that you specify in the setting Start timecode (timecodeStart). If you don&#39;t specify a value for Timecode source, the service will use Embedded by default. For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "timecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    /// <p>Specify the timecode that you want the service to use for this input&#39;s initial frame. To use this setting, you must set the Timecode source setting, located under the input settings (InputTimecodeSource), to Specified start (SPECIFIEDSTART). For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "timecodeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_start: Option<String>,
    /// <p>Input video selectors contain the video settings for the input. Each of your inputs can have up to one video selector.</p>
    #[serde(rename = "videoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>To transcode only portions of your input, include one input clip for each part of your input that you want in your output. All input clips that you specify will be included in every output of the job. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/assembling-multiple-inputs-and-input-clips.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputClipping {
    /// <p>Set End timecode (EndTimecode) to the end of the portion of the input you are clipping. The frame corresponding to the End timecode value is included in the clip. Start timecode or End timecode may be left blank, but not both. Use the format HH:MM:SS:FF or HH:MM:SS;FF, where HH is the hour, MM is the minute, SS is the second, and FF is the frame number. When choosing this value, take into account your setting for timecode source under input settings (InputTimecodeSource). For example, if you have embedded timecodes that start at 01:00:00:00 and you want your clip to end six minutes into the video, use 01:06:00:00.</p>
    #[serde(rename = "endTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_timecode: Option<String>,
    /// <p>Set Start timecode (StartTimecode) to the beginning of the portion of the input you are clipping. The frame corresponding to the Start timecode value is included in the clip. Start timecode or End timecode may be left blank, but not both. Use the format HH:MM:SS:FF or HH:MM:SS;FF, where HH is the hour, MM is the minute, SS is the second, and FF is the frame number. When choosing this value, take into account your setting for Input timecode source. For example, if you have embedded timecodes that start at 01:00:00:00 and you want your clip to begin five minutes into the video, use 01:05:00:00.</p>
    #[serde(rename = "startTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timecode: Option<String>,
}

/// <p>Settings for decrypting any input files that you encrypt before you upload them to Amazon S3. MediaConvert can decrypt files only when you use AWS Key Management Service (KMS) to encrypt the data key that you use to encrypt your content.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputDecryptionSettings {
    /// <p>Specify the encryption mode that you used to encrypt your input files.</p>
    #[serde(rename = "decryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decryption_mode: Option<String>,
    /// <p>Warning! Don&#39;t provide your encryption key in plaintext. Your job settings could be intercepted, making your encrypted content vulnerable. Specify the encrypted version of the data key that you used to encrypt your content. The data key must be encrypted by AWS Key Management Service (KMS). The key can be 128, 192, or 256 bits.</p>
    #[serde(rename = "encryptedDecryptionKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_decryption_key: Option<String>,
    /// <p>Specify the initialization vector that you used when you encrypted your content before uploading it to Amazon S3. You can use a 16-byte initialization vector with any encryption mode. Or, you can use a 12-byte initialization vector with GCM or CTR. MediaConvert accepts only initialization vectors that are base64-encoded.</p>
    #[serde(rename = "initializationVector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_vector: Option<String>,
    /// <p>Specify the AWS Region for AWS Key Management Service (KMS) that you used to encrypt your data key, if that Region is different from the one you are using for AWS Elemental MediaConvert.</p>
    #[serde(rename = "kmsKeyRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_region: Option<String>,
}

/// <p>Specified video input in a template.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputTemplate {
    /// <p>Use audio selector groups to combine multiple sidecar audio inputs so that you can assign them to a single output audio tab (AudioDescription). Note that, if you&#39;re working with embedded audio, it&#39;s simpler to assign multiple input tracks into a single audio selector rather than use an audio selector group.</p>
    #[serde(rename = "audioSelectorGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selector_groups: Option<::std::collections::HashMap<String, AudioSelectorGroup>>,
    /// <p>Use Audio selectors (AudioSelectors) to specify a track or set of tracks from the input that you will use in your outputs. You can use multiple Audio selectors per input.</p>
    #[serde(rename = "audioSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_selectors: Option<::std::collections::HashMap<String, AudioSelector>>,
    /// <p>Use captions selectors to specify the captions data from your input that you use in your outputs. You can use up to 20 captions selectors per input.</p>
    #[serde(rename = "captionSelectors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_selectors: Option<::std::collections::HashMap<String, CaptionSelector>>,
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame. If you specify a value here, it will override any value that you specify in the output setting Cropping selection (crop).</p>
    #[serde(rename = "crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Enable Deblock (InputDeblockFilter) to produce smoother motion in the output. Default is disabled. Only manually controllable for MPEG2 and uncompressed video inputs.</p>
    #[serde(rename = "deblockFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deblock_filter: Option<String>,
    /// <p>Enable Denoise (InputDenoiseFilter) to filter noise from the input.  Default is disabled. Only applicable to MPEG2, H.264, H.265, and uncompressed video inputs.</p>
    #[serde(rename = "denoiseFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub denoise_filter: Option<String>,
    /// <p>Specify how the transcoding service applies the denoise and deblock filters. You must also enable the filters separately, with Denoise (InputDenoiseFilter) and Deblock (InputDeblockFilter). * Auto - The transcoding service determines whether to apply filtering, depending on input type and quality. * Disable - The input is not filtered. This is true even if you use the API to enable them in (InputDeblockFilter) and (InputDeblockFilter). * Force - The input is filtered regardless of input type.</p>
    #[serde(rename = "filterEnable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_enable: Option<String>,
    /// <p>Use Filter strength (FilterStrength) to adjust the magnitude the input filter settings (Deblock and Denoise). The range is -5 to 5. Default is 0.</p>
    #[serde(rename = "filterStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_strength: Option<i64>,
    /// <p>Enable the image inserter feature to include a graphic overlay on your video. Enable or disable this feature for each input individually. This setting is disabled by default.</p>
    #[serde(rename = "imageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>(InputClippings) contains sets of start and end times that together specify a portion of the input to be used in the outputs. If you provide only a start time, the clip will be the entire input from that point to the end. If you provide only an end time, it will be the entire input up to that point. When you specify more than one input clip, the transcoding service creates the job outputs by stringing the clips together in the order you specify them.</p>
    #[serde(rename = "inputClippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_clippings: Option<Vec<InputClipping>>,
    /// <p>When you have a progressive segmented frame (PsF) input, use this setting to flag the input as PsF. MediaConvert doesn&#39;t automatically detect PsF. Therefore, flagging your input as PsF results in better preservation of video quality when you do deinterlacing and frame rate conversion. If you don&#39;t specify, the default value is Auto (AUTO). Auto is the correct setting for all inputs that are not PsF. Don&#39;t set this value to PsF when your input is interlaced. Doing so creates horizontal interlacing artifacts.</p>
    #[serde(rename = "inputScanType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_scan_type: Option<String>,
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black. If you specify a value here, it will override any value that you specify in the output setting Selection placement (position). If you specify a value here, this will override any AFD values in your input, even if you set Respond to AFD (RespondToAfd) to Respond (RESPOND). If you specify a value here, this will ignore anything that you specify for the setting Scaling Behavior (scalingBehavior).</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Program (programNumber) to select a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported. Default is the first program within the transport stream. If the program you specify doesn&#39;t exist, the transcoding service will use this default.</p>
    #[serde(rename = "programNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Set PSI control (InputPsiControl) for transport stream inputs to specify which data the demux process to scans. * Ignore PSI - Scan all PIDs for audio and video. * Use PSI - Scan only PSI data.</p>
    #[serde(rename = "psiControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psi_control: Option<String>,
    /// <p>Use this Timecode source setting, located under the input settings (InputTimecodeSource), to specify how the service counts input video frames. This input frame count affects only the behavior of features that apply to a single input at a time, such as input clipping and synchronizing some captions formats. Choose Embedded (EMBEDDED) to use the timecodes in your input video. Choose Start at zero (ZEROBASED) to start the first frame at zero. Choose Specified start (SPECIFIEDSTART) to start the first frame at the timecode that you specify in the setting Start timecode (timecodeStart). If you don&#39;t specify a value for Timecode source, the service will use Embedded by default. For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "timecodeSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_source: Option<String>,
    /// <p>Specify the timecode that you want the service to use for this input&#39;s initial frame. To use this setting, you must set the Timecode source setting, located under the input settings (InputTimecodeSource), to Specified start (SPECIFIEDSTART). For more information about timecodes, see https://docs.aws.amazon.com/console/mediaconvert/timecode.</p>
    #[serde(rename = "timecodeStart")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_start: Option<String>,
    /// <p>Input video selectors contain the video settings for the input. Each of your inputs can have up to one video selector.</p>
    #[serde(rename = "videoSelector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_selector: Option<VideoSelector>,
}

/// <p>These settings apply to a specific graphic overlay. You can include multiple overlays in your job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InsertableImage {
    /// <p>Specify the time, in milliseconds, for the image to remain on the output video. This duration includes fade-in time but not fade-out time.</p>
    #[serde(rename = "duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>Specify the length of time, in milliseconds, between the Start time that you specify for the image insertion and the time that the image appears at full opacity. Full opacity is the level that you specify for the opacity setting. If you don&#39;t specify a value for Fade-in, the image will appear abruptly at the overlay start time.</p>
    #[serde(rename = "fadeIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_in: Option<i64>,
    /// <p>Specify the length of time, in milliseconds, between the end of the time that you have specified for the image overlay Duration and when the overlaid image has faded to total transparency. If you don&#39;t specify a value for Fade-out, the image will disappear abruptly at the end of the inserted image duration.</p>
    #[serde(rename = "fadeOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fade_out: Option<i64>,
    /// <p>Specify the height of the inserted image in pixels. If you specify a value that&#39;s larger than the video resolution height, the service will crop your overlaid image to fit. To use the native height of the image, keep this setting blank.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Specify the HTTP, HTTPS, or Amazon S3 location of the image that you want to overlay on the video. Use a PNG or TGA file.</p>
    #[serde(rename = "imageInserterInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter_input: Option<String>,
    /// <p>Specify the distance, in pixels, between the inserted image and the left edge of the video frame. Required for any image overlay that you specify.</p>
    #[serde(rename = "imageX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i64>,
    /// <p>Specify the distance, in pixels, between the overlaid image and the top edge of the video frame. Required for any image overlay that you specify.</p>
    #[serde(rename = "imageY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i64>,
    /// <p>Specify how overlapping inserted images appear. Images with higher values for Layer appear on top of images with lower values for Layer.</p>
    #[serde(rename = "layer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer: Option<i64>,
    /// <p>Use Opacity (Opacity) to specify how much of the underlying video shows through the inserted image. 0 is transparent and 100 is fully opaque. Default is 50.</p>
    #[serde(rename = "opacity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opacity: Option<i64>,
    /// <p>Specify the timecode of the frame that you want the overlay to first appear on. This must be in timecode (HH:MM:SS:FF or HH:MM:SS;FF) format. Remember to take into account your timecode source settings.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// <p>Specify the width of the inserted image in pixels. If you specify a value that&#39;s larger than the video resolution width, the service will crop your overlaid image to fit. To use the native width of the image, keep this setting blank.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Each job converts an input file into an output file or files. For more information, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Job {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content.</p>
    #[serde(rename = "accelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>Describes whether the current job is running with accelerated transcoding. For jobs that have Acceleration (AccelerationMode) set to DISABLED, AccelerationStatus is always NOT<em>APPLICABLE. For jobs that have Acceleration (AccelerationMode) set to ENABLED or PREFERRED, AccelerationStatus is one of the other states. AccelerationStatus is IN</em>PROGRESS initially, while the service determines whether the input files and job settings are compatible with accelerated transcoding. If they are, AcclerationStatus is ACCELERATED. If your input files and job settings aren&#39;t compatible with accelerated transcoding, the service either fails your job or runs it without accelerated transcoding, depending on how you set Acceleration (AccelerationMode). When the service runs your job without accelerated transcoding, AccelerationStatus is NOT_ACCELERATED.</p>
    #[serde(rename = "accelerationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_status: Option<String>,
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tag type that AWS Billing and Cost Management will use to sort your AWS Elemental MediaConvert costs on any billing report that you set up.</p>
    #[serde(rename = "billingTagsSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_tags_source: Option<String>,
    /// <p>The time, in Unix epoch format in seconds, when the job got created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>A job&#39;s phase can be PROBING, TRANSCODING OR UPLOADING</p>
    #[serde(rename = "currentPhase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<String>,
    /// <p>Error code for the job</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<i64>,
    /// <p>Error message of Job</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>Optional list of hop destinations.</p>
    #[serde(rename = "hopDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// <p>A portion of the job&#39;s ARN, unique within your AWS Elemental MediaConvert resources</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>An estimate of how far your job has progressed. This estimate is shown as a percentage of the total time from when your job leaves its queue to when your output files appear in your output Amazon S3 bucket. AWS Elemental MediaConvert provides jobPercentComplete in CloudWatch STATUS_UPDATE events and in the response to GetJob and ListJobs requests. The jobPercentComplete estimate is reliable for the following input containers: Quicktime, Transport Stream, MP4, and MXF. For some jobs, the service can&#39;t provide information about job progress. In those cases, jobPercentComplete returns a null value.</p>
    #[serde(rename = "jobPercentComplete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_percent_complete: Option<i64>,
    /// <p>The job template that the job is created from, if it is created from a job template.</p>
    #[serde(rename = "jobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<String>,
    /// <p>Provides messages from the service about jobs that you have already successfully submitted.</p>
    #[serde(rename = "messages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<JobMessages>,
    /// <p>List of output group details</p>
    #[serde(rename = "outputGroupDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_details: Option<Vec<OutputGroupDetail>>,
    /// <p>Relative priority on the job.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>When you create a job, you can specify a queue to send it to. If you don&#39;t specify, the job will go to the default queue. For more about queues, see the User Guide topic at https://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>The job&#39;s queue hopping history.</p>
    #[serde(rename = "queueTransitions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_transitions: Option<Vec<QueueTransition>>,
    /// <p>The number of times that the service automatically attempted to process your job after encountering an error.</p>
    #[serde(rename = "retryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_count: Option<i64>,
    /// <p>The IAM role you use for creating this job. For details about permissions, see the User Guide topic at the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html</p>
    #[serde(rename = "role")]
    pub role: String,
    /// <p>JobSettings contains all the transcode settings for a job.</p>
    #[serde(rename = "settings")]
    pub settings: JobSettings,
    /// <p>Enable this setting when you run a test job to estimate how many reserved transcoding slots (RTS) you need. When this is enabled, MediaConvert runs your job from an on-demand queue with similar performance to what you will see with one RTS in a reserved queue. This setting is disabled by default.</p>
    #[serde(rename = "simulateReservedQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub simulate_reserved_queue: Option<String>,
    /// <p>A job&#39;s status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "statusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>Information about when jobs are submitted, started, and finished is specified in Unix epoch format in seconds.</p>
    #[serde(rename = "timing")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timing: Option<Timing>,
    /// <p>User-defined metadata that you want to associate with an MediaConvert job. You specify metadata in key/value pairs.</p>
    #[serde(rename = "userMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Provides messages from the service about jobs that you have already successfully submitted.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobMessages {
    /// <p>List of messages that are informational only and don&#39;t indicate a problem with your job.</p>
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<Vec<String>>,
    /// <p>List of messages that warn about conditions that might cause your job not to run or to fail.</p>
    #[serde(rename = "warning")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<Vec<String>>,
}

/// <p>JobSettings contains all the transcode settings for a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobSettings {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.</p>
    #[serde(rename = "adAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>Settings for ad avail blanking.  Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.</p>
    #[serde(rename = "availBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    /// <p>Settings for Event Signaling And Messaging (ESAM). If you don&#39;t do ad insertion, you can ignore these settings.</p>
    #[serde(rename = "esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<EsamSettings>,
    /// <p>Use Inputs (inputs) to define source file used in the transcode job. There can be multiple inputs add in a job. These inputs will be concantenated together to create the output.</p>
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    /// <p>Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.</p>
    #[serde(rename = "kantarWatermark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kantar_watermark: Option<KantarWatermarkSettings>,
    /// <p>Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.</p>
    #[serde(rename = "motionImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_image_inserter: Option<MotionImageInserter>,
    /// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
    #[serde(rename = "nielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that  MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to  specifying these values, you also need to set up your cloud TIC server. These settings apply to  every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]</p>
    #[serde(rename = "nielsenNonLinearWatermark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_non_linear_watermark: Option<NielsenNonLinearWatermarkSettings>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "outputGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_groups: Option<Vec<OutputGroup>>,
    /// <p>These settings control how the service handles timecodes throughout the job. These settings don&#39;t affect input clipping.</p>
    #[serde(rename = "timecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    /// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in any HLS outputs. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
    #[serde(rename = "timedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

/// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobTemplate {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content.</p>
    #[serde(rename = "accelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional category you create to organize your job templates.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The timestamp in epoch seconds for Job template creation.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description you create for each job template.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Optional list of hop destinations.</p>
    #[serde(rename = "hopDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// <p>The timestamp in epoch seconds when the Job template was last updated.</p>
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name you create for each job template. Each name must be unique within your account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Relative priority on the job.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>Optional. The queue that jobs created from this template are assigned to. If you don&#39;t specify this, jobs will go to the default queue.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "settings")]
    pub settings: JobTemplateSettings,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "statusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
    /// <p>A job template can be of two types: system or custom. System or built-in job templates can&#39;t be modified or deleted by the user.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobTemplateSettings {
    /// <p>When specified, this offset (in milliseconds) is added to the input Ad Avail PTS time.</p>
    #[serde(rename = "adAvailOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_avail_offset: Option<i64>,
    /// <p>Settings for ad avail blanking.  Video can be blanked or overlaid with an image, and audio muted during SCTE-35 triggered ad avails.</p>
    #[serde(rename = "availBlanking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avail_blanking: Option<AvailBlanking>,
    /// <p>Settings for Event Signaling And Messaging (ESAM). If you don&#39;t do ad insertion, you can ignore these settings.</p>
    #[serde(rename = "esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub esam: Option<EsamSettings>,
    /// <p>Use Inputs (inputs) to define the source file used in the transcode job. There can only be one input in a job template.  Using the API, you can include multiple inputs when referencing a job template.</p>
    #[serde(rename = "inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<InputTemplate>>,
    /// <p>Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.</p>
    #[serde(rename = "kantarWatermark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kantar_watermark: Option<KantarWatermarkSettings>,
    /// <p>Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.</p>
    #[serde(rename = "motionImageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motion_image_inserter: Option<MotionImageInserter>,
    /// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
    #[serde(rename = "nielsenConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_configuration: Option<NielsenConfiguration>,
    /// <p>Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that  MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to  specifying these values, you also need to set up your cloud TIC server. These settings apply to  every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]</p>
    #[serde(rename = "nielsenNonLinearWatermark")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_non_linear_watermark: Option<NielsenNonLinearWatermarkSettings>,
    /// <p>(OutputGroups) contains one group of settings for each set of outputs that share a common package type. All unpackaged files (MPEG-4, MPEG-2 TS, Quicktime, MXF, and no container) are grouped in a single output group as well. Required in (OutputGroups) is a group of settings that apply to the whole group. This required object depends on the value you set for (Type) under (OutputGroups)&gt;(OutputGroupSettings). Type, settings object pairs are as follows. * FILE<em>GROUP</em>SETTINGS, FileGroupSettings * HLS<em>GROUP</em>SETTINGS, HlsGroupSettings * DASH<em>ISO</em>GROUP<em>SETTINGS, DashIsoGroupSettings * MS</em>SMOOTH<em>GROUP</em>SETTINGS, MsSmoothGroupSettings * CMAF<em>GROUP</em>SETTINGS, CmafGroupSettings</p>
    #[serde(rename = "outputGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_groups: Option<Vec<OutputGroup>>,
    /// <p>These settings control how the service handles timecodes throughout the job. These settings don&#39;t affect input clipping.</p>
    #[serde(rename = "timecodeConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_config: Option<TimecodeConfig>,
    /// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in any HLS outputs. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
    #[serde(rename = "timedMetadataInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_insertion: Option<TimedMetadataInsertion>,
}

/// <p>Use these settings only when you use Kantar watermarking. Specify the values that MediaConvert uses to generate and place Kantar watermarks in your output audio. These settings apply to every output in your job. In addition to specifying these values, you also need to store your Kantar credentials in AWS Secrets Manager. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/kantar-watermarking.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KantarWatermarkSettings {
    /// <p>Provide an audio channel name from your Kantar audio license.</p>
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Specify a unique identifier for Kantar to use for this piece of content.</p>
    #[serde(rename = "contentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_reference: Option<String>,
    /// <p>Provide the name of the AWS Secrets Manager secret where your Kantar credentials are stored. Note that your MediaConvert service role must provide access to this secret. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/granting-permissions-for-mediaconvert-to-access-secrets-manager-secret.html. For instructions on creating a secret, see https://docs.aws.amazon.com/secretsmanager/latest/userguide/tutorials_basic.html, in the AWS Secrets Manager User Guide.</p>
    #[serde(rename = "credentialsSecretName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials_secret_name: Option<String>,
    /// <p>Optional. Specify an offset, in whole seconds, from the start of your output and the beginning of the watermarking. When you don&#39;t specify an offset, Kantar defaults to zero.</p>
    #[serde(rename = "fileOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_offset: Option<f64>,
    /// <p>Provide your Kantar license ID number. You should get this number from Kantar.</p>
    #[serde(rename = "kantarLicenseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kantar_license_id: Option<i64>,
    /// <p>Provide the HTTPS endpoint to the Kantar server. You should get this endpoint from Kantar.</p>
    #[serde(rename = "kantarServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kantar_server_url: Option<String>,
    /// <p>Optional. Specify the Amazon S3 bucket where you want MediaConvert to store your Kantar watermark XML logs. When you don&#39;t specify a bucket, MediaConvert doesn&#39;t save these logs. Note that your MediaConvert service role must provide access to this location. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/iam-role.html</p>
    #[serde(rename = "logDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination: Option<String>,
    /// <p>You can optionally use this field to specify the first timestamp that Kantar embeds during watermarking. Kantar suggests that you be very cautious when using this Kantar feature, and that you use it only on channels that are managed specifically for use with this feature by your Audience Measurement Operator. For more information about this feature, contact Kantar technical support.</p>
    #[serde(rename = "metadata3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_3: Option<String>,
    /// <p>Additional metadata that MediaConvert sends to Kantar. Maximum length is 50 characters.</p>
    #[serde(rename = "metadata4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_4: Option<String>,
    /// <p>Additional metadata that MediaConvert sends to Kantar. Maximum length is 50 characters.</p>
    #[serde(rename = "metadata5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_5: Option<String>,
    /// <p>Additional metadata that MediaConvert sends to Kantar. Maximum length is 50 characters.</p>
    #[serde(rename = "metadata6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_6: Option<String>,
    /// <p>Additional metadata that MediaConvert sends to Kantar. Maximum length is 50 characters.</p>
    #[serde(rename = "metadata7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_7: Option<String>,
    /// <p>Additional metadata that MediaConvert sends to Kantar. Maximum length is 50 characters.</p>
    #[serde(rename = "metadata8")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_8: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobTemplatesRequest {
    /// <p>Optionally, specify a job template category to limit responses to only job templates from that category.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. When you request a list of job templates, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by name.</p>
    #[serde(rename = "listBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of job templates, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of job templates.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobTemplatesResponse {
    /// <p>List of Job templates.</p>
    #[serde(rename = "jobTemplates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_templates: Option<Vec<JobTemplate>>,
    /// <p>Use this string to request the next batch of job templates.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobsRequest {
    /// <p>Optional. Number of jobs, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Optional. Use this string, provided with the response to a previous request, to request the next batch of jobs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    /// <p>Optional. Provide a queue name to get back only jobs from that queue.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>Optional. A job&#39;s status can be SUBMITTED, PROGRESSING, COMPLETE, CANCELED, or ERROR.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobsResponse {
    /// <p>List of jobs</p>
    #[serde(rename = "jobs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
    /// <p>Use this string to request the next batch of jobs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListPresetsRequest {
    /// <p>Optionally, specify a preset category to limit responses to only presets from that category.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>Optional. When you request a list of presets, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by name.</p>
    #[serde(rename = "listBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of presets, up to twenty, that will be returned at one time</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of presets.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListPresetsResponse {
    /// <p>Use this string to request the next batch of presets.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of presets</p>
    #[serde(rename = "presets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presets: Option<Vec<Preset>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListQueuesRequest {
    /// <p>Optional. When you request a list of queues, you can choose to list them alphabetically by NAME or chronologically by CREATION_DATE. If you don&#39;t specify, the service will list them by creation date.</p>
    #[serde(rename = "listBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_by: Option<String>,
    /// <p>Optional. Number of queues, up to twenty, that will be returned at one time.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Use this string, provided with the response to a previous request, to request the next batch of queues.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Optional. When you request lists of resources, you can specify whether they are sorted in ASCENDING or DESCENDING order. Default varies by resource.</p>
    #[serde(rename = "order")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListQueuesResponse {
    /// <p>Use this string to request the next batch of queues.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of queues.</p>
    #[serde(rename = "queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<Queue>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to list tags for. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "arn")]
    pub arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The Amazon Resource Name (ARN) and tags for an AWS Elemental MediaConvert resource.</p>
    #[serde(rename = "resourceTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_tags: Option<ResourceTags>,
}

/// <p>Settings for SCTE-35 signals from ESAM. Include this in your job settings to put SCTE-35 markers in your HLS and transport stream outputs at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct M2tsScte35Esam {
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream generated by ESAM.</p>
    #[serde(rename = "scte35EsamPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam_pid: Option<i64>,
}

/// <p>MPEG-2 TS container settings. These apply to outputs in a File output group when the output&#39;s container (ContainerType) is MPEG-2 Transport Stream (M2TS). In these assets, data is organized by the program map table (PMT). Each transport stream program contains subsets of data, including audio, video, and metadata. Each of these subsets of data has a numerical label called a packet identifier (PID). Each transport stream program corresponds to one MediaConvert output. The PMT lists the types of data in a program along with their PID. Downstream systems and players use the program map table to look up the PID for each type of data it accesses and then uses the PIDs to locate specific data within the asset.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct M2tsSettings {
    /// <p>Selects between the DVB and ATSC buffer models for Dolby Digital audio.</p>
    #[serde(rename = "audioBufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_buffer_model: Option<String>,
    /// <p>Specify this setting only when your output will be consumed by a downstream repackaging workflow that is sensitive to very small duration differences between video and audio. For this situation, choose Match video duration (MATCH<em>VIDEO</em>DURATION). In all other cases, keep the default value, Default codec duration (DEFAULT<em>CODEC</em>DURATION). When you choose Match video duration, MediaConvert pads the output audio streams with silence or trims them to ensure that the total duration of each audio stream is at least as long as the total duration of the video stream. After padding or trimming, the audio stream duration is no more than one frame longer than the video stream. MediaConvert applies audio padding or trimming only to the end of the last segment of the output. For unsegmented outputs, MediaConvert adds padding only to the end of the file. When you keep the default value, any minor discrepancies between audio and video duration will depend on your output audio codec.</p>
    #[serde(rename = "audioDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<String>,
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "audioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Specify the packet identifiers (PIDs) for any elementary audio streams you include in this output. Specify multiple PIDs as a JSON array. Default is the range 482-492.</p>
    #[serde(rename = "audioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    /// <p>Specify the output bitrate of the transport stream in bits per second. Setting to 0 lets the muxer automatically determine the appropriate bitrate. Other common values are 3750000, 7500000, and 15000000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Controls what buffer model to use for accurate interleaving. If set to MULTIPLEX, use multiplex  buffer model. If set to NONE, this can lead to lower latency, but low-memory devices may not be able to play back the stream without interruptions.</p>
    #[serde(rename = "bufferModel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_model: Option<String>,
    /// <p>Use these settings to insert a DVB Network Information Table (NIT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
    #[serde(rename = "dvbNitSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_nit_settings: Option<DvbNitSettings>,
    /// <p>Use these settings to insert a DVB Service Description Table (SDT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
    #[serde(rename = "dvbSdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sdt_settings: Option<DvbSdtSettings>,
    /// <p>Specify the packet identifiers (PIDs) for DVB subtitle data included in this output. Specify multiple PIDs as a JSON array. Default is the range 460-479.</p>
    #[serde(rename = "dvbSubPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_sub_pids: Option<Vec<i64>>,
    /// <p>Use these settings to insert a DVB Time and Date Table (TDT) in the transport stream of this output. When you work directly in your JSON job specification, include this object only when your job has a transport stream output and the container settings contain the object M2tsSettings.</p>
    #[serde(rename = "dvbTdtSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_tdt_settings: Option<DvbTdtSettings>,
    /// <p>Specify the packet identifier (PID) for DVB teletext data you include in this output. Default is 499.</p>
    #[serde(rename = "dvbTeletextPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dvb_teletext_pid: Option<i64>,
    /// <p>When set to VIDEO<em>AND</em>FIXED<em>INTERVALS, audio EBP markers will be added to partitions 3 and 4. The interval between these additional markers will be fixed, and will be slightly shorter than the video EBP marker interval. When set to VIDEO</em>INTERVAL, these additional markers will not be inserted. Only applicable when EBP segmentation markers are is selected (segmentationMarkers is EBP or EBP_LEGACY).</p>
    #[serde(rename = "ebpAudioInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_audio_interval: Option<String>,
    /// <p>Selects which PIDs to place EBP markers on. They can either be placed only on the video PID, or on both the video PID and all audio PIDs. Only applicable when EBP segmentation markers are is selected (segmentationMarkers is EBP or EBP_LEGACY).</p>
    #[serde(rename = "ebpPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebp_placement: Option<String>,
    /// <p>Controls whether to include the ES Rate field in the PES header.</p>
    #[serde(rename = "esRateInPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es_rate_in_pes: Option<String>,
    /// <p>Keep the default value (DEFAULT) unless you know that your audio EBP markers are incorrectly appearing before your video EBP markers. To correct this problem, set this value to Force (FORCE).</p>
    #[serde(rename = "forceTsVideoEbpOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_ts_video_ebp_order: Option<String>,
    /// <p>The length, in seconds, of each fragment. Only used with EBP markers.</p>
    #[serde(rename = "fragmentTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_time: Option<f64>,
    /// <p>Specify the maximum time, in milliseconds, between Program Clock References (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "maxPcrInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pcr_interval: Option<i64>,
    /// <p>When set, enforces that Encoder Boundary Points do not come within the specified time interval of each other by looking ahead at input video. If another EBP is going to come in within the specified time interval, the current EBP is not emitted, and the segment is &quot;stretched&quot; to the next marker. The lookahead value does not add latency to the system. The Live Event must be configured elsewhere to create sufficient latency to make the lookahead accurate.</p>
    #[serde(rename = "minEbpInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ebp_interval: Option<i64>,
    /// <p>If INSERT, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "nielsenId3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3: Option<String>,
    /// <p>Value in bits per second of extra null packets to insert into the transport stream. This can be used if a downstream encryption system requires periodic null packets.</p>
    #[serde(rename = "nullPacketBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub null_packet_bitrate: Option<f64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "patInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    /// <p>When set to PCR<em>EVERY</em>PES_PACKET, a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
    #[serde(rename = "pcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Specify the packet identifier (PID) for the program clock reference (PCR) in this output. If you do not specify a value, the service will use the value for Video PID (VideoPid).</p>
    #[serde(rename = "pcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    /// <p>Specify the number of milliseconds between instances of the program map table (PMT) in the output transport stream.</p>
    #[serde(rename = "pmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Specify the packet identifier (PID) for the program map table (PMT) itself. Default is 480.</p>
    #[serde(rename = "pmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    /// <p>Specify the packet identifier (PID) of the private metadata stream. Default is 503.</p>
    #[serde(rename = "privateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    /// <p>Use Program number (programNumber) to specify the program number used in the program map table (PMT) for this output. Default is 1. Program numbers and program map tables are parts of MPEG-2 transport stream containers, used for organizing data.</p>
    #[serde(rename = "programNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>When set to CBR, inserts null packets into transport stream to fill specified bitrate. When set to VBR, the bitrate setting acts as the maximum bitrate, but the output will not be padded up to that bitrate.</p>
    #[serde(rename = "rateMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_mode: Option<String>,
    /// <p>Include this in your job settings to put SCTE-35 markers in your HLS and transport stream outputs at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<M2tsScte35Esam>,
    /// <p>Specify the packet identifier (PID) of the SCTE-35 stream in the transport stream.</p>
    #[serde(rename = "scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    /// <p>For SCTE-35 markers from your input-- Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want SCTE-35 markers in this output. For SCTE-35 markers from an ESAM XML document-- Choose None (NONE). Also provide the ESAM XML as a string in the setting Signal processing notification XML (sccXml). Also enable ESAM SCTE-35 (include the property scte35Esam).</p>
    #[serde(rename = "scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    /// <p>Inserts segmentation markers at each segmentation<em>time period. rai</em>segstart sets the Random Access Indicator bit in the adaptation field. rai<em>adapt sets the RAI bit and adds the current timecode in the private data bytes. psi</em>segstart inserts PAT and PMT tables at the start of segments. ebp adds Encoder Boundary Point information to the adaptation field as per OpenCable specification OC-SP-EBP-I01-130118. ebp_legacy adds Encoder Boundary Point information to the adaptation field using a legacy proprietary format.</p>
    #[serde(rename = "segmentationMarkers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_markers: Option<String>,
    /// <p>The segmentation style parameter controls how segmentation markers are inserted into the transport stream. With avails, it is possible that segments may be truncated, which can influence where future segmentation markers are inserted. When a segmentation style of &quot;reset<em>cadence&quot; is selected and a segment is truncated due to an avail, we will reset the segmentation cadence. This means the subsequent segment will have a duration of of $segmentation</em>time seconds. When a segmentation style of &quot;maintain<em>cadence&quot; is selected and a segment is truncated due to an avail, we will not reset the segmentation cadence. This means the subsequent segment will likely be truncated as well. However, all segments after that will have a duration of $segmentation</em>time seconds. Note that EBP lookahead is a slight exception to this rule.</p>
    #[serde(rename = "segmentationStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_style: Option<String>,
    /// <p>Specify the length, in seconds, of each segment. Required unless markers is set to <em>none</em>.</p>
    #[serde(rename = "segmentationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segmentation_time: Option<f64>,
    /// <p>Specify the packet identifier (PID) for timed metadata in this output. Default is 502.</p>
    #[serde(rename = "timedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    /// <p>Specify the ID for the transport stream itself in the program map table for this output. Transport stream IDs and program map tables are parts of MPEG-2 transport stream containers, used for organizing data.</p>
    #[serde(rename = "transportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Specify the packet identifier (PID) of the elementary video stream in the transport stream.</p>
    #[serde(rename = "videoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i64>,
}

/// <p>These settings relate to the MPEG-2 transport stream (MPEG2-TS) container for the MPEG2-TS segments in your HLS outputs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct M3u8Settings {
    /// <p>Specify this setting only when your output will be consumed by a downstream repackaging workflow that is sensitive to very small duration differences between video and audio. For this situation, choose Match video duration (MATCH<em>VIDEO</em>DURATION). In all other cases, keep the default value, Default codec duration (DEFAULT<em>CODEC</em>DURATION). When you choose Match video duration, MediaConvert pads the output audio streams with silence or trims them to ensure that the total duration of each audio stream is at least as long as the total duration of the video stream. After padding or trimming, the audio stream duration is no more than one frame longer than the video stream. MediaConvert applies audio padding or trimming only to the end of the last segment of the output. For unsegmented outputs, MediaConvert adds padding only to the end of the file. When you keep the default value, any minor discrepancies between audio and video duration will depend on your output audio codec.</p>
    #[serde(rename = "audioDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<String>,
    /// <p>The number of audio frames to insert for each PES packet.</p>
    #[serde(rename = "audioFramesPerPes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_frames_per_pes: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary audio stream(s) in the transport stream. Multiple values are accepted, and can be entered in ranges and/or by comma separation.</p>
    #[serde(rename = "audioPids")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_pids: Option<Vec<i64>>,
    /// <p>Specify the maximum time, in milliseconds, between Program Clock References (PCRs) inserted into the transport stream.</p>
    #[serde(rename = "maxPcrInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pcr_interval: Option<i64>,
    /// <p>If INSERT, Nielsen inaudible tones for media tracking will be detected in the input audio and an equivalent ID3 tag will be inserted in the output.</p>
    #[serde(rename = "nielsenId3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nielsen_id_3: Option<String>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "patInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pat_interval: Option<i64>,
    /// <p>When set to PCR<em>EVERY</em>PES_PACKET a Program Clock Reference value is inserted for every Packetized Elementary Stream (PES) header. This parameter is effective only when the PCR PID is the same as the video or audio elementary stream.</p>
    #[serde(rename = "pcrControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_control: Option<String>,
    /// <p>Packet Identifier (PID) of the Program Clock Reference (PCR) in the transport stream. When no value is given, the encoder will assign the same value as the Video PID.</p>
    #[serde(rename = "pcrPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pcr_pid: Option<i64>,
    /// <p>The number of milliseconds between instances of this table in the output transport stream.</p>
    #[serde(rename = "pmtInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_interval: Option<i64>,
    /// <p>Packet Identifier (PID) for the Program Map Table (PMT) in the transport stream.</p>
    #[serde(rename = "pmtPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_pid: Option<i64>,
    /// <p>Packet Identifier (PID) of the private metadata stream in the transport stream.</p>
    #[serde(rename = "privateMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_metadata_pid: Option<i64>,
    /// <p>The value of the program number field in the Program Map Table.</p>
    #[serde(rename = "programNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Packet Identifier (PID) of the SCTE-35 stream in the transport stream.</p>
    #[serde(rename = "scte35Pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_pid: Option<i64>,
    /// <p>For SCTE-35 markers from your input-- Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want SCTE-35 markers in this output. For SCTE-35 markers from an ESAM XML document-- Choose None (NONE) if you don&#39;t want manifest conditioning. Choose Passthrough (PASSTHROUGH) and choose Ad markers (adMarkers) if you do want manifest conditioning. In both cases, also provide the ESAM XML as a string in the setting Signal processing notification XML (sccXml).</p>
    #[serde(rename = "scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
    /// <p>Applies only to HLS outputs. Use this setting to specify whether the service inserts the ID3 timed metadata from the input in this output.</p>
    #[serde(rename = "timedMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata: Option<String>,
    /// <p>Packet Identifier (PID) of the timed metadata stream in the transport stream.</p>
    #[serde(rename = "timedMetadataPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timed_metadata_pid: Option<i64>,
    /// <p>The value of the transport stream ID field in the Program Map Table.</p>
    #[serde(rename = "transportStreamId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_stream_id: Option<i64>,
    /// <p>Packet Identifier (PID) of the elementary video stream in the transport stream.</p>
    #[serde(rename = "videoPid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_pid: Option<i64>,
}

/// <p>Overlay motion graphics on top of your video. The motion graphics that you specify here appear on all outputs in all output groups. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/motion-graphic-overlay.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionImageInserter {
    /// <p>If your motion graphic asset is a .mov file, keep this setting unspecified. If your motion graphic asset is a series of .png files, specify the frame rate of the overlay in frames per second, as a fraction. For example, specify 24 fps as 24/1. Make sure that the number of images in your series matches the frame rate and your intended overlay duration. For example, if you want a 30-second overlay at 30 fps, you should have 900 .png images. This overlay frame rate doesn&#39;t need to match the frame rate of the underlying video.</p>
    #[serde(rename = "framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<MotionImageInsertionFramerate>,
    /// <p>Specify the .mov file or series of .png files that you want to overlay on your video. For .png files, provide the file name of the first file in the series. Make sure that the names of the .png files end with sequential numbers that specify the order that they are played in. For example, overlay<em>000.png, overlay</em>001.png, overlay<em>002.png, and so on. The sequence must start at zero, and each image file name must have the same number of digits. Pad your initial file names with enough zeros to complete the sequence. For example, if the first image is overlay</em>0.png, there can be only 10 images in the sequence, with the last image being overlay<em>9.png. But if the first image is overlay</em>00.png, there can be 100 images in the sequence.</p>
    #[serde(rename = "input")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    /// <p>Choose the type of motion graphic asset that you are providing for your overlay. You can choose either a .mov file or a series of .png files.</p>
    #[serde(rename = "insertionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insertion_mode: Option<String>,
    /// <p>Use Offset to specify the placement of your motion graphic overlay on the video frame. Specify in pixels, from the upper-left corner of the frame. If you don&#39;t specify an offset, the service scales your overlay to the full size of the frame. Otherwise, the service inserts the overlay at its native resolution and scales the size up or down with any video scaling.</p>
    #[serde(rename = "offset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<MotionImageInsertionOffset>,
    /// <p>Specify whether your motion graphic overlay repeats on a loop or plays only once.</p>
    #[serde(rename = "playback")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playback: Option<String>,
    /// <p>Specify when the motion overlay begins. Use timecode format (HH:MM:SS:FF or HH:MM:SS;FF). Make sure that the timecode you provide here takes into account how you have set up your timecode configuration under both job settings and input settings. The simplest way to do that is to set both to start at 0. If you need to set up your job to follow timecodes embedded in your source that don&#39;t start at zero, make sure that you specify a start time that is after the first embedded timecode. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/setting-up-timecode.html Find job-wide and input timecode configuration settings in your JSON job settings specification at settings&gt;timecodeConfig&gt;source and settings&gt;inputs&gt;timecodeSource.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

/// <p>For motion overlays that don&#39;t have a built-in frame rate, specify the frame rate of the overlay in frames per second, as a fraction. For example, specify 24 fps as 24/1. The overlay frame rate doesn&#39;t need to match the frame rate of the underlying video.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionImageInsertionFramerate {
    /// <p>The bottom of the fraction that expresses your overlay frame rate. For example, if your frame rate is 24 fps, set this value to 1.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>The top of the fraction that expresses your overlay frame rate. For example, if your frame rate is 24 fps, set this value to 24.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
}

/// <p>Specify the offset between the upper-left corner of the video frame and the top left corner of the overlay.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MotionImageInsertionOffset {
    /// <p>Set the distance, in pixels, between the overlay and the left edge of the video frame.</p>
    #[serde(rename = "imageX")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_x: Option<i64>,
    /// <p>Set the distance, in pixels, between the overlay and the top edge of the video frame.</p>
    #[serde(rename = "imageY")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_y: Option<i64>,
}

/// <p>These settings relate to your QuickTime MOV output container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MovSettings {
    /// <p>When enabled, include &#39;clap&#39; atom if appropriate for the video output settings.</p>
    #[serde(rename = "clapAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clap_atom: Option<String>,
    /// <p>When enabled, file composition times will start at zero, composition times in the &#39;ctts&#39; (composition time to sample) box for B-frames will be negative, and a &#39;cslg&#39; (composition shift least greatest) box will be included per 14496-1 amendment 1. This improves compatibility with Apple players and tools.</p>
    #[serde(rename = "cslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    /// <p>When set to XDCAM, writes MPEG2 video streams into the QuickTime file using XDCAM fourcc codes. This increases compatibility with Apple editors and players, but may decrease compatibility with other players. Only applicable when the video codec is MPEG2.</p>
    #[serde(rename = "mpeg2FourCCControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_four_cc_control: Option<String>,
    /// <p>To make this output compatible with Omenon, keep the default value, OMNEON. Unless you need Omneon compatibility, set this value to NONE. When you keep the default value, OMNEON, MediaConvert increases the length of the edit list atom. This might cause file rejections when a recipient of the output file doesn&#39;t expct this extra padding.</p>
    #[serde(rename = "paddingControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_control: Option<String>,
    /// <p>Always keep the default value (SELF_CONTAINED) for this setting.</p>
    #[serde(rename = "reference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value MP2.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mp2Settings {
    /// <p>Specify the average bitrate in bits per second.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Set Channels to specify the number of channels in this output audio track. Choosing Mono in the console will give you 1 output channel; choosing Stereo will give you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value MP3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mp3Settings {
    /// <p>Specify the average bitrate in bits per second.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Choosing Mono on the console gives you 1 output channel; choosing Stereo gives you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Specify whether the service encodes this MP3 audio output with a constant bitrate (CBR) or a variable bitrate (VBR).</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Sample rate in hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Required when you set Bitrate control mode (rateControlMode) to VBR. Specify the audio quality of this MP3 output from 0 (highest quality) to 9 (lowest quality).</p>
    #[serde(rename = "vbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<i64>,
}

/// <p>These settings relate to your MP4 output container. You can create audio only outputs with this container. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/supported-codecs-containers-audio-only.html#output-codecs-and-containers-supported-for-audio-only.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mp4Settings {
    /// <p>Specify this setting only when your output will be consumed by a downstream repackaging workflow that is sensitive to very small duration differences between video and audio. For this situation, choose Match video duration (MATCH<em>VIDEO</em>DURATION). In all other cases, keep the default value, Default codec duration (DEFAULT<em>CODEC</em>DURATION). When you choose Match video duration, MediaConvert pads the output audio streams with silence or trims them to ensure that the total duration of each audio stream is at least as long as the total duration of the video stream. After padding or trimming, the audio stream duration is no more than one frame longer than the video stream. MediaConvert applies audio padding or trimming only to the end of the last segment of the output. For unsegmented outputs, MediaConvert adds padding only to the end of the file. When you keep the default value, any minor discrepancies between audio and video duration will depend on your output audio codec.</p>
    #[serde(rename = "audioDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<String>,
    /// <p>When enabled, file composition times will start at zero, composition times in the &#39;ctts&#39; (composition time to sample) box for B-frames will be negative, and a &#39;cslg&#39; (composition shift least greatest) box will be included per 14496-1 amendment 1. This improves compatibility with Apple players and tools.</p>
    #[serde(rename = "cslgAtom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cslg_atom: Option<String>,
    /// <p>Ignore this setting unless compliance to the CTTS box version specification matters in your workflow. Specify a value of 1 to set your CTTS box version to 1 and make your output compliant with the specification. When you specify a value of 1, you must also set CSLG atom (cslgAtom) to the value INCLUDE. Keep the default value 0 to set your CTTS box version to 0. This can provide backward compatibility for some players and packagers.</p>
    #[serde(rename = "cttsVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctts_version: Option<i64>,
    /// <p>Inserts a free-space box immediately after the moov box.</p>
    #[serde(rename = "freeSpaceBox")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_space_box: Option<String>,
    /// <p>If set to PROGRESSIVE_DOWNLOAD, the MOOV atom is relocated to the beginning of the archive as required for progressive downloading. Otherwise it is placed normally at the end.</p>
    #[serde(rename = "moovPlacement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moov_placement: Option<String>,
    /// <p>Overrides the &quot;Major Brand&quot; field in the output file. Usually not necessary to specify.</p>
    #[serde(rename = "mp4MajorBrand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mp_4_major_brand: Option<String>,
}

/// <p>These settings relate to the fragmented MP4 container for the segments in your DASH outputs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MpdSettings {
    /// <p>Optional. Choose Include (INCLUDE) to have MediaConvert mark up your DASH manifest with <Accessibility> elements for embedded 608 captions. This markup isn&#39;t generally required, but some video players require it to discover and play embedded 608 captions. Keep the default value, Exclude (EXCLUDE), to leave these elements out. When you enable this setting, this is the markup that MediaConvert includes in your manifest: <Accessibility schemeIdUri="urn:scte:dash:cc:cea-608:2015" value="CC1=eng"/></p>
    #[serde(rename = "accessibilityCaptionHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessibility_caption_hints: Option<String>,
    /// <p>Specify this setting only when your output will be consumed by a downstream repackaging workflow that is sensitive to very small duration differences between video and audio. For this situation, choose Match video duration (MATCH<em>VIDEO</em>DURATION). In all other cases, keep the default value, Default codec duration (DEFAULT<em>CODEC</em>DURATION). When you choose Match video duration, MediaConvert pads the output audio streams with silence or trims them to ensure that the total duration of each audio stream is at least as long as the total duration of the video stream. After padding or trimming, the audio stream duration is no more than one frame longer than the video stream. MediaConvert applies audio padding or trimming only to the end of the last segment of the output. For unsegmented outputs, MediaConvert adds padding only to the end of the file. When you keep the default value, any minor discrepancies between audio and video duration will depend on your output audio codec.</p>
    #[serde(rename = "audioDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_duration: Option<String>,
    /// <p>Use this setting only in DASH output groups that include sidecar TTML or IMSC captions.  You specify sidecar captions in a separate output from your audio and video. Choose Raw (RAW) for captions in a single XML file in a raw container. Choose Fragmented MPEG-4 (FRAGMENTED_MP4) for captions in XML format contained within fragmented MP4 files. This set of fragmented MP4 files is separate from your video and audio fragmented MP4 files.</p>
    #[serde(rename = "captionContainerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_container_type: Option<String>,
    /// <p>Use this setting only when you specify SCTE-35 markers from ESAM. Choose INSERT to put SCTE-35 markers in this output at the insertion points that you specify in an ESAM XML document. Provide the document in the setting SCC XML (sccXml).</p>
    #[serde(rename = "scte35Esam")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_esam: Option<String>,
    /// <p>Ignore this setting unless you have SCTE-35 markers in your input video file. Choose Passthrough (PASSTHROUGH) if you want SCTE-35 markers that appear in your input to also appear in this output. Choose None (NONE) if you don&#39;t want those SCTE-35 markers in this output.</p>
    #[serde(rename = "scte35Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scte_35_source: Option<String>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value MPEG2.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Mpeg2Settings {
    /// <p>Specify the strength of any adaptive quantization filters that you enable. The value that you choose here applies to the following settings: Spatial adaptive quantization (spatialAdaptiveQuantization), and Temporal adaptive quantization (temporalAdaptiveQuantization).</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Specify the average bitrate in bits per second. Required for VBR and CBR. For MS Smooth outputs, bitrates must be unique when rounded down to the nearest multiple of 1000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Use Level (Mpeg2CodecLevel) to set the MPEG-2 level for the video output.</p>
    #[serde(rename = "codecLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_level: Option<String>,
    /// <p>Use Profile (Mpeg2CodecProfile) to set the MPEG-2 profile for the video output.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>Choose Adaptive to improve subjective video quality for high-motion content. This will cause the service to use fewer B-frames (which infer information based on other frames) for high-motion portions of the video and more B-frames for low-motion portions. The maximum number of B-frames is limited by the value you provide for the setting B frames between reference frames (numberBFramesBetweenReferenceFrames).</p>
    #[serde(rename = "dynamicSubGop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_sub_gop: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames or seconds. Must be greater than zero.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Indicates if the GOP Size in MPEG2 is specified in frames or seconds. If seconds the system will convert the GOP Size into a frame count at run time.</p>
    #[serde(rename = "gopSizeUnits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size_units: Option<String>,
    /// <p>Percentage of the buffer that should initially be filled (HRD buffer model).</p>
    #[serde(rename = "hrdBufferInitialFillPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_initial_fill_percentage: Option<i64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Use Intra DC precision (Mpeg2IntraDcPrecision) to set quantization precision for intra-block DC coefficients. If you choose the value auto, the service will automatically select the precision based on the per-frame compression ratio.</p>
    #[serde(rename = "intraDcPrecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_dc_precision: Option<String>,
    /// <p>Maximum bitrate in bits/second. For example, enter five megabits per second as 5000000.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Enforces separation between repeated (cadence) I-frames and I-frames inserted by Scene Change Detection. If a scene change I-frame is within I-interval frames of a cadence I-frame, the GOP is shrunk and/or stretched to the scene change I-frame. GOP stretch requires enabling lookahead as well as setting I-interval. The normal cadence resumes for the next GOP. This setting is only used when Scene Change Detect is enabled. Note: Maximum GOP stretch = GOP size + Min-I-interval - 1</p>
    #[serde(rename = "minIInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_i_interval: Option<i64>,
    /// <p>Number of B-frames between reference frames.</p>
    #[serde(rename = "numberBFramesBetweenReferenceFrames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_b_frames_between_reference_frames: Option<i64>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio (PAR) for this output. The default behavior, Follow source (INITIALIZE<em>FROM</em>SOURCE), uses the PAR from your input video for your output. To specify a different PAR in the console, choose any value other than Follow source. To specify a different PAR by editing the JSON job specification, choose SPECIFIED. When you choose SPECIFIED for this setting, you must also specify values for the parNumerator and parDenominator settings.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, single-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Use Rate control mode (Mpeg2RateControlMode) to specify whether the bitrate is variable (vbr) or constant (cbr).</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Enable this setting to insert I-frames at scene changes that the service automatically detects. This improves video quality and is enabled by default.</p>
    #[serde(rename = "sceneChangeDetect")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_change_detect: Option<String>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output. When you enable slow PAL, MediaConvert relabels the video frames to 25 fps and resamples your audio to keep it synchronized with the video. Note that enabling this setting will slightly reduce the duration of your video. Required settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Ignore this setting unless you need to comply with a specification that requires a specific value. If you don&#39;t have a specification requirement, we recommend that you adjust the softness of your output by using a lower value for the setting Sharpness (sharpness) or by enabling a noise reducer filter (noiseReducerFilter). The Softness (softness) setting specifies the quantization matrices that the encoder uses. Keep the default value, 0, to use the AWS Elemental default matrices. Choose a value from 17 to 128 to use planar interpolation. Increasing values from 17 to 128 result in increasing reduction of high-frequency data. The value 128 results in the softest video.</p>
    #[serde(rename = "softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>Keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on spatial variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas that can sustain more distortion with no noticeable visual degradation and uses more bits on areas where any small distortion will be noticeable. For example, complex textured blocks are encoded with fewer bits and smooth textured blocks are encoded with more bits. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen with a lot of complex texture, you might choose to disable this feature. Related setting: When you enable spatial adaptive quantization, set the value for Adaptive quantization (adaptiveQuantization) depending on your content. For homogeneous content, such as cartoons and video games, set it to Low. For content with a wider variety of textures, set it to High or Higher.</p>
    #[serde(rename = "spatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>Specify whether this output&#39;s video uses the D10 syntax. Keep the default value to  not use the syntax. Related settings: When you choose D10 (D<em>10) for your MXF  profile (profile), you must also set this value to to D10 (D</em>10).</p>
    #[serde(rename = "syntax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax: Option<String>,
    /// <p>When you do frame rate conversion from 23.976 frames per second (fps) to 29.97 fps, and your output scan type is interlaced, you can optionally enable hard or soft telecine to create a smoother picture. Hard telecine (HARD) produces a 29.97i output. Soft telecine (SOFT) produces an output with a 23.976 output that signals to the video player device to do the conversion during play back. When you keep the default value, None (NONE), MediaConvert does a standard frame rate conversion to 29.97 without doing anything with the field polarity to create a smoother picture.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on temporal variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas of the frame that aren&#39;t moving and uses more bits on complex objects with sharp edges that move a lot. For example, this feature improves the readability of text tickers on newscasts and scoreboards on sports matches. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen that doesn&#39;t have moving objects with sharp edges, such as sports athletes&#39; faces, you might choose to disable this feature. Related setting: When you enable temporal quantization, adjust the strength of the filter with the setting Adaptive quantization (adaptiveQuantization).</p>
    #[serde(rename = "temporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
}

/// <p>Specify the details for each additional Microsoft Smooth Streaming manifest that you want the service to generate for this output group. Each manifest can reference a different subset of outputs in the group.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsSmoothAdditionalManifest {
    /// <p>Specify a name modifier that the service adds to the name of this manifest to make it different from the file names of the other main manifests in the output group. For example, say that the default main manifest for your Microsoft Smooth group is film-name.ismv. If you enter &quot;-no-premium&quot; for this setting, then the file name the service generates for this top-level manifest is film-name-no-premium.ismv.</p>
    #[serde(rename = "manifestNameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_name_modifier: Option<String>,
    /// <p>Specify the outputs that you want this additional top-level manifest to reference.</p>
    #[serde(rename = "selectedOutputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_outputs: Option<Vec<String>>,
}

/// <p>If you are using DRM, set DRM System (MsSmoothEncryptionSettings) to specify the value SpekeKeyProvider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsSmoothEncryptionSettings {
    /// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
    #[serde(rename = "spekeKeyProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speke_key_provider: Option<SpekeKeyProvider>,
}

/// <p>Settings related to your Microsoft Smooth Streaming output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to MS<em>SMOOTH</em>GROUP_SETTINGS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MsSmoothGroupSettings {
    /// <p>By default, the service creates one .ism Microsoft Smooth Streaming manifest for each Microsoft Smooth Streaming output group in your job. This default manifest references every output in the output group. To create additional manifests that reference a subset of the outputs in the output group, specify a list of them here.</p>
    #[serde(rename = "additionalManifests")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_manifests: Option<Vec<MsSmoothAdditionalManifest>>,
    /// <p>COMBINE<em>DUPLICATE</em>STREAMS combines identical audio encoding settings across a Microsoft Smooth output group into a single audio stream.</p>
    #[serde(rename = "audioDeduplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_deduplication: Option<String>,
    /// <p>Use Destination (Destination) to specify the S3 output location and the output filename base. Destination accepts format identifiers. If you do not specify the base filename in the URI, the service will use the filename of the input file. If your job has multiple inputs, the service uses the filename of the first input file.</p>
    #[serde(rename = "destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// <p>Settings associated with the destination. Will vary based on the type of destination</p>
    #[serde(rename = "destinationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_settings: Option<DestinationSettings>,
    /// <p>If you are using DRM, set DRM System (MsSmoothEncryptionSettings) to specify the value SpekeKeyProvider.</p>
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<MsSmoothEncryptionSettings>,
    /// <p>Use Fragment length (FragmentLength) to specify the mp4 fragment sizes in seconds. Fragment length must be compatible with GOP size and frame rate.</p>
    #[serde(rename = "fragmentLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_length: Option<i64>,
    /// <p>Use Manifest encoding (MsSmoothManifestEncoding) to specify the encoding format for the server and client manifest. Valid options are utf8 and utf16.</p>
    #[serde(rename = "manifestEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_encoding: Option<String>,
}

/// <p>These settings relate to your MXF output container.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MxfSettings {
    /// <p>Optional. When you have AFD signaling set up in your output video stream, use this setting to choose whether to also include it in the MXF wrapper. Choose Don&#39;t copy (NO<em>COPY) to exclude AFD signaling from the MXF wrapper. Choose Copy from video stream (COPY</em>FROM_VIDEO) to copy the AFD values from the video stream for this output to the MXF wrapper. Regardless of which option you choose, the AFD values remain in the video stream. Related settings: To set up your output to include or exclude AFD values, see AfdSignaling, under VideoDescription. On the console, find AFD signaling under the output&#39;s video encoding settings.</p>
    #[serde(rename = "afdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>Specify the MXF profile, also called shim, for this output. When you choose Auto, MediaConvert chooses a profile based on the video codec and resolution. For a list of codecs supported with each MXF profile, see https://docs.aws.amazon.com/mediaconvert/latest/ug/codecs-supported-with-each-mxf-profile.html. For more information about the automatic selection behavior, see https://docs.aws.amazon.com/mediaconvert/latest/ug/default-automatic-selection-of-mxf-profiles.html.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Specify the XAVC profile settings for MXF outputs when you set your MXF profile to XAVC.</p>
    #[serde(rename = "xavcProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_profile_settings: Option<MxfXavcProfileSettings>,
}

/// <p>Specify the XAVC profile settings for MXF outputs when you set your MXF profile to XAVC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MxfXavcProfileSettings {
    /// <p>To create an output that complies with the XAVC file format guidelines for interoperability, keep the default value, Drop frames for compliance (DROP<em>FRAMES</em>FOR<em>COMPLIANCE). To include all frames from your input in this output, keep the default setting, Allow any duration (ALLOW</em>ANY_DURATION). The number of frames that MediaConvert excludes when you set this to Drop frames for compliance depends on the output frame rate and duration.</p>
    #[serde(rename = "durationMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_mode: Option<String>,
    /// <p>Specify a value for this setting only for outputs that you set up with one of these two XAVC profiles: XAVC HD Intra CBG (XAVC<em>HD</em>INTRA<em>CBG) or XAVC 4K Intra CBG (XAVC</em>4K<em>INTRA</em>CBG). Specify the amount of space in each frame that the service reserves for ancillary data, such as teletext captions. The default value for this setting is 1492 bytes per frame. This should be sufficient to prevent overflow unless you have multiple pages of teletext captions data. If you have a large amount of teletext data, specify a larger number.</p>
    #[serde(rename = "maxAncDataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_anc_data_size: Option<i64>,
}

/// <p>For forensic video watermarking, MediaConvert supports Nagra NexGuard File Marker watermarking. MediaConvert supports both PreRelease Content (NGPR/G2) and OTT Streaming workflows.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NexGuardFileMarkerSettings {
    /// <p>Use the base64 license string that Nagra provides you. Enter it directly in your JSON job specification or in the console. Required when you include Nagra NexGuard File Marker watermarking (NexGuardWatermarkingSettings) in your job.</p>
    #[serde(rename = "license")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// <p>Specify the payload ID that you want associated with this output. Valid values vary depending on your Nagra NexGuard forensic watermarking workflow. Required when you include Nagra NexGuard File Marker watermarking (NexGuardWatermarkingSettings) in your job. For PreRelease Content (NGPR/G2), specify an integer from 1 through 4,194,303. You must generate a unique ID for each asset you watermark, and keep a record of which ID you have assigned to each asset. Neither Nagra nor MediaConvert keep track of the relationship between output files and your IDs. For OTT Streaming, create two adaptive bitrate (ABR) stacks for each asset. Do this by setting up two output groups. For one output group, set the value of Payload ID (payload) to 0 in every output. For the other output group, set Payload ID (payload) to 1 in every output.</p>
    #[serde(rename = "payload")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<i64>,
    /// <p>Enter one of the watermarking preset strings that Nagra provides you. Required when you include Nagra NexGuard File Marker watermarking (NexGuardWatermarkingSettings) in your job.</p>
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// <p>Optional. Ignore this setting unless Nagra support directs you to specify a value. When you don&#39;t specify a value here, the Nagra NexGuard library uses its default value.</p>
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<String>,
}

/// <p>Settings for your Nielsen configuration. If you don&#39;t do Nielsen measurement and analytics, ignore these settings. When you enable Nielsen configuration (nielsenConfiguration), MediaConvert enables PCM to ID3 tagging for all outputs in the job. To enable Nielsen configuration programmatically, include an instance of nielsenConfiguration in your JSON job specification. Even if you don&#39;t include any children of nielsenConfiguration, you still enable the setting.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NielsenConfiguration {
    /// <p>Nielsen has discontinued the use of breakout code functionality. If you must include this property, set the value to zero.</p>
    #[serde(rename = "breakoutCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakout_code: Option<i64>,
    /// <p>Use Distributor ID (DistributorID) to specify the distributor ID that is assigned to your organization by Neilsen.</p>
    #[serde(rename = "distributorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distributor_id: Option<String>,
}

/// <p>Ignore these settings unless you are using Nielsen non-linear watermarking. Specify the values that  MediaConvert uses to generate and place Nielsen watermarks in your output audio. In addition to  specifying these values, you also need to set up your cloud TIC server. These settings apply to  every output in your job. The MediaConvert implementation is currently with the following Nielsen versions: Nielsen Watermark SDK Version 5.2.1 Nielsen NLM Watermark Engine Version 1.2.7 Nielsen Watermark Authenticator [SID_TIC] Version [5.0.0]</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NielsenNonLinearWatermarkSettings {
    /// <p>Choose the type of Nielsen watermarks that you want in your outputs. When you choose NAES 2 and NW (NAES2<em>AND</em>NW), you must provide a value for the setting SID (sourceId). When you choose CBET (CBET), you must provide a value for the setting CSID (cbetSourceId). When you choose NAES 2, NW, and CBET (NAES2<em>AND</em>NW<em>AND</em>CBET), you must provide values for both of these settings.</p>
    #[serde(rename = "activeWatermarkProcess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_watermark_process: Option<String>,
    /// <p>Optional. Use this setting when you want the service to include an ADI file in the Nielsen  metadata .zip file. To provide an ADI file, store it in Amazon S3 and provide a URL to it  here. The URL should be in the following format: S3://bucket/path/ADI-file. For more information about the metadata .zip file, see the setting Metadata destination (metadataDestination).</p>
    #[serde(rename = "adiFilename")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adi_filename: Option<String>,
    /// <p>Use the asset ID that you provide to Nielsen to uniquely identify this asset. Required for all Nielsen non-linear watermarking.</p>
    #[serde(rename = "assetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_id: Option<String>,
    /// <p>Use the asset name that you provide to Nielsen for this asset. Required for all Nielsen non-linear watermarking.</p>
    #[serde(rename = "assetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    /// <p>Use the CSID that Nielsen provides to you. This CBET source ID should be unique to your Nielsen account but common to all of your output assets that have CBET watermarking. Required when you choose a value for the setting Watermark types (ActiveWatermarkProcess) that includes CBET.</p>
    #[serde(rename = "cbetSourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cbet_source_id: Option<String>,
    /// <p>Optional. If this asset uses an episode ID with Nielsen, provide it here.</p>
    #[serde(rename = "episodeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub episode_id: Option<String>,
    /// <p>Specify the Amazon S3 location where you want MediaConvert to save your Nielsen non-linear metadata .zip file. This Amazon S3 bucket must be in the same Region as the one where you do your MediaConvert transcoding. If you want to include an ADI file in this .zip file, use the setting ADI file (adiFilename) to specify it. MediaConvert delivers the Nielsen metadata .zip files only to your metadata destination Amazon S3 bucket. It doesn&#39;t deliver the .zip files to Nielsen. You are responsible for delivering the metadata .zip files to Nielsen.</p>
    #[serde(rename = "metadataDestination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_destination: Option<String>,
    /// <p>Use the SID that Nielsen provides to you. This source ID should be unique to your Nielsen account but common to all of your output assets. Required for all Nielsen non-linear watermarking. This ID should be unique to your Nielsen account but common to all of your output assets. Required for all Nielsen non-linear watermarking.</p>
    #[serde(rename = "sourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_id: Option<i64>,
    /// <p>Required. Specify whether your source content already contains Nielsen non-linear watermarks. When you set this value to Watermarked (WATERMARKED), the service fails the job. Nielsen requires that you add non-linear watermarking to only clean content that doesn&#39;t already  have non-linear Nielsen watermarks.</p>
    #[serde(rename = "sourceWatermarkStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_watermark_status: Option<String>,
    /// <p>Specify the endpoint for the TIC server that you have deployed and configured in the AWS Cloud. Required for all Nielsen non-linear watermarking. MediaConvert can&#39;t connect directly to a TIC server. Instead, you must use API Gateway to provide a RESTful interface between MediaConvert and a TIC server that you deploy in your AWS account. For more information on deploying a TIC server in your AWS account and the required API Gateway, contact Nielsen support.</p>
    #[serde(rename = "ticServerUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tic_server_url: Option<String>,
    /// <p>To create assets that have the same TIC values in each audio track, keep the default value Share TICs (SAME<em>TICS</em>PER<em>TRACK). To create assets that have unique TIC values for each audio track, choose Use unique TICs (RESERVE</em>UNIQUE<em>TICS</em>PER_TRACK).</p>
    #[serde(rename = "uniqueTicPerAudioTrack")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_tic_per_audio_track: Option<String>,
}

/// <p>Enable the Noise reducer (NoiseReducer) feature to remove noise from your video output if necessary. Enable or disable this feature for each output individually. This setting is disabled by default. When you enable Noise reducer (NoiseReducer), you must also select a value for Noise reducer filter (NoiseReducerFilter).</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NoiseReducer {
    /// <p>Use Noise reducer filter (NoiseReducerFilter) to select one of the following spatial image filtering functions. To use this setting, you must also enable Noise reducer (NoiseReducer). * Bilateral preserves edges while reducing noise. * Mean (softest), Gaussian, Lanczos, and Sharpen (sharpest) do convolution filtering. * Conserve does min/max noise reduction. * Spatial does frequency-domain filtering based on JND principles. * Temporal optimizes video quality for complex motion.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// <p>Settings for a noise reducer filter</p>
    #[serde(rename = "filterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_settings: Option<NoiseReducerFilterSettings>,
    /// <p>Noise reducer filter settings for spatial filter.</p>
    #[serde(rename = "spatialFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_filter_settings: Option<NoiseReducerSpatialFilterSettings>,
    /// <p>Noise reducer filter settings for temporal filter.</p>
    #[serde(rename = "temporalFilterSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_filter_settings: Option<NoiseReducerTemporalFilterSettings>,
}

/// <p>Settings for a noise reducer filter</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NoiseReducerFilterSettings {
    /// <p>Relative strength of noise reducing filter. Higher values produce stronger filtering.</p>
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i64>,
}

/// <p>Noise reducer filter settings for spatial filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NoiseReducerSpatialFilterSettings {
    /// <p>Specify strength of post noise reduction sharpening filter, with 0 disabling the filter and 3 enabling it at maximum strength.</p>
    #[serde(rename = "postFilterSharpenStrength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_filter_sharpen_strength: Option<i64>,
    /// <p>The speed of the filter, from -2 (lower speed) to 3 (higher speed), with 0 being the nominal value.</p>
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// <p>Relative strength of noise reducing filter. Higher values produce stronger filtering.</p>
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i64>,
}

/// <p>Noise reducer filter settings for temporal filter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NoiseReducerTemporalFilterSettings {
    /// <p>Use Aggressive mode for content that has complex motion. Higher values produce stronger temporal filtering. This filters highly complex scenes more aggressively and creates better VQ for low bitrate outputs.</p>
    #[serde(rename = "aggressiveMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggressive_mode: Option<i64>,
    /// <p>Optional. When you set Noise reducer (noiseReducer) to Temporal (TEMPORAL), you can use this setting to apply sharpening. The default behavior, Auto (AUTO), allows the transcoder to determine whether to apply filtering, depending on input type and quality. When you set Noise reducer to Temporal, your output bandwidth is reduced. When Post temporal sharpening is also enabled, that bandwidth reduction is smaller.</p>
    #[serde(rename = "postTemporalSharpening")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_temporal_sharpening: Option<String>,
    /// <p>The speed of the filter (higher number is faster). Low setting reduces bit rate at the cost of transcode time, high setting improves transcode time at the cost of bit rate.</p>
    #[serde(rename = "speed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<i64>,
    /// <p>Specify the strength of the noise reducing filter on this output. Higher values produce stronger filtering. We recommend the following value ranges, depending on the result that you want: * 0-2 for complexity reduction with minimal sharpness loss * 2-8 for complexity reduction with image preservation * 8-16 for a high level of complexity reduction</p>
    #[serde(rename = "strength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strength: Option<i64>,
}

/// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value OPUS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OpusSettings {
    /// <p>Optional. Specify the average bitrate in bits per second. Valid values are multiples of 8000, from 32000 through 192000. The default value is 96000, which we recommend for quality and bandwidth.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Choosing Mono on the console gives you 1 output channel; choosing Stereo gives you 2. In the API, valid values are 1 and 2.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Optional. Sample rate in hz. Valid values are 16000, 24000, and 48000. The default value is 48000.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>Each output in your job is a collection of settings that describes how you want MediaConvert to encode a single output file or stream. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/create-outputs.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Output {
    /// <p>(AudioDescriptions) contains groups of audio encoding settings organized by audio codec. Include one instance of (AudioDescriptions) per output. (AudioDescriptions) can contain multiple groups of encoding settings.</p>
    #[serde(rename = "audioDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,
    /// <p>(CaptionDescriptions) contains groups of captions settings. For each output that has captions, include one instance of (CaptionDescriptions). (CaptionDescriptions) can contain multiple groups of captions settings.</p>
    #[serde(rename = "captionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescription>>,
    /// <p>Container specific settings.</p>
    #[serde(rename = "containerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    /// <p>Use Extension (Extension) to specify the file extension for outputs in File output groups. If you do not specify a value, the service will use default extensions by container type as follows * MPEG-2 transport stream, m2ts * Quicktime, mov * MXF container, mxf * MPEG-4 container, mp4 * WebM container, webm * No Container, the service will use codec extensions (e.g. AAC, H265, H265, AC3)</p>
    #[serde(rename = "extension")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extension: Option<String>,
    /// <p>Use Name modifier (NameModifier) to have the service add a string to the end of each output filename. You specify the base filename as part of your destination URI. When you create multiple outputs in the same output group, Name modifier (NameModifier) is required. Name modifier also accepts format identifiers. For DASH ISO outputs, if you use the format identifiers $Number$ or $Time$ in one output, you must use them in the same way in all outputs of the output group.</p>
    #[serde(rename = "nameModifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_modifier: Option<String>,
    /// <p>Specific settings for this type of output.</p>
    #[serde(rename = "outputSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_settings: Option<OutputSettings>,
    /// <p>Use Preset (Preset) to specify a preset for your transcoding settings. Provide the system or custom preset name. You can specify either Preset (Preset) or Container settings (ContainerSettings), but not both.</p>
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// <p>VideoDescription contains a group of video encoding settings. The specific video settings depend on the video codec that you choose for the property codec. Include one instance of VideoDescription per output.</p>
    #[serde(rename = "videoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>OutputChannel mapping settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputChannelMapping {
    /// <p>Use this setting to specify your remix values when they are integers, such as -10, 0, or 4.</p>
    #[serde(rename = "inputChannels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_channels: Option<Vec<i64>>,
    /// <p>Use this setting to specify your remix values when they have a decimal component, such as -10.312, 0.08, or 4.9. MediaConvert rounds your remixing values to the nearest thousandth.</p>
    #[serde(rename = "inputChannelsFineTune")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_channels_fine_tune: Option<Vec<f64>>,
}

/// <p>Details regarding output</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputDetail {
    /// <p>Duration in milliseconds</p>
    #[serde(rename = "durationInMs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_ms: Option<i64>,
    /// <p>Contains details about the output&#39;s video stream</p>
    #[serde(rename = "videoDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_details: Option<VideoDetail>,
}

/// <p>Group of outputs</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputGroup {
    /// <p>Use automated encoding to have MediaConvert choose your encoding settings for you, based on characteristics of your input video.</p>
    #[serde(rename = "automatedEncodingSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_encoding_settings: Option<AutomatedEncodingSettings>,
    /// <p>Use Custom Group Name (CustomName) to specify a name for the output group. This value is displayed on the console and can make your job settings JSON more human-readable. It does not affect your outputs. Use up to twelve characters that are either letters, numbers, spaces, or underscores.</p>
    #[serde(rename = "customName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// <p>Name of the output group</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Output Group settings, including type</p>
    #[serde(rename = "outputGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_group_settings: Option<OutputGroupSettings>,
    /// <p>This object holds groups of encoding settings, one group of settings per output.</p>
    #[serde(rename = "outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

/// <p>Contains details about the output groups specified in the job settings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OutputGroupDetail {
    /// <p>Details about the output</p>
    #[serde(rename = "outputDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_details: Option<Vec<OutputDetail>>,
}

/// <p>Output Group settings, including type</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputGroupSettings {
    /// <p>Settings related to your CMAF output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to CMAF<em>GROUP</em>SETTINGS.</p>
    #[serde(rename = "cmafGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmaf_group_settings: Option<CmafGroupSettings>,
    /// <p>Settings related to your DASH output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to DASH<em>ISO</em>GROUP_SETTINGS.</p>
    #[serde(rename = "dashIsoGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_iso_group_settings: Option<DashIsoGroupSettings>,
    /// <p>Settings related to your File output group. MediaConvert uses this group of settings to generate a single standalone file, rather than a streaming package. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to FILE<em>GROUP</em>SETTINGS.</p>
    #[serde(rename = "fileGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_group_settings: Option<FileGroupSettings>,
    /// <p>Settings related to your HLS output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to HLS<em>GROUP</em>SETTINGS.</p>
    #[serde(rename = "hlsGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_group_settings: Option<HlsGroupSettings>,
    /// <p>Settings related to your Microsoft Smooth Streaming output package. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/outputs-file-ABR.html. When you work directly in your JSON job specification, include this object and any required children when you set Type, under OutputGroupSettings, to MS<em>SMOOTH</em>GROUP_SETTINGS.</p>
    #[serde(rename = "msSmoothGroupSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ms_smooth_group_settings: Option<MsSmoothGroupSettings>,
    /// <p>Type of output group (File group, Apple HLS, DASH ISO, Microsoft Smooth Streaming, CMAF)</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Specific settings for this type of output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputSettings {
    /// <p>Settings for HLS output groups</p>
    #[serde(rename = "hlsSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_settings: Option<HlsSettings>,
}

/// <p>If you work with a third party video watermarking partner, use the group of settings that correspond with your watermarking partner to include watermarks in your output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PartnerWatermarking {
    /// <p>For forensic video watermarking, MediaConvert supports Nagra NexGuard File Marker watermarking. MediaConvert supports both PreRelease Content (NGPR/G2) and OTT Streaming workflows.</p>
    #[serde(rename = "nexguardFileMarkerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nexguard_file_marker_settings: Option<NexGuardFileMarkerSettings>,
}

/// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Preset {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>An optional category you create to organize your presets.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The timestamp in epoch seconds for preset creation.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description you create for each preset.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds when the preset was last updated.</p>
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name you create for each preset. Each name must be unique within your account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Settings for preset</p>
    #[serde(rename = "settings")]
    pub settings: PresetSettings,
    /// <p>A preset can be of two types: system or custom. System or built-in preset can&#39;t be modified or deleted by the user.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Settings for preset</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PresetSettings {
    /// <p>(AudioDescriptions) contains groups of audio encoding settings organized by audio codec. Include one instance of (AudioDescriptions) per output. (AudioDescriptions) can contain multiple groups of encoding settings.</p>
    #[serde(rename = "audioDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_descriptions: Option<Vec<AudioDescription>>,
    /// <p>This object holds groups of settings related to captions for one output. For each output that has captions, include one instance of CaptionDescriptions.</p>
    #[serde(rename = "captionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_descriptions: Option<Vec<CaptionDescriptionPreset>>,
    /// <p>Container specific settings.</p>
    #[serde(rename = "containerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_settings: Option<ContainerSettings>,
    /// <p>VideoDescription contains a group of video encoding settings. The specific video settings depend on the video codec that you choose for the property codec. Include one instance of VideoDescription per output.</p>
    #[serde(rename = "videoDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_description: Option<VideoDescription>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value PRORES.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProresSettings {
    /// <p>This setting applies only to ProRes 4444 and ProRes 4444 XQ outputs that you create from inputs that use 4:4:4 chroma sampling. Set Preserve 4:4:4 sampling (PRESERVE<em>444</em>SAMPLING) to allow outputs to also use 4:4:4 chroma sampling. You must specify a value for this setting when your output codec profile supports 4:4:4 chroma sampling. Related Settings: When you set Chroma sampling to Preserve 4:4:4 sampling (PRESERVE<em>444</em>SAMPLING), you must choose an output codec profile that supports 4:4:4 chroma sampling. These values for Profile (CodecProfile) support 4:4:4 chroma sampling: Apple ProRes 4444 (APPLE<em>PRORES</em>4444) or Apple ProRes 4444 XQ (APPLE<em>PRORES</em>4444<em>XQ). When you set Chroma sampling to Preserve 4:4:4 sampling, you must disable all video preprocessors except for Nexguard file marker (PartnerWatermarking). When you set Chroma sampling to Preserve 4:4:4 sampling and use framerate conversion, you must set Frame rate conversion algorithm (FramerateConversionAlgorithm) to Drop duplicate (DUPLICATE</em>DROP).</p>
    #[serde(rename = "chromaSampling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chroma_sampling: Option<String>,
    /// <p>Use Profile (ProResCodecProfile) to specify the type of Apple ProRes codec to use for this output.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio (PAR) for this output. The default behavior, Follow source (INITIALIZE<em>FROM</em>SOURCE), uses the PAR from your input video for your output. To specify a different PAR in the console, choose any value other than Follow source. To specify a different PAR by editing the JSON job specification, choose SPECIFIED. When you choose SPECIFIED for this setting, you must also specify values for the parNumerator and parDenominator settings.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output. When you enable slow PAL, MediaConvert relabels the video frames to 25 fps and resamples your audio to keep it synchronized with the video. Note that enabling this setting will slightly reduce the duration of your video. Required settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>When you do frame rate conversion from 23.976 frames per second (fps) to 29.97 fps, and your output scan type is interlaced, you can optionally enable hard telecine (HARD) to create a smoother picture. When you keep the default value, None (NONE), MediaConvert does a standard frame rate conversion to 29.97 without doing anything with the field polarity to create a smoother picture.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
}

/// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Queue {
    /// <p>An identifier for this resource that is unique within all of AWS.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The timestamp in epoch seconds for when you created the queue.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>An optional description that you create for each queue.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The timestamp in epoch seconds for when you most recently updated the queue.</p>
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    /// <p>A name that you create for each queue. Each name must be unique within your account.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specifies whether the pricing plan for the queue is on-demand or reserved. For on-demand, you pay per minute, billed in increments of .01 minute. For reserved, you pay for the transcoding capacity of the entire queue, regardless of how much or how little you use it. Reserved pricing requires a 12-month commitment.</p>
    #[serde(rename = "pricingPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_plan: Option<String>,
    /// <p>The estimated number of jobs with a PROGRESSING status.</p>
    #[serde(rename = "progressingJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progressing_jobs_count: Option<i64>,
    /// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
    #[serde(rename = "reservationPlan")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan: Option<ReservationPlan>,
    /// <p>Queues can be ACTIVE or PAUSED. If you pause a queue, the service won&#39;t begin processing jobs in that queue. Jobs that are running when you pause the queue continue to run until they finish or result in an error.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The estimated number of jobs with a SUBMITTED status.</p>
    #[serde(rename = "submittedJobsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_jobs_count: Option<i64>,
    /// <p>Specifies whether this on-demand queue is system or custom. System queues are built in. You can&#39;t modify or delete system queues. You can create and modify custom queues.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Description of the source and destination queues between which the job has moved, along with the timestamp of the move</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct QueueTransition {
    /// <p>The queue that the job was on after the transition.</p>
    #[serde(rename = "destinationQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_queue: Option<String>,
    /// <p>The queue that the job was on before the transition.</p>
    #[serde(rename = "sourceQueue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_queue: Option<String>,
    /// <p>The time, in Unix epoch format, that the job moved from the source queue to the destination queue.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Use Rectangle to identify a specific area of the video frame.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Rectangle {
    /// <p>Height of rectangle in pixels. Specify only even numbers.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Width of rectangle in pixels. Specify only even numbers.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// <p>The distance, in pixels, between the rectangle and the left edge of the video frame. Specify only even numbers.</p>
    #[serde(rename = "x")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<i64>,
    /// <p>The distance, in pixels, between the rectangle and the top edge of the video frame. Specify only even numbers.</p>
    #[serde(rename = "y")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<i64>,
}

/// <p>Use Manual audio remixing (RemixSettings) to adjust audio levels for each audio channel in each output of your job. With audio remixing, you can output more or fewer audio channels than your input audio source provides.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RemixSettings {
    /// <p>Channel mapping (ChannelMapping) contains the group of fields that hold the remixing value for each channel, in dB. Specify remix values to indicate how much of the content from your input audio channel you want in your output audio channels. Each instance of the InputChannels or InputChannelsFineTune array specifies these values for one output channel. Use one instance of this array for each output channel. In the console, each array corresponds to a column in the graphical depiction of the mapping matrix. The rows of the graphical matrix correspond to input channels. Valid values are within the range from -60 (mute) through 6. A setting of 0 passes the input channel unchanged to the output channel (no attenuation or amplification). Use InputChannels or InputChannelsFineTune to specify your remix values. Don&#39;t use both.</p>
    #[serde(rename = "channelMapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_mapping: Option<ChannelMapping>,
    /// <p>Specify the number of audio channels from your input that you want to use in your output. With remixing, you might combine or split the data in these channels, so the number of channels in your final output might be different. If you are doing both input channel mapping and output channel mapping, the number of output channels in your input mapping must be the same as the number of input channels in your output mapping.</p>
    #[serde(rename = "channelsIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_in: Option<i64>,
    /// <p>Specify the number of channels in this output after remixing. Valid values: 1, 2, 4, 6, 8... 64. (1 and even numbers to 64.) If you are doing both input channel mapping and output channel mapping, the number of output channels in your input mapping must be the same as the number of input channels in your output mapping.</p>
    #[serde(rename = "channelsOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels_out: Option<i64>,
}

/// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReservationPlan {
    /// <p>The length of the term of your reserved queue pricing plan commitment.</p>
    #[serde(rename = "commitment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commitment: Option<String>,
    /// <p>The timestamp in epoch seconds for when the current pricing plan term for this reserved queue expires.</p>
    #[serde(rename = "expiresAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<f64>,
    /// <p>The timestamp in epoch seconds for when you set up the current pricing plan for this reserved queue.</p>
    #[serde(rename = "purchasedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_at: Option<f64>,
    /// <p>Specifies whether the term of your reserved queue pricing plan is automatically extended (AUTO_RENEW) or expires (EXPIRE) at the end of the term.</p>
    #[serde(rename = "renewalType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_type: Option<String>,
    /// <p>Specifies the number of reserved transcode slots (RTS) for this queue. The number of RTS determines how many jobs the queue can process in parallel; each RTS can process one job at a time. When you increase this number, you extend your existing commitment with a new 12-month commitment for a larger number of RTS. The new commitment begins when you purchase the additional capacity. You can&#39;t decrease the number of RTS in your reserved queue.</p>
    #[serde(rename = "reservedSlots")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_slots: Option<i64>,
    /// <p>Specifies whether the pricing plan for your reserved queue is ACTIVE or EXPIRED.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Details about the pricing plan for your reserved queue. Required for reserved queues and not applicable to on-demand queues.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReservationPlanSettings {
    /// <p>The length of the term of your reserved queue pricing plan commitment.</p>
    #[serde(rename = "commitment")]
    pub commitment: String,
    /// <p>Specifies whether the term of your reserved queue pricing plan is automatically extended (AUTO_RENEW) or expires (EXPIRE) at the end of the term. When your term is auto renewed, you extend your commitment by 12 months from the auto renew date. You can cancel this commitment.</p>
    #[serde(rename = "renewalType")]
    pub renewal_type: String,
    /// <p>Specifies the number of reserved transcode slots (RTS) for this queue. The number of RTS determines how many jobs the queue can process in parallel; each RTS can process one job at a time. You can&#39;t decrease the number of RTS in your reserved queue. You can increase the number of RTS by extending your existing commitment with a new 12-month commitment for the larger number. The new commitment begins when you purchase the additional capacity. You can&#39;t cancel your commitment or revert to your original commitment after you increase the capacity.</p>
    #[serde(rename = "reservedSlots")]
    pub reserved_slots: i64,
}

/// <p>The Amazon Resource Name (ARN) and tags for an AWS Elemental MediaConvert resource.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourceTags {
    /// <p>The Amazon Resource Name (ARN) of the resource.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The tags for the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don&#39;t use this setting, S3 automatically applies the default access control list PRIVATE.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3DestinationAccessControl {
    /// <p>Choose an Amazon S3 canned ACL for MediaConvert to apply to this output.</p>
    #[serde(rename = "cannedAcl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_acl: Option<String>,
}

/// <p>Settings associated with S3 destination</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3DestinationSettings {
    /// <p>Optional. Have MediaConvert automatically apply Amazon S3 access control for the outputs in this output group. When you don&#39;t use this setting, S3 automatically applies the default access control list PRIVATE.</p>
    #[serde(rename = "accessControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<S3DestinationAccessControl>,
    /// <p>Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.</p>
    #[serde(rename = "encryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<S3EncryptionSettings>,
}

/// <p>Settings for how your job outputs are encrypted as they are uploaded to Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3EncryptionSettings {
    /// <p>Specify how you want your data keys managed. AWS uses data keys to encrypt your content. AWS also encrypts the data keys themselves, using a customer master key (CMK), and then stores the encrypted data keys alongside your encrypted content. Use this setting to specify which AWS service manages the CMK. For simplest set up, choose Amazon S3 (SERVER<em>SIDE</em>ENCRYPTION<em>S3). If you want your master key to be managed by AWS Key Management Service (KMS), choose AWS KMS (SERVER</em>SIDE<em>ENCRYPTION</em>KMS). By default, when you choose AWS KMS, KMS uses the AWS managed customer master key (CMK) associated with Amazon S3 to encrypt your data keys. You can optionally choose to specify a different, customer managed CMK. Do so by specifying the Amazon Resource Name (ARN) of the key for the setting  KMS ARN (kmsKeyArn).</p>
    #[serde(rename = "encryptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    /// <p>Optionally, specify the customer master key (CMK) that you want to use to encrypt the data key that AWS uses to encrypt your output content. Enter the Amazon Resource Name (ARN) of the CMK. To use this setting, you must also set Server-side encryption (S3ServerSideEncryptionType) to AWS KMS (SERVER<em>SIDE</em>ENCRYPTION_KMS). If you set Server-side encryption to AWS KMS but don&#39;t specify a CMK here, AWS uses the AWS managed CMK associated with Amazon S3.</p>
    #[serde(rename = "kmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
}

/// <p>Settings related to SCC captions. SCC is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/scc-srt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to SCC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SccDestinationSettings {
    /// <p>Set Framerate (SccDestinationFramerate) to make sure that the captions and the video are synchronized in the output. Specify a frame rate that matches the frame rate of the associated video. If the video frame rate is 29.97, choose 29.97 dropframe (FRAMERATE<em>29</em>97<em>DROPFRAME) only if the video has video</em>insertion=true and drop<em>frame</em>timecode=true; otherwise, choose 29.97 non-dropframe (FRAMERATE<em>29</em>97<em>NON</em>DROPFRAME).</p>
    #[serde(rename = "framerate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate: Option<String>,
}

/// <p>If your output group type is HLS, DASH, or Microsoft Smooth, use these settings when doing DRM encryption with a SPEKE-compliant key provider.  If your output group type is CMAF, use the SpekeKeyProviderCmaf settings instead.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SpekeKeyProvider {
    /// <p>If you want your key provider to encrypt the content keys that it provides to MediaConvert, set up a certificate with a master key using AWS Certificate Manager. Specify the certificate&#39;s Amazon Resource Name (ARN) here.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Specify the resource ID that your SPEKE-compliant key provider uses to identify this content.</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>Relates to SPEKE implementation. DRM system identifiers. DASH output groups support a max of two system ids. Other group types support one system id. See
    /// https://dashif.org/identifiers/content_protection/ for more details.</p>
    #[serde(rename = "systemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_ids: Option<Vec<String>>,
    /// <p>Specify the URL to the key server that your SPEKE-compliant DRM key provider uses to provide keys for encrypting your content.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>If your output group type is CMAF, use these settings when doing DRM encryption with a SPEKE-compliant key provider. If your output group type is HLS, DASH, or Microsoft Smooth, use the SpekeKeyProvider settings instead.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SpekeKeyProviderCmaf {
    /// <p>If you want your key provider to encrypt the content keys that it provides to MediaConvert, set up a certificate with a master key using AWS Certificate Manager. Specify the certificate&#39;s Amazon Resource Name (ARN) here.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>Specify the DRM system IDs that you want signaled in the DASH manifest that MediaConvert creates as part of this CMAF package. The DASH manifest can currently signal up to three system IDs. For more information, see https://dashif.org/identifiers/content_protection/.</p>
    #[serde(rename = "dashSignaledSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dash_signaled_system_ids: Option<Vec<String>>,
    /// <p>Specify the DRM system ID that you want signaled in the HLS manifest that MediaConvert creates as part of this CMAF package. The HLS manifest can currently signal only one system ID. For more information, see https://dashif.org/identifiers/content_protection/.</p>
    #[serde(rename = "hlsSignaledSystemIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hls_signaled_system_ids: Option<Vec<String>>,
    /// <p>Specify the resource ID that your SPEKE-compliant key provider uses to identify this content.</p>
    #[serde(rename = "resourceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    /// <p>Specify the URL to the key server that your SPEKE-compliant DRM key provider uses to provide keys for encrypting your content.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Use these settings to set up encryption with a static key provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct StaticKeyProvider {
    /// <p>Relates to DRM implementation. Sets the value of the KEYFORMAT attribute. Must be &#39;identity&#39; or a reverse DNS string. May be omitted to indicate an implicit value of &#39;identity&#39;.</p>
    #[serde(rename = "keyFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format: Option<String>,
    /// <p>Relates to DRM implementation. Either a single positive integer version value or a slash delimited list of version values (1/2/3).</p>
    #[serde(rename = "keyFormatVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_format_versions: Option<String>,
    /// <p>Relates to DRM implementation. Use a 32-character hexidecimal string to specify Key Value (StaticKeyValue).</p>
    #[serde(rename = "staticKeyValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_key_value: Option<String>,
    /// <p>Relates to DRM implementation. The location of the license server used for protecting content.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to tag. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The tags that you want to add to the resource. You can tag resources with a key-value pair or with only a key.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Settings related to teletext captions. Set up teletext captions in the same output as your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/teletext-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to TELETEXT.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TeletextDestinationSettings {
    /// <p>Set pageNumber to the Teletext page number for the destination captions for this output. This value must be a three-digit hexadecimal string; strings ending in -FF are invalid. If you are passing through the entire set of Teletext data, do not use this field.</p>
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
    /// <p>Specify the page types for this Teletext page. If you don&#39;t specify a value here, the service sets the page type to the default value Subtitle (PAGE<em>TYPE</em>SUBTITLE). If you pass through the entire set of Teletext data, don&#39;t use this field. When you pass through a set of Teletext pages, your output has the same page types as your input.</p>
    #[serde(rename = "pageTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_types: Option<Vec<String>>,
}

/// <p>Settings specific to Teletext caption sources, including Page number.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TeletextSourceSettings {
    /// <p>Use Page Number (PageNumber) to specify the three-digit hexadecimal page number that will be used for Teletext captions. Do not use this setting if you are passing through teletext from the input source to output.</p>
    #[serde(rename = "pageNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_number: Option<String>,
}

/// <p>Settings for burning the output timecode and specified prefix into the output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimecodeBurnin {
    /// <p>Use Font Size (FontSize) to set the font size of any burned-in timecode. Valid values are 10, 16, 32, 48.</p>
    #[serde(rename = "fontSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i64>,
    /// <p>Use Position (Position) under under Timecode burn-in (TimecodeBurnIn) to specify the location the burned-in timecode on output video.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
    /// <p>Use Prefix (Prefix) to place ASCII characters before any burned-in timecode. For example, a prefix of &quot;EZ-&quot; will result in the timecode &quot;EZ-00:00:00:00&quot;. Provide either the characters themselves or the ASCII code equivalents. The supported range of characters is 0x20 through 0x7e. This includes letters, numbers, and all special characters represented on a standard English keyboard.</p>
    #[serde(rename = "prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// <p>These settings control how the service handles timecodes throughout the job. These settings don&#39;t affect input clipping.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimecodeConfig {
    /// <p>If you use an editing platform that relies on an anchor timecode, use Anchor Timecode (Anchor) to specify a timecode that will match the input video frame to the output video frame. Use 24-hour format with frame number, (HH:MM:SS:FF) or (HH:MM:SS;FF). This setting ignores frame rate conversion. System behavior for Anchor Timecode varies depending on your setting for Source (TimecodeSource). * If Source (TimecodeSource) is set to Specified Start (SPECIFIEDSTART), the first input frame is the specified value in Start Timecode (Start). Anchor Timecode (Anchor) and Start Timecode (Start) are used calculate output timecode. * If Source (TimecodeSource) is set to Start at 0 (ZEROBASED)  the  first frame is 00:00:00:00. * If Source (TimecodeSource) is set to Embedded (EMBEDDED), the  first frame is the timecode value on the first input frame of the input.</p>
    #[serde(rename = "anchor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// <p>Use Source (TimecodeSource) to set how timecodes are handled within this job. To make sure that your video, audio, captions, and markers are synchronized and that time-based features, such as image inserter, work correctly, choose the Timecode source option that matches your assets. All timecodes are in a 24-hour format with frame number (HH:MM:SS:FF). * Embedded (EMBEDDED) - Use the timecode that is in the input video. If no embedded timecode is in the source, the service will use Start at 0 (ZEROBASED) instead. * Start at 0 (ZEROBASED) - Set the timecode of the initial frame to 00:00:00:00. * Specified Start (SPECIFIEDSTART) - Set the timecode of the initial frame to a value other than zero. You use Start timecode (Start) to provide this value.</p>
    #[serde(rename = "source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// <p>Only use when you set Source (TimecodeSource) to Specified start (SPECIFIEDSTART). Use Start timecode (Start) to specify the timecode for the initial frame. Use 24-hour format with frame number, (HH:MM:SS:FF) or (HH:MM:SS;FF).</p>
    #[serde(rename = "start")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    /// <p>Only applies to outputs that support program-date-time stamp. Use Timestamp offset (TimestampOffset) to overwrite the timecode date without affecting the time and frame number. Provide the new date as a string in the format &quot;yyyy-mm-dd&quot;.  To use Time stamp offset, you must also enable Insert program-date-time (InsertProgramDateTime) in the output settings. For example, if the date part of your timecodes is 2002-1-25 and you want to change it to one year later, set Timestamp offset (TimestampOffset) to 2003-1-25.</p>
    #[serde(rename = "timestampOffset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_offset: Option<String>,
}

/// <p>Enable Timed metadata insertion (TimedMetadataInsertion) to include ID3 tags in any HLS outputs. To include timed metadata, you must enable it here, enable it in each output container, and specify tags and timecodes in ID3 insertion (Id3Insertion) objects.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TimedMetadataInsertion {
    /// <p>Id3Insertions contains the array of Id3Insertion instances.</p>
    #[serde(rename = "id3Insertions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_3_insertions: Option<Vec<Id3Insertion>>,
}

/// <p>Information about when jobs are submitted, started, and finished is specified in Unix epoch format in seconds.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Timing {
    /// <p>The time, in Unix epoch format, that the transcoding job finished</p>
    #[serde(rename = "finishTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<f64>,
    /// <p>The time, in Unix epoch format, that transcoding for the job began.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The time, in Unix epoch format, that you submitted the job.</p>
    #[serde(rename = "submitTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_time: Option<f64>,
}

/// <p>Settings specific to caption sources that are specified by track number. Currently, this is only IMSC captions in an IMF package. If your caption source is IMSC 1.1 in a separate xml file, use FileSourceSettings instead of TrackSourceSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TrackSourceSettings {
    /// <p>Use this setting to select a single captions track from a source. Track numbers correspond to the order in the captions source file. For IMF sources, track numbering is based on the order that the captions appear in the CPL. For example, use 1 to select the captions asset that is listed first in the CPL. To include more than one captions track in your job outputs, create multiple input captions selectors. Specify one track per selector.</p>
    #[serde(rename = "trackNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_number: Option<i64>,
}

/// <p>Settings related to TTML captions. TTML is a sidecar format that holds captions in a file that is separate from the video container. Set up sidecar captions in the same output group, but different output from your video. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/ttml-and-webvtt-output-captions.html. When you work directly in your JSON job specification, include this object and any required children when you set destinationType to TTML.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TtmlDestinationSettings {
    /// <p>Pass through style and position information from a TTML-like input source (TTML, IMSC, SMPTE-TT) to the TTML output.</p>
    #[serde(rename = "stylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) of the resource that you want to remove tags from. To get the ARN, send a GET request with the resource name.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The keys of the tags that you want to remove from the resource.</p>
    #[serde(rename = "tagKeys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateJobTemplateRequest {
    /// <p>Accelerated transcoding can significantly speed up jobs with long, visually complex content. Outputs that use this feature incur pro-tier pricing. For information about feature limitations, see the AWS Elemental MediaConvert User Guide.</p>
    #[serde(rename = "accelerationSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceleration_settings: Option<AccelerationSettings>,
    /// <p>The new category for the job template, if you are changing it.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The new description for the job template, if you are changing it.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Optional list of hop destinations.</p>
    #[serde(rename = "hopDestinations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hop_destinations: Option<Vec<HopDestination>>,
    /// <p>The name of the job template you are modifying</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Specify the relative priority for this job. In any given queue, the service begins processing the job with the highest value first. When more than one job has the same priority, the service begins processing the job that you submitted first. If you don&#39;t specify a priority, the service uses the default value 0.</p>
    #[serde(rename = "priority")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    /// <p>The new queue for the job template, if you are changing it.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    /// <p>JobTemplateSettings contains all the transcode settings saved in the template that will be applied to jobs created from it.</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<JobTemplateSettings>,
    /// <p>Specify how often MediaConvert sends STATUS_UPDATE events to Amazon CloudWatch Events. Set the interval, in seconds, between status updates. MediaConvert sends an update at this interval from the time the service begins processing your job to the time it completes the transcode or encounters an error.</p>
    #[serde(rename = "statusUpdateInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_update_interval: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateJobTemplateResponse {
    /// <p>A job template is a pre-made set of encoding instructions that you can use to quickly create a job.</p>
    #[serde(rename = "jobTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_template: Option<JobTemplate>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdatePresetRequest {
    /// <p>The new category for the preset, if you are changing it.</p>
    #[serde(rename = "category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// <p>The new description for the preset, if you are changing it.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the preset you are modifying.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Settings for preset</p>
    #[serde(rename = "settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PresetSettings>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdatePresetResponse {
    /// <p>A preset is a collection of preconfigured media conversion settings that you want MediaConvert to apply to the output during the conversion process.</p>
    #[serde(rename = "preset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<Preset>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateQueueRequest {
    /// <p>The new description for the queue, if you are changing it.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the queue that you are modifying.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The new details of your pricing plan for your reserved queue. When you set up a new pricing plan to replace an expired one, you enter into another 12-month commitment. When you add capacity to your queue by increasing the number of RTS, you extend the term of your commitment to 12 months from when you add capacity. After you make these commitments, you can&#39;t cancel them.</p>
    #[serde(rename = "reservationPlanSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_plan_settings: Option<ReservationPlanSettings>,
    /// <p>Pause or activate a queue by changing its status between ACTIVE and PAUSED. If you pause a queue, jobs in that queue won&#39;t begin. Jobs that are running when you pause the queue continue to run until they finish or result in an error.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateQueueResponse {
    /// <p>You can use queues to manage the resources that are available to your AWS account for running multiple transcoding jobs at the same time. If you don&#39;t specify a queue, the service sends all jobs through the default queue. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html.</p>
    #[serde(rename = "queue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<Queue>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VC3</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vc3Settings {
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Optional. Choose the scan line type for this output. If you don&#39;t specify a value, MediaConvert will create a progressive output.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Use this setting for interlaced outputs, when your output frame rate is half of your input frame rate. In this situation, choose Optimized interlacing (INTERLACED_OPTIMIZE) to create a better quality interlaced output. In this case, each progressive frame from the input corresponds to an interlaced field in the output. Keep the default value, Basic interlacing (INTERLACED), for all other output frame rates. With basic interlacing, MediaConvert performs any frame rate conversion first and then interlaces the frames. When you choose Optimized interlacing and you set your output frame rate to a value that isn&#39;t suitable for optimized interlacing, MediaConvert automatically falls back to basic interlacing. Required settings: To use optimized interlacing, you must set Telecine (telecine) to None (NONE) or Soft (SOFT). You can&#39;t use optimized interlacing for hard telecine outputs. You must also set Interlace mode (interlaceMode) to a value other than Progressive (PROGRESSIVE).</p>
    #[serde(rename = "scanTypeConversionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scan_type_conversion_mode: Option<String>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output by relabeling the video frames and resampling your audio. Note that enabling this setting will slightly reduce the duration of your video. Related settings: You must also set Framerate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>When you do frame rate conversion from 23.976 frames per second (fps) to 29.97 fps, and your output scan type is interlaced, you can optionally enable hard telecine (HARD) to create a smoother picture. When you keep the default value, None (NONE), MediaConvert does a standard frame rate conversion to 29.97 without doing anything with the field polarity to create a smoother picture.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
    /// <p>Specify the VC3 class to choose the quality characteristics for this output. VC3 class, together with the settings Framerate (framerateNumerator and framerateDenominator) and Resolution (height and width), determine your output bitrate. For example, say that your video resolution is 1920x1080 and your framerate is 29.97. Then Class 145 (CLASS<em>145) gives you an output with a bitrate of approximately 145 Mbps and Class 220 (CLASS</em>220) gives you and output with a bitrate of approximately 220 Mbps. VC3 class also specifies the color bit depth of your output.</p>
    #[serde(rename = "vc3Class")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vc_3_class: Option<String>,
}

/// <p>Video codec settings, (CodecSettings) under (VideoDescription), contains the group of settings related to video encoding. The settings in this group vary depending on the value that you choose for Video codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * AV1, Av1Settings * AVC<em>INTRA, AvcIntraSettings * FRAME</em>CAPTURE, FrameCaptureSettings * H<em>264, H264Settings * H</em>265, H265Settings * MPEG2, Mpeg2Settings * PRORES, ProresSettings * VC3, Vc3Settings * VP8, Vp8Settings * VP9, Vp9Settings * XAVC, XavcSettings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoCodecSettings {
    /// <p>Required when you set Codec, under VideoDescription&gt;CodecSettings to the value AV1.</p>
    #[serde(rename = "av1Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av_1_settings: Option<Av1Settings>,
    /// <p>Required when you choose AVC-Intra for your output video codec. For more information about the AVC-Intra settings, see the relevant specification. For detailed information about SD and HD in AVC-Intra, see https://ieeexplore.ieee.org/document/7290936. For information about 4K/2K in AVC-Intra, see https://pro-av.panasonic.net/en/avc-ultra/AVC-ULTRAoverview.pdf.</p>
    #[serde(rename = "avcIntraSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avc_intra_settings: Option<AvcIntraSettings>,
    /// <p>Specifies the video codec. This must be equal to one of the enum values defined by the object  VideoCodec.</p>
    #[serde(rename = "codec")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec: Option<String>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value FRAME_CAPTURE.</p>
    #[serde(rename = "frameCaptureSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_capture_settings: Option<FrameCaptureSettings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value H_264.</p>
    #[serde(rename = "h264Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h264_settings: Option<H264Settings>,
    /// <p>Settings for H265 codec</p>
    #[serde(rename = "h265Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h265_settings: Option<H265Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value MPEG2.</p>
    #[serde(rename = "mpeg2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mpeg_2_settings: Option<Mpeg2Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value PRORES.</p>
    #[serde(rename = "proresSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prores_settings: Option<ProresSettings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VC3</p>
    #[serde(rename = "vc3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vc_3_settings: Option<Vc3Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VP8.</p>
    #[serde(rename = "vp8Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_8_settings: Option<Vp8Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VP9.</p>
    #[serde(rename = "vp9Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vp_9_settings: Option<Vp9Settings>,
    /// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value XAVC.</p>
    #[serde(rename = "xavcSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_settings: Option<XavcSettings>,
}

/// <p>Settings related to video encoding of your output. The specific video settings depend on the video codec that you choose. When you work directly in your JSON job specification, include one instance of Video description (VideoDescription) per output.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoDescription {
    /// <p>This setting only applies to H.264, H.265, and MPEG2 outputs. Use Insert AFD signaling (AfdSignaling) to specify whether the service includes AFD values in the output video data and what those values are. * Choose None to remove all AFD values from this output. * Choose Fixed to ignore input AFD values and instead encode the value specified in the job. * Choose Auto to calculate output AFD values based on the input AFD scaler data.</p>
    #[serde(rename = "afdSignaling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afd_signaling: Option<String>,
    /// <p>The anti-alias filter is automatically applied to all outputs. The service no longer accepts the value DISABLED for AntiAlias. If you specify that in your job, the service will ignore the setting.</p>
    #[serde(rename = "antiAlias")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_alias: Option<String>,
    /// <p>Video codec settings, (CodecSettings) under (VideoDescription), contains the group of settings related to video encoding. The settings in this group vary depending on the value that you choose for Video codec (Codec). For each codec enum that you choose, define the corresponding settings object. The following lists the codec enum, settings object pairs. * AV1, Av1Settings * AVC<em>INTRA, AvcIntraSettings * FRAME</em>CAPTURE, FrameCaptureSettings * H<em>264, H264Settings * H</em>265, H265Settings * MPEG2, Mpeg2Settings * PRORES, ProresSettings * VC3, Vc3Settings * VP8, Vp8Settings * VP9, Vp9Settings * XAVC, XavcSettings</p>
    #[serde(rename = "codecSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_settings: Option<VideoCodecSettings>,
    /// <p>Choose Insert (INSERT) for this setting to include color metadata in this output. Choose Ignore (IGNORE) to exclude color metadata from this output. If you don&#39;t specify a value, the service sets this to Insert by default.</p>
    #[serde(rename = "colorMetadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_metadata: Option<String>,
    /// <p>Use Cropping selection (crop) to specify the video area that the service will include in the output video frame.</p>
    #[serde(rename = "crop")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crop: Option<Rectangle>,
    /// <p>Applies only to 29.97 fps outputs. When this feature is enabled, the service will use drop-frame timecode on outputs. If it is not possible to use drop-frame timecode, the system will fall back to non-drop-frame. This setting is enabled by default when Timecode insertion (TimecodeInsertion) is enabled.</p>
    #[serde(rename = "dropFrameTimecode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_frame_timecode: Option<String>,
    /// <p>Applies only if you set AFD Signaling(AfdSignaling) to Fixed (FIXED). Use Fixed (FixedAfd) to specify a four-bit AFD value which the service will write on all  frames of this video output.</p>
    #[serde(rename = "fixedAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_afd: Option<i64>,
    /// <p>Use the Height (Height) setting to define the video resolution height for this output. Specify in pixels. If you don&#39;t provide a value here, the service will use the input height.</p>
    #[serde(rename = "height")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// <p>Use Selection placement (position) to define the video area in your output frame. The area outside of the rectangle that you specify here is black.</p>
    #[serde(rename = "position")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Rectangle>,
    /// <p>Use Respond to AFD (RespondToAfd) to specify how the service changes the video itself in response to AFD values in the input. * Choose Respond to clip the input video frame according to the AFD value, input display aspect ratio, and output display aspect ratio. * Choose Passthrough to include the input AFD values. Do not choose this when AfdSignaling is set to (NONE). A preferred implementation of this workflow is to set RespondToAfd to (NONE) and set AfdSignaling to (AUTO). * Choose None to remove all input AFD values from this output.</p>
    #[serde(rename = "respondToAfd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub respond_to_afd: Option<String>,
    /// <p>Specify how the service handles outputs that have a different aspect ratio from the input aspect ratio. Choose Stretch to output (STRETCH<em>TO</em>OUTPUT) to have the service stretch your video image to fit. Keep the setting Default (DEFAULT) to have the service letterbox your video instead. This setting overrides any value that you specify for the setting Selection placement (position) in this output.</p>
    #[serde(rename = "scalingBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_behavior: Option<String>,
    /// <p>Use Sharpness (Sharpness) setting to specify the strength of anti-aliasing. This setting changes the width of the anti-alias filter kernel used for scaling. Sharpness only applies if your output resolution is different from your input resolution. 0 is the softest setting, 100 the sharpest, and 50 recommended for most content.</p>
    #[serde(rename = "sharpness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sharpness: Option<i64>,
    /// <p>Applies only to H.264, H.265, MPEG2, and ProRes outputs. Only enable Timecode insertion when the input frame rate is identical to the output frame rate. To include timecodes in this output, set Timecode insertion (VideoTimecodeInsertion) to PIC<em>TIMING</em>SEI. To leave them out, set it to DISABLED. Default is DISABLED. When the service inserts timecodes in an output, by default, it uses any embedded timecodes from the input. If none are present, the service will set the timecode for the first output frame to zero. To change this default behavior, adjust the settings under Timecode configuration (TimecodeConfig). In the console, these settings are located under Job &gt; Job settings &gt; Timecode configuration. Note - Timecode source under input settings (InputTimecodeSource) does not affect the timecodes that are inserted in the output. Source under Job settings &gt; Timecode configuration (TimecodeSource) does.</p>
    #[serde(rename = "timecodeInsertion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_insertion: Option<String>,
    /// <p>Find additional transcoding features under Preprocessors (VideoPreprocessors). Enable the features at each output individually. These features are disabled by default.</p>
    #[serde(rename = "videoPreprocessors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_preprocessors: Option<VideoPreprocessor>,
    /// <p>Use Width (Width) to define the video resolution width, in pixels, for this output. If you don&#39;t provide a value here, the service will use the input width.</p>
    #[serde(rename = "width")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

/// <p>Contains details about the output&#39;s video stream</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VideoDetail {
    /// <p>Height in pixels for the output</p>
    #[serde(rename = "heightInPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height_in_px: Option<i64>,
    /// <p>Width in pixels for the output</p>
    #[serde(rename = "widthInPx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_in_px: Option<i64>,
}

/// <p>Find additional transcoding features under Preprocessors (VideoPreprocessors). Enable the features at each output individually. These features are disabled by default.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoPreprocessor {
    /// <p>Use these settings to convert the color space or to modify properties such as hue and contrast for this output. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/converting-the-color-space.html.</p>
    #[serde(rename = "colorCorrector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_corrector: Option<ColorCorrector>,
    /// <p>Use the deinterlacer to produce smoother motion and a clearer picture. For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-scan-type.html.</p>
    #[serde(rename = "deinterlacer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deinterlacer: Option<Deinterlacer>,
    /// <p>Enable Dolby Vision feature to produce Dolby Vision compatible video output.</p>
    #[serde(rename = "dolbyVision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dolby_vision: Option<DolbyVision>,
    /// <p>Enable HDR10+ analyis and metadata injection. Compatible with HEVC only.</p>
    #[serde(rename = "hdr10Plus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_plus: Option<Hdr10Plus>,
    /// <p>Enable the Image inserter (ImageInserter) feature to include a graphic overlay on your video. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
    #[serde(rename = "imageInserter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_inserter: Option<ImageInserter>,
    /// <p>Enable the Noise reducer (NoiseReducer) feature to remove noise from your video output if necessary. Enable or disable this feature for each output individually. This setting is disabled by default.</p>
    #[serde(rename = "noiseReducer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub noise_reducer: Option<NoiseReducer>,
    /// <p>If you work with a third party video watermarking partner, use the group of settings that correspond with your watermarking partner to include watermarks in your output.</p>
    #[serde(rename = "partnerWatermarking")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_watermarking: Option<PartnerWatermarking>,
    /// <p>Settings for burning the output timecode and specified prefix into the output.</p>
    #[serde(rename = "timecodeBurnin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timecode_burnin: Option<TimecodeBurnin>,
}

/// <p>Input video selectors contain the video settings for the input. Each of your inputs can have up to one video selector.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VideoSelector {
    /// <p>Ignore this setting unless this input is a QuickTime animation with an alpha channel. Use this setting to create separate Key and Fill outputs. In each output, specify which part of the input MediaConvert uses. Leave this setting at the default value DISCARD to delete the alpha channel and preserve the video. Set it to REMAP<em>TO</em>LUMA to delete the video and map the alpha channel to the luma channel of your outputs.</p>
    #[serde(rename = "alphaBehavior")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_behavior: Option<String>,
    /// <p>If your input video has accurate color space metadata, or if you don&#39;t know about color space, leave this set to the default value Follow (FOLLOW). The service will automatically detect your input color space. If your input video has metadata indicating the wrong color space, specify the accurate color space here. If your input video is HDR 10 and the SMPTE ST 2086 Mastering Display Color Volume static metadata isn&#39;t present in your video stream, or if that metadata is present but not accurate, choose Force HDR 10 (FORCE_HDR10) here and specify correct values in the input HDR 10 metadata (Hdr10Metadata) settings. For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
    #[serde(rename = "colorSpace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space: Option<String>,
    /// <p>There are two sources for color metadata, the input file and the job input settings Color space (ColorSpace) and HDR master display information settings(Hdr10Metadata). The Color space usage setting determines which takes precedence. Choose Force (FORCE) to use color metadata from the input job settings. If you don&#39;t specify values for those settings, the service defaults to using metadata from your input. FALLBACK - Choose Fallback (FALLBACK) to use color metadata from the source when it is present. If there&#39;s no color metadata in your input file, the service defaults to using values you specify in the input settings.</p>
    #[serde(rename = "colorSpaceUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_space_usage: Option<String>,
    /// <p>Use these settings to provide HDR 10 metadata that is missing or inaccurate in your input video. Appropriate values vary depending on the input video and must be provided by a color grader. The color grader generates these values during the HDR 10 mastering process. The valid range for each of these settings is 0 to 50,000. Each increment represents 0.00002 in CIE1931 color coordinate. Related settings - When you specify these values, you must also set Color space (ColorSpace) to HDR 10 (HDR10). To specify whether the the values you specify here take precedence over the values in the metadata of your input file, set Color space usage (ColorSpaceUsage). To specify whether color metadata is included in an output, set Color metadata (ColorMetadata). For more information about MediaConvert HDR jobs, see https://docs.aws.amazon.com/console/mediaconvert/hdr.</p>
    #[serde(rename = "hdr10Metadata")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hdr_10_metadata: Option<Hdr10Metadata>,
    /// <p>Use PID (Pid) to select specific video data from an input file. Specify this value as an integer; the system automatically converts it to the hexidecimal value. For example, 257 selects PID 0x101. A PID, or packet identifier, is an identifier for a set of data in an MPEG-2 transport stream container.</p>
    #[serde(rename = "pid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid: Option<i64>,
    /// <p>Selects a specific program from within a multi-program transport stream. Note that Quad 4K is not currently supported.</p>
    #[serde(rename = "programNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub program_number: Option<i64>,
    /// <p>Use Rotate (InputRotate) to specify how the service rotates your video. You can choose automatic rotation or specify a rotation. You can specify a clockwise rotation of 0, 90, 180, or 270 degrees. If your input video container is .mov or .mp4 and your input has rotation metadata, you can choose Automatic to have the service rotate your video according to the rotation specified in the metadata. The rotation must be within one degree of 90, 180, or 270 degrees. If the rotation metadata specifies any other rotation, the service will default to no rotation. By default, the service does no rotation, even if your input video has rotation metadata. The service doesn&#39;t pass through rotation metadata.</p>
    #[serde(rename = "rotate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate: Option<String>,
    /// <p>Use this setting when your input video codec is AVC-Intra. Ignore this setting for all other inputs. If the sample range metadata in your input video is accurate, or if you don&#39;t know about sample range, keep the default value, Follow (FOLLOW), for this setting. When you do, the service automatically detects your input sample range. If your input video has metadata indicating the wrong sample range, specify the accurate sample range here. When you do, MediaConvert ignores any sample range information in the input metadata. Regardless of whether MediaConvert uses the input sample range or the sample range that you specify, MediaConvert uses the sample range for transcoding and also writes it to the output metadata.</p>
    #[serde(rename = "sampleRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_range: Option<String>,
}

/// <p>Required when you set Codec, under AudioDescriptions&gt;CodecSettings, to the value Vorbis.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct VorbisSettings {
    /// <p>Optional. Specify the number of channels in this output audio track. Choosing Mono on the console gives you 1 output channel; choosing Stereo gives you 2. In the API, valid values are 1 and 2. The default value is 2.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>Optional. Specify the audio sample rate in Hz. Valid values are 22050, 32000, 44100, and 48000. The default value is 48000.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
    /// <p>Optional. Specify the variable audio quality of this Vorbis output from -1 (lowest quality, ~45 kbit/s) to 10 (highest quality, ~500 kbit/s). The default value is 4 (~128 kbit/s). Values 5 and 6 are approximately 160 and 192 kbit/s, respectively.</p>
    #[serde(rename = "vbrQuality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vbr_quality: Option<i64>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VP8.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vp8Settings {
    /// <p>Target bitrate in bits/second. For example, enter five megabits per second as 5000000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames. Must be greater than zero.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Optional. Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Ignore this setting unless you set qualityTuningLevel to MULTI_PASS. Optional. Specify the maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. The default behavior uses twice the target bitrate as the maximum bitrate.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio (PAR) for this output. The default behavior, Follow source (INITIALIZE<em>FROM</em>SOURCE), uses the PAR from your input video for your output. To specify a different PAR in the console, choose any value other than Follow source. To specify a different PAR by editing the JSON job specification, choose SPECIFIED. When you choose SPECIFIED for this setting, you must also specify values for the parNumerator and parDenominator settings.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, multi-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>With the VP8 codec, you can use only the variable bitrate (VBR) rate control mode.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value VP9.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Vp9Settings {
    /// <p>Target bitrate in bits/second. For example, enter five megabits per second as 5000000.</p>
    #[serde(rename = "bitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<i64>,
    /// <p>If you are using the console, use the Framerate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list or choose Custom. The framerates shown in the dropdown list are decimal approximations of fractions. If you choose Custom, specify your frame rate as a fraction. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example,  24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>GOP Length (keyframe interval) in frames. Must be greater than zero.</p>
    #[serde(rename = "gopSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_size: Option<f64>,
    /// <p>Size of buffer (HRD buffer model) in bits. For example, enter five megabits as 5000000.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Ignore this setting unless you set qualityTuningLevel to MULTI_PASS. Optional. Specify the maximum bitrate in bits/second. For example, enter five megabits per second as 5000000. The default behavior uses twice the target bitrate as the maximum bitrate.</p>
    #[serde(rename = "maxBitrate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bitrate: Option<i64>,
    /// <p>Optional. Specify how the service determines the pixel aspect ratio for this output. The default behavior is to use the same pixel aspect ratio as your input video.</p>
    #[serde(rename = "parControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_control: Option<String>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parDenominator is 33.</p>
    #[serde(rename = "parDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_denominator: Option<i64>,
    /// <p>Required when you set Pixel aspect ratio (parControl) to SPECIFIED. On the console, this corresponds to any value other than Follow source. When you specify an output pixel aspect ratio (PAR) that is different from your input video PAR, provide your output PAR as a ratio. For example, for D1/DV NTSC widescreen, you would specify the ratio 40:33. In this example, the value for parNumerator is 40.</p>
    #[serde(rename = "parNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub par_numerator: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, multi-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>With the VP9 codec, you can use only the variable bitrate (VBR) rate control mode.</p>
    #[serde(rename = "rateControlMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_control_mode: Option<String>,
}

/// <p>Required when you set (Codec) under (AudioDescriptions)&gt;(CodecSettings) to the value WAV.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WavSettings {
    /// <p>Specify Bit depth (BitDepth), in bits per sample, to choose the encoding quality for this audio track.</p>
    #[serde(rename = "bitDepth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i64>,
    /// <p>Specify the number of channels in this output audio track. Valid values are 1 and even numbers up to 64. For example, 1, 2, 4, 6, and so on, up to 64.</p>
    #[serde(rename = "channels")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channels: Option<i64>,
    /// <p>The service defaults to using RIFF for WAV outputs. If your output audio is likely to exceed 4 GB in file size, or if you otherwise need the extended support of the RF64 format, set your output WAV file format to RF64.</p>
    #[serde(rename = "format")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// <p>Sample rate in Hz.</p>
    #[serde(rename = "sampleRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_rate: Option<i64>,
}

/// <p>WEBVTT Destination Settings</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebvttDestinationSettings {
    /// <p>Choose Enabled (ENABLED) to have MediaConvert use the font style, color, and position information from the captions source in the input. Keep the default value, Disabled (DISABLED), for simplified output captions.</p>
    #[serde(rename = "stylePassthrough")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_passthrough: Option<String>,
}

/// <p>Settings specific to WebVTT sources in HLS alternative rendition group. Specify the properties (renditionGroupId, renditionName or renditionLanguageCode) to identify the unique subtitle track among the alternative rendition groups present in the HLS manifest. If no unique track is found, or multiple tracks match the specified properties, the job fails. If there is only one subtitle track in the rendition group, the settings can be left empty and the default subtitle track will be chosen. If your caption source is a sidecar file, use FileSourceSettings instead of WebvttHlsSourceSettings.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct WebvttHlsSourceSettings {
    /// <p>Optional. Specify alternative group ID</p>
    #[serde(rename = "renditionGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_group_id: Option<String>,
    /// <p>Optional. Specify ISO 639-2 or ISO 639-3 code in the language property</p>
    #[serde(rename = "renditionLanguageCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_language_code: Option<String>,
    /// <p>Optional. Specify media name</p>
    #[serde(rename = "renditionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendition_name: Option<String>,
}

/// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>4K</em>INTRA_CBG.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Xavc4kIntraCbgProfileSettings {
    /// <p>Specify the XAVC Intra 4k (CBG) Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.</p>
    #[serde(rename = "xavcClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_class: Option<String>,
}

/// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>4K</em>INTRA_VBR.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Xavc4kIntraVbrProfileSettings {
    /// <p>Specify the XAVC Intra 4k (VBR) Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.</p>
    #[serde(rename = "xavcClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_class: Option<String>,
}

/// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC_4K.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Xavc4kProfileSettings {
    /// <p>Specify the XAVC 4k (Long GOP) Bitrate Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.</p>
    #[serde(rename = "bitrateClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate_class: Option<String>,
    /// <p>Specify the codec profile for this output. Choose High, 8-bit, 4:2:0 (HIGH) or High, 10-bit, 4:2:2 (HIGH_422). These profiles are specified in ITU-T H.264.</p>
    #[serde(rename = "codecProfile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codec_profile: Option<String>,
    /// <p>The best way to set up adaptive quantization is to keep the default value, Auto (AUTO), for the setting Adaptive quantization (XavcAdaptiveQuantization). When you do so, MediaConvert automatically applies the best types of quantization for your video content. Include this setting in your JSON job specification only when you choose to change the default value for Adaptive quantization. Enable this setting to have the encoder reduce I-frame pop. I-frame pop appears as a visual flicker that can arise when the encoder saves bits by copying some macroblocks many times from frame to frame, and then refreshes them at the I-frame. When you enable this setting, the encoder updates these macroblocks slightly more often to smooth out the flicker. This setting is disabled by default. Related setting: In addition to enabling this setting, you must also set Adaptive quantization (adaptiveQuantization) to a value other than Off (OFF) or Auto (AUTO). Use Adaptive quantization to adjust the degree of smoothing that Flicker adaptive quantization provides.</p>
    #[serde(rename = "flickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>Specify whether the encoder uses B-frames as reference frames for other pictures in the same GOP. Choose Allow (ENABLED) to allow the encoder to use B-frames as reference frames. Choose Don&#39;t allow (DISABLED) to prevent the encoder from using B-frames as reference frames.</p>
    #[serde(rename = "gopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>Specify the size of the buffer that MediaConvert uses in the HRD buffer model for this output. Specify this value in bits; for example, enter five megabits as 5000000. When you don&#39;t set this value, or you set it to zero, MediaConvert calculates the default by doubling the bitrate of this output point.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, single-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
}

/// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>HD</em>INTRA_CBG.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct XavcHdIntraCbgProfileSettings {
    /// <p>Specify the XAVC Intra HD (CBG) Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.</p>
    #[serde(rename = "xavcClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_class: Option<String>,
}

/// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC_HD.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct XavcHdProfileSettings {
    /// <p>Specify the XAVC HD (Long GOP) Bitrate Class to set the bitrate of your output. Outputs of the same class have similar image quality over the operating points that are valid for that class.</p>
    #[serde(rename = "bitrateClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate_class: Option<String>,
    /// <p>The best way to set up adaptive quantization is to keep the default value, Auto (AUTO), for the setting Adaptive quantization (XavcAdaptiveQuantization). When you do so, MediaConvert automatically applies the best types of quantization for your video content. Include this setting in your JSON job specification only when you choose to change the default value for Adaptive quantization. Enable this setting to have the encoder reduce I-frame pop. I-frame pop appears as a visual flicker that can arise when the encoder saves bits by copying some macroblocks many times from frame to frame, and then refreshes them at the I-frame. When you enable this setting, the encoder updates these macroblocks slightly more often to smooth out the flicker. This setting is disabled by default. Related setting: In addition to enabling this setting, you must also set Adaptive quantization (adaptiveQuantization) to a value other than Off (OFF) or Auto (AUTO). Use Adaptive quantization to adjust the degree of smoothing that Flicker adaptive quantization provides.</p>
    #[serde(rename = "flickerAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flicker_adaptive_quantization: Option<String>,
    /// <p>Specify whether the encoder uses B-frames as reference frames for other pictures in the same GOP. Choose Allow (ENABLED) to allow the encoder to use B-frames as reference frames. Choose Don&#39;t allow (DISABLED) to prevent the encoder from using B-frames as reference frames.</p>
    #[serde(rename = "gopBReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_b_reference: Option<String>,
    /// <p>Frequency of closed GOPs. In streaming applications, it is recommended that this be set to 1 so a decoder joining mid-stream will receive an IDR frame as quickly as possible. Setting this value to 0 will break output segmenting.</p>
    #[serde(rename = "gopClosedCadence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gop_closed_cadence: Option<i64>,
    /// <p>Specify the size of the buffer that MediaConvert uses in the HRD buffer model for this output. Specify this value in bits; for example, enter five megabits as 5000000. When you don&#39;t set this value, or you set it to zero, MediaConvert calculates the default by doubling the bitrate of this output point.</p>
    #[serde(rename = "hrdBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hrd_buffer_size: Option<i64>,
    /// <p>Choose the scan line type for the output. Keep the default value, Progressive (PROGRESSIVE) to create a progressive output, regardless of the scan type of your input. Use Top field first (TOP<em>FIELD) or Bottom field first (BOTTOM</em>FIELD) to create an output that&#39;s interlaced with the same field polarity throughout. Use Follow, default top (FOLLOW<em>TOP</em>FIELD) or Follow, default bottom (FOLLOW<em>BOTTOM</em>FIELD) to produce outputs with the same field polarity as the source. For jobs that have multiple inputs, the output field polarity might change over the course of the output. Follow behavior depends on the input scan type. If the source is interlaced, the output will be interlaced with the same polarity as the source. If the source is progressive, the output will be interlaced with top field bottom field first, depending on which of the Follow options you choose.</p>
    #[serde(rename = "interlaceMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interlace_mode: Option<String>,
    /// <p>Optional. Use Quality tuning level (qualityTuningLevel) to choose how you want to trade off encoding speed for output video quality. The default behavior is faster, lower quality, single-pass encoding.</p>
    #[serde(rename = "qualityTuningLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_tuning_level: Option<String>,
    /// <p>Number of slices per picture. Must be less than or equal to the number of macroblock rows for progressive pictures, and less than or equal to half the number of macroblock rows for interlaced pictures.</p>
    #[serde(rename = "slices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slices: Option<i64>,
    /// <p>Ignore this setting unless you set Frame rate (framerateNumerator divided by framerateDenominator) to 29.970. If your input framerate is 23.976, choose Hard (HARD). Otherwise, keep the default value None (NONE). For more information, see https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-telecine-and-inverse-telecine.html.</p>
    #[serde(rename = "telecine")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telecine: Option<String>,
}

/// <p>Required when you set (Codec) under (VideoDescription)&gt;(CodecSettings) to the value XAVC.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct XavcSettings {
    /// <p>Keep the default value, Auto (AUTO), for this setting to have MediaConvert automatically apply the best types of quantization for your video content. When you want to apply your quantization settings manually, you must set Adaptive quantization (adaptiveQuantization) to a value other than Auto (AUTO). Use this setting to specify the strength of any adaptive quantization filters that you enable. If you don&#39;t want MediaConvert to do any adaptive quantization in this transcode, set Adaptive quantization to Off (OFF). Related settings: The value that you choose here applies to the following settings: Flicker adaptive quantization (flickerAdaptiveQuantization), Spatial adaptive quantization (spatialAdaptiveQuantization), and Temporal adaptive quantization (temporalAdaptiveQuantization).</p>
    #[serde(rename = "adaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_quantization: Option<String>,
    /// <p>Optional. Choose a specific entropy encoding mode only when you want to override XAVC recommendations. If you choose the value auto, MediaConvert uses the mode that the XAVC file format specifies given this output&#39;s operating point.</p>
    #[serde(rename = "entropyEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entropy_encoding: Option<String>,
    /// <p>If you are using the console, use the Frame rate setting to specify the frame rate for this output. If you want to keep the same frame rate as the input video, choose Follow source. If you want to do frame rate conversion, choose a frame rate from the dropdown list. The framerates shown in the dropdown list are decimal approximations of fractions. If you are creating your transcoding job specification as a JSON file without the console, use FramerateControl to specify which value the service uses for the frame rate for this output. Choose INITIALIZE<em>FROM</em>SOURCE if you want the service to use the frame rate from the input. Choose SPECIFIED if you want the service to use the frame rate that you specify in the settings FramerateNumerator and FramerateDenominator.</p>
    #[serde(rename = "framerateControl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_control: Option<String>,
    /// <p>Choose the method that you want MediaConvert to use when increasing or decreasing the frame rate. We recommend using drop duplicate (DUPLICATE_DROP) for numerically simple conversions, such as 60 fps to 30 fps. For numerically complex conversions, you can use interpolate (INTERPOLATE) to avoid stutter. This results in a smooth picture, but might introduce undesirable video artifacts. For complex frame rate conversions, especially if your source video has already been converted from its original cadence, use FrameFormer (FRAMEFORMER) to do motion-compensated interpolation. FrameFormer chooses the best conversion method frame by frame. Note that using FrameFormer increases the transcoding time and incurs a significant add-on cost.</p>
    #[serde(rename = "framerateConversionAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_conversion_algorithm: Option<String>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example, 24000 / 1001 = 23.976 fps. Use FramerateDenominator to specify the denominator of this fraction. In this example, use 1001 for the value of FramerateDenominator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Frame rate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateDenominator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_denominator: Option<i64>,
    /// <p>When you use the API for transcode jobs that use frame rate conversion, specify the frame rate as a fraction. For example, 24000 / 1001 = 23.976 fps. Use FramerateNumerator to specify the numerator of this fraction. In this example, use 24000 for the value of FramerateNumerator. When you use the console for transcode jobs that use frame rate conversion, provide the value as a decimal number for Framerate. In this example, specify 23.976.</p>
    #[serde(rename = "framerateNumerator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framerate_numerator: Option<i64>,
    /// <p>Specify the XAVC profile for this output. For more information, see the Sony documentation at https://www.xavc-info.org/. Note that MediaConvert doesn&#39;t support the interlaced video XAVC operating points for XAVC<em>HD</em>INTRA<em>CBG. To create an interlaced XAVC output, choose the profile XAVC</em>HD.</p>
    #[serde(rename = "profile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
    /// <p>Ignore this setting unless your input frame rate is 23.976 or 24 frames per second (fps). Enable slow PAL to create a 25 fps output by relabeling the video frames and resampling your audio. Note that enabling this setting will slightly reduce the duration of your video. Related settings: You must also set Frame rate to 25. In your JSON job specification, set (framerateControl) to (SPECIFIED), (framerateNumerator) to 25 and (framerateDenominator) to 1.</p>
    #[serde(rename = "slowPal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_pal: Option<String>,
    /// <p>Ignore this setting unless your downstream workflow requires that you specify it explicitly. Otherwise, we recommend that you adjust the softness of your output by using a lower value for the setting Sharpness (sharpness) or by enabling a noise reducer filter (noiseReducerFilter). The Softness (softness) setting specifies the quantization matrices that the encoder uses. Keep the default value, 0, for flat quantization. Choose the value 1 or 16 to use the default JVT softening quantization matricies from the H.264 specification. Choose a value from 17 to 128 to use planar interpolation. Increasing values from 17 to 128 result in increasing reduction of high-frequency data. The value 128 results in the softest video.</p>
    #[serde(rename = "softness")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub softness: Option<i64>,
    /// <p>The best way to set up adaptive quantization is to keep the default value, Auto (AUTO), for the setting Adaptive quantization (adaptiveQuantization). When you do so, MediaConvert automatically applies the best types of quantization for your video content. Include this setting in your JSON job specification only when you choose to change the default value for Adaptive quantization. For this setting, keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on spatial variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas that can sustain more distortion with no noticeable visual degradation and uses more bits on areas where any small distortion will be noticeable. For example, complex textured blocks are encoded with fewer bits and smooth textured blocks are encoded with more bits. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen with a lot of complex texture, you might choose to disable this feature. Related setting: When you enable spatial adaptive quantization, set the value for Adaptive quantization (adaptiveQuantization) depending on your content. For homogeneous content, such as cartoons and video games, set it to Low. For content with a wider variety of textures, set it to High or Higher.</p>
    #[serde(rename = "spatialAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spatial_adaptive_quantization: Option<String>,
    /// <p>The best way to set up adaptive quantization is to keep the default value, Auto (AUTO), for the setting Adaptive quantization (adaptiveQuantization). When you do so, MediaConvert automatically applies the best types of quantization for your video content. Include this setting in your JSON job specification only when you choose to change the default value for Adaptive quantization. For this setting, keep the default value, Enabled (ENABLED), to adjust quantization within each frame based on temporal variation of content complexity. When you enable this feature, the encoder uses fewer bits on areas of the frame that aren&#39;t moving and uses more bits on complex objects with sharp edges that move a lot. For example, this feature improves the readability of text tickers on newscasts and scoreboards on sports matches. Enabling this feature will almost always improve your video quality. Note, though, that this feature doesn&#39;t take into account where the viewer&#39;s attention is likely to be. If viewers are likely to be focusing their attention on a part of the screen that doesn&#39;t have moving objects with sharp edges, such as sports athletes&#39; faces, you might choose to disable this feature. Related setting: When you enable temporal adaptive quantization, adjust the strength of the filter with the setting Adaptive quantization (adaptiveQuantization).</p>
    #[serde(rename = "temporalAdaptiveQuantization")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temporal_adaptive_quantization: Option<String>,
    /// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>4K</em>INTRA_CBG.</p>
    #[serde(rename = "xavc4kIntraCbgProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_4k_intra_cbg_profile_settings: Option<Xavc4kIntraCbgProfileSettings>,
    /// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>4K</em>INTRA_VBR.</p>
    #[serde(rename = "xavc4kIntraVbrProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_4k_intra_vbr_profile_settings: Option<Xavc4kIntraVbrProfileSettings>,
    /// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC_4K.</p>
    #[serde(rename = "xavc4kProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_4k_profile_settings: Option<Xavc4kProfileSettings>,
    /// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC<em>HD</em>INTRA_CBG.</p>
    #[serde(rename = "xavcHdIntraCbgProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_hd_intra_cbg_profile_settings: Option<XavcHdIntraCbgProfileSettings>,
    /// <p>Required when you set (Profile) under (VideoDescription)&gt;(CodecSettings)&gt;(XavcSettings) to the value XAVC_HD.</p>
    #[serde(rename = "xavcHdProfileSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xavc_hd_profile_settings: Option<XavcHdProfileSettings>,
}

/// Errors returned by AssociateCertificate
#[derive(Debug, PartialEq)]
pub enum AssociateCertificateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl AssociateCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AssociateCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(AssociateCertificateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(AssociateCertificateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(AssociateCertificateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(AssociateCertificateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(AssociateCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AssociateCertificateError::TooManyRequests(
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
impl fmt::Display for AssociateCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateCertificateError::BadRequest(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::Conflict(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::Forbidden(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            AssociateCertificateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AssociateCertificateError {}
/// Errors returned by CancelJob
#[derive(Debug, PartialEq)]
pub enum CancelJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CancelJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CancelJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CancelJobError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CancelJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CancelJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CancelJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CancelJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CancelJobError::Conflict(ref cause) => write!(f, "{}", cause),
            CancelJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CancelJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CancelJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CancelJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobError {}
/// Errors returned by CreateJob
#[derive(Debug, PartialEq)]
pub enum CreateJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateJobError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateJobError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateJobError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateJobError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateJobError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobError {}
/// Errors returned by CreateJobTemplate
#[derive(Debug, PartialEq)]
pub enum CreateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateJobTemplateError {}
/// Errors returned by CreatePreset
#[derive(Debug, PartialEq)]
pub enum CreatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreatePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreatePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreatePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreatePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreatePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreatePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreatePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreatePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreatePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreatePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            CreatePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreatePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreatePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            CreatePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreatePresetError {}
/// Errors returned by CreateQueue
#[derive(Debug, PartialEq)]
pub enum CreateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl CreateQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(CreateQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(CreateQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(CreateQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(CreateQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(CreateQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            CreateQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            CreateQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            CreateQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            CreateQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            CreateQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateQueueError {}
/// Errors returned by DeleteJobTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeleteJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteJobTemplateError {}
/// Errors returned by DeletePreset
#[derive(Debug, PartialEq)]
pub enum DeletePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeletePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeletePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeletePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeletePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeletePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeletePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeletePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeletePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeletePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeletePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            DeletePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeletePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeletePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            DeletePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeletePresetError {}
/// Errors returned by DeleteQueue
#[derive(Debug, PartialEq)]
pub enum DeleteQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DeleteQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DeleteQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DeleteQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DeleteQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DeleteQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DeleteQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            DeleteQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteQueueError {}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DescribeEndpointsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DescribeEndpointsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DescribeEndpointsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DescribeEndpointsError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DescribeEndpointsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeEndpointsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointsError::BadRequest(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Conflict(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::Forbidden(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::NotFound(ref cause) => write!(f, "{}", cause),
            DescribeEndpointsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointsError {}
/// Errors returned by DisassociateCertificate
#[derive(Debug, PartialEq)]
pub enum DisassociateCertificateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl DisassociateCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DisassociateCertificateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(DisassociateCertificateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(DisassociateCertificateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(DisassociateCertificateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(DisassociateCertificateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(DisassociateCertificateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DisassociateCertificateError::TooManyRequests(
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
impl fmt::Display for DisassociateCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateCertificateError::BadRequest(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::Conflict(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::Forbidden(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::NotFound(ref cause) => write!(f, "{}", cause),
            DisassociateCertificateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DisassociateCertificateError {}
/// Errors returned by GetJob
#[derive(Debug, PartialEq)]
pub enum GetJobError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetJobError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJobError::BadRequest(err.msg))
                }
                "ConflictException" => return RusotoError::Service(GetJobError::Conflict(err.msg)),
                "ForbiddenException" => {
                    return RusotoError::Service(GetJobError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJobError::InternalServerError(err.msg))
                }
                "NotFoundException" => return RusotoError::Service(GetJobError::NotFound(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJobError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJobError::Conflict(ref cause) => write!(f, "{}", cause),
            GetJobError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJobError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJobError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobError {}
/// Errors returned by GetJobTemplate
#[derive(Debug, PartialEq)]
pub enum GetJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetJobTemplateError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            GetJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetJobTemplateError {}
/// Errors returned by GetPreset
#[derive(Debug, PartialEq)]
pub enum GetPresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetPresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetPresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetPresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetPresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetPresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetPresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetPresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetPresetError::Conflict(ref cause) => write!(f, "{}", cause),
            GetPresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetPresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetPresetError::NotFound(ref cause) => write!(f, "{}", cause),
            GetPresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPresetError {}
/// Errors returned by GetQueue
#[derive(Debug, PartialEq)]
pub enum GetQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl GetQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(GetQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(GetQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(GetQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(GetQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(GetQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            GetQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            GetQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            GetQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            GetQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            GetQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetQueueError {}
/// Errors returned by ListJobTemplates
#[derive(Debug, PartialEq)]
pub enum ListJobTemplatesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListJobTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobTemplatesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJobTemplatesError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListJobTemplatesError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListJobTemplatesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListJobTemplatesError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJobTemplatesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListJobTemplatesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobTemplatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobTemplatesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::Conflict(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJobTemplatesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobTemplatesError {}
/// Errors returned by ListJobs
#[derive(Debug, PartialEq)]
pub enum ListJobsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListJobsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListJobsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListJobsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListJobsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListJobsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListJobsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListJobsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListJobsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListJobsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListJobsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListJobsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobsError {}
/// Errors returned by ListPresets
#[derive(Debug, PartialEq)]
pub enum ListPresetsError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListPresetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPresetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListPresetsError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListPresetsError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListPresetsError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListPresetsError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListPresetsError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListPresetsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListPresetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListPresetsError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListPresetsError::Conflict(ref cause) => write!(f, "{}", cause),
            ListPresetsError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListPresetsError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListPresetsError::NotFound(ref cause) => write!(f, "{}", cause),
            ListPresetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListPresetsError {}
/// Errors returned by ListQueues
#[derive(Debug, PartialEq)]
pub enum ListQueuesError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListQueuesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListQueuesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListQueuesError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListQueuesError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(ListQueuesError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(ListQueuesError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(ListQueuesError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListQueuesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListQueuesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListQueuesError::BadRequest(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Conflict(ref cause) => write!(f, "{}", cause),
            ListQueuesError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListQueuesError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListQueuesError::NotFound(ref cause) => write!(f, "{}", cause),
            ListQueuesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListQueuesError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(ListTagsForResourceError::Conflict(err.msg))
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
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsForResourceError::TooManyRequests(err.msg))
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
            ListTagsForResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(TagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(TagResourceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(TagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(TagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(TagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            TagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            TagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            TagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            TagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UntagResourceError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UntagResourceError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UntagResourceError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UntagResourceError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UntagResourceError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::BadRequest(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Conflict(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Forbidden(ref cause) => write!(f, "{}", cause),
            UntagResourceError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UntagResourceError::NotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateJobTemplate
#[derive(Debug, PartialEq)]
pub enum UpdateJobTemplateError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdateJobTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateJobTemplateError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateJobTemplateError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateJobTemplateError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateJobTemplateError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateJobTemplateError::InternalServerError(
                        err.msg,
                    ))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateJobTemplateError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateJobTemplateError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateJobTemplateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateJobTemplateError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateJobTemplateError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateJobTemplateError {}
/// Errors returned by UpdatePreset
#[derive(Debug, PartialEq)]
pub enum UpdatePresetError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdatePresetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePresetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdatePresetError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdatePresetError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdatePresetError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdatePresetError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdatePresetError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdatePresetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdatePresetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdatePresetError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdatePresetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdatePresetError {}
/// Errors returned by UpdateQueue
#[derive(Debug, PartialEq)]
pub enum UpdateQueueError {
    /// <p>The service can&#39;t process your request because of a problem in the request. Please check your request form and syntax.</p>
    BadRequest(String),
    /// <p>The service couldn&#39;t complete your request because there is a conflict with the current state of the resource.</p>
    Conflict(String),
    /// <p>You don&#39;t have permissions for this action with the credentials you sent.</p>
    Forbidden(String),
    /// <p>The service encountered an unexpected condition and can&#39;t fulfill your request.</p>
    InternalServerError(String),
    /// <p>The resource you requested doesn&#39;t exist.</p>
    NotFound(String),
    /// <p>Too many requests have been sent in too short of a time. The service limits the rate at which it will accept requests.</p>
    TooManyRequests(String),
}

impl UpdateQueueError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateQueueError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "BadRequestException" => {
                    return RusotoError::Service(UpdateQueueError::BadRequest(err.msg))
                }
                "ConflictException" => {
                    return RusotoError::Service(UpdateQueueError::Conflict(err.msg))
                }
                "ForbiddenException" => {
                    return RusotoError::Service(UpdateQueueError::Forbidden(err.msg))
                }
                "InternalServerErrorException" => {
                    return RusotoError::Service(UpdateQueueError::InternalServerError(err.msg))
                }
                "NotFoundException" => {
                    return RusotoError::Service(UpdateQueueError::NotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateQueueError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateQueueError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateQueueError::BadRequest(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::Conflict(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::Forbidden(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::InternalServerError(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::NotFound(ref cause) => write!(f, "{}", cause),
            UpdateQueueError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateQueueError {}
/// Trait representing the capabilities of the MediaConvert API. MediaConvert clients implement this trait.
#[async_trait]
pub trait MediaConvert {
    /// <p>Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.</p>
    async fn associate_certificate(
        &self,
        input: AssociateCertificateRequest,
    ) -> Result<AssociateCertificateResponse, RusotoError<AssociateCertificateError>>;

    /// <p>Permanently cancel a job. Once you have canceled a job, you can&#39;t start it again.</p>
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>>;

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>>;

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> Result<CreateJobTemplateResponse, RusotoError<CreateJobTemplateError>>;

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    async fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> Result<CreatePresetResponse, RusotoError<CreatePresetError>>;

    /// <p>Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html</p>
    async fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> Result<CreateQueueResponse, RusotoError<CreateQueueError>>;

    /// <p>Permanently delete a job template you have created.</p>
    async fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> Result<DeleteJobTemplateResponse, RusotoError<DeleteJobTemplateError>>;

    /// <p>Permanently delete a preset you have created.</p>
    async fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> Result<DeletePresetResponse, RusotoError<DeletePresetError>>;

    /// <p>Permanently delete a queue you have created.</p>
    async fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> Result<DeleteQueueResponse, RusotoError<DeleteQueueError>>;

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>>;

    /// <p>Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource.</p>
    async fn disassociate_certificate(
        &self,
        input: DisassociateCertificateRequest,
    ) -> Result<DisassociateCertificateResponse, RusotoError<DisassociateCertificateError>>;

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>>;

    /// <p>Retrieve the JSON for a specific job template.</p>
    async fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> Result<GetJobTemplateResponse, RusotoError<GetJobTemplateError>>;

    /// <p>Retrieve the JSON for a specific preset.</p>
    async fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> Result<GetPresetResponse, RusotoError<GetPresetError>>;

    /// <p>Retrieve the JSON for a specific queue.</p>
    async fn get_queue(
        &self,
        input: GetQueueRequest,
    ) -> Result<GetQueueResponse, RusotoError<GetQueueError>>;

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    async fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> Result<ListJobTemplatesResponse, RusotoError<ListJobTemplatesError>>;

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>>;

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    async fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> Result<ListPresetsResponse, RusotoError<ListPresetsError>>;

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>>;

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Modify one of your existing job templates.</p>
    async fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> Result<UpdateJobTemplateResponse, RusotoError<UpdateJobTemplateError>>;

    /// <p>Modify one of your existing presets.</p>
    async fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> Result<UpdatePresetResponse, RusotoError<UpdatePresetError>>;

    /// <p>Modify one of your existing queues.</p>
    async fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> Result<UpdateQueueResponse, RusotoError<UpdateQueueError>>;
}
/// A client for the MediaConvert API.
#[derive(Clone)]
pub struct MediaConvertClient {
    client: Client,
    region: region::Region,
}

impl MediaConvertClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> MediaConvertClient {
        MediaConvertClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> MediaConvertClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        MediaConvertClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> MediaConvertClient {
        MediaConvertClient { client, region }
    }
}

#[async_trait]
impl MediaConvert for MediaConvertClient {
    /// <p>Associates an AWS Certificate Manager (ACM) Amazon Resource Name (ARN) with AWS Elemental MediaConvert.</p>
    #[allow(unused_mut)]
    async fn associate_certificate(
        &self,
        input: AssociateCertificateRequest,
    ) -> Result<AssociateCertificateResponse, RusotoError<AssociateCertificateError>> {
        let request_uri = "/2017-08-29/certificates";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<AssociateCertificateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateCertificateError::from_response(response))
        }
    }

    /// <p>Permanently cancel a job. Once you have canceled a job, you can&#39;t start it again.</p>
    #[allow(unused_mut)]
    async fn cancel_job(
        &self,
        input: CancelJobRequest,
    ) -> Result<CancelJobResponse, RusotoError<CancelJobError>> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobError::from_response(response))
        }
    }

    /// <p>Create a new transcoding job. For information about jobs and job settings, see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[allow(unused_mut)]
    async fn create_job(
        &self,
        input: CreateJobRequest,
    ) -> Result<CreateJobResponse, RusotoError<CreateJobError>> {
        let request_uri = "/2017-08-29/jobs";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<CreateJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobError::from_response(response))
        }
    }

    /// <p>Create a new job template. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[allow(unused_mut)]
    async fn create_job_template(
        &self,
        input: CreateJobTemplateRequest,
    ) -> Result<CreateJobTemplateResponse, RusotoError<CreateJobTemplateError>> {
        let request_uri = "/2017-08-29/jobTemplates";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<CreateJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateJobTemplateError::from_response(response))
        }
    }

    /// <p>Create a new preset. For information about job templates see the User Guide at http://docs.aws.amazon.com/mediaconvert/latest/ug/what-is.html</p>
    #[allow(unused_mut)]
    async fn create_preset(
        &self,
        input: CreatePresetRequest,
    ) -> Result<CreatePresetResponse, RusotoError<CreatePresetError>> {
        let request_uri = "/2017-08-29/presets";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<CreatePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreatePresetError::from_response(response))
        }
    }

    /// <p>Create a new transcoding queue. For information about queues, see Working With Queues in the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/working-with-queues.html</p>
    #[allow(unused_mut)]
    async fn create_queue(
        &self,
        input: CreateQueueRequest,
    ) -> Result<CreateQueueResponse, RusotoError<CreateQueueError>> {
        let request_uri = "/2017-08-29/queues";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<CreateQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateQueueError::from_response(response))
        }
    }

    /// <p>Permanently delete a job template you have created.</p>
    #[allow(unused_mut)]
    async fn delete_job_template(
        &self,
        input: DeleteJobTemplateRequest,
    ) -> Result<DeleteJobTemplateResponse, RusotoError<DeleteJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteJobTemplateError::from_response(response))
        }
    }

    /// <p>Permanently delete a preset you have created.</p>
    #[allow(unused_mut)]
    async fn delete_preset(
        &self,
        input: DeletePresetRequest,
    ) -> Result<DeletePresetResponse, RusotoError<DeletePresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeletePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeletePresetError::from_response(response))
        }
    }

    /// <p>Permanently delete a queue you have created.</p>
    #[allow(unused_mut)]
    async fn delete_queue(
        &self,
        input: DeleteQueueRequest,
    ) -> Result<DeleteQueueResponse, RusotoError<DeleteQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteQueueError::from_response(response))
        }
    }

    /// <p>Send an request with an empty body to the regional API endpoint to get your account API endpoint.</p>
    #[allow(unused_mut)]
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsRequest,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
        let request_uri = "/2017-08-29/endpoints";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<DescribeEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeEndpointsError::from_response(response))
        }
    }

    /// <p>Removes an association between the Amazon Resource Name (ARN) of an AWS Certificate Manager (ACM) certificate and an AWS Elemental MediaConvert resource.</p>
    #[allow(unused_mut)]
    async fn disassociate_certificate(
        &self,
        input: DisassociateCertificateRequest,
    ) -> Result<DisassociateCertificateResponse, RusotoError<DisassociateCertificateError>> {
        let request_uri = format!("/2017-08-29/certificates/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("DELETE", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateCertificateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateCertificateError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific completed transcoding job.</p>
    #[allow(unused_mut)]
    async fn get_job(
        &self,
        input: GetJobRequest,
    ) -> Result<GetJobResponse, RusotoError<GetJobError>> {
        let request_uri = format!("/2017-08-29/jobs/{id}", id = input.id);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<GetJobResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific job template.</p>
    #[allow(unused_mut)]
    async fn get_job_template(
        &self,
        input: GetJobTemplateRequest,
    ) -> Result<GetJobTemplateResponse, RusotoError<GetJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetJobTemplateError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific preset.</p>
    #[allow(unused_mut)]
    async fn get_preset(
        &self,
        input: GetPresetRequest,
    ) -> Result<GetPresetResponse, RusotoError<GetPresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPresetError::from_response(response))
        }
    }

    /// <p>Retrieve the JSON for a specific queue.</p>
    #[allow(unused_mut)]
    async fn get_queue(
        &self,
        input: GetQueueRequest,
    ) -> Result<GetQueueResponse, RusotoError<GetQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetQueueError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your job templates. This will return the templates themselves, not just a list of them. To retrieve the next twenty templates, use the nextToken string returned with the array</p>
    #[allow(unused_mut)]
    async fn list_job_templates(
        &self,
        input: ListJobTemplatesRequest,
    ) -> Result<ListJobTemplatesResponse, RusotoError<ListJobTemplatesError>> {
        let request_uri = "/2017-08-29/jobTemplates";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.category {
            params.put("category", x);
        }
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
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
                .deserialize::<ListJobTemplatesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobTemplatesError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your most recently created jobs. This array includes in-process, completed, and errored jobs. This will return the jobs themselves, not just a list of the jobs. To retrieve the twenty next most recent jobs, use the nextToken string returned with the array.</p>
    #[allow(unused_mut)]
    async fn list_jobs(
        &self,
        input: ListJobsRequest,
    ) -> Result<ListJobsResponse, RusotoError<ListJobsError>> {
        let request_uri = "/2017-08-29/jobs";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
        }
        if let Some(ref x) = input.queue {
            params.put("queue", x);
        }
        if let Some(ref x) = input.status {
            params.put("status", x);
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
                .deserialize::<ListJobsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobsError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your presets. This will return the presets themselves, not just a list of them. To retrieve the next twenty presets, use the nextToken string returned with the array.</p>
    #[allow(unused_mut)]
    async fn list_presets(
        &self,
        input: ListPresetsRequest,
    ) -> Result<ListPresetsResponse, RusotoError<ListPresetsError>> {
        let request_uri = "/2017-08-29/presets";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.category {
            params.put("category", x);
        }
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
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
                .deserialize::<ListPresetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListPresetsError::from_response(response))
        }
    }

    /// <p>Retrieve a JSON array of up to twenty of your queues. This will return the queues themselves, not just a list of them. To retrieve the next twenty queues, use the nextToken string returned with the array.</p>
    #[allow(unused_mut)]
    async fn list_queues(
        &self,
        input: ListQueuesRequest,
    ) -> Result<ListQueuesResponse, RusotoError<ListQueuesError>> {
        let request_uri = "/2017-08-29/queues";

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.list_by {
            params.put("listBy", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.order {
            params.put("order", x);
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
                .deserialize::<ListQueuesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListQueuesError::from_response(response))
        }
    }

    /// <p>Retrieve the tags for a MediaConvert resource.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/2017-08-29/tags/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("GET", "mediaconvert", &self.region, &request_uri);
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

    /// <p>Add tags to a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = "/2017-08-29/tags";

        let mut request = SignedRequest::new("POST", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Remove tags from a MediaConvert queue, preset, or job template. For information about tagging, see the User Guide at https://docs.aws.amazon.com/mediaconvert/latest/ug/tagging-resources.html</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/2017-08-29/tags/{arn}", arn = input.arn);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Modify one of your existing job templates.</p>
    #[allow(unused_mut)]
    async fn update_job_template(
        &self,
        input: UpdateJobTemplateRequest,
    ) -> Result<UpdateJobTemplateResponse, RusotoError<UpdateJobTemplateError>> {
        let request_uri = format!("/2017-08-29/jobTemplates/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<UpdateJobTemplateResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateJobTemplateError::from_response(response))
        }
    }

    /// <p>Modify one of your existing presets.</p>
    #[allow(unused_mut)]
    async fn update_preset(
        &self,
        input: UpdatePresetRequest,
    ) -> Result<UpdatePresetResponse, RusotoError<UpdatePresetError>> {
        let request_uri = format!("/2017-08-29/presets/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<UpdatePresetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdatePresetError::from_response(response))
        }
    }

    /// <p>Modify one of your existing queues.</p>
    #[allow(unused_mut)]
    async fn update_queue(
        &self,
        input: UpdateQueueRequest,
    ) -> Result<UpdateQueueResponse, RusotoError<UpdateQueueError>> {
        let request_uri = format!("/2017-08-29/queues/{name}", name = input.name);

        let mut request = SignedRequest::new("PUT", "mediaconvert", &self.region, &request_uri);
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
                .deserialize::<UpdateQueueResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateQueueError::from_response(response))
        }
    }
}
