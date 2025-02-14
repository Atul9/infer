extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};

#[derive(Debug, Eq, PartialEq)]
enum DocType {
	// DOC,
	DOCX,
	// XLS,
	XLSX,
	// PPT,
	PPTX,
	OOXLM,
}

/// Returns whether a buffer is Microsoft Word Document (DOC) data.
pub fn is_doc(buf: &[u8]) -> bool {
	return buf.len() > 7
		&& buf[0] == 0xD0
		&& buf[1] == 0xCF
		&& buf[2] == 0x11
		&& buf[3] == 0xE0
		&& buf[4] == 0xA1
		&& buf[5] == 0xB1
		&& buf[6] == 0x1A
		&& buf[7] == 0xE1;
}

/// Returns whether a buffer is Microsoft Word Open XML Format Document (DOCX) data.
pub fn is_docx(buf: &[u8]) -> bool {
	match msooxml(buf) {
		Some(typ) => typ == DocType::DOCX,
		None => false,
	}
}

/// Returns whether a buffer is Microsoft Excel 97-2003 Worksheet (XLS) data.
pub fn is_xls(buf: &[u8]) -> bool {
	return buf.len() > 7
		&& buf[0] == 0xD0
		&& buf[1] == 0xCF
		&& buf[2] == 0x11
		&& buf[3] == 0xE0
		&& buf[4] == 0xA1
		&& buf[5] == 0xB1
		&& buf[6] == 0x1A
		&& buf[7] == 0xE1;
}

/// Returns whether a buffer is Microsoft Excel Open XML Format Spreadsheet (XLSX) data.
pub fn is_xlsx(buf: &[u8]) -> bool {
	match msooxml(buf) {
		Some(typ) => typ == DocType::XLSX,
		None => false,
	}
}

/// Returns whether a buffer is Microsoft PowerPoint 97-2003 Presentation (PPT) data.
pub fn is_ppt(buf: &[u8]) -> bool {
	return buf.len() > 7
		&& buf[0] == 0xD0
		&& buf[1] == 0xCF
		&& buf[2] == 0x11
		&& buf[3] == 0xE0
		&& buf[4] == 0xA1
		&& buf[5] == 0xB1
		&& buf[6] == 0x1A
		&& buf[7] == 0xE1;
}


/// Returns whether a buffer is Microsoft PowerPoint Open XML Presentation (PPTX) data.
pub fn is_pptx(buf: &[u8]) -> bool {
	match msooxml(buf) {
		Some(typ) => typ == DocType::PPTX,
		None => false,
	}
}

fn msooxml(buf: &[u8]) -> Option<DocType> {
	let signature = ['P' as u8, 'K' as u8, 0x03, 0x04];

	// start by checking for ZIP local file header signature
	if !compare_bytes(buf, &signature, 0) {
		return None;
	}

	let v = check_msooml(buf, 0x1E);
	if v.is_some() {
		return v;
	}

	if !compare_bytes(buf, "[Content_Types].xml".as_bytes(), 0x1E)
		&& !compare_bytes(buf, "_rels/.rels".as_bytes(), 0x1E)
	{
		return None;
	}

	// skip to the second local file header
	// since some documents include a 520-byte extra field following the file
	// header, we need to scan for the next header
	let mut start_offset = (LittleEndian::read_u32(&buf[18..22]) + 49) as usize;
	let mut idxo = search(buf, start_offset, 6000);
	if idxo.is_none() {
		return None;
	}

	// now skip to the *third* local file header; again, we need to scan due to a
	// 520-byte extra field following the file header
	let mut idx = idxo.unwrap();
	start_offset += idx + 4 + 26;
	idxo = search(buf, start_offset, 6000);
	if idxo.is_none() {
		return None;
	}

	// and check the subdirectory name to determine which type of OOXML
	// file we have.  Correct the mimetype with the registered ones:
	// http://technet.microsoft.com/en-us/library/cc179224.aspx
	idx = idxo.unwrap();
	start_offset += idx + 4 + 26;
	let mut typo = check_msooml(buf, start_offset);
	if typo.is_some() {
		return typo;
	}

	// OpenOffice/Libreoffice orders ZIP entry differently, so check the 4th file
	start_offset += 26;
	idxo = search(buf, start_offset, 6000);
	if idxo.is_none() {
		return Some(DocType::OOXLM);
	}

	start_offset += idxo.unwrap() + 4 + 26;
	typo = check_msooml(buf, start_offset);
	if typo.is_some() {
		return typo;
	}

	return Some(DocType::OOXLM);
}

fn compare_bytes(slice: &[u8], sub_slice: &[u8], start_offset: usize) -> bool {
	let sl = sub_slice.len();

	if start_offset + sl > slice.len() {
		return false;
	}

	for (i, v) in slice.iter().skip(start_offset).take(sl).enumerate() {
		let v2 = sub_slice[i];

		if *v != v2 {
			return false;
		}
	}

	return true;
}

fn check_msooml(buf: &[u8], offset: usize) -> Option<DocType> {
	if compare_bytes(
		buf,
		&['w' as u8, 'o' as u8, 'r' as u8, 'd' as u8, '/' as u8],
		offset,
	) {
		return Some(DocType::DOCX);
	} else if compare_bytes(buf, &['p' as u8, 'p' as u8, 't' as u8, '/' as u8], offset) {
		return Some(DocType::PPTX);
	} else if compare_bytes(buf, &['x' as u8, 'l' as u8, '/' as u8], offset) {
		return Some(DocType::XLSX);
	} else {
		return None;
	}
}

fn search(buf: &[u8], start: usize, range: usize) -> Option<usize> {
	let length = buf.len();
	let mut end = start + range;
	let signature: &[_] = &['P' as u8, 'K' as u8, 0x03, 0x04];

	if end > length {
		end = length;
	}

	if start >= end {
		return None;
	}

	return buf[start..end]
		.windows(signature.len())
		.position(|window| window == signature);
}
