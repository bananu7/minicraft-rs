
        let vertex_list = [
            // Ground plane
            Vertex { position: [00.0, 0.0,  0.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [00.0, 0.0, 10.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [10.0, 0.0, 10.0], color: [0.3, 0.3, 0.3] },
            Vertex { position: [10.0, 0.0,  0.0], color: [0.3, 0.3, 0.3] },
        ];

        let vertex_buffer = {
            glium::VertexBuffer::new(display, &vertex_list).unwrap()
        };

        let index_buffer = glium::IndexBuffer::new(
            display,
            PrimitiveType::TrianglesList,
            &[
              0u16, 1, 2, 0, 2, 3,
              4, 5, 6, 4, 6, 7,
              8, 9,10, 8,10,11,
              12,13,14,12,14,15,
              16,17,18,16,18,19,
              20,21,22,20,22,23,
            ]
        ).unwrap();
