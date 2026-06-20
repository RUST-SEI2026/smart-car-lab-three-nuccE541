use executor::{Pose, BusExecutor as Executor};
#[test]
fn should_return_x_plus_1_and_facing_n_given_command_is_l_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("L");

    // then
    let expected_pose = Pose::new(1, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_and_facing_s_given_command_is_bl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BL");

    // then
    let expected_pose = Pose::new(-1, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_2_and_facing_n_given_command_is_fl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FL");

    // then
    let expected_pose = Pose::new(2, 0, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_2_and_facing_s_given_command_is_fbl_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBL");

    // then
    let expected_pose = Pose::new(-2, 0, 'S');
    assert_eq!(expected_pose, executor.query());
}
