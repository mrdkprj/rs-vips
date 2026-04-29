// (c) Copyright 2019-2025 OLX
// (c) Copyright 2025 mrdkprj

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Access {
    ///  `Random` -> VIPS_ACCESS_RANDOM = 0
    Random = 0,
    ///  `Sequential` -> VIPS_ACCESS_SEQUENTIAL = 1
    Sequential = 1,
    ///  `SequentialUnbuffered` -> VIPS_ACCESS_SEQUENTIAL_UNBUFFERED = 2
    SequentialUnbuffered = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Align {
    ///  `Low` -> VIPS_ALIGN_LOW = 0
    Low = 0,
    ///  `Centre` -> VIPS_ALIGN_CENTRE = 1
    Centre = 1,
    ///  `High` -> VIPS_ALIGN_HIGH = 2
    High = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Angle {
    ///  `D0` -> VIPS_ANGLE_D0 = 0
    D0 = 0,
    ///  `D90` -> VIPS_ANGLE_D90 = 1
    D90 = 1,
    ///  `D180` -> VIPS_ANGLE_D180 = 2
    D180 = 2,
    ///  `D270` -> VIPS_ANGLE_D270 = 3
    D270 = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Angle45 {
    ///  `D0` -> VIPS_ANGLE45_D0 = 0
    D0 = 0,
    ///  `D45` -> VIPS_ANGLE45_D45 = 1
    D45 = 1,
    ///  `D90` -> VIPS_ANGLE45_D90 = 2
    D90 = 2,
    ///  `D135` -> VIPS_ANGLE45_D135 = 3
    D135 = 3,
    ///  `D180` -> VIPS_ANGLE45_D180 = 4
    D180 = 4,
    ///  `D225` -> VIPS_ANGLE45_D225 = 5
    D225 = 5,
    ///  `D270` -> VIPS_ANGLE45_D270 = 6
    D270 = 6,
    ///  `D315` -> VIPS_ANGLE45_D315 = 7
    D315 = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum BandFormat {
    ///  `Notset` -> VIPS_FORMAT_NOTSET = -1
    Notset = -1,
    ///  `Uchar` -> VIPS_FORMAT_UCHAR = 0
    Uchar = 0,
    ///  `Char` -> VIPS_FORMAT_CHAR = 1
    Char = 1,
    ///  `Ushort` -> VIPS_FORMAT_USHORT = 2
    Ushort = 2,
    ///  `Short` -> VIPS_FORMAT_SHORT = 3
    Short = 3,
    ///  `Uint` -> VIPS_FORMAT_UINT = 4
    Uint = 4,
    ///  `Int` -> VIPS_FORMAT_INT = 5
    Int = 5,
    ///  `Float` -> VIPS_FORMAT_FLOAT = 6
    Float = 6,
    ///  `Complex` -> VIPS_FORMAT_COMPLEX = 7
    Complex = 7,
    ///  `Double` -> VIPS_FORMAT_DOUBLE = 8
    Double = 8,
    ///  `Dpcomplex` -> VIPS_FORMAT_DPCOMPLEX = 9
    Dpcomplex = 9,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum BlendMode {
    ///  `Clear` -> VIPS_BLEND_MODE_CLEAR = 0
    Clear = 0,
    ///  `Source` -> VIPS_BLEND_MODE_SOURCE = 1
    Source = 1,
    ///  `Over` -> VIPS_BLEND_MODE_OVER = 2
    Over = 2,
    ///  `In` -> VIPS_BLEND_MODE_IN = 3
    In = 3,
    ///  `Out` -> VIPS_BLEND_MODE_OUT = 4
    Out = 4,
    ///  `Atop` -> VIPS_BLEND_MODE_ATOP = 5
    Atop = 5,
    ///  `Dest` -> VIPS_BLEND_MODE_DEST = 6
    Dest = 6,
    ///  `DestOver` -> VIPS_BLEND_MODE_DEST_OVER = 7
    DestOver = 7,
    ///  `DestIn` -> VIPS_BLEND_MODE_DEST_IN = 8
    DestIn = 8,
    ///  `DestOut` -> VIPS_BLEND_MODE_DEST_OUT = 9
    DestOut = 9,
    ///  `DestAtop` -> VIPS_BLEND_MODE_DEST_ATOP = 10
    DestAtop = 10,
    ///  `Xor` -> VIPS_BLEND_MODE_XOR = 11
    Xor = 11,
    ///  `Add` -> VIPS_BLEND_MODE_ADD = 12
    Add = 12,
    ///  `Saturate` -> VIPS_BLEND_MODE_SATURATE = 13
    Saturate = 13,
    ///  `Multiply` -> VIPS_BLEND_MODE_MULTIPLY = 14
    Multiply = 14,
    ///  `Screen` -> VIPS_BLEND_MODE_SCREEN = 15
    Screen = 15,
    ///  `Overlay` -> VIPS_BLEND_MODE_OVERLAY = 16
    Overlay = 16,
    ///  `Darken` -> VIPS_BLEND_MODE_DARKEN = 17
    Darken = 17,
    ///  `Lighten` -> VIPS_BLEND_MODE_LIGHTEN = 18
    Lighten = 18,
    ///  `ColourDodge` -> VIPS_BLEND_MODE_COLOUR_DODGE = 19
    ColourDodge = 19,
    ///  `ColourBurn` -> VIPS_BLEND_MODE_COLOUR_BURN = 20
    ColourBurn = 20,
    ///  `HardLight` -> VIPS_BLEND_MODE_HARD_LIGHT = 21
    HardLight = 21,
    ///  `SoftLight` -> VIPS_BLEND_MODE_SOFT_LIGHT = 22
    SoftLight = 22,
    ///  `Difference` -> VIPS_BLEND_MODE_DIFFERENCE = 23
    Difference = 23,
    ///  `Exclusion` -> VIPS_BLEND_MODE_EXCLUSION = 24
    Exclusion = 24,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Coding {
    ///  `Error` -> VIPS_CODING_ERROR = -1
    Error = -1,
    ///  `None` -> VIPS_CODING_NONE = 0
    None = 0,
    ///  `Labq` -> VIPS_CODING_LABQ = 2
    Labq = 2,
    ///  `Rad` -> VIPS_CODING_RAD = 6
    Rad = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Combine {
    ///  `Max` -> VIPS_COMBINE_MAX = 0
    Max = 0,
    ///  `Sum` -> VIPS_COMBINE_SUM = 1
    Sum = 1,
    ///  `Min` -> VIPS_COMBINE_MIN = 2
    Min = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum CombineMode {
    ///  `Set` -> VIPS_COMBINE_MODE_SET = 0
    Set = 0,
    ///  `Add` -> VIPS_COMBINE_MODE_ADD = 1
    Add = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum CompassDirection {
    ///  `Centre` -> VIPS_COMPASS_DIRECTION_CENTRE = 0
    Centre = 0,
    ///  `North` -> VIPS_COMPASS_DIRECTION_NORTH = 1
    North = 1,
    ///  `East` -> VIPS_COMPASS_DIRECTION_EAST = 2
    East = 2,
    ///  `South` -> VIPS_COMPASS_DIRECTION_SOUTH = 3
    South = 3,
    ///  `West` -> VIPS_COMPASS_DIRECTION_WEST = 4
    West = 4,
    ///  `NorthEast` -> VIPS_COMPASS_DIRECTION_NORTH_EAST = 5
    NorthEast = 5,
    ///  `SouthEast` -> VIPS_COMPASS_DIRECTION_SOUTH_EAST = 6
    SouthEast = 6,
    ///  `SouthWest` -> VIPS_COMPASS_DIRECTION_SOUTH_WEST = 7
    SouthWest = 7,
    ///  `NorthWest` -> VIPS_COMPASS_DIRECTION_NORTH_WEST = 8
    NorthWest = 8,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Direction {
    ///  `Horizontal` -> VIPS_DIRECTION_HORIZONTAL = 0
    Horizontal = 0,
    ///  `Vertical` -> VIPS_DIRECTION_VERTICAL = 1
    Vertical = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Extend {
    ///  `Black` -> VIPS_EXTEND_BLACK = 0
    Black = 0,
    ///  `Copy` -> VIPS_EXTEND_COPY = 1
    Copy = 1,
    ///  `Repeat` -> VIPS_EXTEND_REPEAT = 2
    Repeat = 2,
    ///  `Mirror` -> VIPS_EXTEND_MIRROR = 3
    Mirror = 3,
    ///  `White` -> VIPS_EXTEND_WHITE = 4
    White = 4,
    ///  `Background` -> VIPS_EXTEND_BACKGROUND = 5
    Background = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum FailOn {
    ///  `None` -> VIPS_FAIL_ON_NONE = 0
    None = 0,
    ///  `Truncated` -> VIPS_FAIL_ON_TRUNCATED = 1
    Truncated = 1,
    ///  `Error` -> VIPS_FAIL_ON_ERROR = 2
    Error = 2,
    ///  `Warning` -> VIPS_FAIL_ON_WARNING = 3
    Warning = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignDzContainer {
    ///  `Fs` -> VIPS_FOREIGN_DZ_CONTAINER_FS = 0
    Fs = 0,
    ///  `Zip` -> VIPS_FOREIGN_DZ_CONTAINER_ZIP = 1
    Zip = 1,
    ///  `Szi` -> VIPS_FOREIGN_DZ_CONTAINER_SZI = 2
    Szi = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignDzDepth {
    ///  `Onepixel` -> VIPS_FOREIGN_DZ_DEPTH_ONEPIXEL = 0
    Onepixel = 0,
    ///  `Onetile` -> VIPS_FOREIGN_DZ_DEPTH_ONETILE = 1
    Onetile = 1,
    ///  `One` -> VIPS_FOREIGN_DZ_DEPTH_ONE = 2
    One = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignDzLayout {
    ///  `Dz` -> VIPS_FOREIGN_DZ_LAYOUT_DZ = 0
    Dz = 0,
    ///  `Zoomify` -> VIPS_FOREIGN_DZ_LAYOUT_ZOOMIFY = 1
    Zoomify = 1,
    ///  `Google` -> VIPS_FOREIGN_DZ_LAYOUT_GOOGLE = 2
    Google = 2,
    ///  `Iiif` -> VIPS_FOREIGN_DZ_LAYOUT_IIIF = 3
    Iiif = 3,
    ///  `Iiif3` -> VIPS_FOREIGN_DZ_LAYOUT_IIIF3 = 4
    Iiif3 = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignFlags {
    ///  `None` -> VIPS_FOREIGN_NONE = 0
    None = 0,
    ///  `Partial` -> VIPS_FOREIGN_PARTIAL = 1
    Partial = 1,
    ///  `Bigendian` -> VIPS_FOREIGN_BIGENDIAN = 2
    Bigendian = 2,
    ///  `Sequential` -> VIPS_FOREIGN_SEQUENTIAL = 4
    Sequential = 4,
    ///  `All` -> VIPS_FOREIGN_ALL = 7
    All = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignHeifCompression {
    ///  `Hevc` -> VIPS_FOREIGN_HEIF_COMPRESSION_HEVC = 1
    Hevc = 1,
    ///  `Avc` -> VIPS_FOREIGN_HEIF_COMPRESSION_AVC = 2
    Avc = 2,
    ///  `Jpeg` -> VIPS_FOREIGN_HEIF_COMPRESSION_JPEG = 3
    Jpeg = 3,
    ///  `Av1` -> VIPS_FOREIGN_HEIF_COMPRESSION_AV1 = 4
    Av1 = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignHeifEncoder {
    ///  `Auto` -> VIPS_FOREIGN_HEIF_ENCODER_AUTO = 0
    Auto = 0,
    ///  `Aom` -> VIPS_FOREIGN_HEIF_ENCODER_AOM = 1
    Aom = 1,
    ///  `Rav1E` -> VIPS_FOREIGN_HEIF_ENCODER_RAV1E = 2
    Rav1E = 2,
    ///  `Svt` -> VIPS_FOREIGN_HEIF_ENCODER_SVT = 3
    Svt = 3,
    ///  `X265` -> VIPS_FOREIGN_HEIF_ENCODER_X265 = 4
    X265 = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignKeep {
    ///  `None` -> VIPS_FOREIGN_KEEP_NONE = 0
    None = 0,
    ///  `Exif` -> VIPS_FOREIGN_KEEP_EXIF = 1
    Exif = 1,
    ///  `Xmp` -> VIPS_FOREIGN_KEEP_XMP = 2
    Xmp = 2,
    ///  `Iptc` -> VIPS_FOREIGN_KEEP_IPTC = 4
    Iptc = 4,
    ///  `Icc` -> VIPS_FOREIGN_KEEP_ICC = 8
    Icc = 8,
    ///  `Other` -> VIPS_FOREIGN_KEEP_OTHER = 16
    Other = 16,
    ///  `Gainmap` -> VIPS_FOREIGN_KEEP_GAINMAP = 32
    Gainmap = 32,
    ///  `All` -> VIPS_FOREIGN_KEEP_ALL = 63
    All = 63,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignPdfPageBox {
    ///  `Medum` -> VIPS_FOREIGN_PDF_PAGE_BOX_MEDIA = 0
    Medum = 0,
    ///  `Crop` -> VIPS_FOREIGN_PDF_PAGE_BOX_CROP = 1
    Crop = 1,
    ///  `Trim` -> VIPS_FOREIGN_PDF_PAGE_BOX_TRIM = 2
    Trim = 2,
    ///  `Bleed` -> VIPS_FOREIGN_PDF_PAGE_BOX_BLEED = 3
    Bleed = 3,
    ///  `Art` -> VIPS_FOREIGN_PDF_PAGE_BOX_ART = 4
    Art = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignPngFilter {
    ///  `None` -> VIPS_FOREIGN_PNG_FILTER_NONE = 8
    None = 8,
    ///  `Sub` -> VIPS_FOREIGN_PNG_FILTER_SUB = 16
    Sub = 16,
    ///  `Up` -> VIPS_FOREIGN_PNG_FILTER_UP = 32
    Up = 32,
    ///  `Avg` -> VIPS_FOREIGN_PNG_FILTER_AVG = 64
    Avg = 64,
    ///  `Paeth` -> VIPS_FOREIGN_PNG_FILTER_PAETH = 128
    Paeth = 128,
    ///  `All` -> VIPS_FOREIGN_PNG_FILTER_ALL = 248
    All = 248,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignPpmFormat {
    ///  `Pbm` -> VIPS_FOREIGN_PPM_FORMAT_PBM = 0
    Pbm = 0,
    ///  `Pgm` -> VIPS_FOREIGN_PPM_FORMAT_PGM = 1
    Pgm = 1,
    ///  `Ppm` -> VIPS_FOREIGN_PPM_FORMAT_PPM = 2
    Ppm = 2,
    ///  `Pfm` -> VIPS_FOREIGN_PPM_FORMAT_PFM = 3
    Pfm = 3,
    ///  `Pnm` -> VIPS_FOREIGN_PPM_FORMAT_PNM = 4
    Pnm = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignSubsample {
    ///  `Auto` -> VIPS_FOREIGN_SUBSAMPLE_AUTO = 0
    Auto = 0,
    ///  `On` -> VIPS_FOREIGN_SUBSAMPLE_ON = 1
    On = 1,
    ///  `Off` -> VIPS_FOREIGN_SUBSAMPLE_OFF = 2
    Off = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffCompression {
    ///  `None` -> VIPS_FOREIGN_TIFF_COMPRESSION_NONE = 0
    None = 0,
    ///  `Jpeg` -> VIPS_FOREIGN_TIFF_COMPRESSION_JPEG = 1
    Jpeg = 1,
    ///  `Deflate` -> VIPS_FOREIGN_TIFF_COMPRESSION_DEFLATE = 2
    Deflate = 2,
    ///  `Packbits` -> VIPS_FOREIGN_TIFF_COMPRESSION_PACKBITS = 3
    Packbits = 3,
    ///  `Ccittfax4` -> VIPS_FOREIGN_TIFF_COMPRESSION_CCITTFAX4 = 4
    Ccittfax4 = 4,
    ///  `Lzw` -> VIPS_FOREIGN_TIFF_COMPRESSION_LZW = 5
    Lzw = 5,
    ///  `Webp` -> VIPS_FOREIGN_TIFF_COMPRESSION_WEBP = 6
    Webp = 6,
    ///  `Zstd` -> VIPS_FOREIGN_TIFF_COMPRESSION_ZSTD = 7
    Zstd = 7,
    ///  `Jp2K` -> VIPS_FOREIGN_TIFF_COMPRESSION_JP2K = 8
    Jp2K = 8,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffPredictor {
    ///  `None` -> VIPS_FOREIGN_TIFF_PREDICTOR_NONE = 1
    None = 1,
    ///  `Horizontal` -> VIPS_FOREIGN_TIFF_PREDICTOR_HORIZONTAL = 2
    Horizontal = 2,
    ///  `Float` -> VIPS_FOREIGN_TIFF_PREDICTOR_FLOAT = 3
    Float = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignTiffResunit {
    ///  `Cm` -> VIPS_FOREIGN_TIFF_RESUNIT_CM = 0
    Cm = 0,
    ///  `Inch` -> VIPS_FOREIGN_TIFF_RESUNIT_INCH = 1
    Inch = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum ForeignWebpPreset {
    ///  `Default` -> VIPS_FOREIGN_WEBP_PRESET_DEFAULT = 0
    Default = 0,
    ///  `Picture` -> VIPS_FOREIGN_WEBP_PRESET_PICTURE = 1
    Picture = 1,
    ///  `Photo` -> VIPS_FOREIGN_WEBP_PRESET_PHOTO = 2
    Photo = 2,
    ///  `Drawing` -> VIPS_FOREIGN_WEBP_PRESET_DRAWING = 3
    Drawing = 3,
    ///  `Icon` -> VIPS_FOREIGN_WEBP_PRESET_ICON = 4
    Icon = 4,
    ///  `Text` -> VIPS_FOREIGN_WEBP_PRESET_TEXT = 5
    Text = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Intent {
    ///  `Perceptual` -> VIPS_INTENT_PERCEPTUAL = 0
    Perceptual = 0,
    ///  `Relative` -> VIPS_INTENT_RELATIVE = 1
    Relative = 1,
    ///  `Saturation` -> VIPS_INTENT_SATURATION = 2
    Saturation = 2,
    ///  `Absolute` -> VIPS_INTENT_ABSOLUTE = 3
    Absolute = 3,
    ///  `Auto` -> VIPS_INTENT_AUTO = 32
    Auto = 32,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Interesting {
    ///  `None` -> VIPS_INTERESTING_NONE = 0
    None = 0,
    ///  `Centre` -> VIPS_INTERESTING_CENTRE = 1
    Centre = 1,
    ///  `Entropy` -> VIPS_INTERESTING_ENTROPY = 2
    Entropy = 2,
    ///  `Attention` -> VIPS_INTERESTING_ATTENTION = 3
    Attention = 3,
    ///  `Low` -> VIPS_INTERESTING_LOW = 4
    Low = 4,
    ///  `High` -> VIPS_INTERESTING_HIGH = 5
    High = 5,
    ///  `All` -> VIPS_INTERESTING_ALL = 6
    All = 6,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Interpretation {
    ///  `Error` -> VIPS_INTERPRETATION_ERROR = -1
    Error = -1,
    ///  `Multiband` -> VIPS_INTERPRETATION_MULTIBAND = 0
    Multiband = 0,
    ///  `BW` -> VIPS_INTERPRETATION_B_W = 1
    BW = 1,
    ///  `Histogram` -> VIPS_INTERPRETATION_HISTOGRAM = 10
    Histogram = 10,
    ///  `Xyz` -> VIPS_INTERPRETATION_XYZ = 12
    Xyz = 12,
    ///  `Lab` -> VIPS_INTERPRETATION_LAB = 13
    Lab = 13,
    ///  `Cmyk` -> VIPS_INTERPRETATION_CMYK = 15
    Cmyk = 15,
    ///  `Labq` -> VIPS_INTERPRETATION_LABQ = 16
    Labq = 16,
    ///  `Rgb` -> VIPS_INTERPRETATION_RGB = 17
    Rgb = 17,
    ///  `Cmc` -> VIPS_INTERPRETATION_CMC = 18
    Cmc = 18,
    ///  `Lch` -> VIPS_INTERPRETATION_LCH = 19
    Lch = 19,
    ///  `Labs` -> VIPS_INTERPRETATION_LABS = 21
    Labs = 21,
    ///  `Srgb` -> VIPS_INTERPRETATION_sRGB = 22
    Srgb = 22,
    ///  `Yxy` -> VIPS_INTERPRETATION_YXY = 23
    Yxy = 23,
    ///  `Fourier` -> VIPS_INTERPRETATION_FOURIER = 24
    Fourier = 24,
    ///  `Rgb16` -> VIPS_INTERPRETATION_RGB16 = 25
    Rgb16 = 25,
    ///  `Grey16` -> VIPS_INTERPRETATION_GREY16 = 26
    Grey16 = 26,
    ///  `Matrix` -> VIPS_INTERPRETATION_MATRIX = 27
    Matrix = 27,
    ///  `Scrgb` -> VIPS_INTERPRETATION_scRGB = 28
    Scrgb = 28,
    ///  `Hsv` -> VIPS_INTERPRETATION_HSV = 29
    Hsv = 29,
    ///  `Oklab` -> VIPS_INTERPRETATION_OKLAB = 30
    Oklab = 30,
    ///  `Oklch` -> VIPS_INTERPRETATION_OKLCH = 31
    Oklch = 31,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Kernel {
    ///  `Nearest` -> VIPS_KERNEL_NEAREST = 0
    Nearest = 0,
    ///  `Linear` -> VIPS_KERNEL_LINEAR = 1
    Linear = 1,
    ///  `Cubic` -> VIPS_KERNEL_CUBIC = 2
    Cubic = 2,
    ///  `Mitchell` -> VIPS_KERNEL_MITCHELL = 3
    Mitchell = 3,
    ///  `Lanczos2` -> VIPS_KERNEL_LANCZOS2 = 4
    Lanczos2 = 4,
    ///  `Lanczos3` -> VIPS_KERNEL_LANCZOS3 = 5
    Lanczos3 = 5,
    ///  `Mks2013` -> VIPS_KERNEL_MKS2013 = 6
    Mks2013 = 6,
    ///  `Mks2021` -> VIPS_KERNEL_MKS2021 = 7
    Mks2021 = 7,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationBoolean {
    ///  `And` -> VIPS_OPERATION_BOOLEAN_AND = 0
    And = 0,
    ///  `Or` -> VIPS_OPERATION_BOOLEAN_OR = 1
    Or = 1,
    ///  `Eor` -> VIPS_OPERATION_BOOLEAN_EOR = 2
    Eor = 2,
    ///  `Lshift` -> VIPS_OPERATION_BOOLEAN_LSHIFT = 3
    Lshift = 3,
    ///  `Rshift` -> VIPS_OPERATION_BOOLEAN_RSHIFT = 4
    Rshift = 4,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationComplex {
    ///  `Polar` -> VIPS_OPERATION_COMPLEX_POLAR = 0
    Polar = 0,
    ///  `Rect` -> VIPS_OPERATION_COMPLEX_RECT = 1
    Rect = 1,
    ///  `Conj` -> VIPS_OPERATION_COMPLEX_CONJ = 2
    Conj = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationComplex2 {
    ///  `CrossPhase` -> VIPS_OPERATION_COMPLEX2_CROSS_PHASE = 0
    CrossPhase = 0,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationComplexget {
    ///  `Real` -> VIPS_OPERATION_COMPLEXGET_REAL = 0
    Real = 0,
    ///  `Imag` -> VIPS_OPERATION_COMPLEXGET_IMAG = 1
    Imag = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationMath {
    ///  `Sin` -> VIPS_OPERATION_MATH_SIN = 0
    Sin = 0,
    ///  `Cos` -> VIPS_OPERATION_MATH_COS = 1
    Cos = 1,
    ///  `Tan` -> VIPS_OPERATION_MATH_TAN = 2
    Tan = 2,
    ///  `Asin` -> VIPS_OPERATION_MATH_ASIN = 3
    Asin = 3,
    ///  `Acos` -> VIPS_OPERATION_MATH_ACOS = 4
    Acos = 4,
    ///  `Atan` -> VIPS_OPERATION_MATH_ATAN = 5
    Atan = 5,
    ///  `Log` -> VIPS_OPERATION_MATH_LOG = 6
    Log = 6,
    ///  `Log10` -> VIPS_OPERATION_MATH_LOG10 = 7
    Log10 = 7,
    ///  `Exp` -> VIPS_OPERATION_MATH_EXP = 8
    Exp = 8,
    ///  `Exp10` -> VIPS_OPERATION_MATH_EXP10 = 9
    Exp10 = 9,
    ///  `Sinh` -> VIPS_OPERATION_MATH_SINH = 10
    Sinh = 10,
    ///  `Cosh` -> VIPS_OPERATION_MATH_COSH = 11
    Cosh = 11,
    ///  `Tanh` -> VIPS_OPERATION_MATH_TANH = 12
    Tanh = 12,
    ///  `Asinh` -> VIPS_OPERATION_MATH_ASINH = 13
    Asinh = 13,
    ///  `Acosh` -> VIPS_OPERATION_MATH_ACOSH = 14
    Acosh = 14,
    ///  `Atanh` -> VIPS_OPERATION_MATH_ATANH = 15
    Atanh = 15,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationMath2 {
    ///  `Pow` -> VIPS_OPERATION_MATH2_POW = 0
    Pow = 0,
    ///  `Wop` -> VIPS_OPERATION_MATH2_WOP = 1
    Wop = 1,
    ///  `Atan2` -> VIPS_OPERATION_MATH2_ATAN2 = 2
    Atan2 = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationMorphology {
    ///  `Erode` -> VIPS_OPERATION_MORPHOLOGY_ERODE = 0
    Erode = 0,
    ///  `Dilate` -> VIPS_OPERATION_MORPHOLOGY_DILATE = 1
    Dilate = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationRelational {
    ///  `Equal` -> VIPS_OPERATION_RELATIONAL_EQUAL = 0
    Equal = 0,
    ///  `Noteq` -> VIPS_OPERATION_RELATIONAL_NOTEQ = 1
    Noteq = 1,
    ///  `Less` -> VIPS_OPERATION_RELATIONAL_LESS = 2
    Less = 2,
    ///  `Lesseq` -> VIPS_OPERATION_RELATIONAL_LESSEQ = 3
    Lesseq = 3,
    ///  `More` -> VIPS_OPERATION_RELATIONAL_MORE = 4
    More = 4,
    ///  `Moreeq` -> VIPS_OPERATION_RELATIONAL_MOREEQ = 5
    Moreeq = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum OperationRound {
    ///  `Rint` -> VIPS_OPERATION_ROUND_RINT = 0
    Rint = 0,
    ///  `Ceil` -> VIPS_OPERATION_ROUND_CEIL = 1
    Ceil = 1,
    ///  `Floor` -> VIPS_OPERATION_ROUND_FLOOR = 2
    Floor = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum PCS {
    ///  `Lab` -> VIPS_PCS_LAB = 0
    Lab = 0,
    ///  `Xyz` -> VIPS_PCS_XYZ = 1
    Xyz = 1,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Precision {
    ///  `Integer` -> VIPS_PRECISION_INTEGER = 0
    Integer = 0,
    ///  `Float` -> VIPS_PRECISION_FLOAT = 1
    Float = 1,
    ///  `Approximate` -> VIPS_PRECISION_APPROXIMATE = 2
    Approximate = 2,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum RegionShrink {
    ///  `Mean` -> VIPS_REGION_SHRINK_MEAN = 0
    Mean = 0,
    ///  `Median` -> VIPS_REGION_SHRINK_MEDIAN = 1
    Median = 1,
    ///  `Mode` -> VIPS_REGION_SHRINK_MODE = 2
    Mode = 2,
    ///  `Max` -> VIPS_REGION_SHRINK_MAX = 3
    Max = 3,
    ///  `Min` -> VIPS_REGION_SHRINK_MIN = 4
    Min = 4,
    ///  `Nearest` -> VIPS_REGION_SHRINK_NEAREST = 5
    Nearest = 5,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum SdfShape {
    ///  `Circle` -> VIPS_SDF_SHAPE_CIRCLE = 0
    Circle = 0,
    ///  `Box` -> VIPS_SDF_SHAPE_BOX = 1
    Box = 1,
    ///  `RoundedBox` -> VIPS_SDF_SHAPE_ROUNDED_BOX = 2
    RoundedBox = 2,
    ///  `Line` -> VIPS_SDF_SHAPE_LINE = 3
    Line = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum Size {
    ///  `Both` -> VIPS_SIZE_BOTH = 0
    Both = 0,
    ///  `Up` -> VIPS_SIZE_UP = 1
    Up = 1,
    ///  `Down` -> VIPS_SIZE_DOWN = 2
    Down = 2,
    ///  `Force` -> VIPS_SIZE_FORCE = 3
    Force = 3,
}

#[derive(Copy, Clone, Debug, FromPrimitive, ToPrimitive, PartialEq, PartialOrd)]
pub enum TextWrap {
    ///  `Word` -> VIPS_TEXT_WRAP_WORD = 0
    Word = 0,
    ///  `Char` -> VIPS_TEXT_WRAP_CHAR = 1
    Char = 1,
    ///  `WordChar` -> VIPS_TEXT_WRAP_WORD_CHAR = 2
    WordChar = 2,
    ///  `None` -> VIPS_TEXT_WRAP_NONE = 3
    None = 3,
}
