use serde::Deserialize;
use crate::utils::transform_str_to_enum;
use super::Entity;

pub use crate::Url;

/// Represents a file or media attachment that can be added to a status.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct Attachment {
    // Required attributes
    id: String,
    #[serde(deserialize_with = "transform_str_to_enum")]
    r#type: AttachmentType,
    url: crate::Url,
    preview_url: crate::Url,

    // Optional attributes
    remote_url: Option<crate::Url>,
    text_url: Option<crate::Url>,
    meta: Option<AttachmentMeta>,
    description: Option<String>,
    blurhash: Option<String>,
}

impl Attachment {
    /// Get the ID of this attachment in the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Get the ID of this attachment in the database.
    pub fn id_to_owned(&self) -> String {
        self.id.clone()
    }

    /// Get the type of this attachment as `AttachmentType`.
    pub fn r#type(&self) -> AttachmentType {
        self.r#type
    }

    /// Get the location of the original full-size attachment.
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// Get the location of a scaled-down preview of this attachment.
    pub fn preview_url(&self) -> &Url {
        &self.preview_url
    }

    /// Get the location of the full-size original attachment on the remote website.
    pub fn remote_url(&self) -> &Option<Url> {
        &self.remote_url
    }

    /// Get a shorter URL for this attachment.
    pub fn text_url(&self) -> &Option<Url> {
        &self.text_url
    }

    /// Get a metadata returned by Paperclip.
    pub fn meta(&self) -> &Option<AttachmentMeta> {
        &self.meta
    }

    /// Get an alternate text that describes what is in the media attachment, to be used for the visually impaired or when media attachments do not load.
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    /// Get a hash computed by the BlurHash algorithm, for generating colorful preview thumbnails when media has not been downloaded yet.
    pub fn blurhash(&self) -> &Option<String> {
        &self.blurhash
    }
}

impl Entity for Attachment {}

/// Metadata returned by Paperclip.
#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct AttachmentMeta {
    length: Option<String>,
    duration: Option<f64>,
    fps: Option<u64>,
    size: Option<String>,
    width: Option<u64>,
    height: Option<u64>,
    aspect: Option<f64>,
    audio_encode: Option<String>,
    audio_bitrate: Option<String>,
    audio_channels: Option<String>,
    original: Option<AttachmentMetaSub>,
    small: Option<AttachmentMetaSub>,
    focus: Option<Focus>,
}

impl AttachmentMeta {
    /// For example: "0:01:28.65"
    pub fn length(&self) -> &Option<String> {
        &self.length
    }

    /// For example: 88.65
    pub fn duration(&self) -> &Option<f64> {
        &self.duration
    }

    /// For example: 24
    pub fn fps(&self) -> &Option<u64> {
        &self.fps
    }

    /// For example: "1280x720"
    pub fn size(&self) -> &Option<String> {
        &self.size
    }

    /// For example: 1280
    pub fn width(&self) -> &Option<u64> {
        &self.width
    }

    /// For example: 720
    pub fn height(&self) -> &Option<u64> {
        &self.height
    }

    /// For example: 1.7777777777777777
    pub fn aspect(&self) -> &Option<f64> {
        &self.aspect
    }

    /// For example: "aac (LC) (mp4a / 0x6134706D)"
    pub fn audio_encode(&self) -> &Option<String> {
        &self.audio_encode
    }

    /// For example: "44100 Hz"
    pub fn audio_bitrate(&self) -> &Option<String> {
        &self.audio_bitrate
    }

    /// For example: "stereo"
    pub fn audio_channels(&self) -> &Option<String> {
        &self.audio_channels
    }

    /// Get the `AttachmentMetaSub`.
    pub fn original(&self) -> &Option<AttachmentMetaSub> {
        &self.original
    }

    /// Get the `AttachmentMetaSub`.
    pub fn small(&self) -> &Option<AttachmentMetaSub> {
        &self.small
    }

    /// Get the focal points as `Focus`.
    pub fn focus(&self) -> &Option<Focus> {
        &self.focus
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Deserialize)]
pub struct AttachmentMetaSub {
    // for Image, Gifv, Video
    width: Option<u64>,
    height: Option<u64>,
    size: Option<String>,
    aspect: Option<f64>,

    // for Audio, Gifv, Video
    duration: Option<f64>,
    bitrate: Option<u64>,

    // for Gifv, Video
    frame_rate: Option<String>,
}

impl AttachmentMetaSub {
    /// For example: 640
    pub fn width(&self) -> &Option<u64> {
        &self.width
    }

    /// For example: 480
    pub fn height(&self) -> &Option<u64> {
        &self.height
    }

    /// For example: "640x480"
    pub fn size(&self) -> &Option<String> {
        &self.size
    }

    /// For example: 1.3333333333333333
    pub fn aspect(&self) -> &Option<f64> {
        &self.aspect
    }

    /// For example: 88.654
    pub fn duration(&self) -> &Option<f64> {
        &self.duration
    }

    /// For example: 862056
    pub fn bitrate(&self) -> &Option<u64> {
        &self.bitrate
    }

    /// For example: "6159375/249269"
    pub fn frame_rate(&self) -> &Option<String> {
        &self.frame_rate
    }
}

/// In summary, floating points range from -1.0 to 1.0, left-to-right or bottom-to-top. (0,0) is the center of the image. (0.5, 0.5) would be in the center of the upper-right quadrant. (-0.5, -0.5) would be in the center of the lower-left quadrant. For reference, thumbnails in the Mastodon frontend are most commonly 16:9.
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Deserialize)]
pub struct Focus {
    x: f64,
    y: f64,
}

impl Focus {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

/// The type of the attachment.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy, Deserialize)]
pub enum AttachmentType {
    /// Static image.
    Image,

    /// Looping, soundless animation.
    Gifv,

    /// Video clip.
    Video,

    /// Audio track.
    Audio,

    /// Unsupported or unrecongnized file type.
    Unknown,
}

use std::{ fmt, str::FromStr };

impl fmt::Display for AttachmentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttachmentType::Image => write!(f, "image"),
            AttachmentType::Gifv => write!(f, "gifv"),
            AttachmentType::Video => write!(f, "video"),
            AttachmentType::Audio => write!(f, "audio"),
            AttachmentType::Unknown => write!(f, "unknown"),
        }
    }
}

impl FromStr for AttachmentType {
    type Err = crate::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "image" => Ok(AttachmentType::Image),
            "gifv" => Ok(AttachmentType::Gifv),
            "video" => Ok(AttachmentType::Video),
            "audio" => Ok(AttachmentType::Audio),
            "unknown" => Ok(AttachmentType::Unknown),
            _ => Err(crate::Error::ParseAttachmentTypeError(s.to_owned())),
        }
    }
}
