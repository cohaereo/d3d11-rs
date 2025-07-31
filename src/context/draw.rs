use super::DeviceContext;

impl DeviceContext {
    pub fn draw(&self, vertex_count: u32, start_vertex_location: u32) {
        unsafe {
            self.0.Draw(vertex_count, start_vertex_location);
        }
    }

    pub fn draw_auto(&self) {
        unsafe {
            self.0.DrawAuto();
        }
    }

    pub fn draw_indexed(
        &self,
        index_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
    ) {
        unsafe {
            self.0
                .DrawIndexed(index_count, start_index_location, base_vertex_location);
        }
    }

    pub fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawIndexedInstanced(
                index_count_per_instance,
                instance_count,
                start_index_location,
                base_vertex_location,
                start_instance_location,
            );
        }
    }

    // pub fn draw_indexed_instanced_indirect(
    //     &self,
    //     buffer_for_args: &ID3D11Buffer,
    //     aligned_byte_offset_for_args: u32,
    // ) {
    //     unsafe {
    //         self.0
    //             .DrawIndexedInstancedIndirect(buffer_for_args, aligned_byte_offset_for_args);
    //     }
    // }

    pub fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    ) {
        unsafe {
            self.0.DrawInstanced(
                vertex_count_per_instance,
                instance_count,
                start_vertex_location,
                start_instance_location,
            );
        }
    }

    // pub fn draw_instanced_indirect(
    //     &self,
    //     buffer_for_args: &ID3D11Buffer,
    //     aligned_byte_offset_for_args: u32,
    // ) {
    //     unsafe {
    //         self.0
    //             .DrawInstancedIndirect(buffer_for_args, aligned_byte_offset_for_args);
    //     }
    // }
}
