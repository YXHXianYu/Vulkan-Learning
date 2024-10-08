
pub mod mandelbrot_shader {
    vulkano_shaders::shader!{
        ty: "compute",
        path: "assets/shaders/mandelbrot.comp"
    }
}

pub mod ray_tracer_shader {
    vulkano_shaders::shader!{
        ty: "compute",
        path: "assets/shaders/ray_tracer_games101_branch.comp"
    }
}

// pub mod test_shader {
//     vulkano_shaders::shader!{
//         ty: "compute",
//         src: r"
//         "
//     }
// }