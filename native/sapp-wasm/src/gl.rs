pub const GL_GLES_PROTOTYPES: u32 = 1;
pub const GL_ES_VERSION_2_0: u32 = 1;
pub const GL_DEPTH_BUFFER_BIT: u32 = 256;
pub const GL_STENCIL_BUFFER_BIT: u32 = 1024;
pub const GL_COLOR_BUFFER_BIT: u32 = 16384;
pub const GL_FALSE: u32 = 0;
pub const GL_TRUE: u32 = 1;
pub const GL_POINTS: u32 = 0;
pub const GL_LINES: u32 = 1;
pub const GL_LINE_LOOP: u32 = 2;
pub const GL_LINE_STRIP: u32 = 3;
pub const GL_TRIANGLES: u32 = 4;
pub const GL_TRIANGLE_STRIP: u32 = 5;
pub const GL_TRIANGLE_FAN: u32 = 6;
pub const GL_ZERO: u32 = 0;
pub const GL_ONE: u32 = 1;
pub const GL_SRC_COLOR: u32 = 768;
pub const GL_ONE_MINUS_SRC_COLOR: u32 = 769;
pub const GL_SRC_ALPHA: u32 = 770;
pub const GL_ONE_MINUS_SRC_ALPHA: u32 = 771;
pub const GL_DST_ALPHA: u32 = 772;
pub const GL_ONE_MINUS_DST_ALPHA: u32 = 773;
pub const GL_DST_COLOR: u32 = 774;
pub const GL_ONE_MINUS_DST_COLOR: u32 = 775;
pub const GL_SRC_ALPHA_SATURATE: u32 = 776;
pub const GL_FUNC_ADD: u32 = 32774;
pub const GL_BLEND_EQUATION: u32 = 32777;
pub const GL_BLEND_EQUATION_RGB: u32 = 32777;
pub const GL_BLEND_EQUATION_ALPHA: u32 = 34877;
pub const GL_FUNC_SUBTRACT: u32 = 32778;
pub const GL_FUNC_REVERSE_SUBTRACT: u32 = 32779;
pub const GL_BLEND_DST_RGB: u32 = 32968;
pub const GL_BLEND_SRC_RGB: u32 = 32969;
pub const GL_BLEND_DST_ALPHA: u32 = 32970;
pub const GL_BLEND_SRC_ALPHA: u32 = 32971;
pub const GL_CONSTANT_COLOR: u32 = 32769;
pub const GL_ONE_MINUS_CONSTANT_COLOR: u32 = 32770;
pub const GL_CONSTANT_ALPHA: u32 = 32771;
pub const GL_ONE_MINUS_CONSTANT_ALPHA: u32 = 32772;
pub const GL_BLEND_COLOR: u32 = 32773;
pub const GL_ARRAY_BUFFER: u32 = 34962;
pub const GL_ELEMENT_ARRAY_BUFFER: u32 = 34963;
pub const GL_ARRAY_BUFFER_BINDING: u32 = 34964;
pub const GL_ELEMENT_ARRAY_BUFFER_BINDING: u32 = 34965;
pub const GL_STREAM_DRAW: u32 = 35040;
pub const GL_STATIC_DRAW: u32 = 35044;
pub const GL_DYNAMIC_DRAW: u32 = 35048;
pub const GL_BUFFER_SIZE: u32 = 34660;
pub const GL_BUFFER_USAGE: u32 = 34661;
pub const GL_CURRENT_VERTEX_ATTRIB: u32 = 34342;
pub const GL_FRONT: u32 = 1028;
pub const GL_BACK: u32 = 1029;
pub const GL_FRONT_AND_BACK: u32 = 1032;
pub const GL_TEXTURE_2D: u32 = 3553;
pub const GL_CULL_FACE: u32 = 2884;
pub const GL_BLEND: u32 = 3042;
pub const GL_DITHER: u32 = 3024;
pub const GL_STENCIL_TEST: u32 = 2960;
pub const GL_DEPTH_TEST: u32 = 2929;
pub const GL_SCISSOR_TEST: u32 = 3089;
pub const GL_POLYGON_OFFSET_FILL: u32 = 32823;
pub const GL_SAMPLE_ALPHA_TO_COVERAGE: u32 = 32926;
pub const GL_SAMPLE_COVERAGE: u32 = 32928;
pub const GL_NO_ERROR: u32 = 0;
pub const GL_INVALID_ENUM: u32 = 1280;
pub const GL_INVALID_VALUE: u32 = 1281;
pub const GL_INVALID_OPERATION: u32 = 1282;
pub const GL_OUT_OF_MEMORY: u32 = 1285;
pub const GL_CW: u32 = 2304;
pub const GL_CCW: u32 = 2305;
pub const GL_LINE_WIDTH: u32 = 2849;
pub const GL_ALIASED_POINT_SIZE_RANGE: u32 = 33901;
pub const GL_ALIASED_LINE_WIDTH_RANGE: u32 = 33902;
pub const GL_CULL_FACE_MODE: u32 = 2885;
pub const GL_FRONT_FACE: u32 = 2886;
pub const GL_DEPTH_RANGE: u32 = 2928;
pub const GL_DEPTH_WRITEMASK: u32 = 2930;
pub const GL_DEPTH_CLEAR_VALUE: u32 = 2931;
pub const GL_DEPTH_FUNC: u32 = 2932;
pub const GL_STENCIL_CLEAR_VALUE: u32 = 2961;
pub const GL_STENCIL_FUNC: u32 = 2962;
pub const GL_STENCIL_FAIL: u32 = 2964;
pub const GL_STENCIL_PASS_DEPTH_FAIL: u32 = 2965;
pub const GL_STENCIL_PASS_DEPTH_PASS: u32 = 2966;
pub const GL_STENCIL_REF: u32 = 2967;
pub const GL_STENCIL_VALUE_MASK: u32 = 2963;
pub const GL_STENCIL_WRITEMASK: u32 = 2968;
pub const GL_STENCIL_BACK_FUNC: u32 = 34816;
pub const GL_STENCIL_BACK_FAIL: u32 = 34817;
pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL: u32 = 34818;
pub const GL_STENCIL_BACK_PASS_DEPTH_PASS: u32 = 34819;
pub const GL_STENCIL_BACK_REF: u32 = 36003;
pub const GL_STENCIL_BACK_VALUE_MASK: u32 = 36004;
pub const GL_STENCIL_BACK_WRITEMASK: u32 = 36005;
pub const GL_VIEWPORT: u32 = 2978;
pub const GL_SCISSOR_BOX: u32 = 3088;
pub const GL_COLOR_CLEAR_VALUE: u32 = 3106;
pub const GL_COLOR_WRITEMASK: u32 = 3107;
pub const GL_UNPACK_ALIGNMENT: u32 = 3317;
pub const GL_PACK_ALIGNMENT: u32 = 3333;
pub const GL_MAX_TEXTURE_SIZE: u32 = 3379;
pub const GL_MAX_VIEWPORT_DIMS: u32 = 3386;
pub const GL_SUBPIXEL_BITS: u32 = 3408;
pub const GL_RED_BITS: u32 = 3410;
pub const GL_GREEN_BITS: u32 = 3411;
pub const GL_BLUE_BITS: u32 = 3412;
pub const GL_ALPHA_BITS: u32 = 3413;
pub const GL_DEPTH_BITS: u32 = 3414;
pub const GL_STENCIL_BITS: u32 = 3415;
pub const GL_POLYGON_OFFSET_UNITS: u32 = 10752;
pub const GL_POLYGON_OFFSET_FACTOR: u32 = 32824;
pub const GL_TEXTURE_BINDING_2D: u32 = 32873;
pub const GL_SAMPLE_BUFFERS: u32 = 32936;
pub const GL_SAMPLES: u32 = 32937;
pub const GL_SAMPLE_COVERAGE_VALUE: u32 = 32938;
pub const GL_SAMPLE_COVERAGE_INVERT: u32 = 32939;
pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS: u32 = 34466;
pub const GL_COMPRESSED_TEXTURE_FORMATS: u32 = 34467;
pub const GL_DONT_CARE: u32 = 4352;
pub const GL_FASTEST: u32 = 4353;
pub const GL_NICEST: u32 = 4354;
pub const GL_GENERATE_MIPMAP_HINT: u32 = 33170;
pub const GL_BYTE: u32 = 5120;
pub const GL_UNSIGNED_BYTE: u32 = 5121;
pub const GL_SHORT: u32 = 5122;
pub const GL_UNSIGNED_SHORT: u32 = 5123;
pub const GL_INT: u32 = 5124;
pub const GL_UNSIGNED_INT: u32 = 5125;
pub const GL_FLOAT: u32 = 5126;
pub const GL_FIXED: u32 = 5132;
pub const GL_DEPTH_COMPONENT: u32 = 6402;
pub const GL_ALPHA: u32 = 6406;
pub const GL_RGB: u32 = 6407;
pub const GL_RGBA: u32 = 6408;
pub const GL_LUMINANCE: u32 = 6409;
pub const GL_LUMINANCE_ALPHA: u32 = 6410;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 32819;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 32820;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 33635;
pub const GL_FRAGMENT_SHADER: u32 = 35632;
pub const GL_VERTEX_SHADER: u32 = 35633;
pub const GL_MAX_VERTEX_ATTRIBS: u32 = 34921;
pub const GL_MAX_VERTEX_UNIFORM_VECTORS: u32 = 36347;
pub const GL_MAX_VARYING_VECTORS: u32 = 36348;
pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 35661;
pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS: u32 = 35660;
pub const GL_MAX_TEXTURE_IMAGE_UNITS: u32 = 34930;
pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 36349;
pub const GL_SHADER_TYPE: u32 = 35663;
pub const GL_DELETE_STATUS: u32 = 35712;
pub const GL_LINK_STATUS: u32 = 35714;
pub const GL_VALIDATE_STATUS: u32 = 35715;
pub const GL_ATTACHED_SHADERS: u32 = 35717;
pub const GL_ACTIVE_UNIFORMS: u32 = 35718;
pub const GL_ACTIVE_UNIFORM_MAX_LENGTH: u32 = 35719;
pub const GL_ACTIVE_ATTRIBUTES: u32 = 35721;
pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH: u32 = 35722;
pub const GL_SHADING_LANGUAGE_VERSION: u32 = 35724;
pub const GL_CURRENT_PROGRAM: u32 = 35725;
pub const GL_NEVER: u32 = 512;
pub const GL_LESS: u32 = 513;
pub const GL_EQUAL: u32 = 514;
pub const GL_LEQUAL: u32 = 515;
pub const GL_GREATER: u32 = 516;
pub const GL_NOTEQUAL: u32 = 517;
pub const GL_GEQUAL: u32 = 518;
pub const GL_ALWAYS: u32 = 519;
pub const GL_KEEP: u32 = 7680;
pub const GL_REPLACE: u32 = 7681;
pub const GL_INCR: u32 = 7682;
pub const GL_DECR: u32 = 7683;
pub const GL_INVERT: u32 = 5386;
pub const GL_INCR_WRAP: u32 = 34055;
pub const GL_DECR_WRAP: u32 = 34056;
pub const GL_VENDOR: u32 = 7936;
pub const GL_RENDERER: u32 = 7937;
pub const GL_VERSION: u32 = 7938;
pub const GL_EXTENSIONS: u32 = 7939;
pub const GL_NEAREST: u32 = 9728;
pub const GL_LINEAR: u32 = 9729;
pub const GL_NEAREST_MIPMAP_NEAREST: u32 = 9984;
pub const GL_LINEAR_MIPMAP_NEAREST: u32 = 9985;
pub const GL_NEAREST_MIPMAP_LINEAR: u32 = 9986;
pub const GL_LINEAR_MIPMAP_LINEAR: u32 = 9987;
pub const GL_TEXTURE_MAG_FILTER: u32 = 10240;
pub const GL_TEXTURE_MIN_FILTER: u32 = 10241;
pub const GL_TEXTURE_WRAP_S: u32 = 10242;
pub const GL_TEXTURE_WRAP_T: u32 = 10243;
pub const GL_TEXTURE: u32 = 5890;
pub const GL_TEXTURE_CUBE_MAP: u32 = 34067;
pub const GL_TEXTURE_BINDING_CUBE_MAP: u32 = 34068;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X: u32 = 34069;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X: u32 = 34070;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y: u32 = 34071;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y: u32 = 34072;
pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z: u32 = 34073;
pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z: u32 = 34074;
pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 34076;
pub const GL_TEXTURE0: u32 = 33984;
pub const GL_TEXTURE1: u32 = 33985;
pub const GL_TEXTURE2: u32 = 33986;
pub const GL_TEXTURE3: u32 = 33987;
pub const GL_TEXTURE4: u32 = 33988;
pub const GL_TEXTURE5: u32 = 33989;
pub const GL_TEXTURE6: u32 = 33990;
pub const GL_TEXTURE7: u32 = 33991;
pub const GL_TEXTURE8: u32 = 33992;
pub const GL_TEXTURE9: u32 = 33993;
pub const GL_TEXTURE10: u32 = 33994;
pub const GL_TEXTURE11: u32 = 33995;
pub const GL_TEXTURE12: u32 = 33996;
pub const GL_TEXTURE13: u32 = 33997;
pub const GL_TEXTURE14: u32 = 33998;
pub const GL_TEXTURE15: u32 = 33999;
pub const GL_TEXTURE16: u32 = 34000;
pub const GL_TEXTURE17: u32 = 34001;
pub const GL_TEXTURE18: u32 = 34002;
pub const GL_TEXTURE19: u32 = 34003;
pub const GL_TEXTURE20: u32 = 34004;
pub const GL_TEXTURE21: u32 = 34005;
pub const GL_TEXTURE22: u32 = 34006;
pub const GL_TEXTURE23: u32 = 34007;
pub const GL_TEXTURE24: u32 = 34008;
pub const GL_TEXTURE25: u32 = 34009;
pub const GL_TEXTURE26: u32 = 34010;
pub const GL_TEXTURE27: u32 = 34011;
pub const GL_TEXTURE28: u32 = 34012;
pub const GL_TEXTURE29: u32 = 34013;
pub const GL_TEXTURE30: u32 = 34014;
pub const GL_TEXTURE31: u32 = 34015;
pub const GL_ACTIVE_TEXTURE: u32 = 34016;
pub const GL_REPEAT: u32 = 10497;
pub const GL_CLAMP_TO_EDGE: u32 = 33071;
pub const GL_MIRRORED_REPEAT: u32 = 33648;
pub const GL_FLOAT_VEC2: u32 = 35664;
pub const GL_FLOAT_VEC3: u32 = 35665;
pub const GL_FLOAT_VEC4: u32 = 35666;
pub const GL_INT_VEC2: u32 = 35667;
pub const GL_INT_VEC3: u32 = 35668;
pub const GL_INT_VEC4: u32 = 35669;
pub const GL_BOOL: u32 = 35670;
pub const GL_BOOL_VEC2: u32 = 35671;
pub const GL_BOOL_VEC3: u32 = 35672;
pub const GL_BOOL_VEC4: u32 = 35673;
pub const GL_FLOAT_MAT2: u32 = 35674;
pub const GL_FLOAT_MAT3: u32 = 35675;
pub const GL_FLOAT_MAT4: u32 = 35676;
pub const GL_SAMPLER_2D: u32 = 35678;
pub const GL_SAMPLER_CUBE: u32 = 35680;
pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED: u32 = 34338;
pub const GL_VERTEX_ATTRIB_ARRAY_SIZE: u32 = 34339;
pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE: u32 = 34340;
pub const GL_VERTEX_ATTRIB_ARRAY_TYPE: u32 = 34341;
pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED: u32 = 34922;
pub const GL_VERTEX_ATTRIB_ARRAY_POINTER: u32 = 34373;
pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING: u32 = 34975;
pub const GL_IMPLEMENTATION_COLOR_READ_TYPE: u32 = 35738;
pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT: u32 = 35739;
pub const GL_COMPILE_STATUS: u32 = 35713;
pub const GL_INFO_LOG_LENGTH: u32 = 35716;
pub const GL_SHADER_SOURCE_LENGTH: u32 = 35720;
pub const GL_SHADER_COMPILER: u32 = 36346;
pub const GL_SHADER_BINARY_FORMATS: u32 = 36344;
pub const GL_NUM_SHADER_BINARY_FORMATS: u32 = 36345;
pub const GL_LOW_FLOAT: u32 = 36336;
pub const GL_MEDIUM_FLOAT: u32 = 36337;
pub const GL_HIGH_FLOAT: u32 = 36338;
pub const GL_LOW_INT: u32 = 36339;
pub const GL_MEDIUM_INT: u32 = 36340;
pub const GL_HIGH_INT: u32 = 36341;
pub const GL_FRAMEBUFFER: u32 = 36160;
pub const GL_RENDERBUFFER: u32 = 36161;
pub const GL_RGBA4: u32 = 32854;
pub const GL_RGB5_A1: u32 = 32855;
pub const GL_RGB565: u32 = 36194;
pub const GL_DEPTH_COMPONENT16: u32 = 33189;
pub const GL_STENCIL_INDEX8: u32 = 36168;
pub const GL_RENDERBUFFER_WIDTH: u32 = 36162;
pub const GL_RENDERBUFFER_HEIGHT: u32 = 36163;
pub const GL_RENDERBUFFER_INTERNAL_FORMAT: u32 = 36164;
pub const GL_RENDERBUFFER_RED_SIZE: u32 = 36176;
pub const GL_RENDERBUFFER_GREEN_SIZE: u32 = 36177;
pub const GL_RENDERBUFFER_BLUE_SIZE: u32 = 36178;
pub const GL_RENDERBUFFER_ALPHA_SIZE: u32 = 36179;
pub const GL_RENDERBUFFER_DEPTH_SIZE: u32 = 36180;
pub const GL_RENDERBUFFER_STENCIL_SIZE: u32 = 36181;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE: u32 = 36048;
pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME: u32 = 36049;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL: u32 = 36050;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE: u32 = 36051;
pub const GL_COLOR_ATTACHMENT0: u32 = 36064;
pub const GL_DEPTH_ATTACHMENT: u32 = 36096;
pub const GL_STENCIL_ATTACHMENT: u32 = 36128;
pub const GL_NONE: u32 = 0;
pub const GL_FRAMEBUFFER_COMPLETE: u32 = 36053;
pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT: u32 = 36054;
pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT: u32 = 36055;
pub const GL_FRAMEBUFFER_INCOMPLETE_DIMENSIONS: u32 = 36057;
pub const GL_FRAMEBUFFER_UNSUPPORTED: u32 = 36061;
pub const GL_FRAMEBUFFER_BINDING: u32 = 36006;
pub const GL_RENDERBUFFER_BINDING: u32 = 36007;
pub const GL_MAX_RENDERBUFFER_SIZE: u32 = 34024;
pub const GL_INVALID_FRAMEBUFFER_OPERATION: u32 = 1286;
pub const GL_ES_VERSION_3_0: u32 = 1;
pub const GL_READ_BUFFER: u32 = 3074;
pub const GL_UNPACK_ROW_LENGTH: u32 = 3314;
pub const GL_UNPACK_SKIP_ROWS: u32 = 3315;
pub const GL_UNPACK_SKIP_PIXELS: u32 = 3316;
pub const GL_PACK_ROW_LENGTH: u32 = 3330;
pub const GL_PACK_SKIP_ROWS: u32 = 3331;
pub const GL_PACK_SKIP_PIXELS: u32 = 3332;
pub const GL_COLOR: u32 = 6144;
pub const GL_DEPTH: u32 = 6145;
pub const GL_STENCIL: u32 = 6146;
pub const GL_RED: u32 = 6403;
pub const GL_RGB8: u32 = 32849;
pub const GL_RGBA8: u32 = 32856;
pub const GL_RGB10_A2: u32 = 32857;
pub const GL_TEXTURE_BINDING_3D: u32 = 32874;
pub const GL_UNPACK_SKIP_IMAGES: u32 = 32877;
pub const GL_UNPACK_IMAGE_HEIGHT: u32 = 32878;
pub const GL_TEXTURE_3D: u32 = 32879;
pub const GL_TEXTURE_WRAP_R: u32 = 32882;
pub const GL_MAX_3D_TEXTURE_SIZE: u32 = 32883;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: u32 = 33640;
pub const GL_MAX_ELEMENTS_VERTICES: u32 = 33000;
pub const GL_MAX_ELEMENTS_INDICES: u32 = 33001;
pub const GL_TEXTURE_MIN_LOD: u32 = 33082;
pub const GL_TEXTURE_MAX_LOD: u32 = 33083;
pub const GL_TEXTURE_BASE_LEVEL: u32 = 33084;
pub const GL_TEXTURE_MAX_LEVEL: u32 = 33085;
pub const GL_MIN: u32 = 32775;
pub const GL_MAX: u32 = 32776;
pub const GL_DEPTH_COMPONENT24: u32 = 33190;
pub const GL_MAX_TEXTURE_LOD_BIAS: u32 = 34045;
pub const GL_TEXTURE_COMPARE_MODE: u32 = 34892;
pub const GL_TEXTURE_COMPARE_FUNC: u32 = 34893;
pub const GL_CURRENT_QUERY: u32 = 34917;
pub const GL_QUERY_RESULT: u32 = 34918;
pub const GL_QUERY_RESULT_AVAILABLE: u32 = 34919;
pub const GL_TIME_ELAPSED: u32 = 35007;
pub const GL_BUFFER_MAPPED: u32 = 35004;
pub const GL_BUFFER_MAP_POINTER: u32 = 35005;
pub const GL_STREAM_READ: u32 = 35041;
pub const GL_STREAM_COPY: u32 = 35042;
pub const GL_STATIC_READ: u32 = 35045;
pub const GL_STATIC_COPY: u32 = 35046;
pub const GL_DYNAMIC_READ: u32 = 35049;
pub const GL_DYNAMIC_COPY: u32 = 35050;
pub const GL_MAX_DRAW_BUFFERS: u32 = 34852;
pub const GL_DRAW_BUFFER0: u32 = 34853;
pub const GL_DRAW_BUFFER1: u32 = 34854;
pub const GL_DRAW_BUFFER2: u32 = 34855;
pub const GL_DRAW_BUFFER3: u32 = 34856;
pub const GL_DRAW_BUFFER4: u32 = 34857;
pub const GL_DRAW_BUFFER5: u32 = 34858;
pub const GL_DRAW_BUFFER6: u32 = 34859;
pub const GL_DRAW_BUFFER7: u32 = 34860;
pub const GL_DRAW_BUFFER8: u32 = 34861;
pub const GL_DRAW_BUFFER9: u32 = 34862;
pub const GL_DRAW_BUFFER10: u32 = 34863;
pub const GL_DRAW_BUFFER11: u32 = 34864;
pub const GL_DRAW_BUFFER12: u32 = 34865;
pub const GL_DRAW_BUFFER13: u32 = 34866;
pub const GL_DRAW_BUFFER14: u32 = 34867;
pub const GL_DRAW_BUFFER15: u32 = 34868;
pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35657;
pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS: u32 = 35658;
pub const GL_SAMPLER_3D: u32 = 35679;
pub const GL_SAMPLER_2D_SHADOW: u32 = 35682;
pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT: u32 = 35723;
pub const GL_PIXEL_PACK_BUFFER: u32 = 35051;
pub const GL_PIXEL_UNPACK_BUFFER: u32 = 35052;
pub const GL_PIXEL_PACK_BUFFER_BINDING: u32 = 35053;
pub const GL_PIXEL_UNPACK_BUFFER_BINDING: u32 = 35055;
pub const GL_FLOAT_MAT2x3: u32 = 35685;
pub const GL_FLOAT_MAT2x4: u32 = 35686;
pub const GL_FLOAT_MAT3x2: u32 = 35687;
pub const GL_FLOAT_MAT3x4: u32 = 35688;
pub const GL_FLOAT_MAT4x2: u32 = 35689;
pub const GL_FLOAT_MAT4x3: u32 = 35690;
pub const GL_SRGB: u32 = 35904;
pub const GL_SRGB8: u32 = 35905;
pub const GL_SRGB8_ALPHA8: u32 = 35907;
pub const GL_COMPARE_REF_TO_TEXTURE: u32 = 34894;
pub const GL_MAJOR_VERSION: u32 = 33307;
pub const GL_MINOR_VERSION: u32 = 33308;
pub const GL_NUM_EXTENSIONS: u32 = 33309;
pub const GL_RGBA32F: u32 = 34836;
pub const GL_RGB32F: u32 = 34837;
pub const GL_RGBA16F: u32 = 34842;
pub const GL_RGB16F: u32 = 34843;
pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER: u32 = 35069;
pub const GL_MAX_ARRAY_TEXTURE_LAYERS: u32 = 35071;
pub const GL_MIN_PROGRAM_TEXEL_OFFSET: u32 = 35076;
pub const GL_MAX_PROGRAM_TEXEL_OFFSET: u32 = 35077;
pub const GL_MAX_VARYING_COMPONENTS: u32 = 35659;
pub const GL_TEXTURE_2D_ARRAY: u32 = 35866;
pub const GL_TEXTURE_BINDING_2D_ARRAY: u32 = 35869;
pub const GL_R11F_G11F_B10F: u32 = 35898;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 35899;
pub const GL_RGB9_E5: u32 = 35901;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: u32 = 35902;
pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH: u32 = 35958;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE: u32 = 35967;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS: u32 = 35968;
pub const GL_TRANSFORM_FEEDBACK_VARYINGS: u32 = 35971;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_START: u32 = 35972;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE: u32 = 35973;
pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN: u32 = 35976;
pub const GL_RASTERIZER_DISCARD: u32 = 35977;
pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS: u32 = 35978;
pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS: u32 = 35979;
pub const GL_INTERLEAVED_ATTRIBS: u32 = 35980;
pub const GL_SEPARATE_ATTRIBS: u32 = 35981;
pub const GL_TRANSFORM_FEEDBACK_BUFFER: u32 = 35982;
pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING: u32 = 35983;
pub const GL_RGBA32UI: u32 = 36208;
pub const GL_RGB32UI: u32 = 36209;
pub const GL_RGBA16UI: u32 = 36214;
pub const GL_RGB16UI: u32 = 36215;
pub const GL_RGBA8UI: u32 = 36220;
pub const GL_RGB8UI: u32 = 36221;
pub const GL_RGBA32I: u32 = 36226;
pub const GL_RGB32I: u32 = 36227;
pub const GL_RGBA16I: u32 = 36232;
pub const GL_RGB16I: u32 = 36233;
pub const GL_RGBA8I: u32 = 36238;
pub const GL_RGB8I: u32 = 36239;
pub const GL_RED_INTEGER: u32 = 36244;
pub const GL_RGB_INTEGER: u32 = 36248;
pub const GL_RGBA_INTEGER: u32 = 36249;
pub const GL_SAMPLER_2D_ARRAY: u32 = 36289;
pub const GL_SAMPLER_2D_ARRAY_SHADOW: u32 = 36292;
pub const GL_SAMPLER_CUBE_SHADOW: u32 = 36293;
pub const GL_UNSIGNED_INT_VEC2: u32 = 36294;
pub const GL_UNSIGNED_INT_VEC3: u32 = 36295;
pub const GL_UNSIGNED_INT_VEC4: u32 = 36296;
pub const GL_INT_SAMPLER_2D: u32 = 36298;
pub const GL_INT_SAMPLER_3D: u32 = 36299;
pub const GL_INT_SAMPLER_CUBE: u32 = 36300;
pub const GL_INT_SAMPLER_2D_ARRAY: u32 = 36303;
pub const GL_UNSIGNED_INT_SAMPLER_2D: u32 = 36306;
pub const GL_UNSIGNED_INT_SAMPLER_3D: u32 = 36307;
pub const GL_UNSIGNED_INT_SAMPLER_CUBE: u32 = 36308;
pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY: u32 = 36311;
pub const GL_BUFFER_ACCESS_FLAGS: u32 = 37151;
pub const GL_BUFFER_MAP_LENGTH: u32 = 37152;
pub const GL_BUFFER_MAP_OFFSET: u32 = 37153;
pub const GL_DEPTH_COMPONENT32F: u32 = 36012;
pub const GL_DEPTH32F_STENCIL8: u32 = 36013;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 36269;
pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING: u32 = 33296;
pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE: u32 = 33297;
pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE: u32 = 33298;
pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE: u32 = 33299;
pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE: u32 = 33300;
pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE: u32 = 33301;
pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE: u32 = 33302;
pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE: u32 = 33303;
pub const GL_FRAMEBUFFER_DEFAULT: u32 = 33304;
pub const GL_FRAMEBUFFER_UNDEFINED: u32 = 33305;
pub const GL_DEPTH_STENCIL_ATTACHMENT: u32 = 33306;
pub const GL_DEPTH_STENCIL: u32 = 34041;
pub const GL_UNSIGNED_INT_24_8: u32 = 34042;
pub const GL_DEPTH24_STENCIL8: u32 = 35056;
pub const GL_UNSIGNED_NORMALIZED: u32 = 35863;
pub const GL_DRAW_FRAMEBUFFER_BINDING: u32 = 36006;
pub const GL_READ_FRAMEBUFFER: u32 = 36008;
pub const GL_DRAW_FRAMEBUFFER: u32 = 36009;
pub const GL_READ_FRAMEBUFFER_BINDING: u32 = 36010;
pub const GL_RENDERBUFFER_SAMPLES: u32 = 36011;
pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER: u32 = 36052;
pub const GL_MAX_COLOR_ATTACHMENTS: u32 = 36063;
pub const GL_COLOR_ATTACHMENT1: u32 = 36065;
pub const GL_COLOR_ATTACHMENT2: u32 = 36066;
pub const GL_COLOR_ATTACHMENT3: u32 = 36067;
pub const GL_COLOR_ATTACHMENT4: u32 = 36068;
pub const GL_COLOR_ATTACHMENT5: u32 = 36069;
pub const GL_COLOR_ATTACHMENT6: u32 = 36070;
pub const GL_COLOR_ATTACHMENT7: u32 = 36071;
pub const GL_COLOR_ATTACHMENT8: u32 = 36072;
pub const GL_COLOR_ATTACHMENT9: u32 = 36073;
pub const GL_COLOR_ATTACHMENT10: u32 = 36074;
pub const GL_COLOR_ATTACHMENT11: u32 = 36075;
pub const GL_COLOR_ATTACHMENT12: u32 = 36076;
pub const GL_COLOR_ATTACHMENT13: u32 = 36077;
pub const GL_COLOR_ATTACHMENT14: u32 = 36078;
pub const GL_COLOR_ATTACHMENT15: u32 = 36079;
pub const GL_COLOR_ATTACHMENT16: u32 = 36080;
pub const GL_COLOR_ATTACHMENT17: u32 = 36081;
pub const GL_COLOR_ATTACHMENT18: u32 = 36082;
pub const GL_COLOR_ATTACHMENT19: u32 = 36083;
pub const GL_COLOR_ATTACHMENT20: u32 = 36084;
pub const GL_COLOR_ATTACHMENT21: u32 = 36085;
pub const GL_COLOR_ATTACHMENT22: u32 = 36086;
pub const GL_COLOR_ATTACHMENT23: u32 = 36087;
pub const GL_COLOR_ATTACHMENT24: u32 = 36088;
pub const GL_COLOR_ATTACHMENT25: u32 = 36089;
pub const GL_COLOR_ATTACHMENT26: u32 = 36090;
pub const GL_COLOR_ATTACHMENT27: u32 = 36091;
pub const GL_COLOR_ATTACHMENT28: u32 = 36092;
pub const GL_COLOR_ATTACHMENT29: u32 = 36093;
pub const GL_COLOR_ATTACHMENT30: u32 = 36094;
pub const GL_COLOR_ATTACHMENT31: u32 = 36095;
pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE: u32 = 36182;
pub const GL_MAX_SAMPLES: u32 = 36183;
pub const GL_HALF_FLOAT: u32 = 5131;
pub const GL_MAP_READ_BIT: u32 = 1;
pub const GL_MAP_WRITE_BIT: u32 = 2;
pub const GL_MAP_INVALIDATE_RANGE_BIT: u32 = 4;
pub const GL_MAP_INVALIDATE_BUFFER_BIT: u32 = 8;
pub const GL_MAP_FLUSH_EXPLICIT_BIT: u32 = 16;
pub const GL_MAP_UNSYNCHRONIZED_BIT: u32 = 32;
pub const GL_RG: u32 = 33319;
pub const GL_RG_INTEGER: u32 = 33320;
pub const GL_R8: u32 = 33321;
pub const GL_RG8: u32 = 33323;
pub const GL_R16F: u32 = 33325;
pub const GL_R32F: u32 = 33326;
pub const GL_RG16F: u32 = 33327;
pub const GL_RG32F: u32 = 33328;
pub const GL_R8I: u32 = 33329;
pub const GL_R8UI: u32 = 33330;
pub const GL_R16I: u32 = 33331;
pub const GL_R16UI: u32 = 33332;
pub const GL_R32I: u32 = 33333;
pub const GL_R32UI: u32 = 33334;
pub const GL_RG8I: u32 = 33335;
pub const GL_RG8UI: u32 = 33336;
pub const GL_RG16I: u32 = 33337;
pub const GL_RG16UI: u32 = 33338;
pub const GL_RG32I: u32 = 33339;
pub const GL_RG32UI: u32 = 33340;
pub const GL_VERTEX_ARRAY_BINDING: u32 = 34229;
pub const GL_R8_SNORM: u32 = 36756;
pub const GL_RG8_SNORM: u32 = 36757;
pub const GL_RGB8_SNORM: u32 = 36758;
pub const GL_RGBA8_SNORM: u32 = 36759;
pub const GL_SIGNED_NORMALIZED: u32 = 36764;
pub const GL_PRIMITIVE_RESTART_FIXED_INDEX: u32 = 36201;
pub const GL_COPY_READ_BUFFER: u32 = 36662;
pub const GL_COPY_WRITE_BUFFER: u32 = 36663;
pub const GL_COPY_READ_BUFFER_BINDING: u32 = 36662;
pub const GL_COPY_WRITE_BUFFER_BINDING: u32 = 36663;
pub const GL_UNIFORM_BUFFER: u32 = 35345;
pub const GL_UNIFORM_BUFFER_BINDING: u32 = 35368;
pub const GL_UNIFORM_BUFFER_START: u32 = 35369;
pub const GL_UNIFORM_BUFFER_SIZE: u32 = 35370;
pub const GL_MAX_VERTEX_UNIFORM_BLOCKS: u32 = 35371;
pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS: u32 = 35373;
pub const GL_MAX_COMBINED_UNIFORM_BLOCKS: u32 = 35374;
pub const GL_MAX_UNIFORM_BUFFER_BINDINGS: u32 = 35375;
pub const GL_MAX_UNIFORM_BLOCK_SIZE: u32 = 35376;
pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS: u32 = 35377;
pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS: u32 = 35379;
pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT: u32 = 35380;
pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH: u32 = 35381;
pub const GL_ACTIVE_UNIFORM_BLOCKS: u32 = 35382;
pub const GL_UNIFORM_TYPE: u32 = 35383;
pub const GL_UNIFORM_SIZE: u32 = 35384;
pub const GL_UNIFORM_NAME_LENGTH: u32 = 35385;
pub const GL_UNIFORM_BLOCK_INDEX: u32 = 35386;
pub const GL_UNIFORM_OFFSET: u32 = 35387;
pub const GL_UNIFORM_ARRAY_STRIDE: u32 = 35388;
pub const GL_UNIFORM_MATRIX_STRIDE: u32 = 35389;
pub const GL_UNIFORM_IS_ROW_MAJOR: u32 = 35390;
pub const GL_UNIFORM_BLOCK_BINDING: u32 = 35391;
pub const GL_UNIFORM_BLOCK_DATA_SIZE: u32 = 35392;
pub const GL_UNIFORM_BLOCK_NAME_LENGTH: u32 = 35393;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS: u32 = 35394;
pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES: u32 = 35395;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER: u32 = 35396;
pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER: u32 = 35398;
pub const GL_INVALID_INDEX: u32 = 4294967295;
pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS: u32 = 37154;
pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS: u32 = 37157;
pub const GL_MAX_SERVER_WAIT_TIMEOUT: u32 = 37137;
pub const GL_OBJECT_TYPE: u32 = 37138;
pub const GL_SYNC_CONDITION: u32 = 37139;
pub const GL_SYNC_STATUS: u32 = 37140;
pub const GL_SYNC_FLAGS: u32 = 37141;
pub const GL_SYNC_FENCE: u32 = 37142;
pub const GL_SYNC_GPU_COMMANDS_COMPLETE: u32 = 37143;
pub const GL_UNSIGNALED: u32 = 37144;
pub const GL_SIGNALED: u32 = 37145;
pub const GL_ALREADY_SIGNALED: u32 = 37146;
pub const GL_TIMEOUT_EXPIRED: u32 = 37147;
pub const GL_CONDITION_SATISFIED: u32 = 37148;
pub const GL_WAIT_FAILED: u32 = 37149;
pub const GL_SYNC_FLUSH_COMMANDS_BIT: u32 = 1;
pub const GL_TIMEOUT_IGNORED: i32 = -1;
pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR: u32 = 35070;
pub const GL_ANY_SAMPLES_PASSED: u32 = 35887;
pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE: u32 = 36202;
pub const GL_SAMPLER_BINDING: u32 = 35097;
pub const GL_RGB10_A2UI: u32 = 36975;
pub const GL_TEXTURE_SWIZZLE_R: u32 = 36418;
pub const GL_TEXTURE_SWIZZLE_G: u32 = 36419;
pub const GL_TEXTURE_SWIZZLE_B: u32 = 36420;
pub const GL_TEXTURE_SWIZZLE_A: u32 = 36421;
pub const GL_GREEN: u32 = 6404;
pub const GL_BLUE: u32 = 6405;
pub const GL_INT_2_10_10_10_REV: u32 = 36255;
pub const GL_TRANSFORM_FEEDBACK: u32 = 36386;
pub const GL_TRANSFORM_FEEDBACK_PAUSED: u32 = 36387;
pub const GL_TRANSFORM_FEEDBACK_ACTIVE: u32 = 36388;
pub const GL_TRANSFORM_FEEDBACK_BINDING: u32 = 36389;
pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT: u32 = 33367;
pub const GL_PROGRAM_BINARY_LENGTH: u32 = 34625;
pub const GL_NUM_PROGRAM_BINARY_FORMATS: u32 = 34814;
pub const GL_PROGRAM_BINARY_FORMATS: u32 = 34815;
pub const GL_COMPRESSED_R11_EAC: u32 = 37488;
pub const GL_COMPRESSED_SIGNED_R11_EAC: u32 = 37489;
pub const GL_COMPRESSED_RG11_EAC: u32 = 37490;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: u32 = 37491;
pub const GL_COMPRESSED_RGB8_ETC2: u32 = 37492;
pub const GL_COMPRESSED_SRGB8_ETC2: u32 = 37493;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37494;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 37495;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: u32 = 37496;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 37497;
pub const GL_TEXTURE_IMMUTABLE_FORMAT: u32 = 37167;
pub const GL_MAX_ELEMENT_INDEX: u32 = 36203;
pub const GL_NUM_SAMPLE_COUNTS: u32 = 37760;
pub const GL_TEXTURE_IMMUTABLE_LEVELS: u32 = 33503;

pub type khronos_int32_t = i32;
pub type khronos_uint32_t = u32;
pub type khronos_int64_t = i64;
pub type khronos_uint64_t = u64;
pub type khronos_int8_t = ::std::os::raw::c_schar;
pub type khronos_uint8_t = ::std::os::raw::c_uchar;
pub type khronos_int16_t = ::std::os::raw::c_short;
pub type khronos_uint16_t = ::std::os::raw::c_ushort;
pub type khronos_intptr_t = ::std::os::raw::c_long;
pub type khronos_uintptr_t = ::std::os::raw::c_ulong;
pub type khronos_ssize_t = ::std::os::raw::c_long;
pub type khronos_usize_t = ::std::os::raw::c_ulong;
pub type khronos_float_t = f32;
pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_stime_nanoseconds_t = khronos_int64_t;

pub type GLbyte = khronos_int8_t;
pub type GLclampf = khronos_float_t;
pub type GLfixed = khronos_int32_t;
pub type GLshort = ::std::os::raw::c_short;
pub type GLushort = ::std::os::raw::c_ushort;
pub type GLvoid = ::std::os::raw::c_void;

pub type GLint64 = khronos_int64_t;
pub type GLuint64 = khronos_uint64_t;
pub type GLenum = ::std::os::raw::c_uint;
pub type GLuint = ::std::os::raw::c_uint;
pub type GLchar = ::std::os::raw::c_char;
pub type GLfloat = khronos_float_t;
pub type GLsizeiptr = khronos_ssize_t;
pub type GLintptr = khronos_intptr_t;
pub type GLbitfield = ::std::os::raw::c_uint;
pub type GLint = ::std::os::raw::c_int;
pub type GLboolean = ::std::os::raw::c_uchar;
pub type GLsizei = ::std::os::raw::c_int;
pub type GLubyte = khronos_uint8_t;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __GLsync {
    _unused: [u8; 0],
}
pub type GLsync = *mut __GLsync;

extern "C" {
    pub fn glActiveTexture(texture: GLenum);
}
extern "C" {
    pub fn glAttachShader(program: GLuint, shader: GLuint);
}
extern "C" {
    pub fn glBindAttribLocation(program: GLuint, index: GLuint, name: *const GLchar);
}
extern "C" {
    pub fn glBindBuffer(target: GLenum, buffer: GLuint);
}
extern "C" {
    pub fn glBindFramebuffer(target: GLenum, framebuffer: GLuint);
}
extern "C" {
    pub fn glBindRenderbuffer(target: GLenum, renderbuffer: GLuint);
}
extern "C" {
    pub fn glBindTexture(target: GLenum, texture: GLuint);
}
extern "C" {
    pub fn glBlendColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}
extern "C" {
    pub fn glBlendEquation(mode: GLenum);
}
extern "C" {
    pub fn glBlendEquationSeparate(modeRGB: GLenum, modeAlpha: GLenum);
}
extern "C" {
    pub fn glBlendFunc(sfactor: GLenum, dfactor: GLenum);
}
extern "C" {
    pub fn glBlendFuncSeparate(
        sfactorRGB: GLenum,
        dfactorRGB: GLenum,
        sfactorAlpha: GLenum,
        dfactorAlpha: GLenum,
    );
}
extern "C" {
    pub fn glBufferData(
        target: GLenum,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void,
        usage: GLenum,
    );
}
extern "C" {
    pub fn glBufferSubData(
        target: GLenum,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glCheckFramebufferStatus(target: GLenum) -> GLenum;
}
extern "C" {
    pub fn glClear(mask: GLbitfield);
}
extern "C" {
    pub fn glClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat);
}
extern "C" {
    pub fn glClearDepthf(d: GLfloat);
}
extern "C" {
    pub fn glClearStencil(s: GLint);
}
extern "C" {
    pub fn glColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean);
}
extern "C" {
    pub fn glCompileShader(shader: GLuint);
}
extern "C" {
    pub fn glCompressedTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glCompressedTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glCopyTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
    );
}
extern "C" {
    pub fn glCopyTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glCreateProgram() -> GLuint;
}
extern "C" {
    pub fn glCreateShader(type_: GLenum) -> GLuint;
}
extern "C" {
    pub fn glCullFace(mode: GLenum);
}
extern "C" {
    pub fn glDeleteBuffers(n: GLsizei, buffers: *const GLuint);
}
extern "C" {
    pub fn glDeleteFramebuffers(n: GLsizei, framebuffers: *const GLuint);
}
extern "C" {
    pub fn glDeleteProgram(program: GLuint);
}
extern "C" {
    pub fn glDeleteRenderbuffers(n: GLsizei, renderbuffers: *const GLuint);
}
extern "C" {
    pub fn glDeleteShader(shader: GLuint);
}
extern "C" {
    pub fn glDeleteTextures(n: GLsizei, textures: *const GLuint);
}
extern "C" {
    pub fn glDepthFunc(func: GLenum);
}
extern "C" {
    pub fn glDepthMask(flag: GLboolean);
}
extern "C" {
    pub fn glDepthRangef(n: GLfloat, f: GLfloat);
}
extern "C" {
    pub fn glDetachShader(program: GLuint, shader: GLuint);
}
extern "C" {
    pub fn glDisable(cap: GLenum);
}
extern "C" {
    pub fn glDisableVertexAttribArray(index: GLuint);
}
extern "C" {
    pub fn glDrawArrays(mode: GLenum, first: GLint, count: GLsizei);
}
extern "C" {
    pub fn glDrawElements(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glEnable(cap: GLenum);
}
extern "C" {
    pub fn glEnableVertexAttribArray(index: GLuint);
}
extern "C" {
    pub fn glFinish();
}
extern "C" {
    pub fn glFlush();
}
extern "C" {
    pub fn glFramebufferRenderbuffer(
        target: GLenum,
        attachment: GLenum,
        renderbuffertarget: GLenum,
        renderbuffer: GLuint,
    );
}
extern "C" {
    pub fn glFramebufferTexture2D(
        target: GLenum,
        attachment: GLenum,
        textarget: GLenum,
        texture: GLuint,
        level: GLint,
    );
}
extern "C" {
    pub fn glFrontFace(mode: GLenum);
}
extern "C" {
    pub fn glGenBuffers(n: GLsizei, buffers: *mut GLuint);
}
extern "C" {
    pub fn glGenerateMipmap(target: GLenum);
}
extern "C" {
    pub fn glGenFramebuffers(n: GLsizei, framebuffers: *mut GLuint);
}
extern "C" {
    pub fn glGenRenderbuffers(n: GLsizei, renderbuffers: *mut GLuint);
}
extern "C" {
    pub fn glGenTextures(n: GLsizei, textures: *mut GLuint);
}
extern "C" {
    pub fn glGetActiveAttrib(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
extern "C" {
    pub fn glGetActiveUniform(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLint,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
extern "C" {
    pub fn glGetAttachedShaders(
        program: GLuint,
        maxCount: GLsizei,
        count: *mut GLsizei,
        shaders: *mut GLuint,
    );
}
extern "C" {
    pub fn glGetAttribLocation(program: GLuint, name: *const GLchar) -> GLint;
}
extern "C" {
    pub fn glGetBooleanv(pname: GLenum, data: *mut GLboolean);
}
extern "C" {
    pub fn glGetBufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetError() -> GLenum;
}
extern "C" {
    pub fn glGetFloatv(pname: GLenum, data: *mut GLfloat);
}
extern "C" {
    pub fn glGetFramebufferAttachmentParameteriv(
        target: GLenum,
        attachment: GLenum,
        pname: GLenum,
        params: *mut GLint,
    );
}
extern "C" {
    pub fn glGetIntegerv(pname: GLenum, data: *mut GLint);
}
extern "C" {
    pub fn glGetProgramiv(program: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetProgramInfoLog(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
}
extern "C" {
    pub fn glGetRenderbufferParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetShaderiv(shader: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetShaderInfoLog(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        infoLog: *mut GLchar,
    );
}
extern "C" {
    pub fn glGetShaderPrecisionFormat(
        shadertype: GLenum,
        precisiontype: GLenum,
        range: *mut GLint,
        precision: *mut GLint,
    );
}
extern "C" {
    pub fn glGetShaderSource(
        shader: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        source: *mut GLchar,
    );
}
extern "C" {
    pub fn glGetString(name: GLenum) -> *const GLubyte;
}
extern "C" {
    pub fn glGetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat);
}
extern "C" {
    pub fn glGetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetUniformfv(program: GLuint, location: GLint, params: *mut GLfloat);
}
extern "C" {
    pub fn glGetUniformiv(program: GLuint, location: GLint, params: *mut GLint);
}
extern "C" {
    pub fn glGetUniformLocation(program: GLuint, name: *const GLchar) -> GLint;
}
extern "C" {
    pub fn glGetVertexAttribfv(index: GLuint, pname: GLenum, params: *mut GLfloat);
}
extern "C" {
    pub fn glGetVertexAttribiv(index: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetVertexAttribPointerv(
        index: GLuint,
        pname: GLenum,
        pointer: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glHint(target: GLenum, mode: GLenum);
}
extern "C" {
    pub fn glIsBuffer(buffer: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glIsEnabled(cap: GLenum) -> GLboolean;
}
extern "C" {
    pub fn glIsFramebuffer(framebuffer: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glIsProgram(program: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glIsRenderbuffer(renderbuffer: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glIsShader(shader: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glIsTexture(texture: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glLineWidth(width: GLfloat);
}
extern "C" {
    pub fn glLinkProgram(program: GLuint);
}
extern "C" {
    pub fn glPixelStorei(pname: GLenum, param: GLint);
}
extern "C" {
    pub fn glPolygonOffset(factor: GLfloat, units: GLfloat);
}
extern "C" {
    pub fn glReadPixels(
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glReleaseShaderCompiler();
}
extern "C" {
    pub fn glRenderbufferStorage(
        target: GLenum,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glSampleCoverage(value: GLfloat, invert: GLboolean);
}
extern "C" {
    pub fn glScissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
extern "C" {
    pub fn glShaderBinary(
        count: GLsizei,
        shaders: *const GLuint,
        binaryformat: GLenum,
        binary: *const ::std::os::raw::c_void,
        length: GLsizei,
    );
}
extern "C" {
    pub fn glShaderSource(
        shader: GLuint,
        count: GLsizei,
        string: *const *const GLchar,
        length: *const GLint,
    );
}
extern "C" {
    pub fn glStencilFunc(func: GLenum, ref_: GLint, mask: GLuint);
}
extern "C" {
    pub fn glStencilFuncSeparate(face: GLenum, func: GLenum, ref_: GLint, mask: GLuint);
}
extern "C" {
    pub fn glStencilMask(mask: GLuint);
}
extern "C" {
    pub fn glStencilMaskSeparate(face: GLenum, mask: GLuint);
}
extern "C" {
    pub fn glStencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum);
}
extern "C" {
    pub fn glStencilOpSeparate(face: GLenum, sfail: GLenum, dpfail: GLenum, dppass: GLenum);
}
extern "C" {
    pub fn glTexImage2D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glTexParameterf(target: GLenum, pname: GLenum, param: GLfloat);
}
extern "C" {
    pub fn glTexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat);
}
extern "C" {
    pub fn glTexParameteri(target: GLenum, pname: GLenum, param: GLint);
}
extern "C" {
    pub fn glTexParameteriv(target: GLenum, pname: GLenum, params: *const GLint);
}
extern "C" {
    pub fn glTexSubImage2D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glUniform1f(location: GLint, v0: GLfloat);
}
extern "C" {
    pub fn glUniform1fv(location: GLint, count: GLsizei, value: *const GLfloat);
}
extern "C" {
    pub fn glUniform1i(location: GLint, v0: GLint);
}
extern "C" {
    pub fn glUniform1iv(location: GLint, count: GLsizei, value: *const GLint);
}
extern "C" {
    pub fn glUniform2f(location: GLint, v0: GLfloat, v1: GLfloat);
}
extern "C" {
    pub fn glUniform2fv(location: GLint, count: GLsizei, value: *const GLfloat);
}
extern "C" {
    pub fn glUniform2i(location: GLint, v0: GLint, v1: GLint);
}
extern "C" {
    pub fn glUniform2iv(location: GLint, count: GLsizei, value: *const GLint);
}
extern "C" {
    pub fn glUniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat);
}
extern "C" {
    pub fn glUniform3fv(location: GLint, count: GLsizei, value: *const GLfloat);
}
extern "C" {
    pub fn glUniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint);
}
extern "C" {
    pub fn glUniform3iv(location: GLint, count: GLsizei, value: *const GLint);
}
extern "C" {
    pub fn glUniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat);
}
extern "C" {
    pub fn glUniform4fv(location: GLint, count: GLsizei, value: *const GLfloat);
}
extern "C" {
    pub fn glUniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint);
}
extern "C" {
    pub fn glUniform4iv(location: GLint, count: GLsizei, value: *const GLint);
}
extern "C" {
    pub fn glUniformMatrix2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUseProgram(program: GLuint);
}
extern "C" {
    pub fn glValidateProgram(program: GLuint);
}
extern "C" {
    pub fn glVertexAttrib1f(index: GLuint, x: GLfloat);
}
extern "C" {
    pub fn glVertexAttrib1fv(index: GLuint, v: *const GLfloat);
}
extern "C" {
    pub fn glVertexAttrib2f(index: GLuint, x: GLfloat, y: GLfloat);
}
extern "C" {
    pub fn glVertexAttrib2fv(index: GLuint, v: *const GLfloat);
}
extern "C" {
    pub fn glVertexAttrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat);
}
extern "C" {
    pub fn glVertexAttrib3fv(index: GLuint, v: *const GLfloat);
}
extern "C" {
    pub fn glVertexAttrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat);
}
extern "C" {
    pub fn glVertexAttrib4fv(index: GLuint, v: *const GLfloat);
}
extern "C" {
    pub fn glVertexAttribPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glViewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei);
}
extern "C" {
    pub fn glReadBuffer(src: GLenum);
}
extern "C" {
    pub fn glDrawRangeElements(
        mode: GLenum,
        start: GLuint,
        end: GLuint,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        type_: GLenum,
        pixels: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glCopyTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glCompressedTexImage3D(
        target: GLenum,
        level: GLint,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        border: GLint,
        imageSize: GLsizei,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glCompressedTexSubImage3D(
        target: GLenum,
        level: GLint,
        xoffset: GLint,
        yoffset: GLint,
        zoffset: GLint,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
        format: GLenum,
        imageSize: GLsizei,
        data: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glGenQueries(n: GLsizei, ids: *mut GLuint);
}
extern "C" {
    pub fn glDeleteQueries(n: GLsizei, ids: *const GLuint);
}
extern "C" {
    pub fn glIsQuery(id: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glBeginQuery(target: GLenum, id: GLuint);
}
extern "C" {
    pub fn glEndQuery(target: GLenum);
}
extern "C" {
    pub fn glGetQueryiv(target: GLenum, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glQueryCounter(id: GLenum, pname: GLenum);
}
extern "C" {
    pub fn glGetQueryObjectiv(id: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetQueryObjectui64v(id: GLuint, pname: GLenum, params: *mut GLuint64);
}
extern "C" {
    pub fn glUnmapBuffer(target: GLenum) -> GLboolean;
}
extern "C" {
    pub fn glGetBufferPointerv(
        target: GLenum,
        pname: GLenum,
        params: *mut *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glDrawBuffers(n: GLsizei, bufs: *const GLenum);
}
extern "C" {
    pub fn glUniformMatrix2x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix3x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix2x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix4x2fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix3x4fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glUniformMatrix4x3fv(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        value: *const GLfloat,
    );
}
extern "C" {
    pub fn glBlitFramebuffer(
        srcX0: GLint,
        srcY0: GLint,
        srcX1: GLint,
        srcY1: GLint,
        dstX0: GLint,
        dstY0: GLint,
        dstX1: GLint,
        dstY1: GLint,
        mask: GLbitfield,
        filter: GLenum,
    );
}
extern "C" {
    pub fn glRenderbufferStorageMultisample(
        target: GLenum,
        samples: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glFramebufferTextureLayer(
        target: GLenum,
        attachment: GLenum,
        texture: GLuint,
        level: GLint,
        layer: GLint,
    );
}
extern "C" {
    pub fn glMapBufferRange(
        target: GLenum,
        offset: GLintptr,
        length: GLsizeiptr,
        access: GLbitfield,
    ) -> *mut ::std::os::raw::c_void;
}
extern "C" {
    pub fn glFlushMappedBufferRange(target: GLenum, offset: GLintptr, length: GLsizeiptr);
}
extern "C" {
    pub fn glBindVertexArray(array: GLuint);
}
extern "C" {
    pub fn glDeleteVertexArrays(n: GLsizei, arrays: *const GLuint);
}
extern "C" {
    pub fn glGenVertexArrays(n: GLsizei, arrays: *mut GLuint);
}
extern "C" {
    pub fn glIsVertexArray(array: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glGetIntegeri_v(target: GLenum, index: GLuint, data: *mut GLint);
}
extern "C" {
    pub fn glBeginTransformFeedback(primitiveMode: GLenum);
}
extern "C" {
    pub fn glEndTransformFeedback();
}
extern "C" {
    pub fn glBindBufferRange(
        target: GLenum,
        index: GLuint,
        buffer: GLuint,
        offset: GLintptr,
        size: GLsizeiptr,
    );
}
extern "C" {
    pub fn glBindBufferBase(target: GLenum, index: GLuint, buffer: GLuint);
}
extern "C" {
    pub fn glTransformFeedbackVaryings(
        program: GLuint,
        count: GLsizei,
        varyings: *const *const GLchar,
        bufferMode: GLenum,
    );
}
extern "C" {
    pub fn glGetTransformFeedbackVarying(
        program: GLuint,
        index: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        size: *mut GLsizei,
        type_: *mut GLenum,
        name: *mut GLchar,
    );
}
extern "C" {
    pub fn glVertexAttribIPointer(
        index: GLuint,
        size: GLint,
        type_: GLenum,
        stride: GLsizei,
        pointer: *const ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glGetVertexAttribIiv(index: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetVertexAttribIuiv(index: GLuint, pname: GLenum, params: *mut GLuint);
}
extern "C" {
    pub fn glVertexAttribI4i(index: GLuint, x: GLint, y: GLint, z: GLint, w: GLint);
}
extern "C" {
    pub fn glVertexAttribI4ui(index: GLuint, x: GLuint, y: GLuint, z: GLuint, w: GLuint);
}
extern "C" {
    pub fn glVertexAttribI4iv(index: GLuint, v: *const GLint);
}
extern "C" {
    pub fn glVertexAttribI4uiv(index: GLuint, v: *const GLuint);
}
extern "C" {
    pub fn glGetUniformuiv(program: GLuint, location: GLint, params: *mut GLuint);
}
extern "C" {
    pub fn glGetFragDataLocation(program: GLuint, name: *const GLchar) -> GLint;
}
extern "C" {
    pub fn glUniform1ui(location: GLint, v0: GLuint);
}
extern "C" {
    pub fn glUniform2ui(location: GLint, v0: GLuint, v1: GLuint);
}
extern "C" {
    pub fn glUniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint);
}
extern "C" {
    pub fn glUniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint);
}
extern "C" {
    pub fn glUniform1uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
extern "C" {
    pub fn glUniform2uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
extern "C" {
    pub fn glUniform3uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
extern "C" {
    pub fn glUniform4uiv(location: GLint, count: GLsizei, value: *const GLuint);
}
extern "C" {
    pub fn glClearBufferiv(buffer: GLenum, drawbuffer: GLint, value: *const GLint);
}
extern "C" {
    pub fn glClearBufferuiv(buffer: GLenum, drawbuffer: GLint, value: *const GLuint);
}
extern "C" {
    pub fn glClearBufferfv(buffer: GLenum, drawbuffer: GLint, value: *const GLfloat);
}
extern "C" {
    pub fn glClearBufferfi(buffer: GLenum, drawbuffer: GLint, depth: GLfloat, stencil: GLint);
}
extern "C" {
    pub fn glGetStringi(name: GLenum, index: GLuint) -> *const GLubyte;
}
extern "C" {
    pub fn glCopyBufferSubData(
        readTarget: GLenum,
        writeTarget: GLenum,
        readOffset: GLintptr,
        writeOffset: GLintptr,
        size: GLsizeiptr,
    );
}
extern "C" {
    pub fn glGetUniformIndices(
        program: GLuint,
        uniformCount: GLsizei,
        uniformNames: *const *const GLchar,
        uniformIndices: *mut GLuint,
    );
}
extern "C" {
    pub fn glGetActiveUniformsiv(
        program: GLuint,
        uniformCount: GLsizei,
        uniformIndices: *const GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
}
extern "C" {
    pub fn glGetUniformBlockIndex(program: GLuint, uniformBlockName: *const GLchar) -> GLuint;
}
extern "C" {
    pub fn glGetActiveUniformBlockiv(
        program: GLuint,
        uniformBlockIndex: GLuint,
        pname: GLenum,
        params: *mut GLint,
    );
}
extern "C" {
    pub fn glGetActiveUniformBlockName(
        program: GLuint,
        uniformBlockIndex: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        uniformBlockName: *mut GLchar,
    );
}
extern "C" {
    pub fn glUniformBlockBinding(
        program: GLuint,
        uniformBlockIndex: GLuint,
        uniformBlockBinding: GLuint,
    );
}
extern "C" {
    pub fn glDrawArraysInstanced(
        mode: GLenum,
        first: GLint,
        count: GLsizei,
        instancecount: GLsizei,
    );
}
extern "C" {
    pub fn glDrawElementsInstanced(
        mode: GLenum,
        count: GLsizei,
        type_: GLenum,
        indices: *const ::std::os::raw::c_void,
        instancecount: GLsizei,
    );
}
extern "C" {
    pub fn glFenceSync(condition: GLenum, flags: GLbitfield) -> GLsync;
}
extern "C" {
    pub fn glIsSync(sync: GLsync) -> GLboolean;
}
extern "C" {
    pub fn glDeleteSync(sync: GLsync);
}
extern "C" {
    pub fn glClientWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64) -> GLenum;
}
extern "C" {
    pub fn glWaitSync(sync: GLsync, flags: GLbitfield, timeout: GLuint64);
}
extern "C" {
    pub fn glGetInteger64v(pname: GLenum, data: *mut GLint64);
}
extern "C" {
    pub fn glGetSynciv(
        sync: GLsync,
        pname: GLenum,
        bufSize: GLsizei,
        length: *mut GLsizei,
        values: *mut GLint,
    );
}
extern "C" {
    pub fn glGetInteger64i_v(target: GLenum, index: GLuint, data: *mut GLint64);
}
extern "C" {
    pub fn glGetBufferParameteri64v(target: GLenum, pname: GLenum, params: *mut GLint64);
}
extern "C" {
    pub fn glGenSamplers(count: GLsizei, samplers: *mut GLuint);
}
extern "C" {
    pub fn glDeleteSamplers(count: GLsizei, samplers: *const GLuint);
}
extern "C" {
    pub fn glIsSampler(sampler: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glBindSampler(unit: GLuint, sampler: GLuint);
}
extern "C" {
    pub fn glSamplerParameteri(sampler: GLuint, pname: GLenum, param: GLint);
}
extern "C" {
    pub fn glSamplerParameteriv(sampler: GLuint, pname: GLenum, param: *const GLint);
}
extern "C" {
    pub fn glSamplerParameterf(sampler: GLuint, pname: GLenum, param: GLfloat);
}
extern "C" {
    pub fn glSamplerParameterfv(sampler: GLuint, pname: GLenum, param: *const GLfloat);
}
extern "C" {
    pub fn glGetSamplerParameteriv(sampler: GLuint, pname: GLenum, params: *mut GLint);
}
extern "C" {
    pub fn glGetSamplerParameterfv(sampler: GLuint, pname: GLenum, params: *mut GLfloat);
}
extern "C" {
    pub fn glVertexAttribDivisor(index: GLuint, divisor: GLuint);
}
extern "C" {
    pub fn glBindTransformFeedback(target: GLenum, id: GLuint);
}
extern "C" {
    pub fn glDeleteTransformFeedbacks(n: GLsizei, ids: *const GLuint);
}
extern "C" {
    pub fn glGenTransformFeedbacks(n: GLsizei, ids: *mut GLuint);
}
extern "C" {
    pub fn glIsTransformFeedback(id: GLuint) -> GLboolean;
}
extern "C" {
    pub fn glPauseTransformFeedback();
}
extern "C" {
    pub fn glResumeTransformFeedback();
}
extern "C" {
    pub fn glGetProgramBinary(
        program: GLuint,
        bufSize: GLsizei,
        length: *mut GLsizei,
        binaryFormat: *mut GLenum,
        binary: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn glProgramBinary(
        program: GLuint,
        binaryFormat: GLenum,
        binary: *const ::std::os::raw::c_void,
        length: GLsizei,
    );
}
extern "C" {
    pub fn glProgramParameteri(program: GLuint, pname: GLenum, value: GLint);
}
extern "C" {
    pub fn glInvalidateFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
    );
}
extern "C" {
    pub fn glInvalidateSubFramebuffer(
        target: GLenum,
        numAttachments: GLsizei,
        attachments: *const GLenum,
        x: GLint,
        y: GLint,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glTexStorage2D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
    );
}
extern "C" {
    pub fn glTexStorage3D(
        target: GLenum,
        levels: GLsizei,
        internalformat: GLenum,
        width: GLsizei,
        height: GLsizei,
        depth: GLsizei,
    );
}
extern "C" {
    pub fn glGetInternalformativ(
        target: GLenum,
        internalformat: GLenum,
        pname: GLenum,
        bufSize: GLsizei,
        params: *mut GLint,
    );
}
