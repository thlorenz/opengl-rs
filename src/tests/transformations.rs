#[cfg(test)]
mod tests {
    extern crate nalgebra_glm as glm;

    #[test]
    fn translate() {
        let vec = glm::vec4(1.0, 0.0, 0.0, 1.0);
        let dvec = &glm::vec3(1.0, 1.0, 0.0);
        let mut mat: glm::Mat4 = glm::Mat4::identity();
        mat = glm::translate(&mat, &dvec);
        let tvec = mat * vec;
        eprintln!(
            "+++ Translation +++\nvec: {} mut: {} matrix: {} result: {}",
            vec, dvec, mat, tvec
        )
    }

    #[test]
    fn rotate() {
        let vec = glm::vec4(1.0, 0.0, 0.0, 1.0);
        let dvec = &glm::vec3(0.0, 0.0, 1.0);
        let mut trans: glm::Mat4 = glm::Mat4::identity();
        let rad = 90.0_f32.to_radians();
        trans = glm::rotate(&trans, rad, &dvec);
        let tvec = trans * vec;
        eprintln!(
            "+++ Rotation(90 degrees) +++\nvec: {} mut: {} matrix: {} result: {}",
            vec, dvec, trans, tvec
        )
    }

    #[test]
    fn scale() {
        let vec = glm::vec4(1.0, 0.0, 0.0, 1.0);
        let dvec = &glm::vec3(0.5, 0.5, 0.5);
        let mut trans: glm::Mat4 = glm::Mat4::identity();
        trans = glm::scale(&trans, &dvec);
        let tvec = trans * vec;
        eprintln!(
            "+++ Scaling +++\nvec: {} mut: {} matrix: {} result: {}",
            vec, dvec, trans, tvec
        )
    }

    #[test]
    fn scale_and_rotate() {
        let vec = glm::vec4(1.0, 0.0, 0.0, 1.0);
        let rot_vec = glm::vec3(0.0, 0.0, 1.0);
        let scale_vec = &glm::vec3(0.5, 0.5, 0.5);
        let mut trans: glm::Mat4 = glm::Mat4::identity();
        let rad = 90.0_f32.to_radians();
        trans = glm::rotate(&trans, rad, &rot_vec);
        trans = glm::scale(&trans, &scale_vec);
        let tvec = trans * vec;
        eprintln!(
            "+++ Scale then Rotate (90 degrees) +++\nvec: {} scale: {} rot: {} matrix: {} result: {}",
            vec, scale_vec, rot_vec, trans, tvec
        )
    }
}
