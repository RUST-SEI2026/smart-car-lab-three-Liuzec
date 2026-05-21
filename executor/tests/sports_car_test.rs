use executor::{Pose, SportsCarExecutor as Executor};

mod move_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_command_is_m_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("M");
        assert_eq!(Pose::new(2, 0, 'E'), executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_command_is_bm_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("BM");
        assert_eq!(Pose::new(-2, 0, 'E'), executor.query());
    }

    #[test]
    fn should_return_x_plus_4_given_command_is_fm_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FM");
        assert_eq!(Pose::new(4, 0, 'E'), executor.query());
    }

    #[test]
    fn should_return_x_minus_4_given_command_is_fbm_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FBM");
        assert_eq!(Pose::new(-4, 0, 'E'), executor.query());
    }
}

mod turn_left_tests {
    use super::*;

    #[test]
    fn should_return_y_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("L");
        assert_eq!(Pose::new(0, 1, 'N'), executor.query());
    }

    #[test]
    fn should_return_y_plus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("BL");
        assert_eq!(Pose::new(0, 1, 'S'), executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_plus_1_and_facing_n_given_command_is_fl_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FL");
        assert_eq!(Pose::new(1, 1, 'N'), executor.query());
    }

    #[test]
    fn should_return_x_minus_1_y_plus_1_and_facing_s_given_command_is_fbl_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FBL");
        assert_eq!(Pose::new(-1, 1, 'S'), executor.query());
    }
}

mod turn_right_tests {
    use super::*;

    #[test]
    fn should_return_y_minus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("R");
        assert_eq!(Pose::new(0, -1, 'S'), executor.query());
    }

    #[test]
    fn should_return_y_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("BR");
        assert_eq!(Pose::new(0, -1, 'N'), executor.query());
    }

    #[test]
    fn should_return_x_plus_1_y_minus_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FR");
        assert_eq!(Pose::new(1, -1, 'S'), executor.query());
    }

    #[test]
    fn should_return_x_minus_1_y_minus_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);
        executor.execute("FBR");
        assert_eq!(Pose::new(-1, -1, 'N'), executor.query());
    }
}
