use libc::c_uint;
 
bitflags! {
    #[repr(C)]
    pub struct AiPostProcessSteps: c_uint {
        const AIPROCESS_CALC_TANGENT_SPACE = 0x1;
        const AIPROCESS_JOIN_IDENTICAL_VERTICES = 0x2;
        const AIPROCESS_MAKE_LEFT_HANDED = 0x4;
        const AIPROCESS_TRIANGULATE = 0x8;
        const AIPROCESS_REMOVE_COMPONENT = 0x10;
        const AIPROCESS_GEN_NORMALS = 0x20;
        const AIPROCESS_GEN_SMOOTH_NORMALS = 0x40;
        const AIPROCESS_SPLIT_LARGE_MESHES = 0x80;
        const AIPROCESS_PRE_TRANSFORM_VERTICES = 0x100;
        const AIPROCESS_LIMIT_BONE_WEIGHTS = 0x200;
        const AIPROCESS_VALIDATE_DATA_STRUCTURE = 0x400;
        const AIPROCESS_IMPROVE_CACHE_LOCALITY = 0x800;
        const AIPROCESS_REMOVE_REDUNDANT_MATERIALS = 0x1000;
        const AIPROCESS_FIX_INFACING_NORMALS = 0x2000;
        const AIPROCESS_SORT_BY_PTYPE = 0x8000;
        const AIPROCESS_FIND_DEGENERATES = 0x10000;
        const AIPROCESS_FIND_INVALID_DATA = 0x20000;
        const AIPROCESS_GEN_UV_COORDS = 0x40000;
        const AIPROCESS_TRANSFORM_UV_COORDS = 0x80000;
        const AIPROCESS_FIND_INSTANCES = 0x100000;
        const AIPROCESS_OPTIMIZE_MESHES = 0x200000;
        const AIPROCESS_OPTIMIZE_GRAPH = 0x400000;
        const AIPROCESS_FLIP_UVS = 0x800000;
        const AIPROCESS_FLIP_WINDING_ORDER = 0x1000000;
        const AIPROCESS_SPLIT_BY_BONE_COUNT = 0x2000000;
        const AIPROCESS_DEBONE = 0x4000000;

        const AIPROCESS_CONVERT_TO_LEFT_HANDED = Self::AIPROCESS_MAKE_LEFT_HANDED.bits
                                               | Self::AIPROCESS_FLIP_UVS.bits
                                               | Self::AIPROCESS_FLIP_WINDING_ORDER.bits;

        const AIPROCESS_TARGET_REALTIME_FAST = Self::AIPROCESS_CALC_TANGENT_SPACE.bits
                                             | Self::AIPROCESS_GEN_NORMALS.bits
                                             | Self::AIPROCESS_JOIN_IDENTICAL_VERTICES.bits
                                             | Self::AIPROCESS_TRIANGULATE.bits
                                             | Self::AIPROCESS_GEN_UV_COORDS.bits
                                             | Self::AIPROCESS_SORT_BY_PTYPE.bits;

        const AIPROCESS_TARGET_REALTIME_QUALITY = Self::AIPROCESS_CALC_TANGENT_SPACE.bits
                                                | Self::AIPROCESS_GEN_SMOOTH_NORMALS.bits
                                                | Self::AIPROCESS_JOIN_IDENTICAL_VERTICES.bits
                                                | Self::AIPROCESS_IMPROVE_CACHE_LOCALITY.bits
                                                | Self::AIPROCESS_LIMIT_BONE_WEIGHTS.bits
                                                | Self::AIPROCESS_REMOVE_REDUNDANT_MATERIALS.bits
                                                | Self::AIPROCESS_SPLIT_LARGE_MESHES.bits
                                                | Self::AIPROCESS_TRIANGULATE.bits
                                                | Self::AIPROCESS_GEN_UV_COORDS.bits
                                                | Self::AIPROCESS_SORT_BY_PTYPE.bits
                                                | Self::AIPROCESS_FIND_DEGENERATES.bits
                                                | Self::AIPROCESS_FIND_INVALID_DATA.bits;

        const AIPROCESS_TARGET_REALTIME_MAX_QUALITY = Self::AIPROCESS_TARGET_REALTIME_QUALITY.bits
                                                    | Self::AIPROCESS_FIND_INSTANCES.bits
                                                    | Self::AIPROCESS_VALIDATE_DATA_STRUCTURE.bits
                                                    | Self::AIPROCESS_OPTIMIZE_MESHES.bits;
    }
}

