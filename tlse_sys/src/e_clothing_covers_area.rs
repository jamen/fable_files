#[derive(Debug)]
#[repr(C)]
pub enum EClothingCoversArea {
	COVERS_BODY_AREA_NULL = 0,
	COVERS_BODY_AREA_FEET = 1,
	COVERS_BODY_AREA_LEGS = 2,
	COVERS_BODY_AREA_ARSE = 4,
	COVERS_BODY_AREA_BODY = 8,
	COVERS_BODY_AREA_HEAD = 16,
	COVERS_BODY_AREA_ARMS = 32,
	COVERS_BODY_AREA_HANDS = 64,
	COVERS_BODY_AREA_FACE = 128,
	COVERS_BODY_AREA_MOUSTACHE = 256,
	COVERS_BODY_AREA_HORN = 512,
}