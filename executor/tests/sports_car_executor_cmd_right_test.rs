use executor::{Pose, SportsCarExecutor as Executor};
#[test]
fn should_return_y_minus_1_and_facing_s_given_command_is_r_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("R");

    // then
    let expected_pose = Pose::new(0, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_y_minus_1_and_facing_n_given_command_is_br_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("BR");

    // then
    let expected_pose = Pose::new(0, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_plus_1_y_minus_1_and_facing_s_given_command_is_fr_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FR");

    // then
    let expected_pose = Pose::new(1, -1, 'S');
    assert_eq!(expected_pose, executor.query());
}

#[test]
fn should_return_x_minus_1_y_minus_1_and_facing_n_given_command_is_fbr_and_facing_is_e() {
    // given
    let original_pose = Pose::new(0, 0, 'E');
    let mut executor = Executor::with_pose(original_pose);

    // when
    executor.execute("FBR");

    // then
    let expected_pose = Pose::new(-1, -1, 'N');
    assert_eq!(expected_pose, executor.query());
}