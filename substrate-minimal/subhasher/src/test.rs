// hack-ink
use super::*;

const DATA: &[u8] = &[
	180, 247, 240, 59, 235, 197, 110, 190, 150, 188, 82, 234, 94, 211, 21, 157, 69, 160, 206, 58,
	141, 127, 8, 41, 131, 195, 62, 241, 51, 39, 71, 71,
];

#[test]
fn blake2_128_should_work() {
	assert_eq!(
		blake2_128(DATA),
		[229, 139, 8, 235, 70, 70, 242, 62, 137, 189, 226, 103, 108, 142, 31, 238]
	)
}

#[test]
fn blake2_256_should_work() {
	assert_eq!(
		blake2_256(DATA),
		[
			131, 170, 146, 72, 1, 154, 151, 42, 103, 74, 218, 88, 207, 203, 107, 13, 200, 92, 201,
			76, 144, 105, 27, 30, 147, 211, 231, 29, 237, 80, 125, 50
		]
	)
}

#[test]
fn blake2_512_should_work() {
	assert_eq!(
		blake2_512(DATA),
		[
			112, 104, 54, 133, 50, 167, 180, 164, 222, 223, 153, 148, 191, 229, 132, 228, 71, 147,
			4, 232, 219, 0, 60, 238, 54, 88, 219, 122, 217, 56, 173, 212, 171, 44, 232, 214, 46,
			85, 158, 77, 225, 65, 34, 113, 36, 211, 181, 177, 216, 216, 77, 62, 94, 86, 8, 78, 167,
			128, 79, 135, 27, 63, 222, 157
		]
	)
}

#[test]
fn blake2_128_concat_should_work() {
	assert_eq!(
		blake2_128_concat(DATA),
		[
			229, 139, 8, 235, 70, 70, 242, 62, 137, 189, 226, 103, 108, 142, 31, 238, 180, 247,
			240, 59, 235, 197, 110, 190, 150, 188, 82, 234, 94, 211, 21, 157, 69, 160, 206, 58,
			141, 127, 8, 41, 131, 195, 62, 241, 51, 39, 71, 71
		]
	)
}

#[test]
fn twox64_should_work() {
	assert_eq!(twox64(DATA), [58, 176, 83, 220, 107, 106, 113, 225])
}

#[test]
fn twox128_should_work() {
	assert_eq!(
		twox128(DATA),
		[58, 176, 83, 220, 107, 106, 113, 225, 63, 19, 228, 73, 73, 184, 227, 136]
	)
}

#[test]
fn twox256_should_work() {
	assert_eq!(
		twox256(DATA),
		[
			58, 176, 83, 220, 107, 106, 113, 225, 63, 19, 228, 73, 73, 184, 227, 136, 99, 136, 56,
			248, 128, 160, 227, 220, 24, 225, 151, 183, 248, 239, 214, 220
		]
	)
}

#[test]
fn twox64_concat_should_work() {
	assert_eq!(
		twox64_concat(DATA),
		[
			58, 176, 83, 220, 107, 106, 113, 225, 180, 247, 240, 59, 235, 197, 110, 190, 150, 188,
			82, 234, 94, 211, 21, 157, 69, 160, 206, 58, 141, 127, 8, 41, 131, 195, 62, 241, 51,
			39, 71, 71
		]
	)
}

#[test]
fn keccak256_should_work() {
	assert_eq!(
		keccak256(DATA),
		[
			119, 116, 227, 161, 78, 72, 104, 189, 95, 63, 107, 212, 9, 104, 94, 253, 14, 81, 121,
			231, 131, 237, 83, 210, 165, 218, 66, 201, 17, 239, 176, 90
		]
	)
}

#[test]
fn keccak512_should_work() {
	assert_eq!(
		keccak512(DATA),
		[
			231, 174, 179, 22, 249, 128, 128, 152, 231, 215, 25, 152, 212, 100, 44, 120, 74, 236,
			189, 187, 192, 48, 97, 253, 202, 236, 105, 146, 131, 27, 246, 174, 90, 187, 86, 115,
			175, 79, 61, 236, 79, 74, 238, 213, 67, 57, 15, 254, 218, 158, 15, 137, 0, 247, 113,
			245, 96, 67, 18, 129, 119, 177, 188, 234
		]
	)
}

#[test]
fn sha2_256_should_work() {
	assert_eq!(
		sha2_256(DATA),
		[
			93, 10, 233, 171, 98, 92, 231, 197, 14, 92, 237, 7, 159, 65, 79, 40, 22, 27, 27, 100,
			93, 244, 150, 121, 235, 215, 148, 252, 67, 194, 57, 188
		]
	)
}