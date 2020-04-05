#[cfg(test)]
mod field;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::raycast::{raycast, RaycastParams};
    use crate::world::coord::WorldCoord;
    use glm::*;

    #[allow(dead_code)]
    fn generate_test_coord_vec(points: Vec<[i64;3]>) -> Vec<WorldCoord> {
        //points.iter().map(|p|{ WorldCoord::new(p[0], p[1], p[2])}).collect()
        let mut crds = Vec::new();
        for point in points {
            crds.push(WorldCoord::new(point[0], point[1], point[2]));
        }
        return crds
    }

    #[test]
    fn test_simple_raycasts() {
        assert_eq!(
            generate_test_coord_vec(vec![[0,0,0], [1,0,0], [2,0,0]]),
            raycast(RaycastParams {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(1.0, 0.0, 0.0),
                len: 2.5f32,
                include_first: true,
            })
        );

        assert_eq!(
            generate_test_coord_vec(vec![[0,0,0], [0,1,0], [0,2,0]]),
            raycast(RaycastParams {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(0.0, 1.0, 0.0),
                len: 2.5f32,
                include_first: true,
            })
        );

        // negative x
        assert_eq!(
            generate_test_coord_vec(vec![[0,0,0], [-1,0,0], [-2,0,0]]),
            raycast(RaycastParams {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(-1.0, 0.0, 0.0),
                len: 2.5f32,
                include_first: true,
            })
        );
    }

    #[test]
    fn test_angled_raycasts() {
        // 40 degrees
        assert_eq!(
            generate_test_coord_vec(vec![[0,0,0], [1,0,0], [1,0,1], [2,0,1]]),
            raycast(RaycastParams {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(0.766044, 0.0, 0.642788),
                len: 3f32,
                include_first: true,
            })
        );
    }

    #[test]
    fn test_corner_case_raycasts() {
        // 30 degrees - corner case because jumps two blocks at once
        assert_eq!(
            generate_test_coord_vec(vec![[0,0,0], [1,0,0], [1,0,1], [2,0,1], [3,0,1]]),
            raycast(RaycastParams {
                pos: Vec3::new(0.0, 0.0, 0.0),
                dir: Vec3::new(0.8660254, 0.0, 0.5),
                len: 4f32,
                include_first: true,
            })
        );
    }
}