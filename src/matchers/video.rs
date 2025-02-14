/// Returns whether a buffer is M4V video data.
pub fn is_m4v(buf: &[u8]) -> bool {
    return buf.len() > 10
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x4D
        && buf[9] == 0x34
        && buf[10] == 0x56;
}

/// Returns whether a buffer is MKV video data.
pub fn is_mkv(buf: &[u8]) -> bool {
    return (buf.len() > 15
        && buf[0] == 0x1A
        && buf[1] == 0x45
        && buf[2] == 0xDF
        && buf[3] == 0xA3
        && buf[4] == 0x93
        && buf[5] == 0x42
        && buf[6] == 0x82
        && buf[7] == 0x88
        && buf[8] == 0x6D
        && buf[9] == 0x61
        && buf[10] == 0x74
        && buf[11] == 0x72
        && buf[12] == 0x6F
        && buf[13] == 0x73
        && buf[14] == 0x6B
        && buf[15] == 0x61)
        || (buf.len() > 38
            && buf[31] == 0x6D
            && buf[32] == 0x61
            && buf[33] == 0x74
            && buf[34] == 0x72
            && buf[35] == 0x6f
            && buf[36] == 0x73
            && buf[37] == 0x6B
            && buf[38] == 0x61);
}

/// Returns whether a buffer is WEBM video data.
pub fn is_webm(buf: &[u8]) -> bool {
    return buf.len() > 3 && buf[0] == 0x1A && buf[1] == 0x45 && buf[2] == 0xDF && buf[3] == 0xA3;
}

/// Returns whether a buffer is Quicktime MOV video data.
pub fn is_mov(buf: &[u8]) -> bool {
    return buf.len() > 15
        && ((buf[0] == 0x0
            && buf[1] == 0x0
            && buf[2] == 0x0
            && buf[3] == 0x14
            && buf[4] == 0x66
            && buf[5] == 0x74
            && buf[6] == 0x79
            && buf[7] == 0x70)
            || (buf[4] == 0x6d && buf[5] == 0x6f && buf[6] == 0x6f && buf[7] == 0x76)
            || (buf[4] == 0x6d && buf[5] == 0x64 && buf[6] == 0x61 && buf[7] == 0x74)
            || (buf[12] == 0x6d && buf[13] == 0x64 && buf[14] == 0x61 && buf[15] == 0x74));
}

/// Returns whether a buffer is AVI video data.
pub fn is_avi(buf: &[u8]) -> bool {
    return buf.len() > 10
        && buf[0] == 0x52
        && buf[1] == 0x49
        && buf[2] == 0x46
        && buf[3] == 0x46
        && buf[8] == 0x41
        && buf[9] == 0x56
        && buf[10] == 0x49;
}

/// Returns whether a buffer is WMV video data.
pub fn is_wmv(buf: &[u8]) -> bool {
    return buf.len() > 9
        && buf[0] == 0x30
        && buf[1] == 0x26
        && buf[2] == 0xB2
        && buf[3] == 0x75
        && buf[4] == 0x8E
        && buf[5] == 0x66
        && buf[6] == 0xCF
        && buf[7] == 0x11
        && buf[8] == 0xA6
        && buf[9] == 0xD9;
}

/// Returns whether a buffer is MPEG video data.
pub fn is_mpeg(buf: &[u8]) -> bool {
    return buf.len() > 3
        && buf[0] == 0x0
        && buf[1] == 0x0
        && buf[2] == 0x1
        && buf[3] >= 0xb0
        && buf[3] <= 0xbf;
}

/// Returns whether a buffer is FLV video data.
pub fn is_flv(buf: &[u8]) -> bool {
    return buf.len() > 3 && buf[0] == 0x46 && buf[1] == 0x4C && buf[2] == 0x56 && buf[3] == 0x01;
}

/// Returns whether a buffer is MP4 video data.
pub fn is_mp4(buf: &[u8]) -> bool {
    return buf.len() > 11
        && (buf[4] == 'f' as u8 && buf[5] == 't' as u8 && buf[6] == 'y' as u8 && buf[7] == 'p' as u8)
        && ((buf[8] == 'a' as u8 && buf[9] == 'v' as u8 && buf[10] == 'c' as u8 && buf[11] == '1' as u8)
            || (buf[8] == 'd' as u8 && buf[9] == 'a' as u8 && buf[10] == 's' as u8 && buf[11] == 'h' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == '2' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == '3' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == '4' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == '5' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == '6' as u8)
            || (buf[8] == 'i' as u8 && buf[9] == 's' as u8 && buf[10] == 'o' as u8 && buf[11] == 'm' as u8)
            || (buf[8] == 'm' as u8 && buf[9] == 'm' as u8 && buf[10] == 'p' as u8 && buf[11] == '4' as u8)
            || (buf[8] == 'm' as u8 && buf[9] == 'p' as u8 && buf[10] == '4' as u8 && buf[11] == '1' as u8)
            || (buf[8] == 'm' as u8 && buf[9] == 'p' as u8 && buf[10] == '4' as u8 && buf[11] == '2' as u8)
            || (buf[8] == 'm' as u8 && buf[9] == 'p' as u8 && buf[10] == '4' as u8 && buf[11] == 'v' as u8)
            || (buf[8] == 'm' as u8 && buf[9] == 'p' as u8 && buf[10] == '7' as u8 && buf[11] == '1' as u8)
            || (buf[8] == 'M' as u8 && buf[9] == 'S' as u8 && buf[10] == 'N' as u8 && buf[11] == 'V' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'A' as u8 && buf[11] == 'S' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'S' as u8 && buf[11] == 'C' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'S' as u8 && buf[10] == 'D' as u8 && buf[11] == 'C' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'S' as u8 && buf[11] == 'H' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'S' as u8 && buf[11] == 'M' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'S' as u8 && buf[11] == 'P' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'S' as u8 && buf[11] == 'S' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'X' as u8 && buf[11] == 'C' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'X' as u8 && buf[11] == 'H' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'X' as u8 && buf[11] == 'M' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'X' as u8 && buf[11] == 'P' as u8)
            || (buf[8] == 'N' as u8 && buf[9] == 'D' as u8 && buf[10] == 'X' as u8 && buf[11] == 'S' as u8)
            || (buf[8] == 'F' as u8 && buf[9] == '4' as u8 && buf[10] == 'V' as u8 && buf[11] == ' ' as u8)
            || (buf[8] == 'F' as u8 && buf[9] == '4' as u8 && buf[10] == 'P' as u8 && buf[11] == ' ' as u8));
}
