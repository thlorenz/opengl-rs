#[cfg(test)]
mod tests {
    extern crate nalgebra_glm as glm;

    #[test]
    fn orthographic() {
        let origin = (0.0, 0.0);
        let size = (800.0, 600.0);
        let planes = (0.1, 100.0);
        let mat = glm::ortho(origin.0, size.0, origin.1, size.1, planes.0, planes.1);
        eprintln!(
            "origin: {:?}, size: {:?}, planes: {:?} {}",
            origin, size, planes, mat
        );
        let origin = (100.0, 0.0);
        let mat = glm::ortho(origin.0, size.0, origin.1, size.1, planes.0, planes.1);
        eprintln!(
            "origin: {:?}, size: {:?}, planes: {:?}{}",
            origin, size, planes, mat
        );

        let origin = (0.0, 0.0);
        let planes = (50.0, 100.0);
        let mat = glm::ortho(origin.0, size.0, origin.1, size.1, planes.0, planes.1);
        eprintln!(
            "origin: {:?}, size: {:?}, planes: {:?} {}",
            origin, size, planes, mat
        );
    }

    #[test]
    fn perspective() {
        let size = (800.0, 600.0);
        let degrees = 45.0_f32;
        let planes = (0.1, 100.0);
        let mat = glm::perspective(size.0 / size.1, degrees.to_radians(), planes.0, planes.1);
        eprintln!(
            "angle: {}, size: {:?}, planes: {:?} {}",
            degrees, size, planes, mat
        );
        let degrees = 90.0_f32;
        let mat = glm::perspective(size.0 / size.1, degrees.to_radians(), planes.0, planes.1);
        eprintln!(
            "angle: {}, size: {:?}, planes: {:?} {}",
            degrees, size, planes, mat
        );
    }
}
