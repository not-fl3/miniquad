use miniquad_derive::VertexLayout;

#[test]
fn test() {
    // `#[derive(VertexLayout)]` panics if we miss `#[repr(C)]`
    #[repr(C)]
    #[derive(Debug, Clone, PartialEq, VertexLayout)]
    pub struct Vertex {
        pos: [f32; 2],
        color: [u8; 4],
    }

    assert_eq!(
        Vertex::VERTEX_ATTRIBUTES,
        &[
            miniquad::graphics::VertexAttribute::new(
                "pos",
                miniquad::graphics::VertexFormat::Float2,
            ),
            miniquad::graphics::VertexAttribute::new(
                "color",
                miniquad::graphics::VertexFormat::Byte4,
            ),
        ]
    );
}
